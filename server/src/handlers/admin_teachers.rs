use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::NaiveDateTime;
use sqlx::{Pool as sPool, Postgres, FromRow};
use rust_decimal::Decimal;


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

#[get("/admin/teachers")]
pub async fn get_teachers(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT 
            t.id, 
            t.name, 
            t.description, 
            t.avatar_url, 
            t.bio, 
            t.certifications, 
            t.specialties,
            t.experience_years, 
            COALESCE(AVG(tr.rating), 0.0) as average_rating,
            COUNT(tr.rating) as total_ratings,
            t.is_active,
            t.created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM teachers t
        LEFT JOIN teacher_ratings tr ON t.id = tr.teacher_id
        GROUP BY t.id, t.name, t.description, t.avatar_url, t.bio, t.certifications, 
                 t.specialties, t.experience_years, t.is_active, t.created_at
        ORDER BY t.is_active DESC, COALESCE(AVG(tr.rating), 0.0) DESC, t.experience_years DESC
    "#;
    
    match sqlx::query_as::<_, Teacher>(query).fetch_all(sqlxPool.inner()).await {
        Ok(teachers) => {
            Ok(serde_json::to_string(&teachers).unwrap())
        }
        Err(error) => {
            println!("Error querying teachers: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/admin/teachers", data = "<teacher_request>")]
pub async fn create_teacher(
    teacher_request: rocket::serde::json::Json<CreateTeacherRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        WITH new_teacher AS (
            INSERT INTO teachers (name, description, avatar_url, bio, certifications, specialties, experience_years)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, description, avatar_url, bio, certifications, specialties,
                     experience_years, is_active, created_at
        )
        SELECT 
            nt.id,
            nt.name,
            nt.description,
            nt.avatar_url,
            nt.bio,
            nt.certifications,
            nt.specialties,
            nt.experience_years,
            0.0 as average_rating,
            0::bigint as total_ratings,
            nt.is_active,
            nt.created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM new_teacher nt
    "#;
    
    match sqlx::query_as::<_, Teacher>(query)
        .bind(&teacher_request.name)
        .bind(&teacher_request.description)
        .bind(&teacher_request.avatar_url)
        .bind(&teacher_request.bio)
        .bind(&teacher_request.certifications)
        .bind(&teacher_request.specialties)
        .bind(teacher_request.experience_years.unwrap_or(0))
        .fetch_one(sqlxPool.inner()).await {
        Ok(teacher) => {
            Ok(serde_json::to_string(&teacher).unwrap())
        }
        Err(error) => {
            println!("Error creating teacher: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/admin/teachers/<id>", data = "<teacher_request>")]
pub async fn update_teacher(
    id: i32,
    teacher_request: rocket::serde::json::Json<UpdateTeacherRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        WITH updated_teacher AS (
            UPDATE teachers 
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                avatar_url = COALESCE($4, avatar_url),
                bio = COALESCE($5, bio),
                certifications = COALESCE($6, certifications),
                specialties = COALESCE($7, specialties),
                experience_years = COALESCE($8, experience_years),
                is_active = COALESCE($9, is_active),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING id, name, description, avatar_url, bio, certifications, specialties,
                     experience_years, is_active, created_at
        )
        SELECT 
            ut.id,
            ut.name,
            ut.description,
            ut.avatar_url,
            ut.bio,
            ut.certifications,
            ut.specialties,
            ut.experience_years,
            COALESCE(AVG(tr.rating), 0.0) as average_rating,
            COUNT(tr.rating) as total_ratings,
            ut.is_active,
            ut.created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM updated_teacher ut
        LEFT JOIN teacher_ratings tr ON ut.id = tr.teacher_id
        GROUP BY ut.id, ut.name, ut.description, ut.avatar_url, ut.bio, ut.certifications,
                 ut.specialties, ut.experience_years, ut.is_active, ut.created_at
    "#;
    
    match sqlx::query_as::<_, Teacher>(query)
        .bind(id)
        .bind(&teacher_request.name)
        .bind(&teacher_request.description)
        .bind(&teacher_request.avatar_url)
        .bind(&teacher_request.bio)
        .bind(&teacher_request.certifications)
        .bind(&teacher_request.specialties)
        .bind(&teacher_request.experience_years)
        .bind(&teacher_request.is_active)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(teacher)) => {
            Ok(serde_json::to_string(&teacher).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating teacher: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/admin/teachers/<id>")]
pub async fn delete_teacher(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM teachers WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({"success": true, "message": "Teacher deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting teacher: {}", error);
            Err(Status::InternalServerError)
        }
    }
}