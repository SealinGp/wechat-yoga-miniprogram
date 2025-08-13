use rocket::http::Status;
use rocket::State;
use serde_json::json;
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;

#[derive(FromRow)]
struct JsonResult {
    buttons: serde_json::Value,
}

// 获取所有功能按钮
#[get("/yoga/action-buttons")]
pub async fn get_action_buttons(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'icon', icon,
                'link', link,
                'sort_order', sort_order,
                'is_active', is_active
            ) ORDER BY sort_order ASC
        ), '[]'::json) as buttons
        FROM action_buttons
    "#;

    match sqlx::query_as::<_, JsonResult>(query).fetch_one(sqlxPool.inner()).await {
        Ok(result) => {
            Ok(result.buttons.to_string())
        }
        Err(error) => {
            println!("Error querying action buttons: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取活跃的功能按钮
#[get("/yoga/action-buttons/active")]
pub async fn get_active_action_buttons(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'icon', icon,
                'link', link,
                'sort_order', sort_order
            ) ORDER BY sort_order ASC
        ), '[]'::json) as buttons
        FROM action_buttons 
        WHERE is_active = true
    "#;

    match sqlx::query_as::<_, JsonResult>(query).fetch_one(sqlxPool.inner()).await {
        Ok(result) => {
            Ok(result.buttons.to_string())
        }
        Err(error) => {
            println!("Error querying active action buttons: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 创建新的功能按钮
#[derive(FromRow)]
struct CreatedButton {
    id: i32,
}

#[post("/yoga/action-buttons", data = "<data>")]
pub async fn create_action_button(data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 解析 JSON 数据
    let json_data: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid JSON data: {}", data);
            return Ok(json!({
                "success": false,
                "message": "Invalid JSON format"
            }).to_string());
        }
    };

    let query = r#"
        INSERT INTO action_buttons (name, icon, link, sort_order, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
    "#;

    let name = json_data["name"].as_str().unwrap_or("");
    let icon = json_data["icon"].as_str();
    let link = json_data["link"].as_str().unwrap_or("");
    let sort_order = json_data["sort_order"].as_i64().unwrap_or(0) as i32;
    let is_active = json_data["is_active"].as_bool().unwrap_or(true);

    if name.is_empty() || link.is_empty() {
        return Ok(json!({
            "success": false,
            "message": "Name and link are required"
        }).to_string());
    }

    match sqlx::query_as::<_, CreatedButton>(query)
        .bind(name)
        .bind(icon)
        .bind(link)
        .bind(sort_order)
        .bind(is_active)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => {
            Ok(json!({
                "success": true,
                "id": result.id,
                "message": "Action button created successfully"
            }).to_string())
        }
        Err(error) => {
            println!("Error creating action button: {}", error);
            Ok(json!({
                "success": false,
                "message": "Failed to create action button"
            }).to_string())
        }
    }
}

// 更新功能按钮
#[put("/yoga/action-buttons/<id>", data = "<data>")]
pub async fn update_action_button(id: i32, data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    // 解析 JSON 数据
    let json_data: serde_json::Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid JSON data: {}", data);
            return Ok(json!({
                "success": false,
                "message": "Invalid JSON format"
            }).to_string());
        }
    };

    let query = r#"
        UPDATE action_buttons 
        SET name = COALESCE($2, name),
            icon = COALESCE($3, icon),
            link = COALESCE($4, link),
            sort_order = COALESCE($5, sort_order),
            is_active = COALESCE($6, is_active)
        WHERE id = $1
        RETURNING id
    "#;

    let name = json_data["name"].as_str();
    let icon = json_data["icon"].as_str();
    let link = json_data["link"].as_str();
    let sort_order = json_data["sort_order"].as_i64().map(|x| x as i32);
    let is_active = json_data["is_active"].as_bool();

    match sqlx::query_as::<_, CreatedButton>(query)
        .bind(id)
        .bind(name)
        .bind(icon)
        .bind(link)
        .bind(sort_order)
        .bind(is_active)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(_)) => {
            Ok(json!({
                "success": true,
                "message": "Action button updated successfully"
            }).to_string())
        }
        Ok(None) => {
            Ok(json!({
                "success": false,
                "message": "Action button not found"
            }).to_string())
        }
        Err(error) => {
            println!("Error updating action button: {}", error);
            Ok(json!({
                "success": false,
                "message": "Failed to update action button"
            }).to_string())
        }
    }
}

// 删除功能按钮
#[delete("/yoga/action-buttons/<id>")]
pub async fn delete_action_button(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM action_buttons WHERE id = $1";

    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({
                    "success": true,
                    "message": "Action button deleted successfully"
                }).to_string())
            } else {
                Ok(json!({
                    "success": false,
                    "message": "Action button not found"
                }).to_string())
            }
        }
        Err(error) => {
            println!("Error deleting action button: {}", error);
            Ok(json!({
                "success": false,
                "message": "Failed to delete action button"
            }).to_string())
        }
    }
}