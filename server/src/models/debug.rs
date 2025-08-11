use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DebugLogModel {
    pub id: i32,
    pub open_id: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub pixel_ratio: Option<rust_decimal::Decimal>,
    pub screen_height: Option<i32>,
    pub screen_width: Option<i32>,
    pub version: Option<String>,
    pub sdk_version: Option<String>,
    pub platform: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugLogCreateRequest {
    pub open_id: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub pixel_ratio: Option<rust_decimal::Decimal>,
    pub screen_height: Option<i32>,
    pub screen_width: Option<i32>,
    pub version: Option<String>,
    pub sdk_version: Option<String>,
    pub platform: Option<String>,
    pub ip_address: Option<String>,
}