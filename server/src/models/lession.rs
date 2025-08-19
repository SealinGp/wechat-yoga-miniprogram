use std::process::id;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool as sPool, Pool, Postgres, FromRow, Row};
pub use crate::handlers::models::Lesson;
use crate::handlers::models::Teacher;

// Helper function to convert a database row to a Lesson struct
fn row_to_lession(row: &sqlx::postgres::PgRow) -> Lesson {
    let teacher = if let Some(teacher_id) = row.get::<Option<i32>, _>("teacher_id") {
        Some(crate::handlers::models::Teacher {
            id: teacher_id,
            name: row.get("teacher_name"),
            description: row.get("teacher_description"),
            avatar_url: row.get("teacher_avatar_url"),
            bio: row.get("teacher_bio"),
            certifications: row.get("teacher_certifications"),
            specialties: row.get("teacher_specialties"),
            experience_years: row.get("teacher_experience_years"),
            average_rating: row.get("teacher_average_rating"),
            total_ratings: row.get("teacher_total_ratings"),
            created_at: row.get("teacher_created_at"),
            updated_at: row.get("teacher_updated_at"),
            is_active: row.get("teacher_is_active"),
        })
    } else {
        None
    };
    
    let location = if let Some(location_id) = row.get::<Option<i32>, _>("location_id") {
        Some(crate::handlers::models::Location {
            id: location_id,
            name: row.get("location_name"),
            description: row.get("location_description"),
            capacity: row.get("location_capacity"),
            equipment: row.get("location_equipment"),
            facilities: row.get("location_facilities"),
            floor_number: row.get("location_floor_number"),
            room_number: row.get("location_room_number"),
            is_accessible: row.get("location_is_accessible"),
            booking_enabled: row.get("location_booking_enabled"),
            hourly_rate: row.get("location_hourly_rate"),
            images: row.get("location_images"),
            created_at: row.get("location_created_at"),
            updated_at: row.get("location_updated_at"),
            is_active: row.get("location_is_active"),
        })
    } else {
        None
    };

    let t = teacher.unwrap_or_default();
    let l = location.unwrap_or_default();
    Lesson {
        id: row.get("id"),
        title: row.get("title"),
        description: row.get("description"),
        teacher: t,
        location: l,
        lesson_type: row.get("lesson_type"),
        difficulty_level: row.get("difficulty_level"),
        start_time: row.get("start_time"),
        end_time: row.get("end_time"),
        max_students: row.get("max_students"),
        current_students: row.get("current_students"),
        price: row.get("price"),
        equipment_required: row.get("equipment_required"),
        prerequisites: row.get("prerequisites"),
        cancellation_policy: row.get("cancellation_policy"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        is_active: row.get("is_active"),
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LessonModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub location_id: Option<i32>,
    pub lesson_type: String,
    pub difficulty_level: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub max_students: i32,
    pub current_students: i32,
    pub price: Option<rust_decimal::Decimal>,
    pub equipment_required: Option<Vec<String>>,
    pub prerequisites: Option<String>,
    pub cancellation_policy: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}


pub async fn list_lessons(
    start: i32,   // unix seconds
    end: i32,     // unix seconds
    limit: i64,   // 分页限制
    offset: i64,  // 分页偏移
    sqlx_pool: &Pool<Postgres>,
) -> Result<Vec<Lesson>, sqlx::Error> {
    let query = r#"
        SELECT 
            l.id, l.title, l.description, l.lesson_type::TEXT, l.difficulty_level::TEXT,
            l.start_time, l.end_time, l.max_students, l.current_students,
            l.price, l.equipment_required, l.prerequisites, l.cancellation_policy,
            l.notes, l.created_at, l.updated_at, l.is_active,
            l.teacher_id, l.location_id,
            t.name as teacher_name, t.description as teacher_description,
            t.avatar_url as teacher_avatar_url, t.bio as teacher_bio,
            t.certifications as teacher_certifications, t.specialties as teacher_specialties,
            t.experience_years as teacher_experience_years, t.average_rating as teacher_average_rating,
            t.total_ratings as teacher_total_ratings, t.created_at as teacher_created_at,
            t.updated_at as teacher_updated_at, t.is_active as teacher_is_active,
            loc.name as location_name, loc.description as location_description,
            loc.capacity as location_capacity, loc.equipment as location_equipment,
            loc.facilities as location_facilities, loc.floor_number as location_floor_number,
            loc.room_number as location_room_number, loc.is_accessible as location_is_accessible,
            loc.booking_enabled as location_booking_enabled, loc.hourly_rate as location_hourly_rate,
            loc.images as location_images, loc.created_at as location_created_at,
            loc.updated_at as location_updated_at, loc.is_active as location_is_active
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.id
        LEFT JOIN locations loc ON l.location_id = loc.id
        WHERE l.is_active = true
          AND l.start_time BETWEEN to_timestamp($1) AND to_timestamp($2)
        ORDER BY l.start_time
        LIMIT $3 OFFSET $4
    "#;
    
    let lesson_rows = sqlx::query(query)
        .bind(start)   // $1 -> start (unix seconds)
        .bind(end)     // $2 -> end   (unix seconds)
        .bind(limit)   // $3 -> LIMIT
        .bind(offset)  // $4 -> OFFSET
        .fetch_all(sqlx_pool)
        .await?;
    
    let mut lessons = Vec::new();
    for row in lesson_rows {
        lessons.push(row_to_lession(&row));
    }
    
    Ok(lessons)
}


pub async fn get_lesson_by_id(id: i32, sqlxPool: &sPool<Postgres>) -> Result<Option<Lesson>, sqlx::Error> {
    let query = r#"
        SELECT 
            l.id, l.title, l.description, l.lesson_type::TEXT, l.difficulty_level::TEXT,
            l.start_time, l.end_time, l.max_students, l.current_students,
            l.price, l.equipment_required, l.prerequisites, l.cancellation_policy,
            l.notes, l.created_at, l.updated_at, l.is_active,
            l.teacher_id, l.location_id,
            t.name as teacher_name, t.description as teacher_description,
            t.avatar_url as teacher_avatar_url, t.bio as teacher_bio,
            t.certifications as teacher_certifications, t.specialties as teacher_specialties,
            t.experience_years as teacher_experience_years, t.average_rating as teacher_average_rating,
            t.total_ratings as teacher_total_ratings, t.created_at as teacher_created_at,
            t.updated_at as teacher_updated_at, t.is_active as teacher_is_active,
            loc.name as location_name, loc.description as location_description,
            loc.capacity as location_capacity, loc.equipment as location_equipment,
            loc.facilities as location_facilities, loc.floor_number as location_floor_number,
            loc.room_number as location_room_number, loc.is_accessible as location_is_accessible,
            loc.booking_enabled as location_booking_enabled, loc.hourly_rate as location_hourly_rate,
            loc.images as location_images, loc.created_at as location_created_at,
            loc.updated_at as location_updated_at, loc.is_active as location_is_active
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.id
        LEFT JOIN locations loc ON l.location_id = loc.id
        WHERE l.id = $1
    "#;
    
    let lesson_row = sqlx::query(query)
        .bind(id)
        .fetch_optional(sqlxPool).await?;
    
    if let Some(row) = lesson_row {
        Ok(Some(row_to_lession(&row)))
    } else {
        Ok(None)
    }
}

pub async fn update_lesson_status(id: i32, is_active: bool, sqlxPool: &sPool<Postgres>) -> Result<bool, sqlx::Error> {
    let query = "UPDATE lessons SET is_active = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $1";
    
    let result = sqlx::query(query)
        .bind(id)
        .bind(is_active)
        .execute(sqlxPool).await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn delete_lesson(id: i32, sqlxPool: &sPool<Postgres>) -> Result<bool, sqlx::Error> {
    let query = "DELETE FROM lessons WHERE id = $1";
    
    let result = sqlx::query(query)
        .bind(id)
        .execute(sqlxPool).await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn get_lessons_with_teachers(teacher_id: i32, sqlxPool: &sPool<Postgres>) -> Result<Vec<Lesson>, sqlx::Error> {
    let query = r#"
        SELECT 
            l.id, l.title, l.description, l.lesson_type::TEXT, l.difficulty_level::TEXT,
            l.start_time, l.end_time, l.max_students, l.current_students,
            l.price, l.equipment_required, l.prerequisites, l.cancellation_policy,
            l.notes, l.created_at, l.updated_at, l.is_active,
            l.teacher_id, l.location_id,
            t.name as teacher_name, t.description as teacher_description,
            t.avatar_url as teacher_avatar_url, t.bio as teacher_bio,
            t.certifications as teacher_certifications, t.specialties as teacher_specialties,
            t.experience_years as teacher_experience_years, t.average_rating as teacher_average_rating,
            t.total_ratings as teacher_total_ratings, t.created_at as teacher_created_at,
            t.updated_at as teacher_updated_at, t.is_active as teacher_is_active,
            loc.name as location_name, loc.description as location_description,
            loc.capacity as location_capacity, loc.equipment as location_equipment,
            loc.facilities as location_facilities, loc.floor_number as location_floor_number,
            loc.room_number as location_room_number, loc.is_accessible as location_is_accessible,
            loc.booking_enabled as location_booking_enabled, loc.hourly_rate as location_hourly_rate,
            loc.images as location_images, loc.created_at as location_created_at,
            loc.updated_at as location_updated_at, loc.is_active as location_is_active
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.id
        LEFT JOIN locations loc ON l.location_id = loc.id
        WHERE l.teacher_id = $1
        ORDER BY l.start_time
    "#;
    
    let lesson_rows = sqlx::query(query)
        .bind(teacher_id)
        .fetch_all(sqlxPool).await?;
    
    let mut lessons = Vec::new();
    
    for row in lesson_rows {
        lessons.push(row_to_lession(&row));
    }
    
    Ok(lessons)
}



// Simple struct for lesson update requests
#[derive(Debug, Serialize, Deserialize)]
pub struct LessonUpdateData {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub teacher_id: Option<i32>,
    pub location_id: Option<i32>,
    pub lesson_type: Option<String>,
    pub difficulty_level: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub max_students: Option<i32>,
    pub price: Option<rust_decimal::Decimal>,
    pub equipment_required: Option<Vec<String>>,
    pub prerequisites: Option<String>,
    pub cancellation_policy: Option<String>,
    pub notes: Option<String>,
    pub is_active: Option<bool>,
}

// Create a new lesson
pub async fn create_lesson(data: &Lesson, sqlx_pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let query = r#"
        INSERT INTO lessons (
            title, description, teacher_id, location_id, lesson_type, difficulty_level,
            start_time, end_time, max_students, current_students, price, equipment_required,
            prerequisites, cancellation_policy, notes, is_active, created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5::lesson_type, $6::difficulty_level,
            $7, $8, $9, 0, $10, $11,
            $12, $13, $14, $15, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        ) RETURNING id
    "#;
    
    
    let result = sqlx::query_scalar::<_, i32>(query)
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.teacher.id)
        .bind(data.location.id)
        .bind(&data.lesson_type)
        .bind(&data.difficulty_level)
        .bind(&data.start_time)
        .bind(&data.end_time)
        .bind(data.max_students)
        .bind(data.price)
        .bind(&data.equipment_required)
        .bind(&data.prerequisites)
        .bind(&data.cancellation_policy)
        .bind(&data.notes)
        .bind(data.is_active)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(result)
}

pub async fn update_lesson_data(data: &LessonUpdateData, sqlx_pool: &Pool<Postgres>) -> Result<bool, sqlx::Error> {
    let query = r#"
        UPDATE lessons SET
            title = COALESCE($2, title),
            description = COALESCE($3, description),
            teacher_id = COALESCE($4, teacher_id),
            location_id = COALESCE($5, location_id),
            lesson_type = COALESCE($6::lesson_type, lesson_type),
            difficulty_level = COALESCE($7::difficulty_level, difficulty_level),
            start_time = COALESCE($8, start_time),
            end_time = COALESCE($9, end_time),
            max_students = COALESCE($10, max_students),
            price = COALESCE($11, price),
            equipment_required = COALESCE($12, equipment_required),
            prerequisites = COALESCE($13, prerequisites),
            cancellation_policy = COALESCE($14, cancellation_policy),
            notes = COALESCE($15, notes),
            is_active = COALESCE($16, is_active),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
    "#;
    
    let result = sqlx::query(query)
        .bind(data.id)
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.teacher_id)
        .bind(data.location_id)
        .bind(&data.lesson_type)
        .bind(&data.difficulty_level)
        .bind(&data.start_time)
        .bind(&data.end_time)
        .bind(data.max_students)
        .bind(data.price)
        .bind(&data.equipment_required)
        .bind(&data.prerequisites)
        .bind(&data.cancellation_policy)
        .bind(&data.notes)
        .bind(data.is_active)
        .execute(sqlx_pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}