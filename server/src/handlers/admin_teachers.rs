use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};
use rust_decimal::Decimal;
use crate::models::teacher;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Teacher {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
    pub average_rating: Option<Decimal>,
    pub total_ratings: Option<i64>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateTeacherRequest {
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateTeacherRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
    pub is_active: Option<bool>,
}

#[get("/api/admin/teachers")]
pub async fn get_teachers(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match teacher::get_all_teachers(sqlxPool.inner()).await {
        Ok(teachers) => {
            match serde_json::to_string(&teachers) {
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

#[post("/api/admin/teachers", data = "<teacher_request>")]
pub async fn create_teacher(
    teacher_request: rocket::serde::json::Json<CreateTeacherRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let create_request = teacher::TeacherCreateRequest {
        name: teacher_request.name.clone(),
        description: teacher_request.description.clone(),
        avatar_url: teacher_request.avatar_url.clone(),
        bio: teacher_request.bio.clone(),
        certifications: teacher_request.certifications.clone(),
        specialties: teacher_request.specialties.clone(),
        experience_years: teacher_request.experience_years,
    };
    
    match teacher::create_teacher(&create_request, sqlxPool.inner()).await {
        Ok(teacher_id) => {
            Ok(json!({"success": true, "id": teacher_id, "message": "Teacher created successfully"}).to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/api/admin/teachers/<id>", data = "<teacher_request>")]
pub async fn update_teacher(
    id: i32,
    teacher_request: rocket::serde::json::Json<UpdateTeacherRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let update_request = teacher::TeacherUpdateRequest {
        id,
        name: teacher_request.name.clone(),
        description: teacher_request.description.clone(),
        avatar_url: teacher_request.avatar_url.clone(),
        bio: teacher_request.bio.clone(),
        certifications: teacher_request.certifications.clone(),
        specialties: teacher_request.specialties.clone(),
        experience_years: teacher_request.experience_years,
        is_active: teacher_request.is_active,
    };
    
    match teacher::update_teacher(&update_request, sqlxPool.inner()).await {
        Ok(true) => {
            Ok(json!({"success": true, "message": "Teacher updated successfully"}).to_string())
        }
        Ok(false) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/teachers/<id>")]
pub async fn delete_teacher(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match teacher::delete_teacher(id, sqlxPool.inner()).await {
        Ok(response) => {
            Ok(response.to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}