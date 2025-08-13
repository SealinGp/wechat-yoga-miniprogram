use crate::models::location::*;
use sqlx::{Pool as sPool, Postgres, FromRow};
use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;

// 获取所有地点列表
#[get("/yoga/locations")]
pub async fn get_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
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

    match sqlx::query_scalar::<_, serde_json::Value>(query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(locations) => Ok(locations.to_string()),
        Err(error) => {
            println!("Error querying locations: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取可用地点（支持预约的）
#[get("/yoga/locations/available")]
pub async fn get_available_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
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

    match sqlx::query_scalar::<_, serde_json::Value>(query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(locations) => Ok(locations.to_string()),
        Err(error) => {
            println!("Error querying available locations: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 检查地点在特定时间段的可用性
#[get("/yoga/locations/availability?<location_id>&<start_time>&<end_time>")]
pub async fn check_location_availability(
    location_id: i32,
    start_time: String, // ISO 8601 format
    end_time: String,   // ISO 8601 format
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // 解析时间字符串
    let start_time = match chrono::DateTime::parse_from_rfc3339(&start_time) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => {
            return Ok(json!({
                "success": false,
                "message": "Invalid start_time format"
            }).to_string());
        }
    };

    let end_time = match chrono::DateTime::parse_from_rfc3339(&end_time) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => {
            return Ok(json!({
                "success": false,
                "message": "Invalid end_time format"
            }).to_string());
        }
    };

    // 检查地点是否存在且可预约
    let location_query = r#"
        SELECT name, booking_enabled 
        FROM locations 
        WHERE id = $1 AND is_active = true
    "#;

    #[derive(sqlx::FromRow)]
    struct LocationInfo {
        name: String,
        booking_enabled: bool,
    }

    let location_info = match sqlx::query_as::<_, LocationInfo>(location_query)
        .bind(location_id)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(info)) => info,
        Ok(None) => {
            return Ok(json!({
                "success": false,
                "message": "Location not found"
            }).to_string());
        }
        Err(error) => {
            println!("Error checking location: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    if !location_info.booking_enabled {
        return Ok(json!({
            "success": false,
            "message": "Location is not available for booking"
        }).to_string());
    }

    // 检查时间段内是否有冲突的课程
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

    #[derive(sqlx::FromRow)]
    struct ConflictingLesson {
        id: i32,
        title: String,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        teacher_name: Option<String>,
    }

    let conflicting_lessons = match sqlx::query_as::<_, ConflictingLesson>(conflict_query)
        .bind(location_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(sqlxPool.inner()).await {
        Ok(lessons) => lessons.into_iter().map(|lesson| {
            json!({
                "id": lesson.id,
                "title": lesson.title,
                "start_time": lesson.start_time.timestamp(),
                "end_time": lesson.end_time.timestamp(),
                "teacher_name": lesson.teacher_name
            })
        }).collect::<Vec<_>>(),
        Err(error) => {
            println!("Error checking conflicts: {}", error);
            return Err(Status::InternalServerError);
        }
    };

    let is_available = conflicting_lessons.is_empty();

    Ok(json!({
        "success": true,
        "location_id": location_id,
        "location_name": location_info.name,
        "is_available": is_available,
        "conflicting_lessons": conflicting_lessons
    }).to_string())
}

// 获取地点使用统计
#[get("/yoga/locations/<id>/stats")]
pub async fn get_location_stats(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
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

    #[derive(sqlx::FromRow)]
    struct LocationStats {
        id: i32,
        name: String,
        capacity: i32,
        total_lessons: i64,
        active_lessons: i64,
        avg_lesson_duration: f64,
        total_bookings: i64,
    }

    match sqlx::query_as::<_, LocationStats>(query)
        .bind(id)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(stats)) => {
            let result = json!({
                "location_id": stats.id,
                "location_name": stats.name,
                "capacity": stats.capacity,
                "total_lessons": stats.total_lessons,
                "active_lessons": stats.active_lessons,
                "avg_lesson_duration": stats.avg_lesson_duration,
                "total_bookings": stats.total_bookings
            });
            Ok(result.to_string())
        }
        Ok(None) => {
            Ok(json!({
                "success": false,
                "message": "Location not found"
            }).to_string())
        }
        Err(error) => {
            println!("Error querying location stats: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// Admin CRUD operations

#[get("/locations")]
pub async fn get_admin_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT id, name, description, capacity, equipment, facilities, floor_number,
               room_number, is_accessible, booking_enabled, hourly_rate, images, is_active,
               created_at, updated_at
        FROM locations
        ORDER BY floor_number ASC, room_number ASC
    "#;
    
    match sqlx::query_as::<_, LocationAdmin>(query).fetch_all(sqlxPool.inner()).await {
        Ok(locations) => {
            Ok(serde_json::to_string(&locations).unwrap())
        }
        Err(error) => {
            println!("Error querying locations: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct LocationAdmin {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: i32,
    pub room_number: String,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub equipment: Option<Vec<String>>,
    pub facilities: Option<Vec<String>>,
    pub floor_number: i32,
    pub room_number: String,
    pub is_accessible: Option<bool>,
    pub booking_enabled: Option<bool>,
    pub hourly_rate: Option<rust_decimal::Decimal>,
    pub images: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateLocationRequest {
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

#[post("/admin/locations", data = "<location_request>")]
pub async fn create_location(
    location_request: rocket::serde::json::Json<CreateLocationRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = r#"
        INSERT INTO locations (name, description, capacity, equipment, facilities, floor_number, 
                              room_number, is_accessible, booking_enabled, hourly_rate, images)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, name, description, capacity, equipment, facilities, floor_number,
                 room_number, is_accessible, booking_enabled, hourly_rate, images, is_active,
                 created_at, updated_at
    "#;
    
    match sqlx::query_as::<_, LocationAdmin>(query)
        .bind(&location_request.name)
        .bind(&location_request.description)
        .bind(location_request.capacity)
        .bind(&location_request.equipment)
        .bind(&location_request.facilities)
        .bind(location_request.floor_number)
        .bind(&location_request.room_number)
        .bind(location_request.is_accessible.unwrap_or(false))
        .bind(location_request.booking_enabled.unwrap_or(true))
        .bind(location_request.hourly_rate)
        .bind(&location_request.images)
        .fetch_one(sqlxPool.inner()).await {
        Ok(location) => {
            Ok(serde_json::to_string(&location).unwrap())
        }
        Err(error) => {
            println!("Error creating location: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/admin/locations/<id>", data = "<location_request>")]
pub async fn update_location(
    id: i32,
    location_request: rocket::serde::json::Json<UpdateLocationRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
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
    
    match sqlx::query_as::<_, LocationAdmin>(query)
        .bind(id)
        .bind(&location_request.name)
        .bind(&location_request.description)
        .bind(location_request.capacity)
        .bind(&location_request.equipment)
        .bind(&location_request.facilities)
        .bind(location_request.floor_number)
        .bind(&location_request.room_number)
        .bind(location_request.is_accessible)
        .bind(location_request.booking_enabled)
        .bind(location_request.hourly_rate)
        .bind(&location_request.images)
        .bind(location_request.is_active)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(location)) => {
            Ok(serde_json::to_string(&location).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating location: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/admin/locations/<id>")]
pub async fn delete_location(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM locations WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({"success": true, "message": "Location deleted successfully"}).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting location: {}", error);
            Err(Status::InternalServerError)
        }
    }
}