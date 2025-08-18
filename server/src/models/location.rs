use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: bool,
    pub booking_enabled: bool,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdateRequest {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationWithStats {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: Option<i32>,
    pub room_number: Option<String>,
    pub is_accessible: bool,
    pub booking_enabled: bool,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub is_active: bool,
    // 统计信息
    pub total_lessons: i64,
    pub active_lessons: i64,
    pub utilization_rate: Option<f64>, // 使用率百分比
}

// 地点可用性检查结果
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAvailability {
    pub location_id: i32,
    pub location_name: String,
    pub is_available: bool,
    pub conflicting_lessons: Vec<ConflictingLesson>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ConflictingLesson {
    pub id: i32,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub teacher_name: Option<String>,
}

// 地点使用统计
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LocationUsageStats {
    pub location_id: i32,
    pub location_name: String,
    pub total_bookings: i64,
    pub total_hours: f64,
    pub avg_utilization: f64,
    pub peak_hours: Vec<i32>, // 高峰时段（小时）
    pub most_popular_lesson_type: Option<String>,
}

// Helper structs for database operations
#[derive(sqlx::FromRow)]
pub struct LocationInfo {
    pub name: String,
    pub booking_enabled: bool,
}

#[derive(sqlx::FromRow)]
pub struct LocationStats {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub total_lessons: i64,
    pub active_lessons: i64,
    pub avg_lesson_duration: f64,
    pub total_bookings: i64,
}

// Database operations
pub async fn get_all_locations(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'description', description,
                'capacity', capacity,
                'equipment', equipment,
                'facilities', facilities,
                'floor_number', floor_number,
                'room_number', room_number,
                'is_accessible', is_accessible,
                'booking_enabled', booking_enabled,
                'hourly_rate', hourly_rate,
                'images', images,
                'is_active', is_active
            ) ORDER BY floor_number ASC, room_number ASC
        ), '[]'::json) as locations
        FROM locations 
        WHERE is_active = true
    "#;

    sqlx::query_scalar::<_, Value>(query)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn get_available_locations(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'description', description,
                'capacity', capacity,
                'equipment', equipment,
                'facilities', facilities,
                'floor_number', floor_number,
                'room_number', room_number,
                'is_accessible', is_accessible,
                'images', images
            ) ORDER BY floor_number ASC, room_number ASC
        ), '[]'::json) as locations
        FROM locations 
        WHERE is_active = true AND booking_enabled = true
    "#;

    sqlx::query_scalar::<_, Value>(query)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn check_location_availability(
    location_id: i32,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    sqlx_pool: &Pool<Postgres>,
) -> Result<Value, sqlx::Error> {
    // Check if location exists and is bookable
    let location_query = r#"
        SELECT name, booking_enabled 
        FROM locations 
        WHERE id = $1 AND is_active = true
    "#;

    let location_info = sqlx::query_as::<_, LocationInfo>(location_query)
        .bind(location_id)
        .fetch_optional(sqlx_pool)
        .await?;

    let location_info = match location_info {
        Some(info) => info,
        None => {
            return Ok(json!({
                "success": false,
                "message": "Location not found"
            }));
        }
    };

    if !location_info.booking_enabled {
        return Ok(json!({
            "success": false,
            "message": "Location is not available for booking"
        }));
    }

    // Check for conflicting lessons
    let conflict_query = r#"
        SELECT l.id, l.title, l.start_time, l.end_time, t.name as teacher_name
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.id
        WHERE l.location_id = $1 
          AND l.is_active = true
          AND (
            (l.start_time <= $2 AND l.end_time > $2) OR
            (l.start_time < $3 AND l.end_time >= $3) OR
            (l.start_time >= $2 AND l.end_time <= $3)
          )
        ORDER BY l.start_time ASC
    "#;

    let conflicting_lessons = sqlx::query_as::<_, ConflictingLesson>(conflict_query)
        .bind(location_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(sqlx_pool)
        .await?;

    let conflicts: Vec<Value> = conflicting_lessons.into_iter().map(|lesson| {
        json!({
            "id": lesson.id,
            "title": lesson.title,
            "start_time": lesson.start_time.timestamp(),
            "end_time": lesson.end_time.timestamp(),
            "teacher_name": lesson.teacher_name
        })
    }).collect();

    let is_available = conflicts.is_empty();

    Ok(json!({
        "success": true,
        "location_id": location_id,
        "location_name": location_info.name,
        "is_available": is_available,
        "conflicting_lessons": conflicts
    }))
}

pub async fn get_location_statistics(location_id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Option<Value>, sqlx::Error> {
    let query = r#"
        SELECT 
            l.id,
            l.name,
            l.capacity,
            COUNT(lessons.id) as total_lessons,
            COUNT(lessons.id) FILTER (WHERE lessons.is_active = true AND lessons.start_time > CURRENT_TIMESTAMP) as active_lessons,
            COALESCE(AVG(EXTRACT(EPOCH FROM (lessons.end_time - lessons.start_time))/3600.0), 0) as avg_lesson_duration,
            COUNT(b.id) as total_bookings
        FROM locations l
        LEFT JOIN lessons ON l.id = lessons.location_id
        LEFT JOIN bookings b ON lessons.id = b.lesson_id AND b.status = 'confirmed'
        WHERE l.id = $1
        GROUP BY l.id, l.name, l.capacity
    "#;

    let stats = sqlx::query_as::<_, LocationStats>(query)
        .bind(location_id)
        .fetch_optional(sqlx_pool)
        .await?;

    match stats {
        Some(stats) => {
            Ok(Some(json!({
                "location_id": stats.id,
                "location_name": stats.name,
                "capacity": stats.capacity,
                "total_lessons": stats.total_lessons,
                "active_lessons": stats.active_lessons,
                "avg_lesson_duration": stats.avg_lesson_duration,
                "total_bookings": stats.total_bookings
            })))
        }
        None => Ok(None)
    }
}

// Admin CRUD operations
pub async fn get_all_admin_locations(sqlx_pool: &Pool<Postgres>) -> Result<Vec<LocationModel>, sqlx::Error> {
    let query = r#"
        SELECT id, name, description, capacity, equipment, facilities, floor_number,
               room_number, is_accessible, booking_enabled, hourly_rate, images, is_active,
               created_at, updated_at
        FROM locations
        ORDER BY floor_number ASC, room_number ASC
    "#;
    
    sqlx::query_as::<_, LocationModel>(query)
        .fetch_all(sqlx_pool)
        .await
}

pub async fn create_location(data: &LocationCreateRequest, sqlx_pool: &Pool<Postgres>) -> Result<LocationModel, sqlx::Error> {
    let query = r#"
        INSERT INTO locations (name, description, capacity, equipment, facilities, floor_number, 
                              room_number, is_accessible, booking_enabled, hourly_rate, images)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, name, description, capacity, equipment, facilities, floor_number,
                 room_number, is_accessible, booking_enabled, hourly_rate, images, is_active,
                 created_at, updated_at
    "#;
    
    sqlx::query_as::<_, LocationModel>(query)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.capacity)
        .bind(&data.equipment)
        .bind(&data.facilities)
        .bind(data.floor_number.unwrap_or(1))
        .bind(&data.room_number.as_ref().unwrap_or(&"001".to_string()))
        .bind(data.is_accessible.unwrap_or(false))
        .bind(data.booking_enabled.unwrap_or(true))
        .bind(data.hourly_rate)
        .bind(&data.images)
        .fetch_one(sqlx_pool)
        .await
}

pub async fn update_location(data: &LocationUpdateRequest, sqlx_pool: &Pool<Postgres>) -> Result<Option<LocationModel>, sqlx::Error> {
    let query = r#"
        UPDATE locations 
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            capacity = COALESCE($4, capacity),
            equipment = COALESCE($5, equipment),
            facilities = COALESCE($6, facilities),
            floor_number = COALESCE($7, floor_number),
            room_number = COALESCE($8, room_number),
            is_accessible = COALESCE($9, is_accessible),
            booking_enabled = COALESCE($10, booking_enabled),
            hourly_rate = COALESCE($11, hourly_rate),
            images = COALESCE($12, images),
            is_active = COALESCE($13, is_active),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, name, description, capacity, equipment, facilities, floor_number,
                 room_number, is_accessible, booking_enabled, hourly_rate, images, is_active,
                 created_at, updated_at
    "#;
    
    sqlx::query_as::<_, LocationModel>(query)
        .bind(data.id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.capacity)
        .bind(&data.equipment)
        .bind(&data.facilities)
        .bind(data.floor_number)
        .bind(&data.room_number)
        .bind(data.is_accessible)
        .bind(data.booking_enabled)
        .bind(data.hourly_rate)
        .bind(&data.images)
        .bind(data.is_active)
        .fetch_optional(sqlx_pool)
        .await
}

pub async fn delete_location(location_id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let query = "DELETE FROM locations WHERE id = $1";
    
    let result = sqlx::query(query)
        .bind(location_id)
        .execute(sqlx_pool)
        .await?;
    
    if result.rows_affected() > 0 {
        Ok(json!({"success": true, "message": "Location deleted successfully"}))
    } else {
        Ok(json!({"success": false, "message": "Location not found"}))
    }
}
