use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PosterModel {
    pub id: i32,
    pub title: Option<String>,
    pub image: String, // 图片文件名
    pub link_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ActionButtonModel {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub action_type: String,
    pub action_value: i32,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

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
pub struct NoticeModel {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MarketInfoModel {
    pub id: i32,
    pub slogan: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexData {
    pub poster: Vec<PosterModel>,
    pub actions: Vec<ActionButtonModel>,
    pub teachers: Vec<TeacherModel>,
    pub notices: Vec<NoticeModel>,
    pub booked: Vec<crate::models::booking::BookingWithLessonInfo>,
    pub market: Option<MarketInfoModel>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NoticeWithTimeago {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub timeago: String, // 相对时间描述，如 "2小时前"
}