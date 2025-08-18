use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};

use crate::handlers::admin_actions::Action;

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
    pub total_ratings: i64,
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
    pub actions: Vec<Action>,
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

// Helper structs for database operations
#[derive(FromRow)]
pub struct UserIdResult {
    pub id: i32,
}

#[derive(FromRow)]
pub struct JsonResult {
    pub posters: Option<Value>,
}

#[derive(FromRow)]
pub struct ActionsResult {
    pub actions: Option<Value>,
}

#[derive(FromRow)]
pub struct TeachersResult {
    pub teachers: Option<Value>,
}

#[derive(FromRow)]
pub struct NoticesResult {
    pub notices: Option<Value>,
}

#[derive(FromRow)]
pub struct BookedResult {
    pub booked: Option<Value>,
}

#[derive(FromRow)]
pub struct MarketResult {
    pub market: Option<Value>,
}

// Database operations for index page data
pub async fn get_user_id_by_openid(openid: &str, sqlx_pool: &Pool<Postgres>) -> Result<Option<i32>, sqlx::Error> {
    let user_row = sqlx::query_as::<_, UserIdResult>("SELECT id FROM users WHERE open_id = $1")
        .bind(openid)
        .fetch_optional(sqlx_pool)
        .await?;
    
    Ok(user_row.map(|row| row.id))
}

pub async fn get_active_posters(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let poster_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'image', image,
                'title', title,
                'href', link_url
            ) ORDER BY sort_order ASC
        ), '[]'::json) as posters
        FROM posters 
        WHERE is_active = true 
        AND (start_date IS NULL OR start_date <= CURRENT_TIMESTAMP)
        AND (end_date IS NULL OR end_date >= CURRENT_TIMESTAMP)
    "#;

    let row = sqlx::query_as::<_, JsonResult>(poster_query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.posters.unwrap_or(json!([])))
}

pub async fn get_active_action_buttons(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let actions_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'icon', icon,
                'link', link
            ) ORDER BY sort_order ASC
        ), '[]'::json) as actions
        FROM action_buttons 
        WHERE is_active = true
    "#;

    let row = sqlx::query_as::<_, ActionsResult>(actions_query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.actions.unwrap_or(json!([])))
}

pub async fn get_featured_teachers(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let teachers_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'thumbnail', avatar_url,
                'introduction', description,
                'rating', COALESCE(average_rating, 0.0),
                'experience_years', experience_years
            ) ORDER BY average_rating DESC NULLS LAST, experience_years DESC
        ), '[]'::json) as teachers
        FROM teachers 
        WHERE is_active = true 
        LIMIT 5
    "#;

    let row = sqlx::query_as::<_, TeachersResult>(teachers_query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.teachers.unwrap_or(json!([])))
}

pub async fn get_recent_notices(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let notices_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'title', title,
                'timeago', CASE
                    WHEN EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) < 3600 THEN 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 60) || '分钟前'
                    WHEN EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) < 86400 THEN 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 3600) || '小时前'
                    ELSE 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 86400) || '天前'
                END
            ) ORDER BY priority DESC, created_at DESC
        ), '[]'::json) as notices
        FROM notices 
        WHERE is_active = true 
        LIMIT 5
    "#;

    let row = sqlx::query_as::<_, NoticesResult>(notices_query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.notices.unwrap_or(json!([])))
}

pub async fn get_user_upcoming_bookings(user_id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let booked_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', b.id,
                'title', l.title,
                'teacher', t.name,
                'start_time', extract(epoch from l.start_time)::bigint,
                'location', loc.name,
                'lesson_id', l.id
            ) ORDER BY l.start_time ASC
        ), '[]'::json) as booked
        FROM bookings b
        JOIN lessons l ON b.lesson_id = l.id
        LEFT JOIN teachers t ON l.teacher_id = t.id
        LEFT JOIN locations loc ON l.location_id = loc.id
        WHERE b.user_id = $1
          AND b.status = 'confirmed'
          AND l.start_time > CURRENT_TIMESTAMP
        LIMIT 10
    "#;

    let row = sqlx::query_as::<_, BookedResult>(booked_query)
        .bind(user_id)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.booked.unwrap_or(json!([])))
}

pub async fn get_market_info(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let market_query = r#"
        SELECT jsonb_build_object(
            'id', id,
            'slogan', slogan,
            'description', description
        ) as market
        FROM market_info 
        WHERE is_active = true 
        LIMIT 1
    "#;

    let row = sqlx::query_as::<_, MarketResult>(market_query)
        .fetch_optional(sqlx_pool)
        .await?;
    
    match row {
        Some(result) => Ok(result.market.unwrap_or(json!({"id": 1, "slogan": "积分兑换好礼，健康生活更精彩"}))),
        None => Ok(json!({"id": 1, "slogan": "积分兑换好礼，健康生活更精彩"})),
    }
}

pub async fn get_index_data(openid: Option<String>, sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    // Get user ID if openid is provided
    let user_id = if let Some(ref open_id) = openid {
        get_user_id_by_openid(open_id, sqlx_pool).await.unwrap_or(None)
    } else {
        None
    };

    // Fetch all data concurrently (we can parallelize this in the future)
    let posters = get_active_posters(sqlx_pool).await?;
    let actions = get_active_action_buttons(sqlx_pool).await?;
    let teachers = get_featured_teachers(sqlx_pool).await?;
    let notices = get_recent_notices(sqlx_pool).await?;
    let market = get_market_info(sqlx_pool).await?;
    
    let booked = if let Some(uid) = user_id {
        get_user_upcoming_bookings(uid, sqlx_pool).await?
    } else {
        json!([])
    };

    Ok(json!({
        "poster": posters,
        "actions": actions,
        "teachers": teachers,
        "notices": notices,
        "booked": booked,
        "market": market
    }))
}