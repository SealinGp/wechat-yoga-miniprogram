use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherModel {
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherRatingModel {
    pub id: i32,
    pub teacher_id: i32,
    pub user_id: i32,
    pub lesson_id: i32,
    pub rating: rust_decimal::Decimal,
    pub review: Option<String>,
    pub rating_categories: Option<serde_json::Value>,
    pub is_anonymous: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherLessonModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub current_students: i32,
    pub venue: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherRatingCreateRequest {
    pub teacher_id: i32,
    pub user_open_id: String,
    pub lesson_id: i32,
    pub rating: rust_decimal::Decimal,
    pub review: Option<String>,
    pub rating_categories: Option<serde_json::Value>,
    pub is_anonymous: Option<bool>,
}