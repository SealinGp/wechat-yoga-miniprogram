use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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