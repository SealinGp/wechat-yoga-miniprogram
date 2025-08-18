use rocket::http::Status;
use rocket::{post, get, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Utc;
use sqlx::{Pool as sPool, Postgres};
use crate::models::admin_user;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user_id: i32,
    username: String,
}

#[post("/api/admin/login", data = "<login_request>")]
pub async fn admin_login(
    login_request: rocket::serde::json::Json<LoginRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // For demo purposes, check if password matches "admin123" - in production use bcrypt
    if login_request.password == "admin123" {
        match admin_user::authenticate_admin_user(&login_request.username, sqlxPool.inner()).await {
            Ok(Some(user)) => {
                // In a real application, generate a proper JWT token
                // For now, we'll use a simple token format
                let token = format!("admin_{}_{}", user.id, Utc::now().timestamp());
                
                let response = LoginResponse {
                    token,
                    user_id: user.id,
                    username: user.username,
                };
                
                Ok(serde_json::to_string(&response).unwrap())
            }
            Ok(None) => Err(Status::Unauthorized),
            Err(error) => {
                println!("Database error: {}", error);
                Err(Status::InternalServerError)
            }
        }
    } else {
        Err(Status::Unauthorized)
    }
}

#[get("/api/admin/verify?<token>")]
pub async fn admin_verify(
    token: String,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Simple token verification (in production, use JWT)
    if token.starts_with("admin_") {
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() >= 2 {
            if let Ok(user_id) = parts[1].parse::<i32>() {
                match admin_user::verify_admin_user_by_id(user_id, sqlxPool.inner()).await {
                    Ok(Some(user)) => {
                        Ok(json!({
                            "valid": true,
                            "user_id": user_id,
                            "username": user.username
                        }).to_string())
                    }
                    Ok(None) => Err(Status::Unauthorized),
                    Err(_) => Err(Status::InternalServerError)
                }
            } else {
                Err(Status::Unauthorized)
            }
        } else {
            Err(Status::Unauthorized)
        }
    } else {
        Err(Status::Unauthorized)
    }
}