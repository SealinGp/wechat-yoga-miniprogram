use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: i32,
    pub average_rating: Option<rust_decimal::Decimal>,
    pub total_ratings: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherRatingModel {
    pub id: i32,
    pub teacher_id: i32,
    pub user_id: i32,
    pub lesson_id: i32,
    pub rating: rust_decimal::Decimal,
    pub review: Option<String>,
    pub rating_categories: Option<serde_json::Value>,
    pub is_anonymous: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub certifications: Option<Vec<String>>,
    pub specialties: Option<Vec<String>>,
    pub experience_years: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TeacherLessonModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub current_students: i32,
    pub venue: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherRatingCreateRequest {
    pub teacher_id: i32,
    pub user_open_id: String,
    pub lesson_id: i32,
    pub rating: rust_decimal::Decimal,
    pub review: Option<String>,
    pub rating_categories: Option<serde_json::Value>,
    pub is_anonymous: Option<bool>,
}



// impl From<TeacherModel> for Teacher {
//     fn from(model: TeacherModel) -> Self {
//         Teacher {
//             id: model.id,
//             name: model.name,
//             description: model.description,
//             avatar_url: model.avatar_url,
//             bio: model.bio,
//             certifications: model.certifications,
//             specialties: model.specialties,
//             experience_years: model.experience_years,
//             average_rating: model.average_rating,
//             total_ratings: model.total_ratings,
//             created_at: model.created_at,
//             updated_at: model.updated_at,
//             is_active: model.is_active,
//         }
//     }
// }

// Database operations
pub async fn get_all_teachers(sqlx_pool: &Pool<Postgres>) -> Result<Vec<TeacherModel>, sqlx::Error> {
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
            COUNT(tr.id) as total_ratings,
            t.is_active, 
            t.created_at,
            t.updated_at
        FROM teachers t
        LEFT JOIN teacher_ratings tr ON t.id = tr.teacher_id
        GROUP BY t.id, t.name, t.description, t.avatar_url, t.bio, t.certifications, 
                 t.specialties, t.experience_years, t.is_active, t.created_at
        ORDER BY t.is_active DESC, COALESCE(AVG(tr.rating), 0.0) DESC, t.experience_years DESC
    "#;
    
    sqlx::query_as::<_, TeacherModel>(query)
        .fetch_all(sqlx_pool)
        .await
}

pub async fn create_teacher(data: &TeacherCreateRequest, sqlx_pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let query = r#"
        INSERT INTO teachers (name, description, avatar_url, bio, certifications, specialties, experience_years)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
    "#;
    
    let row = sqlx::query_as::<_, (i32,)>(query)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.avatar_url)
        .bind(&data.bio)
        .bind(&data.certifications)
        .bind(&data.specialties)
        .bind(data.experience_years.unwrap_or(0))
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(row.0)
}

pub async fn update_teacher(data: &TeacherUpdateRequest, sqlx_pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
    let mut query_parts = vec!["UPDATE teachers SET updated_at = CURRENT_TIMESTAMP"];
    let mut bind_count = 1;
    
    // For simplicity, create a basic update query with all fields
    let query = r#"
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
    "#;
    
    let result = sqlx::query(query)
        .bind(data.id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.avatar_url)
        .bind(&data.bio)
        .bind(&data.certifications)
        .bind(&data.specialties)
        .bind(data.experience_years)
        .bind(data.is_active)
        .execute(sqlx_pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn delete_teacher(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<serde_json::Value, sqlx::Error> {
    // Check if teacher has any lessons assigned
    let check_lessons_query = "SELECT COUNT(*) as count FROM lessons WHERE teacher_id = $1 AND is_active = true";
    
    let (lesson_count,) = sqlx::query_as::<_, (i64,)>(check_lessons_query)
        .bind(id)
        .fetch_one(sqlx_pool)
        .await?;
    
    if lesson_count > 0 {
        // Teacher has active lessons, perform soft delete
        let soft_delete_query = "UPDATE teachers SET is_active = false, updated_at = CURRENT_TIMESTAMP WHERE id = $1";
        
        let result = sqlx::query(soft_delete_query)
            .bind(id)
            .execute(sqlx_pool)
            .await?;
        
        if result.rows_affected() > 0 {
            Ok(json!({
                "success": true, 
                "message": "Teacher deactivated successfully (has active lessons)",
                "soft_delete": true
            }))
        } else {
            Ok(json!({
                "success": false,
                "message": "Teacher not found"
            }))
        }
    } else {
        // Teacher has no active lessons, safe to hard delete
        let hard_delete_query = "DELETE FROM teachers WHERE id = $1";
        
        let result = sqlx::query(hard_delete_query)
            .bind(id)
            .execute(sqlx_pool)
            .await?;
        
        if result.rows_affected() > 0 {
            Ok(json!({
                "success": true, 
                "message": "Teacher deleted successfully",
                "soft_delete": false
            }))
        } else {
            Ok(json!({
                "success": false,
                "message": "Teacher not found"
            }))
        }
    }
}

pub async fn get_teacher_lessons(
    start_time: i32,
    end_time: i32,
    open_id: String,
    class_type: i32,
    teacher_id: i32,
    sqlx_pool: &Pool<Postgres>
) -> Result<serde_json::Value, sqlx::Error> {
    let query = "select * from fn_teacher_lessons($1,$2,$3,$4,$5)";
    
    sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(start_time)
        .bind(end_time)
        .bind(open_id)
        .bind(class_type)
        .bind(teacher_id)
        .fetch_one(sqlx_pool)
        .await
}