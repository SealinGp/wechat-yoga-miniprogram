use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};

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

// Helper structs for database operations
#[derive(FromRow)]
pub struct UserResult {
    pub result: Option<Value>,
}

#[derive(FromRow)]
pub struct UserIdResult {
    pub id: i32,
}

// Database operations
pub async fn get_user_by_openid(openid: &str, sqlx_pool: &Pool<Postgres>) -> Result<Option<Value>, sqlx::Error> {
    let query = r#"
        SELECT row_to_json(t) as result
        FROM (
            SELECT id, avatar_url, nick_name, 
                   0 as user_type
            FROM users
            WHERE open_id = $1
        ) as t
    "#;
    
    let row = sqlx::query_as::<_, UserResult>(query)
        .bind(openid)
        .fetch_optional(sqlx_pool)
        .await?;
    
    Ok(row.and_then(|r| r.result))
}

pub async fn create_or_update_user(data: Value, sqlx_pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let query = r#"
        INSERT INTO users (open_id, nick_name, avatar_url, phone, created_at, updated_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT (open_id) DO UPDATE SET
            nick_name = COALESCE(EXCLUDED.nick_name, users.nick_name),
            avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
            phone = COALESCE(EXCLUDED.phone, users.phone),
            updated_at = CURRENT_TIMESTAMP
        RETURNING id
    "#;
    
    let open_id = data["open_id"].as_str().unwrap_or("");
    let nick_name = data["nick_name"].as_str();
    let avatar_url = data["avatar_url"].as_str();
    let phone = data["phone"].as_str();
    
    let row = sqlx::query_as::<_, UserIdResult>(query)
        .bind(open_id)
        .bind(nick_name)
        .bind(avatar_url)
        .bind(phone)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.id)
}

pub async fn get_user_booking_statistics(openid: &str, sqlx_pool: &Pool<Postgres>) -> Result<Option<Value>, sqlx::Error> {
    let query = r#"
        SELECT row_to_json(t) as result
        FROM (
            SELECT u.id, u.avatar_url, u.nick_name,
                   0 as user_type,
                   COUNT(b.id) FILTER (WHERE b.status = 'confirmed') as total_bookings,
                   COUNT(b.id) FILTER (WHERE b.status = 'completed') as completed_classes,
                   COUNT(b.id) FILTER (WHERE b.status = 'cancelled') as cancelled_bookings
            FROM users u
            LEFT JOIN bookings b ON u.id = b.user_id
            WHERE u.open_id = $1
            GROUP BY u.id, u.avatar_url, u.nick_name
        ) as t
    "#;
    
    let row = sqlx::query_as::<_, UserResult>(query)
        .bind(openid)
        .fetch_optional(sqlx_pool)
        .await?;
    
    Ok(row.and_then(|r| r.result))
}