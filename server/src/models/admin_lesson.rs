use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminLessonModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub location_id: Option<i32>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub current_students: i32,
    pub price: Option<rust_decimal::Decimal>,
    pub equipment_required: Option<Vec<String>>,
    pub prerequisites: Option<String>,
    pub cancellation_policy: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLessonCreateRequest {
    pub title: String,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub location_id: Option<i32>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub price: Option<rust_decimal::Decimal>,
    pub equipment_required: Option<Vec<String>>,
    pub prerequisites: Option<String>,
    pub cancellation_policy: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminLessonUpdateRequest {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub location_id: Option<i32>,
    pub lesson_type: Option<String>,
    pub difficulty_level: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub max_students: Option<i32>,
    pub price: Option<rust_decimal::Decimal>,
    pub equipment_required: Option<Vec<String>>,
    pub prerequisites: Option<String>,
    pub cancellation_policy: Option<String>,
    pub notes: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminLessonWithTeacher {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub teacher_name: Option<String>,
    pub location_id: Option<i32>,
    pub location_name: Option<String>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub current_students: i32,
    pub price: Option<rust_decimal::Decimal>,
    pub is_active: bool,
}