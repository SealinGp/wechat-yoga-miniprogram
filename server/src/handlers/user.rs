use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres};
use crate::models::user;

#[get("/yoga/user/query?<openid>")]
pub async fn user_query(openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match user::get_user_by_openid(&openid, sqlxPool.inner()).await {
        Ok(Some(result)) => Ok(result.to_string()),
        Ok(None) => Ok("null".to_string()),
        Err(error) => {
            println!("Error querying user: {}", error);
            Err(Status::NoContent)
        }
    }
}
#[post("/yoga/user", data = "<data>")]
pub async fn register_user(data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let json_data: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid JSON data: {}", data);
            return Err(Status::BadRequest);
        }
    };
    
    match user::create_or_update_user(json_data, sqlxPool.inner()).await {
        Ok(user_id) => Ok(user_id.to_string()),
        Err(error) => {
            println!("Error updating user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/user/book/statistics?<id>")]
pub async fn user_book_statistics(id: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match user::get_user_booking_statistics(&id, sqlxPool.inner()).await {
        Ok(Some(result)) => Ok(result.to_string()),
        Ok(None) => {
            let empty_stats = json!({
                "id": 0,
                "avatar_url": null,
                "nick_name": null,
                "user_type": 0,
                "total_bookings": 0,
                "completed_classes": 0,
                "cancelled_bookings": 0
            });
            Ok(empty_stats.to_string())
        }
        Err(error) => {
            println!("Error querying user statistics: {}", error);
            Err(Status::InternalServerError)
        }
    }
}