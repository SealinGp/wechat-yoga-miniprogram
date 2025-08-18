use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};
use crate::models::admin_user;

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

#[get("/api/admin/admin-users")]
pub async fn get_admin_users(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match admin_user::get_all_admin_users(sqlxPool.inner()).await {
        Ok(admin_users) => {
            // Convert to response format without password_hash
            let response_users: Vec<AdminUser> = admin_users.into_iter().map(|user| AdminUser {
                id: Some(user.id),
                username: user.username,
                password_hash: None, // Don't expose password hash
                is_active: Some(user.is_active),
                created_at: Some(user.created_at.naive_utc()),
                updated_at: Some(user.updated_at.naive_utc()),
            }).collect();
            
            match serde_json::to_string(&response_users) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/api/admin/admin-users", data = "<admin_user_request>")]
pub async fn create_admin_user(
    admin_user_request: rocket::serde::json::Json<CreateAdminUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let create_request = admin_user::AdminUserCreateRequest {
        username: admin_user_request.username.clone(),
        password: admin_user_request.password.clone(),
    };
    
    match admin_user::create_admin_user(&create_request, sqlxPool.inner()).await {
        Ok(user) => {
            // Convert to response format without password_hash
            let response_user = AdminUser {
                id: Some(user.id),
                username: user.username,
                password_hash: None, // Don't expose password hash
                is_active: Some(user.is_active),
                created_at: Some(user.created_at.naive_utc()),
                updated_at: Some(user.updated_at.naive_utc()),
            };
            
            match serde_json::to_string(&response_user) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Err(error) => {
            println!("Database error: {}", error);
            if error.to_string().contains("duplicate key") {
                Err(Status::Conflict)
            } else {
                Err(Status::InternalServerError)
            }
        }
    }
}

#[put("/api/admin/admin-users/<id>", data = "<admin_user_request>")]
pub async fn update_admin_user(
    id: i32,
    admin_user_request: rocket::serde::json::Json<UpdateAdminUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let update_request = admin_user::AdminUserUpdateRequest {
        id,
        username: admin_user_request.username.clone(),
        password: admin_user_request.password.clone(),
        is_active: admin_user_request.is_active,
    };
    
    match admin_user::update_admin_user(&update_request, sqlxPool.inner()).await {
        Ok(Some(user)) => {
            // Convert to response format without password_hash
            let response_user = AdminUser {
                id: Some(user.id),
                username: user.username,
                password_hash: None, // Don't expose password hash
                is_active: Some(user.is_active),
                created_at: Some(user.created_at.naive_utc()),
                updated_at: Some(user.updated_at.naive_utc()),
            };
            
            match serde_json::to_string(&response_user) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Ok(None) => {
            // Either not found or protected from deactivation
            if id == 1 && admin_user_request.is_active == Some(false) {
                Ok(json!({"error": "Default admin user cannot be deactivated"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/admin-users/<id>")]
pub async fn delete_admin_user(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match admin_user::delete_admin_user(id, sqlxPool.inner()).await {
        Ok(response) => {
            Ok(response.to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}