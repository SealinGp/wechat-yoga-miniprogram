use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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