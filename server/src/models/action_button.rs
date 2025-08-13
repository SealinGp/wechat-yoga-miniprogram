use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ActionButtonModel {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>, // 图标链接URL
    pub link: String, // 跳转链接，如 /pages/booking/booking
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonCreateRequest {
    pub name: String,
    pub icon: Option<String>,
    pub link: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonUpdateRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<i32>,
}