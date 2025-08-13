use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;

#[derive(FromRow)]
struct UserResult {
    result: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct UserIdResult {
    id: i32,
}

#[get("/yoga/user/query?<openid>")]
pub async fn user_query(openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT row_to_json(t) as result
        FROM (
            SELECT id, avatar_url, nick_name, 
                   0 as user_type
            FROM users
            WHERE open_id = $1
        ) as t
    "#;
    
    match sqlx::query_as::<_, UserResult>(query)
        .bind(&openid)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(row)) => {
            if let Some(result) = row.result {
                Ok(result.to_string())
            } else {
                Ok("null".to_string())
            }
        }
        Ok(None) => {
            // 用户不存在，返回空对象
            Ok("null".to_string())
        }
        Err(error) => {
            println!("Error querying user: {}", error);
            Err(Status::NoContent)
        }
    }
}
#[post("/yoga/user", data = "<data>")]
pub async fn register_user(data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 解析 JSON 数据
    let json_data: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid JSON data: {}", data);
            return Err(Status::BadRequest);
        }
    };
    
    let query = r#"
        INSERT INTO users (open_id, nick_name, avatar_url, phone, created_at, updated_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT (open_id) DO UPDATE SET
            nick_name = COALESCE(EXCLUDED.nick_name, users.nick_name),
            avatar_url = COALESCE(EXCLUDED.avatar_url, users.avatar_url),
            phone = COALESCE(EXCLUDED.phone, users.phone),
            updated_at = CURRENT_TIMESTAMP
        RETURNING id
    "#;
    
    let open_id = json_data["open_id"].as_str().unwrap_or("");
    let nick_name = json_data["nick_name"].as_str();
    let avatar_url = json_data["avatar_url"].as_str();
    let phone = json_data["phone"].as_str();
    
    match sqlx::query_as::<_, UserIdResult>(query)
        .bind(open_id)
        .bind(nick_name)
        .bind(avatar_url)
        .bind(phone)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => {
            Ok(row.id.to_string())
        }
        Err(error) => {
            println!("Error updating user: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/user/book/statistics?<id>")]
pub async fn user_book_statistics(id: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT row_to_json(t) as result
        FROM (
            SELECT u.id, u.avatar_url, u.nick_name,
                   0 as user_type,
                   COUNT(b.id) FILTER (WHERE b.status = 'confirmed') as total_bookings,
                   COUNT(b.id) FILTER (WHERE b.status = 'completed') as completed_classes,
                   COUNT(b.id) FILTER (WHERE b.status = 'cancelled') as cancelled_bookings
            FROM users u
            LEFT JOIN bookings b ON u.id = b.user_id
            WHERE u.open_id = $1
            GROUP BY u.id, u.avatar_url, u.nick_name
        ) as t
    "#;
    
    match sqlx::query_as::<_, UserResult>(query)
        .bind(&id)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(row)) => {
            if let Some(result) = row.result {
                Ok(result.to_string())
            } else {
                let empty_stats = json!({
                    "id": 0,
                    "avatar_url": null,
                    "nick_name": null,
                    "user_type": 0,
                    "total_bookings": 0,
                    "completed_classes": 0,
                    "cancelled_bookings": 0
                });
                Ok(empty_stats.to_string())
            }
        }
        Ok(None) => {
            // 用户不存在，返回空统计
            let empty_stats = json!({
                "id": 0,
                "avatar_url": null,
                "nick_name": null,
                "user_type": 0,
                "total_bookings": 0,
                "completed_classes": 0,
                "cancelled_bookings": 0
            });
            Ok(empty_stats.to_string())
        }
        Err(error) => {
            println!("Error querying user statistics: {}", error);
            Err(Status::InternalServerError)
        }
    }
}