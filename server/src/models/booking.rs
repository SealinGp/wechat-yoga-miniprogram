use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BookingModel {
    pub id: i32,
    pub user_id: i32,
    pub lesson_id: i32,
    pub booking_time: DateTime<Utc>,
    pub status: String,
    pub notes: Option<String>,
    pub payment_status: String,
    pub payment_amount: Option<rust_decimal::Decimal>,
    pub cancellation_reason: Option<String>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub attended: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LessonWithBookingInfo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub teacher_name: Option<String>,
    pub start_time: i64, // Unix timestamp
    pub end_time: i64,   // Unix timestamp
    pub date_time: i64,  // Unix timestamp (same as start_time for compatibility)
    pub max_students: i32,
    pub current_students: i64,
    pub location_name: Option<String>,
    pub is_booked: bool,
    pub booking_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingCreateRequest {
    pub user_open_id: String,
    pub lesson_id: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingUpdateRequest {
    pub id: i32,
    pub status: Option<String>,
    pub notes: Option<String>,
    pub payment_status: Option<String>,
    pub payment_amount: Option<rust_decimal::Decimal>,
    pub cancellation_reason: Option<String>,
    pub attended: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BookingWithLessonInfo {
    pub id: i32,
    pub user_id: i32,
    pub lesson_id: i32,
    pub lesson_title: String,
    pub teacher_name: Option<String>,
    pub location_name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub booking_time: DateTime<Utc>,
    pub notes: Option<String>,
}

// Helper structs for database operations
#[derive(FromRow)]
pub struct JsonResult {
    pub result: Option<Value>,
}

#[derive(FromRow)]
pub struct UserIdResult {
    pub id: i32,
}

#[derive(FromRow)]
pub struct LessonCapacity {
    pub max_students: i32,
    pub current_bookings: i64,
}

#[derive(FromRow)]
pub struct CardCheckResult {
    pub has_valid_card: bool,
}

#[derive(FromRow)]
pub struct BookingResult {
    pub id: i32,
}

#[derive(FromRow)]
pub struct BookingInfo {
    pub id: i32,
    pub user_id: i32,
    pub lesson_id: i32,
}

// Database operations
pub async fn get_lessons_with_booking_status(
    start: i32,
    openid: &str,
    class_type: i32,
    sqlx_pool: &Pool<Postgres>,
) -> Result<Option<Value>, sqlx::Error> {
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
    
    let row = sqlx::query_as::<_, JsonResult>(query)
        .bind(start)
        .bind(openid)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.result)
}

pub async fn create_booking(
    lesson_id: i32,
    openid: &str,
    sqlx_pool: &Pool<Postgres>,
) -> Result<Value, sqlx::Error> {
    // Start transaction
    let mut transaction = sqlx_pool.begin().await?;
    
    // Get user ID
    let user_query = "SELECT id FROM users WHERE open_id = $1";
    let user_row = sqlx::query_as::<_, UserIdResult>(user_query)
        .bind(openid)
        .fetch_optional(&mut *transaction)
        .await?;
    
    let user_id = match user_row {
        Some(row) => row.id,
        None => {
            return Ok(json!({"success": false, "message": "User not found"}));
        }
    };
    
    // Check lesson capacity
    let lesson_query = r#"
        SELECT l.max_students, COUNT(b.id) as current_bookings
        FROM lessons l
        LEFT JOIN bookings b ON l.id = b.lesson_id AND b.status = 'confirmed'
        WHERE l.id = $1 AND l.is_active = true
        GROUP BY l.id, l.max_students
    "#;
    
    let lesson_capacity = sqlx::query_as::<_, LessonCapacity>(lesson_query)
        .bind(lesson_id)
        .fetch_optional(&mut *transaction)
        .await?;
    
    let capacity_info = match lesson_capacity {
        Some(row) => row,
        None => {
            return Ok(json!({"success": false, "message": "Lesson not found"}));
        }
    };
    
    if capacity_info.current_bookings >= capacity_info.max_students as i64 {
        return Ok(json!({"success": false, "message": "Lesson is full"}));
    }
    
    // Check valid membership card
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
    
    let card_check = sqlx::query_as::<_, CardCheckResult>(card_check_query)
        .bind(user_id)
        .bind(lesson_id)
        .fetch_one(&mut *transaction)
        .await?;
    
    if !card_check.has_valid_card {
        return Ok(json!({
            "success": false, 
            "message": "没有有效的会员卡，请先购买会员卡"
        }));
    }
    
    // Create booking
    let book_query = r#"
        INSERT INTO bookings (user_id, lesson_id, booking_time, status)
        VALUES ($1, $2, CURRENT_TIMESTAMP, 'confirmed')
        ON CONFLICT (user_id, lesson_id) DO UPDATE SET
            status = 'confirmed',
            booking_time = CURRENT_TIMESTAMP
        RETURNING id
    "#;
    
    let booking_result = sqlx::query_as::<_, BookingResult>(book_query)
        .bind(user_id)
        .bind(lesson_id)
        .fetch_one(&mut *transaction)
        .await?;
    
    transaction.commit().await?;
    
    Ok(json!({
        "success": true,
        "booking_id": booking_result.id,
        "message": "Booking successful"
    }))
}

pub async fn cancel_booking(
    booking_id: i32,
    openid: &str,
    sqlx_pool: &Pool<Postgres>,
) -> Result<Value, sqlx::Error> {
    // Start transaction
    let mut transaction = sqlx_pool.begin().await?;
    
    // Get booking info
    let booking_info_query = r#"
        SELECT b.id, b.user_id, b.lesson_id
        FROM bookings b
        JOIN users u ON b.user_id = u.id
        WHERE b.id = $1 
          AND u.open_id = $2
          AND b.status = 'confirmed'
    "#;
    
    let booking_info = sqlx::query_as::<_, BookingInfo>(booking_info_query)
        .bind(booking_id)
        .bind(openid)
        .fetch_optional(&mut *transaction)
        .await?;
    
    let (_booking_id, _user_id, _lesson_id) = match booking_info {
        Some(row) => (row.id, row.user_id, row.lesson_id),
        None => {
            return Ok(json!({"success": false, "message": "Booking not found"}));
        }
    };
    
    // Cancel booking
    let cancel_query = r#"
        UPDATE bookings 
        SET status = 'cancelled'
        WHERE id = $1
        RETURNING id
    "#;
    
    let cancelled_booking = sqlx::query_as::<_, BookingResult>(cancel_query)
        .bind(booking_id)
        .fetch_optional(&mut *transaction)
        .await?;
    
    match cancelled_booking {
        Some(row) => {
            let cancelled_id = row.id;
            
            // Handle membership card refund for count-based cards
            let refund_query = r#"
                UPDATE user_membership_cards 
                SET remaining_classes = remaining_classes + mcu.classes_consumed,
                    updated_at = CURRENT_TIMESTAMP
                FROM membership_card_usage mcu
                WHERE user_membership_cards.id = mcu.user_card_id
                  AND mcu.booking_id = $1
                  AND user_membership_cards.card_type = 'count_based'
            "#;
            
            // Execute refund (ignore result)
            let _ = sqlx::query(refund_query)
                .bind(booking_id)
                .execute(&mut *transaction)
                .await;
            
            // Update usage record status
            let update_usage_query = r#"
                UPDATE membership_card_usage
                SET usage_type = 'refund'
                WHERE booking_id = $1
            "#;
            
            let _ = sqlx::query(update_usage_query)
                .bind(booking_id)
                .execute(&mut *transaction)
                .await;
            
            transaction.commit().await?;
            
            Ok(json!({
                "success": true,
                "cancelled_id": cancelled_id,
                "message": "Booking cancelled successfully"
            }))
        }
        None => {
            Ok(json!({"success": false, "message": "Failed to cancel booking"}))
        }
    }
}