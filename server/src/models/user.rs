use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: i32,
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserBookingStatistics {
    pub user_id: i32,
    pub total_bookings: i64,
    pub confirmed_bookings: i64,
    pub cancelled_bookings: i64,
    pub completed_bookings: i64,
    pub no_show_bookings: i64,
    pub attendance_rate: Option<f64>, // (completed / (completed + no_show)) * 100
}