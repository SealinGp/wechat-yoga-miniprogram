use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Notice {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateNoticeRequest {
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateNoticeRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub author: Option<String>,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
}

#[get("/api/admin/notices")]
pub async fn get_notices(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT id, title, content, author, priority, is_active, 
               created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM notices
        ORDER BY priority DESC, created_at DESC
    "#;
    
    match sqlx::query_as::<_, Notice>(query).fetch_all(sqlxPool.inner()).await {
        Ok(notices) => {
            Ok(serde_json::to_string(&notices).unwrap())
        }
        Err(error) => {
            println!("Error querying notices: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/api/admin/notices", data = "<notice_request>")]
pub async fn create_notice(
    notice_request: rocket::serde::json::Json<CreateNoticeRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        INSERT INTO notices (title, content, author, priority)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, content, author, priority, is_active, 
                 created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    match sqlx::query_as::<_, Notice>(query)
        .bind(&notice_request.title)
        .bind(&notice_request.content)
        .bind(&notice_request.author)
        .bind(notice_request.priority.unwrap_or(0))
        .fetch_one(sqlxPool.inner()).await {
        Ok(notice) => {
            Ok(serde_json::to_string(&notice).unwrap())
        }
        Err(error) => {
            println!("Error creating notice: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/api/admin/notices/<id>", data = "<notice_request>")]
pub async fn update_notice(
    id: i32,
    notice_request: rocket::serde::json::Json<UpdateNoticeRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        UPDATE notices 
        SET title = COALESCE($2, title),
            content = COALESCE($3, content),
            author = COALESCE($4, author),
            priority = COALESCE($5, priority),
            is_active = COALESCE($6, is_active)
        WHERE id = $1
        RETURNING id, title, content, author, priority, is_active, 
                 created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    match sqlx::query_as::<_, Notice>(query)
        .bind(id)
        .bind(&notice_request.title)
        .bind(&notice_request.content)
        .bind(&notice_request.author)
        .bind(&notice_request.priority)
        .bind(&notice_request.is_active)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(notice)) => {
            Ok(serde_json::to_string(&notice).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating notice: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/notices/<id>")]
pub async fn delete_notice(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM notices WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({"success": true, "message": "Notice deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting notice: {}", error);
            Err(Status::InternalServerError)
        }
    }
}