use rocket::http::Status;
use rocket::State;
use serde_json::json;
use sqlx::{Pool as sPool, Postgres};
use crate::models::action_button;

// 获取所有功能按钮
#[get("/yoga/action-buttons")]
pub async fn get_action_buttons(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match action_button::get_all_action_buttons(sqlxPool.inner()).await {
        Ok(result) => Ok(json!(result).to_string()),
        Err(error) => {
            println!("Error querying action buttons: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取活跃的功能按钮
#[get("/yoga/action-buttons/active")]
pub async fn get_active_action_buttons(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match action_button::get_active_action_buttons(sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error querying active action buttons: {}", error);
            Ok("[]".to_string())
        }
    }
}



// 更新功能按钮
#[put("/yoga/action-buttons/<id>", data = "<data>")]
pub async fn update_action_button(id: i32, data: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
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

    match action_button::update_action_button(id, json_data, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
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
    match action_button::delete_action_button(id, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error deleting action button: {}", error);
            Ok(json!({
                "success": false,
                "message": "Failed to delete action button"
            }).to_string())
        }
    }
}