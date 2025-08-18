use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use serde_json::{json, Value};

use crate::handlers::admin_actions::Action;


#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonCreateRequest {
    pub name: String,
    pub icon: Option<String>,
    pub link: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonUpdateRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionButtonResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<i32>,
}

// Helper structs
#[derive(FromRow)]
pub struct JsonResult {
    pub buttons: Value,
}

#[derive(FromRow)]
pub struct CreatedButton {
    pub id: i32,
}

// Database operations
pub async fn get_all_action_buttons(sqlxPool: &Pool<Postgres>) -> Result<Vec<Action>, sqlx::Error> {
    let query = r#"
        SELECT id, name, icon, link, sort_order, is_active,
               created_at AT TIME ZONE 'Asia/Shanghai' as created_at
        FROM action_buttons
        ORDER BY sort_order ASC, created_at DESC
    "#;
    let rows = sqlx::query_as::<_,Action>(query).fetch_all(sqlxPool).await?;
    Ok(rows)
}

pub async fn get_active_action_buttons(sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
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

    let result = sqlx::query_as::<_, JsonResult>(query)
        .fetch_one(sqlx_pool)
        .await?;
    
    Ok(result.buttons)
}

pub async fn create_action_button(data: Action, sqlx_pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let query = r#"
        INSERT INTO action_buttons (name, icon, link, sort_order, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
    "#;

    let result = sqlx::query_as::<_, CreatedButton>(query)
        .bind(&data.name)
        .bind(&data.icon)
        .bind(&data.link)
        .bind(&data.sort_order)
        .bind(&data.is_active)
        .fetch_one(sqlx_pool)
        .await?;

    Ok(result.id)
}

pub async fn update_action_button(id: i32, data: Value, sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
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

    let name = data["name"].as_str();
    let icon = data["icon"].as_str();
    let link = data["link"].as_str();
    let sort_order = data["sort_order"].as_i64().map(|x| x as i32);
    let is_active = data["is_active"].as_bool();

    let result = sqlx::query_as::<_, CreatedButton>(query)
        .bind(id)
        .bind(name)
        .bind(icon)
        .bind(link)
        .bind(sort_order)
        .bind(is_active)
        .fetch_optional(sqlx_pool)
        .await?;

    if result.is_some() {
        Ok(json!({
            "success": true,
            "message": "Action button updated successfully"
        }))
    } else {
        Ok(json!({
            "success": false,
            "message": "Action button not found"
        }))
    }
}

pub async fn delete_action_button(id: i32, sqlx_pool: &Pool<Postgres>) -> Result<Value, sqlx::Error> {
    let query = "DELETE FROM action_buttons WHERE id = $1";

    let result = sqlx::query(query)
        .bind(id)
        .execute(sqlx_pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(json!({
            "success": true,
            "message": "Action button deleted successfully"
        }))
    } else {
        Ok(json!({
            "success": false,
            "message": "Action button not found"
        }))
    }
}