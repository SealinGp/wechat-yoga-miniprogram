use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminUserModel {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

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
pub struct AdminUserCreateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserUpdateRequest {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserWithLessons {
    pub id: i32,
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_admin: bool,
    pub total_bookings: i64,
    pub confirmed_bookings: i64,
    pub cancelled_bookings: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBookingStats {
    pub user_id: i32,
    pub total_bookings: i64,
    pub confirmed_bookings: i64,
    pub cancelled_bookings: i64,
    pub completed_bookings: i64,
    pub no_show_bookings: i64,
}