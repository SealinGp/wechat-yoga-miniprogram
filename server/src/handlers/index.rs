use rocket::http::Status;
use rocket::State;
use serde_json::json;
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;
use crate::models::index as index_model;

// Using structs from model layer

#[get("/yoga/index")]
pub async fn index_without_openid(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    index_handler(None, sqlxPool).await
}

#[get("/yoga/index?<openid>")]
pub async fn index(openid: Option<String>, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    index_handler(openid, sqlxPool).await
}

async fn index_handler(openid: Option<String>, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match index_model::get_index_data(openid, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}