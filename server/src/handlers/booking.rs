use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;

#[derive(FromRow)]
struct JsonResult {
    result: Option<Value>,
}

#[derive(FromRow)]
struct UserIdResult {
    id: i32,
}

#[derive(FromRow)]
struct LessonCapacity {
    max_students: i32,
    current_bookings: i64,
}

#[derive(FromRow)]
struct CardCheckResult {
    has_valid_card: bool,
}

#[derive(FromRow)]
struct BookingResult {
    id: i32,
}

#[derive(FromRow)]
struct BookingInfo {
    id: i32,
    user_id: i32,
    lesson_id: i32,
}

#[get("/yoga/lessons?<start>&<openid>&<class_type>")]
pub async fn lessons(
    start: i32,
    openid: String,
    class_type: i32,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // 直接查询课程列表，包含预约状态
    let query = r#"
        SELECT json_agg(
            jsonb_build_object(
                'id', l.id,
                'title', l.title,
                'description', l.description,
                'teacher_id', l.teacher_id,
                'teacher_name', t.name,
                'location_name', loc.name,
                'start_time', extract(epoch from l.start_time)::bigint,
                'end_time', extract(epoch from l.end_time)::bigint,
                'date_time', extract(epoch from l.start_time)::bigint,
                'max_students', l.max_students,
                'current_students', COUNT(b.id) FILTER (WHERE b.status = 'confirmed'),
                'is_booked', COUNT(b2.id) FILTER (WHERE b2.user_id = u.id AND b2.status = 'confirmed') > 0,
                'booking_id', MAX(b2.id) FILTER (WHERE b2.user_id = u.id AND b2.status = 'confirmed')
            )
        ) as result
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.id
        LEFT JOIN locations loc ON l.location_id = loc.id
        LEFT JOIN bookings b ON l.id = b.lesson_id AND b.status = 'confirmed'
        LEFT JOIN users u ON u.open_id = $2
        LEFT JOIN bookings b2 ON l.id = b2.lesson_id AND b2.user_id = u.id
        WHERE l.is_active = true
          AND l.start_time >= to_timestamp($1)
          AND l.start_time <= to_timestamp($1) + INTERVAL '14 days'
        GROUP BY l.id, l.title, l.description, l.teacher_id, t.name, loc.name, l.start_time, 
                 l.end_time, l.max_students, u.id
        ORDER BY l.start_time ASC
    "#;
    
    match sqlx::query_as::<_, JsonResult>(query)
        .bind(start)
        .bind(&openid)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => {
            match row.result {
                Some(lessons) => Ok(lessons.to_string()),
                None => Ok("[]".to_string())
            }
        }
        Err(error) => {
            println!("Error querying lessons: {}", error);
            Ok("[]".to_string())
        }
    }
}
#[get("/yoga/book?<id>&<openid>")]
pub async fn book(id: i32, openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 开始事务
    let mut transaction = match sqlxPool.inner().begin().await {
        Ok(t) => t,
        Err(error) => {
            println!("Error starting transaction: {}", error);
            return Ok("0".to_string());
        }
    };
    
    // 先获取用户ID
    let user_query = "SELECT id FROM users WHERE open_id = $1";
    let user_id: Option<i32> = match sqlx::query_as::<_, UserIdResult>(user_query)
        .bind(&openid)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(row)) => Some(row.id),
        Ok(None) => {
            println!("User not found: {}", openid);
            return Ok("0".to_string());
        }
        Err(error) => {
            println!("Error finding user: {}", error);
            return Ok("0".to_string());
        }
    };
    
    let user_id = user_id.unwrap();
    
    // 检查课程是否存在且有空位
    let lesson_query = r#"
        SELECT l.max_students, COUNT(b.id) as current_bookings
        FROM lessons l
        LEFT JOIN bookings b ON l.id = b.lesson_id AND b.status = 'confirmed'
        WHERE l.id = $1 AND l.is_active = true
        GROUP BY l.id, l.max_students
    "#;
    
    match sqlx::query_as::<_, LessonCapacity>(lesson_query)
        .bind(id)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(row)) => {
            if row.current_bookings >= row.max_students as i64 {
                println!("Lesson is full");
                return Ok("0".to_string());
            }
        }
        Ok(None) => {
            println!("Lesson not found: {}", id);
            return Ok("0".to_string());
        }
        Err(error) => {
            println!("Error checking lesson: {}", error);
            return Ok("0".to_string());
        }
    }
    
    // 检查用户是否有有效的会员卡
    let card_check_query = r#"
        SELECT EXISTS(
            SELECT 1 FROM user_membership_cards umc
            JOIN lessons l ON l.id = $2
            WHERE umc.user_id = $1
            AND umc.status = 'active'
            AND umc.expires_at > CURRENT_TIMESTAMP
            AND (
                umc.applicable_lesson_types IS NULL OR 
                l.lesson_type = ANY(umc.applicable_lesson_types)
            )
            AND (
                umc.card_type = 'unlimited' OR 
                (umc.card_type = 'count_based' AND umc.remaining_classes > 0)
            )
        ) as has_valid_card
    "#;
    
    match sqlx::query_as::<_, CardCheckResult>(card_check_query)
        .bind(user_id)
        .bind(id)
        .fetch_one(&mut *transaction).await {
        Ok(row) => {
            if !row.has_valid_card {
                println!("User {} has no valid membership card for lesson {}", user_id, id);
                return Ok(json!({
                    "success": false, 
                    "message": "没有有效的会员卡，请先购买会员卡"
                }).to_string());
            }
        }
        Err(error) => {
            println!("Error checking membership card: {}", error);
            return Ok("0".to_string());
        }
    }
    
    // 创建预约
    let book_query = r#"
        INSERT INTO bookings (user_id, lesson_id, booking_time, status)
        VALUES ($1, $2, CURRENT_TIMESTAMP, 'confirmed')
        ON CONFLICT (user_id, lesson_id) DO UPDATE SET
            status = 'confirmed',
            booking_time = CURRENT_TIMESTAMP
        RETURNING id
    "#;
    
    match sqlx::query_as::<_, BookingResult>(book_query)
        .bind(user_id)
        .bind(id)
        .fetch_one(&mut *transaction).await {
        Ok(row) => {
            match transaction.commit().await {
                Ok(_) => Ok(row.id.to_string()),
                Err(error) => {
                    println!("Error committing transaction: {}", error);
                    Ok("0".to_string())
                }
            }
        }
        Err(error) => {
            println!("Error creating booking: {}", error);
            Ok("0".to_string())
        }
    }
}
#[get("/yoga/unbook?<id>&<openid>")]
pub async fn unbook(id: i32, openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 开始事务
    let mut transaction = match sqlxPool.inner().begin().await {
        Ok(t) => t,
        Err(error) => {
            println!("Error starting transaction: {}", error);
            return Ok("0".to_string());
        }
    };
    
    // 获取预约信息以便后续退款处理
    let booking_info_query = r#"
        SELECT b.id, b.user_id, b.lesson_id
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        WHERE b.id = $1 
          AND u.open_id = $2
          AND b.status = 'confirmed'
    "#;
    
    let booking_info = match sqlx::query_as::<_, BookingInfo>(booking_info_query)
        .bind(id)
        .bind(&openid)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(row)) => (row.id, row.user_id, row.lesson_id),
        Ok(None) => {
            println!("Booking not found or already cancelled: {}", id);
            return Ok("0".to_string());
        }
        Err(error) => {
            println!("Error finding booking: {}", error);
            return Ok("0".to_string());
        }
    };
    
    let (booking_id, user_id, lesson_id) = booking_info;
    
    // 取消预约
    let cancel_query = r#"
        UPDATE bookings 
        SET status = 'cancelled'
        WHERE id = $1
        RETURNING id
    "#;
    
    match sqlx::query_as::<_, BookingResult>(cancel_query)
        .bind(booking_id)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(row)) => {
            let cancelled_id = row.id;
            
            // 处理会员卡退款 - 如果是次数卡，需要退回次数
            let refund_query = r#"
                UPDATE user_membership_cards 
                SET remaining_classes = remaining_classes + mcu.classes_consumed,
                    updated_at = CURRENT_TIMESTAMP
                FROM membership_card_usage mcu
                WHERE user_membership_cards.id = mcu.user_card_id
                  AND mcu.booking_id = $1
                  AND user_membership_cards.card_type = 'count_based'
            "#;
            
            // 执行退款，但不检查结果
            let _ = sqlx::query(refund_query)
                .bind(booking_id)
                .execute(&mut *transaction).await;
            
            // 更新使用记录状态
            let update_usage_query = r#"
                UPDATE membership_card_usage
                SET usage_type = 'refund'
                WHERE booking_id = $1
            "#;
            
            let _ = sqlx::query(update_usage_query)
                .bind(booking_id)
                .execute(&mut *transaction).await;
            
            match transaction.commit().await {
                Ok(_) => Ok(cancelled_id.to_string()),
                Err(error) => {
                    println!("Error committing transaction: {}", error);
                    Ok("0".to_string())
                }
            }
        }
        Ok(None) => {
            println!("Failed to cancel booking: {}", id);
            Ok("0".to_string())
        }
        Err(error) => {
            println!("Error cancelling booking: {}", error);
            Ok("0".to_string())
        }
    }
}