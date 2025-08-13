use crate::utils::client_real_addr::ClientRealAddr;
use sqlx::{Pool as sPool, Postgres};
use rocket::http::Status;
use rocket::State;
use serde_json::Value;

#[post("/yoga/debug", data = "<data>")]
pub async fn debug(
    client_addr: &ClientRealAddr,
    data: String,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    // 解析 JSON 数据
    let json_data: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid JSON data: {}", data);
            return Ok("0".to_string());
        }
    };
    
    // 直接插入调试信息，不使用存储过程
    let query = r#"
        INSERT INTO debug_logs (
            open_id, brand, model, pixel_ratio, screen_height, screen_width,
            version, sdk_version, platform, ip_address, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP)
    "#;
    
    let open_id = json_data["open_id"].as_str().unwrap_or("");
    let brand = json_data["brand"].as_str().unwrap_or("");
    let model = json_data["model"].as_str().unwrap_or("");
    let pixel_ratio: f64 = json_data["pixel_ratio"].as_f64().unwrap_or(0.0);
    let screen_height: i32 = json_data["screen_height"].as_i64().unwrap_or(0) as i32;
    let screen_width: i32 = json_data["screen_width"].as_i64().unwrap_or(0) as i32;
    let version = json_data["version"].as_str().unwrap_or("");
    let sdk_version = json_data["sdk_version"].as_str().unwrap_or("");
    let platform = json_data["platform"].as_str().unwrap_or("");
    let client_ip = client_addr.ip.to_string();
    
    match sqlx::query(query)
        .bind(open_id)
        .bind(brand)
        .bind(model)
        .bind(pixel_ratio)
        .bind(screen_height)
        .bind(screen_width)
        .bind(version)
        .bind(sdk_version)
        .bind(platform)
        .bind(client_ip)
        .execute(sqlxPool.inner()).await {
        Ok(_) => Ok("1".to_string()),
        Err(error) => {
            println!("Error inserting debug log: {}", error);
            Ok("0".to_string()) // 返回 0 表示插入失败，但不影响主流程
        }
    }
}