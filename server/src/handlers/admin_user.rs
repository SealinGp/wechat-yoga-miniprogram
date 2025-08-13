use sqlx::{Pool as sPool, Postgres, FromRow};
use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json;
#[get("/yoga/admin/user/lessons?<id>&<start>&<end>&<open_id>")]
pub async fn admin_user_lessons(
    id: i32,
    start: i64,
    end: i64,
    open_id: String,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = "select * from fn_admin_user_lessons($1,$2,$3)";
    
    match sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(id)
        .bind(start)
        .bind(end)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/admin/users/all?<open_id>")]
pub async fn admin_users_all(open_id: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 直接查询用户列表，不使用存储过程
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
    
    match sqlx::query_scalar::<_, Option<serde_json::Value>>(query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => {
            match result {
                Some(json_data) => Ok(json_data.to_string()),
                None => Ok("[]".to_string()) // 返回空数组如果没有用户
            }
        }
        Err(error) => {
            println!("Error querying users: {}", error);
            Ok("[]".to_string()) // 返回空数组作为降级处理
        }
    }
}
#[get("/yoga/admin/user?<open_id>&<id>")]
pub async fn admin_user(id:i32,
                        open_id:String,
                        sqlxPool: &State<sPool<Postgres>>,
                        ) -> Result<String, Status> {
    let query = "select * from fn_admin_user($1)";
    
    match sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(id)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error: {}", error);
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

#[get("/admin/users")]
pub async fn get_users(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT id, open_id, nick_name, avatar_url, phone, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
    "#;
    
    match sqlx::query_as::<_, User>(query).fetch_all(sqlxPool.inner()).await {
        Ok(users) => {
            Ok(serde_json::to_string(&users).unwrap())
        }
        Err(error) => {
            println!("Error querying users: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/admin/users", data = "<user_request>")]
pub async fn create_user(
    user_request: rocket::serde::json::Json<CreateUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        INSERT INTO users (open_id, nick_name, avatar_url, phone)
        VALUES ($1, $2, $3, $4)
        RETURNING id, open_id, nick_name, avatar_url, phone, created_at, updated_at
    "#;
    
    match sqlx::query_as::<_, User>(query)
        .bind(&user_request.open_id)
        .bind(&user_request.nick_name)
        .bind(&user_request.avatar_url)
        .bind(&user_request.phone)
        .fetch_one(sqlxPool.inner()).await {
        Ok(user) => {
            Ok(serde_json::to_string(&user).unwrap())
        }
        Err(error) => {
            println!("Error creating user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/admin/users/<id>", data = "<user_request>")]
pub async fn update_user(
    id: i32,
    user_request: rocket::serde::json::Json<UpdateUserRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        UPDATE users 
        SET nick_name = COALESCE($2, nick_name),
            avatar_url = COALESCE($3, avatar_url),
            phone = COALESCE($4, phone),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, open_id, nick_name, avatar_url, phone, created_at, updated_at
    "#;
    
    match sqlx::query_as::<_, User>(query)
        .bind(id)
        .bind(&user_request.nick_name)
        .bind(&user_request.avatar_url)
        .bind(&user_request.phone)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(user)) => {
            Ok(serde_json::to_string(&user).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/admin/users/<id>")]
pub async fn delete_user(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM users WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(serde_json::json!({"success": true, "message": "User deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}