use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MembershipPlanModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub card_type: String,
    pub validity_days: i32,
    pub total_classes: Option<i32>,
    pub price: rust_decimal::Decimal,
    pub original_price: Option<rust_decimal::Decimal>,
    pub applicable_lesson_types: Option<Vec<String>>,
    pub max_bookings_per_day: i32,
    pub transfer_allowed: bool,
    pub refund_allowed: bool,
    pub benefits: Option<Vec<String>>,
    pub restrictions: Option<Vec<String>>,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserMembershipCardModel {
    pub id: i32,
    pub user_id: i32,
    pub plan_id: i32,
    pub card_number: String,
    pub status: String,
    pub card_type: String,
    pub plan_name: String,
    pub validity_days: i32,
    pub total_classes: Option<i32>,
    pub remaining_classes: Option<i32>,
    pub purchased_at: DateTime<Utc>,
    pub activated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub purchase_price: rust_decimal::Decimal,
    pub discount_amount: rust_decimal::Decimal,
    pub actual_paid: rust_decimal::Decimal,
    pub applicable_lesson_types: Option<Vec<String>>,
    pub max_bookings_per_day: i32,
    pub transfer_allowed: bool,
    pub refund_allowed: bool,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MembershipCardUsageModel {
    pub id: i32,
    pub user_card_id: i32,
    pub booking_id: Option<i32>,
    pub lesson_id: i32,
    pub user_id: i32,
    pub usage_type: String,
    pub classes_consumed: i32,
    pub used_at: DateTime<Utc>,
    pub remaining_classes_before: Option<i32>,
    pub remaining_classes_after: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipCardPurchaseRequest {
    pub plan_id: i32,
    pub user_open_id: String,
    pub paid_amount: Option<rust_decimal::Decimal>,
    pub discount_amount: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MembershipCardUsageWithDetails {
    pub id: i32,
    pub lesson_title: String,
    pub lesson_start_time: i64, // Unix timestamp
    pub teacher_name: Option<String>,
    pub usage_type: String,
    pub classes_consumed: i32,
    pub used_at: i64, // Unix timestamp
    pub remaining_classes_after: Option<i32>,
    pub card_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipCardResponse {
    pub success: bool,
    pub message: String,
    pub card_id: Option<i32>,
    pub card_number: Option<String>,
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
pub struct PlanDetails {
    pub name: String,
    pub card_type: String,
    pub validity_days: i32,
    pub total_classes: Option<i32>,
    pub price: Decimal,
    pub applicable_lesson_types: Option<Vec<String>>,
    pub max_bookings_per_day: Option<i32>,
}

#[derive(FromRow)]
pub struct CardCreated {
    pub id: i32,
    pub card_number: String,
}

// Database operations
pub async fn get_membership_plans(sqlx_pool: &Pool<Postgres>) -> Result<Option<Value>, sqlx::Error> {
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
    
    let row = sqlx::query_as::<_, JsonResult>(query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.result)
}

pub async fn get_user_membership_cards(openid: &str, sqlx_pool: &Pool<Postgres>) -> Result<Option<Value>, sqlx::Error> {
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
    
    let row = sqlx::query_as::<_, JsonResult>(query)
        .bind(openid)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.result)
}

pub async fn purchase_membership_card(
    openid: &str,
    plan_id: i32,
    paid_amount: Option<f64>,
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
    
    // Get plan details
    let plan_query = "SELECT name, card_type, validity_days, total_classes, price, applicable_lesson_types, max_bookings_per_day FROM membership_plans WHERE id = $1 AND is_active = true";
    let plan = sqlx::query_as::<_, PlanDetails>(plan_query)
        .bind(plan_id)
        .fetch_optional(&mut *transaction)
        .await?;
    
    let plan = match plan {
        Some(plan) => plan,
        None => {
            return Ok(json!({"success": false, "message": "套餐不存在或已下架"}));
        }
    };
    
    let price_f64 = plan.price.to_string().parse::<f64>().unwrap_or(0.0);
    let actual_paid = paid_amount.unwrap_or(price_f64);
    let discount_amount = price_f64 - actual_paid;
    
    // Create user membership card
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
    
    let card_created = sqlx::query_as::<_, CardCreated>(insert_query)
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
        .fetch_one(&mut *transaction)
        .await?;
    
    transaction.commit().await?;
    
    Ok(json!({
        "success": true, 
        "card_id": card_created.id,
        "card_number": card_created.card_number,
        "message": "会员卡购买成功"
    }))
}

pub async fn get_card_usage(
    openid: &str,
    card_id: Option<i32>,
    sqlx_pool: &Pool<Postgres>,
) -> Result<Option<Value>, sqlx::Error> {
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
            .bind(openid)
            .bind(cid)
            .fetch_one(sqlx_pool)
            .await?
    } else {
        sqlx::query_as::<_, JsonResult>(query)
            .bind(openid)
            .fetch_one(sqlx_pool)
            .await?
    };
    
    Ok(result.result)
}