use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: bool,
    pub booking_enabled: bool,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationWithStats {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: bool,
    pub booking_enabled: bool,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub is_active: bool,
    // 统计信息
    pub total_lessons: i64,
    pub active_lessons: i64,
    pub utilization_rate: Option<f64>, // 使用率百分比
}

// 地点可用性检查结果
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAvailability {
    pub location_id: i32,
    pub location_name: String,
    pub is_available: bool,
    pub conflicting_lessons: Vec<ConflictingLesson>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ConflictingLesson {
    pub id: i32,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub teacher_name: Option<String>,
}

// 地点使用统计
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationUsageStats {
    pub location_id: i32,
    pub location_name: String,
    pub total_bookings: i64,
    pub total_hours: f64,
    pub avg_utilization: f64,
    pub peak_hours: Vec<i32>, // 高峰时段（小时）
    pub most_popular_lesson_type: Option<String>,
}