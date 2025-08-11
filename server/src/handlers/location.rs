use crate::models::location::*;
use deadpool_postgres::Pool;
use rocket::http::Status;
use rocket::State;
use serde_json::json;

// 获取所有地点列表
#[get("/yoga/locations")]
pub async fn get_locations(pool: &State<Pool>) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
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

            match conn.query_one(query, &[]).await {
                Ok(row) => {
                    let locations = row.get::<_, serde_json::Value>("locations");
                    Ok(locations.to_string())
                }
                Err(error) => {
                    println!("Error querying locations: {}", error);
                    Ok("[]".to_string())
                }
            }
        }
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// 获取可用地点（支持预约的）
#[get("/yoga/locations/available")]
pub async fn get_available_locations(pool: &State<Pool>) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
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

            match conn.query_one(query, &[]).await {
                Ok(row) => {
                    let locations = row.get::<_, serde_json::Value>("locations");
                    Ok(locations.to_string())
                }
                Err(error) => {
                    println!("Error querying available locations: {}", error);
                    Ok("[]".to_string())
                }
            }
        }
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// 检查地点在特定时间段的可用性
#[get("/yoga/locations/availability?<location_id>&<start_time>&<end_time>")]
pub async fn check_location_availability(
    location_id: i32,
    start_time: String, // ISO 8601 format
    end_time: String,   // ISO 8601 format
    pool: &State<Pool>,
) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
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

            let (location_name, booking_enabled) = match conn.query_opt(location_query, &[&location_id]).await {
                Ok(Some(row)) => (
                    row.get::<_, String>("name"),
                    row.get::<_, bool>("booking_enabled")
                ),
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

            if !booking_enabled {
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

            let conflicting_lessons = match conn.query(conflict_query, &[&location_id, &start_time, &end_time]).await {
                Ok(rows) => {
                    rows.into_iter().map(|row| {
                        json!({
                            "id": row.get::<_, i32>("id"),
                            "title": row.get::<_, String>("title"),
                            "start_time": row.get::<_, chrono::DateTime<chrono::Utc>>("start_time").timestamp(),
                            "end_time": row.get::<_, chrono::DateTime<chrono::Utc>>("end_time").timestamp(),
                            "teacher_name": row.get::<_, Option<String>>("teacher_name")
                        })
                    }).collect::<Vec<_>>()
                }
                Err(error) => {
                    println!("Error checking conflicts: {}", error);
                    return Err(Status::InternalServerError);
                }
            };

            let is_available = conflicting_lessons.is_empty();

            Ok(json!({
                "success": true,
                "location_id": location_id,
                "location_name": location_name,
                "is_available": is_available,
                "conflicting_lessons": conflicting_lessons
            }).to_string())
        }
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

// 获取地点使用统计
#[get("/yoga/locations/<id>/stats")]
pub async fn get_location_stats(id: i32, pool: &State<Pool>) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
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

            match conn.query_opt(query, &[&id]).await {
                Ok(Some(row)) => {
                    let stats = json!({
                        "location_id": row.get::<_, i32>("id"),
                        "location_name": row.get::<_, String>("name"),
                        "capacity": row.get::<_, i32>("capacity"),
                        "total_lessons": row.get::<_, i64>("total_lessons"),
                        "active_lessons": row.get::<_, i64>("active_lessons"),
                        "avg_lesson_duration": row.get::<_, f64>("avg_lesson_duration"),
                        "total_bookings": row.get::<_, i64>("total_bookings")
                    });
                    Ok(stats.to_string())
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
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}