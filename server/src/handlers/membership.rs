use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(FromRow)]
struct JsonResult {
    result: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct UserIdResult {
    id: i32,
}

#[derive(FromRow)]
struct PlanDetails {
    name: String,
    card_type: String,
    validity_days: i32,
    total_classes: Option<i32>,
    price: Decimal,
    applicable_lesson_types: Option<Vec<String>>,
    max_bookings_per_day: Option<i32>,
}

#[derive(FromRow)]
struct CardCreated {
    id: i32,
    card_number: String,
}

// 获取会员卡套餐列表
#[get("/yoga/membership/plans")]
pub async fn get_plans(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'description', description,
                'card_type', card_type,
                'validity_days', validity_days,
                'total_classes', total_classes,
                'price', price,
                'original_price', original_price,
                'applicable_lesson_types', applicable_lesson_types,
                'max_bookings_per_day', max_bookings_per_day,
                'benefits', benefits,
                'restrictions', restrictions
            ) ORDER BY sort_order ASC
        ) as result
        FROM membership_plans
        WHERE is_active = true
    "#;
    
    match sqlx::query_as::<_, JsonResult>(query).fetch_one(sqlxPool.inner()).await {
        Ok(row) => {
            match row.result {
                Some(plans) => Ok(plans.to_string()),
                None => Ok("[]".to_string())
            }
        }
        Err(error) => {
            println!("Error querying membership plans: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取用户的会员卡列表
#[get("/yoga/membership/cards?<openid>")]
pub async fn get_user_cards(openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT json_agg(
            jsonb_build_object(
                'id', umc.id,
                'card_number', umc.card_number,
                'status', umc.status,
                'card_type', umc.card_type,
                'plan_name', umc.plan_name,
                'total_classes', umc.total_classes,
                'remaining_classes', umc.remaining_classes,
                'expires_at', extract(epoch from umc.expires_at)::bigint,
                'activated_at', extract(epoch from umc.activated_at)::bigint,
                'purchase_price', umc.purchase_price,
                'actual_paid', umc.actual_paid,
                'applicable_lesson_types', umc.applicable_lesson_types,
                'max_bookings_per_day', umc.max_bookings_per_day
            ) ORDER BY 
                CASE 
                    WHEN umc.status = 'active' THEN 1 
                    WHEN umc.status = 'expired' THEN 2 
                    ELSE 3 
                END,
                umc.expires_at DESC
        ) as result
        FROM user_membership_cards umc
        JOIN users u ON umc.user_id = u.id
        WHERE u.open_id = $1
    "#;
    
    match sqlx::query_as::<_, JsonResult>(query)
        .bind(&openid)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => {
            match row.result {
                Some(cards) => Ok(cards.to_string()),
                None => Ok("[]".to_string())
            }
        }
        Err(error) => {
            println!("Error querying user membership cards: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 购买会员卡
#[post("/yoga/membership/purchase?<openid>&<plan_id>&<paid_amount>")]
pub async fn purchase_card(
    openid: String, 
    plan_id: i32, 
    paid_amount: Option<f64>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    // 开始事务
    let mut transaction = match sqlxPool.inner().begin().await {
        Ok(t) => t,
        Err(error) => {
            println!("Error starting transaction: {}", error);
            return Ok(json!({"success": false, "message": "Transaction error"}).to_string());
        }
    };
    
    // 获取用户ID
    let user_query = "SELECT id FROM users WHERE open_id = $1";
    let user_id: Option<i32> = match sqlx::query_as::<_, UserIdResult>(user_query)
        .bind(&openid)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(row)) => Some(row.id),
        Ok(None) => {
            println!("User not found: {}", openid);
            return Ok(json!({"success": false, "message": "User not found"}).to_string());
        }
        Err(error) => {
            println!("Error finding user: {}", error);
            return Ok(json!({"success": false, "message": "Database error"}).to_string());
        }
    };
    
    let user_id = user_id.unwrap();
    
    // 获取套餐信息
    let plan_query = "SELECT name, card_type, validity_days, total_classes, price, applicable_lesson_types, max_bookings_per_day FROM membership_plans WHERE id = $1 AND is_active = true";
    match sqlx::query_as::<_, PlanDetails>(plan_query)
        .bind(plan_id)
        .fetch_optional(&mut *transaction).await {
        Ok(Some(plan)) => {
            let price_f64 = plan.price.to_string().parse::<f64>().unwrap_or(0.0);
            let actual_paid = paid_amount.unwrap_or(price_f64);
            let discount_amount = price_f64 - actual_paid;
            
            // 创建用户会员卡
            let insert_query = r#"
                INSERT INTO user_membership_cards (
                    user_id, plan_id, card_type, plan_name, validity_days,
                    total_classes, remaining_classes, purchase_price, actual_paid,
                    discount_amount, applicable_lesson_types, max_bookings_per_day,
                    activated_at, expires_at
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                    CURRENT_TIMESTAMP, 
                    CURRENT_TIMESTAMP + INTERVAL '1 day' * $5
                )
                RETURNING id, card_number
            "#;
            
            match sqlx::query_as::<_, CardCreated>(insert_query)
                .bind(user_id)
                .bind(plan_id)
                .bind(&plan.card_type)
                .bind(&plan.name)
                .bind(plan.validity_days)
                .bind(&plan.total_classes)
                .bind(&plan.total_classes)
                .bind(plan.price)
                .bind(actual_paid)
                .bind(discount_amount)
                .bind(&plan.applicable_lesson_types)
                .bind(&plan.max_bookings_per_day)
                .fetch_one(&mut *transaction).await {
                Ok(row) => {
                    match transaction.commit().await {
                        Ok(_) => Ok(json!({
                            "success": true, 
                            "card_id": row.id,
                            "card_number": row.card_number,
                            "message": "会员卡购买成功"
                        }).to_string()),
                        Err(error) => {
                            println!("Error committing transaction: {}", error);
                            Ok(json!({"success": false, "message": "提交失败"}).to_string())
                        }
                    }
                }
                Err(error) => {
                    println!("Error creating membership card: {}", error);
                    Ok(json!({"success": false, "message": "创建会员卡失败"}).to_string())
                }
            }
        }
        Ok(None) => {
            Ok(json!({"success": false, "message": "套餐不存在或已下架"}).to_string())
        }
        Err(error) => {
            println!("Error querying plan: {}", error);
            Ok(json!({"success": false, "message": "查询套餐失败"}).to_string())
        }
    }
}

// 获取会员卡使用记录
#[get("/yoga/membership/usage?<openid>&<card_id>")]
pub async fn get_card_usage(
    openid: String, 
    card_id: Option<i32>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    let query = if card_id.is_some() {
        r#"
            SELECT json_agg(
                jsonb_build_object(
                    'id', mcu.id,
                    'lesson_title', l.title,
                    'lesson_start_time', extract(epoch from l.start_time)::bigint,
                    'teacher_name', t.name,
                    'usage_type', mcu.usage_type,
                    'classes_consumed', mcu.classes_consumed,
                    'used_at', extract(epoch from mcu.used_at)::bigint,
                    'remaining_classes_after', mcu.remaining_classes_after
                ) ORDER BY mcu.used_at DESC
            ) as result
            FROM membership_card_usage mcu
            JOIN user_membership_cards umc ON mcu.user_card_id = umc.id
            JOIN users u ON mcu.user_id = u.id
            JOIN lessons l ON mcu.lesson_id = l.id
            LEFT JOIN teachers t ON l.teacher_id = t.id
            WHERE u.open_id = $1 AND umc.id = $2
        "#
    } else {
        r#"
            SELECT json_agg(
                jsonb_build_object(
                    'id', mcu.id,
                    'card_number', umc.card_number,
                    'lesson_title', l.title,
                    'lesson_start_time', extract(epoch from l.start_time)::bigint,
                    'teacher_name', t.name,
                    'usage_type', mcu.usage_type,
                    'classes_consumed', mcu.classes_consumed,
                    'used_at', extract(epoch from mcu.used_at)::bigint,
                    'remaining_classes_after', mcu.remaining_classes_after
                ) ORDER BY mcu.used_at DESC
            ) as result
            FROM membership_card_usage mcu
            JOIN user_membership_cards umc ON mcu.user_card_id = umc.id
            JOIN users u ON mcu.user_id = u.id
            JOIN lessons l ON mcu.lesson_id = l.id
            LEFT JOIN teachers t ON l.teacher_id = t.id
            WHERE u.open_id = $1
        "#
    };
    
    let result = if let Some(cid) = card_id {
        sqlx::query_as::<_, JsonResult>(query)
            .bind(&openid)
            .bind(cid)
            .fetch_one(sqlxPool.inner()).await
    } else {
        sqlx::query_as::<_, JsonResult>(query)
            .bind(&openid)
            .fetch_one(sqlxPool.inner()).await
    };
    
    match result {
        Ok(row) => {
            match row.result {
                Some(usage) => Ok(usage.to_string()),
                None => Ok("[]".to_string())
            }
        }
        Err(error) => {
            println!("Error querying card usage: {}", error);
            Ok("[]".to_string())
        }
    }
}