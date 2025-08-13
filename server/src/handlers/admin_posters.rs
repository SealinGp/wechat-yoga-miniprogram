use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Poster {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub image: String,
    pub link_url: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreatePosterRequest {
    pub title: Option<String>,
    pub image: String,
    pub link_url: Option<String>,
    pub sort_order: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePosterRequest {
    pub title: Option<String>,
    pub image: Option<String>,
    pub link_url: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[get("/admin/posters")]
pub async fn get_posters(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT id, title, image, link_url, sort_order, is_active,
               start_date AT TIME ZONE 'Asia/Shanghai' as start_date,
               end_date AT TIME ZONE 'Asia/Shanghai' as end_date,
               created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM posters
        ORDER BY sort_order ASC, created_at DESC
    "#;
    
    match sqlx::query_as::<_, Poster>(query).fetch_all(sqlxPool.inner()).await {
        Ok(posters) => {
            Ok(serde_json::to_string(&posters).unwrap())
        }
        Err(error) => {
            println!("Error querying posters: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/admin/posters", data = "<poster_request>")]
pub async fn create_poster(
    poster_request: rocket::serde::json::Json<CreatePosterRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        INSERT INTO posters (title, image, link_url, sort_order, start_date, end_date)
        VALUES ($1, $2, $3, $4, $5::timestamp, $6::timestamp)
        RETURNING id, title, image, link_url, sort_order, is_active,
                 start_date AT TIME ZONE 'Asia/Shanghai' as start_date,
                 end_date AT TIME ZONE 'Asia/Shanghai' as end_date,
                 created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    match sqlx::query_as::<_, Poster>(query)
        .bind(&poster_request.title)
        .bind(&poster_request.image)
        .bind(&poster_request.link_url)
        .bind(poster_request.sort_order.unwrap_or(0))
        .bind(&poster_request.start_date)
        .bind(&poster_request.end_date)
        .fetch_one(sqlxPool.inner()).await {
        Ok(poster) => {
            Ok(serde_json::to_string(&poster).unwrap())
        }
        Err(error) => {
            println!("Error creating poster: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/admin/posters/<id>", data = "<poster_request>")]
pub async fn update_poster(
    id: i32,
    poster_request: rocket::serde::json::Json<UpdatePosterRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        UPDATE posters 
        SET title = COALESCE($2, title),
            image = COALESCE($3, image),
            link_url = COALESCE($4, link_url),
            sort_order = COALESCE($5, sort_order),
            is_active = COALESCE($6, is_active),
            start_date = COALESCE($7::timestamp, start_date),
            end_date = COALESCE($8::timestamp, end_date)
        WHERE id = $1
        RETURNING id, title, image, link_url, sort_order, is_active,
                 start_date AT TIME ZONE 'Asia/Shanghai' as start_date,
                 end_date AT TIME ZONE 'Asia/Shanghai' as end_date,
                 created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    match sqlx::query_as::<_, Poster>(query)
        .bind(id)
        .bind(&poster_request.title)
        .bind(&poster_request.image)
        .bind(&poster_request.link_url)
        .bind(&poster_request.sort_order)
        .bind(&poster_request.is_active)
        .bind(&poster_request.start_date)
        .bind(&poster_request.end_date)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(poster)) => {
            Ok(serde_json::to_string(&poster).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating poster: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/admin/posters/<id>")]
pub async fn delete_poster(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM posters WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({"success": true, "message": "Poster deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting poster: {}", error);
            Err(Status::InternalServerError)
        }
    }
}