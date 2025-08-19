use crate::models::location::*;
use sqlx::{Pool as sPool, Postgres, FromRow};
use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;

// 获取所有地点列表
#[get("/yoga/locations")]
pub async fn get_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::get_all_locations(sqlxPool.inner()).await {
        Ok(locations) => Ok(locations.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取可用地点（支持预约的）
#[get("/yoga/locations/available")]
pub async fn get_available_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::get_available_locations(sqlxPool.inner()).await {
        Ok(locations) => Ok(locations.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
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

    match crate::models::location::check_location_availability(location_id, start_time, end_time, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// 获取地点使用统计
#[get("/yoga/locations/<id>/stats")]
pub async fn get_location_stats(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::get_location_statistics(id, sqlxPool.inner()).await {
        Ok(Some(stats)) => Ok(stats.to_string()),
        Ok(None) => {
            Ok(json!({
                "success": false,
                "message": "Location not found"
            }).to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// Admin CRUD operations

#[get("/api/locations")]
pub async fn get_admin_locations(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::get_all_admin_locations(sqlxPool.inner()).await {
        Ok(locations) => {
            // Convert to response format
            let response_locations: Vec<LocationAdmin> = locations.into_iter().map(|loc| LocationAdmin {
                id: Some(loc.id),
                name: loc.name,
                description: loc.description,
                capacity: loc.capacity,
                equipment: loc.equipment,
                facilities: loc.facilities,
                floor_number: loc.floor_number.unwrap_or(1),
                room_number: loc.room_number.unwrap_or("001".to_string()),
                is_accessible: Some(loc.is_accessible),
                booking_enabled: Some(loc.booking_enabled),
                hourly_rate: loc.hourly_rate,
                images: loc.images,
                is_active: Some(loc.is_active),
                created_at: Some(loc.created_at),
                updated_at: Some(loc.updated_at),
            }).collect();
            
            match serde_json::to_string(&response_locations) {
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

// 获取所有地点列表
#[get("/api/admin/locations")]
pub async fn get_locations1(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::get_all_locations(sqlxPool.inner()).await {
        Ok(locations) => Ok(locations.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}

#[post("/api/admin/locations", data = "<location_request>")]
pub async fn create_location(
    location_request: rocket::serde::json::Json<CreateLocationRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let create_request = LocationCreateRequest {
        name: location_request.name.clone(),
        description: location_request.description.clone(),
        capacity: location_request.capacity,
        equipment: location_request.equipment.clone(),
        facilities: location_request.facilities.clone(),
        floor_number: Some(location_request.floor_number),
        room_number: Some(location_request.room_number.clone()),
        is_accessible: location_request.is_accessible,
        booking_enabled: location_request.booking_enabled,
        hourly_rate: location_request.hourly_rate,
        images: location_request.images.clone(),
    };
    
    match crate::models::location::create_location(&create_request, sqlxPool.inner()).await {
        Ok(location) => {
            // Convert to response format
            let response_location = LocationAdmin {
                id: Some(location.id),
                name: location.name,
                description: location.description,
                capacity: location.capacity,
                equipment: location.equipment,
                facilities: location.facilities,
                floor_number: location.floor_number.unwrap_or(1),
                room_number: location.room_number.unwrap_or("001".to_string()),
                is_accessible: Some(location.is_accessible),
                booking_enabled: Some(location.booking_enabled),
                hourly_rate: location.hourly_rate,
                images: location.images,
                is_active: Some(location.is_active),
                created_at: Some(location.created_at),
                updated_at: Some(location.updated_at),
            };
            
            match serde_json::to_string(&response_location) {
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

#[put("/api/admin/locations/<id>", data = "<location_request>")]
pub async fn update_location(
    id: i32,
    location_request: rocket::serde::json::Json<UpdateLocationRequest>,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // Convert handler request to model request
    let update_request = LocationUpdateRequest {
        id,
        name: location_request.name.clone(),
        description: location_request.description.clone(),
        capacity: location_request.capacity,
        equipment: location_request.equipment.clone(),
        facilities: location_request.facilities.clone(),
        floor_number: location_request.floor_number,
        room_number: location_request.room_number.clone(),
        is_accessible: location_request.is_accessible,
        booking_enabled: location_request.booking_enabled,
        hourly_rate: location_request.hourly_rate,
        images: location_request.images.clone(),
        is_active: location_request.is_active,
    };
    
    match crate::models::location::update_location(&update_request, sqlxPool.inner()).await {
        Ok(Some(location)) => {
            // Convert to response format
            let response_location = LocationAdmin {
                id: Some(location.id),
                name: location.name,
                description: location.description,
                capacity: location.capacity,
                equipment: location.equipment,
                facilities: location.facilities,
                floor_number: location.floor_number.unwrap_or(1),
                room_number: location.room_number.unwrap_or("001".to_string()),
                is_accessible: Some(location.is_accessible),
                booking_enabled: Some(location.booking_enabled),
                hourly_rate: location.hourly_rate,
                images: location.images,
                is_active: Some(location.is_active),
                created_at: Some(location.created_at),
                updated_at: Some(location.updated_at),
            };
            
            match serde_json::to_string(&response_location) {
                Ok(json) => Ok(json),
                Err(error) => {
                    println!("JSON serialization error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/locations/<id>")]
pub async fn delete_location(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match crate::models::location::delete_location(id, sqlxPool.inner()).await {
        Ok(response) => {
            Ok(response.to_string())
        }
        Err(error) => {
            println!("Database error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}