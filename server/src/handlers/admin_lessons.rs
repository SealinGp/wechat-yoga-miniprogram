use sqlx::{Pool as sPool, Postgres};
use crate::models::lession::{self, Lesson, LessonCreateRequest};
use rocket::http::Status;
use rocket::State;
use serde_json::json;

#[post("/api/admin/lesson", data = "<data>")]
pub async fn create_lesson(
    data: rocket::serde::json::Json<LessonCreateRequest>,
    sqlx_pool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    match lession::create_lesson(&data.into_inner(), sqlx_pool.inner()).await {
        Ok(lesson_id) => {
            Ok(json!({
                "success": true,
                "id": lesson_id,
                "message": "Lesson created successfully"
            }).to_string())
        }
        Err(error) => {
            println!("Database error creating lesson: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/api/admin/lessons?<start>&<end>&<limit>&<offset>")]
pub async fn admin_lessons(
    start: i32, 
    end: i32, 
    limit: Option<i64>, 
    offset: Option<i64>, 
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    let limit = limit.unwrap_or(50);  // Default limit
    let offset = offset.unwrap_or(0); // Default offset
    
    match lession::list_lessons(start, end, limit, offset, sqlxPool.inner()).await {
        Ok(lessons) => {
            match serde_json::to_string(&lessons) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        },
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/admin/lesson?<id>")]
pub async fn admin_lesson(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match lession::get_lesson_by_id(id, sqlxPool.inner()).await {
        Ok(Some(lesson)) => {
            match serde_json::to_string(&lesson) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        },
        Ok(None) => Err(Status::NotFound),
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/admin/lesson/hidden?<id>&<status>")]
pub async fn admin_lesson_hidden(
    id: i32,
    status: i32,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let is_active = status != 0; // Convert status int to boolean
    
    match lession::update_lesson_status(id, is_active, sqlxPool.inner()).await {
        Ok(true) => Ok("1".to_string()), // Successfully updated
        Ok(false) => Err(Status::NotFound), // No rows affected
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/admin/lesson/delete?<id>")]
pub async fn admin_lesson_delete(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match lession::delete_lesson(id, sqlxPool.inner()).await {
        Ok(true) => Ok("1".to_string()), // Successfully deleted
        Ok(false) => Err(Status::NotFound), // No rows affected
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/api/admin/lessons/and/teachers?<id>")]
pub async fn admin_lessons_and_teachers(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match lession::get_lessons_with_teachers(id, sqlxPool.inner()).await {
        Ok(lessons) => {
            match serde_json::to_string(&lessons) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        },
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[post("/yoga/lesson/update", data = "<data>")]
pub async fn admin_lesson_update(data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // Parse the JSON data into LessonUpdateData struct
    let update_data: lession::LessonUpdateData = match serde_json::from_str(&data) {
        Ok(parsed) => parsed,
        Err(error) => {
            println!("JSON parsing error: {}", error);
            return Err(Status::BadRequest);
        }
    };
    
    match lession::update_lesson_data(&update_data, sqlxPool.inner()).await {
        Ok(true) => Ok("1".to_string()), // Successfully updated
        Ok(false) => Err(Status::NotFound), // No rows affected
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}