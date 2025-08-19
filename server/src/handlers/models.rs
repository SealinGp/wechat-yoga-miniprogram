use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: i32,
    pub average_rating: Option<rust_decimal::Decimal>,
    pub total_ratings: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Location {
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
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub teacher: Teacher,
    pub location: Location,
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
