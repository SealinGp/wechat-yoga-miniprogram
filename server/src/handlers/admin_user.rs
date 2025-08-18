use sqlx::{Pool as sPool, Postgres, FromRow};
use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::models::admin_user as admin_user_model;
#[get("/yoga/admin/user/lessons?<id>&<start>&<end>&<open_id>")]
pub async fn admin_user_lessons(
    id: i32,
    start: i64,
    end: i64,
    open_id: String,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    match admin_user_model::get_admin_user_lessons(id, start, end, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/admin/users/all?<open_id>")]
pub async fn admin_users_all(open_id: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match admin_user_model::get_users_with_stats(sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string()) // Return empty array as fallback
        }
    }
}
#[get("/yoga/admin/user?<open_id>&<id>")]
pub async fn admin_user(id:i32,
                        open_id:String,
                        sqlxPool: &State<sPool<Postgres>>,
                        ) -> Result<String, Status> {
    match admin_user_model::get_admin_user_details(id, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub open_id: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
}

#[get("/api/admin/users")]
pub async fn get_users(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match admin_user_model::get_all_users(sqlxPool.inner()).await {
        Ok(users) => {
            // Convert to response format
            let response_users: Vec<User> = users.into_iter().map(|user| User {
                id: Some(user.id),
                open_id: user.open_id,
                nick_name: user.nick_name,
                avatar_url: user.avatar_url,
                phone: user.phone,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
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

#[post("/api/admin/users", data = "<user_request>")]
pub async fn create_user(
    user_request: rocket::serde::json::Json<CreateUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let create_request = admin_user_model::UserCreateRequest {
        open_id: user_request.open_id.clone(),
        nick_name: user_request.nick_name.clone(),
        avatar_url: user_request.avatar_url.clone(),
        phone: user_request.phone.clone(),
    };
    
    match admin_user_model::create_user(&create_request, sqlxPool.inner()).await {
        Ok(user) => {
            // Convert to response format
            let response_user = User {
                id: Some(user.id),
                open_id: user.open_id,
                nick_name: user.nick_name,
                avatar_url: user.avatar_url,
                phone: user.phone,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
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
            Err(Status::InternalServerError)
        }
    }
}

#[put("/api/admin/users/<id>", data = "<user_request>")]
pub async fn update_user(
    id: i32,
    user_request: rocket::serde::json::Json<UpdateUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let update_request = admin_user_model::UserUpdateRequest {
        id,
        nick_name: user_request.nick_name.clone(),
        avatar_url: user_request.avatar_url.clone(),
        phone: user_request.phone.clone(),
    };
    
    match admin_user_model::update_user(&update_request, sqlxPool.inner()).await {
        Ok(Some(user)) => {
            // Convert to response format
            let response_user = User {
                id: Some(user.id),
                open_id: user.open_id,
                nick_name: user.nick_name,
                avatar_url: user.avatar_url,
                phone: user.phone,
                created_at: Some(user.created_at),
                updated_at: Some(user.updated_at),
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
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/users/<id>")]
pub async fn delete_user(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match admin_user_model::delete_user(id, sqlxPool.inner()).await {
        Ok(response) => {
            Ok(response.to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}