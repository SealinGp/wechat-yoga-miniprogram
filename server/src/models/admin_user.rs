use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::json;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequest {
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub id: i32,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
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

// Admin User database operations
pub async fn get_all_admin_users(sqlx_pool: &Pool<Postgres>) -> Result<Vec<AdminUserModel>, sqlx::Error> {
    let query = r#"
        SELECT id, username, password_hash, is_active, 
               created_at, updated_at
        FROM admin_users
        ORDER BY created_at DESC
    "#;
    
    sqlx::query_as::<_, AdminUserModel>(query)
        .fetch_all(sqlx_pool)
        .await
}

pub async fn create_admin_user(data: &AdminUserCreateRequest, sqlx_pool: &Pool<Postgres>) -> Result<AdminUserModel, sqlx::Error> {
    // Simple password hashing (in production, use bcrypt)
    let password_hash = format!("hash_{}", data.password);
    
    let query = r#"
        INSERT INTO admin_users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, password_hash, is_active, created_at, updated_at
    "#;
    
    sqlx::query_as::<_, AdminUserModel>(query)
        .bind(&data.username)
        .bind(&password_hash)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn update_admin_user(data: &AdminUserUpdateRequest, sqlx_pool: &Pool<Postgres>) -> Result<Option<AdminUserModel>, sqlx::Error> {
    // Protect the default admin user (id = 1) from being deactivated
    if data.id == 1 && data.is_active == Some(false) {
        return Ok(None); // Return None to indicate protection
    }
    
    // Build dynamic query based on provided fields
    let query = r#"
        UPDATE admin_users 
        SET username = COALESCE($2, username),
            password_hash = CASE WHEN $3 IS NOT NULL THEN CONCAT('hash_', $3) ELSE password_hash END,
            is_active = COALESCE($4, is_active),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, username, password_hash, is_active, created_at, updated_at
    "#;
    
    sqlx::query_as::<_, AdminUserModel>(query)
        .bind(data.id)
        .bind(&data.username)
        .bind(&data.password)
        .bind(data.is_active)
        .fetch_optional(sqlx_pool)
        .await
}

pub async fn delete_admin_user(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    // Protect the default admin user (id = 1) from deletion
    if id == 1 {
        return Ok(json!({
            "error": "Default admin user cannot be deleted"
        }));
    }
    
    let query = "DELETE FROM admin_users WHERE id = $1";
    
    let result = sqlx::query(query)
        .bind(id)
        .execute(sqlx_pool)
        .await?;
    
    if result.rows_affected() > 0 {
        Ok(json!({"success": true, "message": "Admin user deleted successfully"}))
    } else {
        Ok(json!({"success": false, "message": "Admin user not found"}))
    }
}

// User (regular user) database operations
pub async fn get_all_users(sqlx_pool: &Pool<Postgres>) -> Result<Vec<UserModel>, sqlx::Error> {
    let query = r#"
        SELECT id, open_id, nick_name, avatar_url, phone, created_at, updated_at, is_admin
        FROM users
        ORDER BY created_at DESC
    "#;
    
    sqlx::query_as::<_, UserModel>(query)
        .fetch_all(sqlx_pool)
        .await
}

pub async fn get_user_by_id(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Option<UserModel>, sqlx::Error> {
    let query = r#"
        SELECT id, open_id, nick_name, avatar_url, phone, created_at, updated_at, is_admin
        FROM users
        WHERE id = $1
    "#;
    
    sqlx::query_as::<_, UserModel>(query)
        .bind(id)
        .fetch_optional(sqlx_pool)
        .await
}

pub async fn create_user(data: &UserCreateRequest, sqlx_pool: &Pool<Postgres>) -> Result<UserModel, sqlx::Error> {
    let query = r#"
        INSERT INTO users (open_id, nick_name, avatar_url, phone)
        VALUES ($1, $2, $3, $4)
        RETURNING id, open_id, nick_name, avatar_url, phone, created_at, updated_at, is_admin
    "#;
    
    sqlx::query_as::<_, UserModel>(query)
        .bind(&data.open_id)
        .bind(&data.nick_name)
        .bind(&data.avatar_url)
        .bind(&data.phone)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn update_user(data: &UserUpdateRequest, sqlx_pool: &Pool<Postgres>) -> Result<Option<UserModel>, sqlx::Error> {
    let query = r#"
        UPDATE users 
        SET nick_name = COALESCE($2, nick_name),
            avatar_url = COALESCE($3, avatar_url),
            phone = COALESCE($4, phone),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, open_id, nick_name, avatar_url, phone, created_at, updated_at, is_admin
    "#;
    
    sqlx::query_as::<_, UserModel>(query)
        .bind(data.id)
        .bind(&data.nick_name)
        .bind(&data.avatar_url)
        .bind(&data.phone)
        .fetch_optional(sqlx_pool)
        .await
}

pub async fn delete_user(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    let query = "DELETE FROM users WHERE id = $1";
    
    let result = sqlx::query(query)
        .bind(id)
        .execute(sqlx_pool)
        .await?;
    
    if result.rows_affected() > 0 {
        Ok(json!({"success": true, "message": "User deleted successfully"}))
    } else {
        Ok(json!({"success": false, "message": "User not found"}))
    }
}

pub async fn get_users_with_stats(sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    let query = r#"
        SELECT json_agg(jsonb_build_object(
            'id', id,
            'open_id', open_id,
            'nick_name', nick_name,
            'avatar_url', avatar_url,
            'phone', phone,
            'created_at', extract(epoch from created_at)::bigint,
            'updated_at', extract(epoch from updated_at)::bigint
        )) as result
        FROM users
        ORDER BY created_at DESC
    "#;
    
    let result = sqlx::query_scalar::<_, Option<serde_json::Value>>(query)
        .fetch_one(sqlx_pool)
        .await?;
    
    match result {
        Some(json_data) => Ok(json_data),
        None => Ok(json!([])) // Return empty array if no users
    }
}

// Stored procedure functions (kept as-is for compatibility)
pub async fn get_admin_user_lessons(id: i32, start: i64, end: i64, sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    let query = "select * from fn_admin_user_lessons($1,$2,$3)";
    
    sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(id)
        .bind(start)
        .bind(end)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn get_admin_user_details(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    let query = "select * from fn_admin_user($1)";
    
    sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(id)
        .fetch_one(sqlx_pool)
        .await
}

// Authentication functions
#[derive(Debug, Serialize, FromRow)]
pub struct AdminUser {
    pub id: i32,
    pub username: String,
}

pub async fn authenticate_admin_user(username: &str, sqlx_pool: &Pool<Postgres>) -> Result<Option<AdminUser>, sqlx::Error> {
    let query = "SELECT id, username FROM admin_users WHERE username = $1 AND is_active = true";
    
    sqlx::query_as::<_, AdminUser>(query)
        .bind(username)
        .fetch_optional(sqlx_pool)
        .await
}

pub async fn verify_admin_user_by_id(user_id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Option<AdminUser>, sqlx::Error> {
    let query = "SELECT id, username FROM admin_users WHERE id = $1 AND is_active = true";
    
    sqlx::query_as::<_, AdminUser>(query)
        .bind(user_id)
        .fetch_optional(sqlx_pool)
        .await
}