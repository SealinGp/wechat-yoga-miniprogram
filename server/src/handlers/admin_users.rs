use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct AdminUser {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateAdminUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateAdminUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
}

#[get("/admin/admin-users")]
pub async fn get_admin_users(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT id, username, null as password_hash, is_active, 
               created_at AT TIME ZONE 'Asia/Shanghai' as created_at,
               updated_at AT TIME ZONE 'Asia/Shanghai' as updated_at
        FROM admin_users
        ORDER BY created_at DESC
    "#;
    
    match sqlx::query_as::<_, AdminUser>(query).fetch_all(sqlxPool.inner()).await {
        Ok(admin_users) => {
            Ok(serde_json::to_string(&admin_users).unwrap())
        }
        Err(error) => {
            println!("Error querying admin users: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/admin/admin-users", data = "<admin_user_request>")]
pub async fn create_admin_user(
    admin_user_request: rocket::serde::json::Json<CreateAdminUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Simple password hashing (in production, use bcrypt)
    let password_hash = format!("hash_{}", admin_user_request.password);
    
    let query = r#"
        INSERT INTO admin_users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, null as password_hash, is_active, 
                 created_at AT TIME ZONE 'Asia/Shanghai' as created_at,
                 updated_at AT TIME ZONE 'Asia/Shanghai' as updated_at
    "#;
    
    match sqlx::query_as::<_, AdminUser>(query)
        .bind(&admin_user_request.username)
        .bind(&password_hash)
        .fetch_one(sqlxPool.inner()).await {
        Ok(admin_user) => {
            Ok(serde_json::to_string(&admin_user).unwrap())
        }
        Err(error) => {
            println!("Error creating admin user: {}", error);
            if error.to_string().contains("duplicate key") {
                Err(Status::Conflict)
            } else {
                Err(Status::InternalServerError)
            }
        }
    }
}

#[put("/admin/admin-users/<id>", data = "<admin_user_request>")]
pub async fn update_admin_user(
    id: i32,
    admin_user_request: rocket::serde::json::Json<UpdateAdminUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Protect the default admin user (id = 1) from being deactivated
    if id == 1 && admin_user_request.is_active == Some(false) {
        return Ok(json!({
            "error": "Default admin user cannot be deactivated"
        }).to_string());
    }
    
    let mut query = "UPDATE admin_users SET updated_at = CURRENT_TIMESTAMP".to_string();
    let mut bind_count = 1;
    let mut params: Vec<String> = vec![id.to_string()];
    
    if let Some(username) = &admin_user_request.username {
        query.push_str(&format!(", username = ${}", bind_count + 1));
        params.push(username.clone());
        bind_count += 1;
    }
    
    if let Some(password) = &admin_user_request.password {
        let password_hash = format!("hash_{}", password);
        query.push_str(&format!(", password_hash = ${}", bind_count + 1));
        params.push(password_hash);
        bind_count += 1;
    }
    
    if let Some(is_active) = admin_user_request.is_active {
        query.push_str(&format!(", is_active = ${}", bind_count + 1));
        params.push(is_active.to_string());
        bind_count += 1;
    }
    
    query.push_str(" WHERE id = $1 RETURNING id, username, null as password_hash, is_active, created_at AT TIME ZONE 'Asia/Shanghai' as created_at, updated_at AT TIME ZONE 'Asia/Shanghai' as updated_at");
    
    // Use dynamic query building with SQLx
    let mut sqlx_query = sqlx::query_as::<_, AdminUser>(&query).bind(id);
    
    if let Some(username) = &admin_user_request.username {
        sqlx_query = sqlx_query.bind(username);
    }
    
    if let Some(password) = &admin_user_request.password {
        let password_hash = format!("hash_{}", password);
        sqlx_query = sqlx_query.bind(password_hash);
    }
    
    if let Some(is_active) = admin_user_request.is_active {
        sqlx_query = sqlx_query.bind(is_active);
    }
    
    match sqlx_query.fetch_optional(sqlxPool.inner()).await {
        Ok(Some(admin_user)) => {
            Ok(serde_json::to_string(&admin_user).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating admin user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/admin/admin-users/<id>")]
pub async fn delete_admin_user(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // Protect the default admin user (id = 1) from deletion
    if id == 1 {
        return Ok(json!({
            "error": "Default admin user cannot be deleted"
        }).to_string());
    }
    
    let query = "DELETE FROM admin_users WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({"success": true, "message": "Admin user deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting admin user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}