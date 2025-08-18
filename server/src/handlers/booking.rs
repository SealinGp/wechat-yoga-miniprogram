use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;
use crate::models::booking;

// Using structs from model layer

#[get("/yoga/lessons?<start>&<openid>&<class_type>")]
pub async fn lessons(
    start: i32,
    openid: String,
    class_type: i32,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    match booking::get_lessons_with_booking_status(start, &openid, class_type, sqlxPool.inner()).await {
        Ok(Some(lessons)) => Ok(lessons.to_string()),
        Ok(None) => Ok("[]".to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}
#[get("/yoga/book?<id>&<openid>")]
pub async fn book(id: i32, openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match booking::create_booking(id, &openid, sqlxPool.inner()).await {
        Ok(result) => {
            // Check if the result indicates success and extract booking ID or return appropriate response
            if let Some(success) = result.get("success").and_then(|v| v.as_bool()) {
                if success {
                    if let Some(booking_id) = result.get("booking_id").and_then(|v| v.as_i64()) {
                        Ok(booking_id.to_string())
                    } else {
                        Ok("1".to_string()) // Success but no booking ID
                    }
                } else {
                    Ok(result.to_string()) // Return error message
                }
            } else {
                Ok("0".to_string()) // Default fallback
            }
        }
        Err(error) => {
            println!("Database error: {}", error);
            Ok("0".to_string())
        }
    }
}
#[get("/yoga/unbook?<id>&<openid>")]
pub async fn unbook(id: i32, openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match booking::cancel_booking(id, &openid, sqlxPool.inner()).await {
        Ok(result) => {
            // Check if the result indicates success and extract cancelled ID or return appropriate response
            if let Some(success) = result.get("success").and_then(|v| v.as_bool()) {
                if success {
                    if let Some(cancelled_id) = result.get("cancelled_id").and_then(|v| v.as_i64()) {
                        Ok(cancelled_id.to_string())
                    } else {
                        Ok("1".to_string()) // Success but no cancelled ID
                    }
                } else {
                    Ok("0".to_string()) // Booking not found or cancellation failed
                }
            } else {
                Ok("0".to_string()) // Default fallback
            }
        }
        Err(error) => {
            println!("Database error: {}", error);
            Ok("0".to_string())
        }
    }
}