use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;

use crate::models::action_button::get_all_action_buttons;

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct Action {
    pub id: Option<i32>,
    pub name: String,
    pub icon: Option<String>,
    pub link: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateActionRequest {
    pub name: String,
    pub icon: Option<String>,
    pub link: String,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateActionRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub link: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[get("/api/admin/actions")]
pub async fn get_actions(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match get_all_action_buttons(sqlxPool.inner()).await {
        Ok(actions) => {
            Ok(serde_json::to_string(&actions).unwrap())
        }
        Err(error) => {
            println!("Error querying actions: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/api/admin/actions", data = "<request>")]
pub async fn create_action(
    request: rocket::serde::json::Json<CreateActionRequest>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    let query = r#"
        INSERT INTO action_buttons (name, icon, link, sort_order, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, icon, link, sort_order, is_active,
                  created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    let sort_order = request.sort_order.unwrap_or(0);
    
    match sqlx::query_as::<_, Action>(query)
        .bind(&request.name)
        .bind(&request.icon)
        .bind(&request.link)
        .bind(sort_order)
        .bind(true)
        .fetch_one(sqlxPool.inner()).await {
        Ok(action) => {
            Ok(serde_json::to_string(&action).unwrap())
        }
        Err(error) => {
            println!("Error creating action: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[put("/api/admin/actions/<id>", data = "<request>")]
pub async fn update_action(
    id: i32,
    request: rocket::serde::json::Json<UpdateActionRequest>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    let query = r#"
        UPDATE action_buttons 
        SET name = COALESCE($2, name),
            icon = COALESCE($3, icon),
            link = COALESCE($4, link),
            sort_order = COALESCE($5, sort_order),
            is_active = COALESCE($6, is_active)
        WHERE id = $1
        RETURNING id, name, icon, link, sort_order, is_active,
                  created_at AT TIME ZONE 'Asia/Shanghai' as created_at
    "#;
    
    match sqlx::query_as::<_, Action>(query)
        .bind(id)
        .bind(&request.name)
        .bind(&request.icon)
        .bind(&request.link)
        .bind(&request.sort_order)
        .bind(&request.is_active)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(action)) => {
            Ok(serde_json::to_string(&action).unwrap())
        }
        Ok(None) => {
            Err(Status::NotFound)
        }
        Err(error) => {
            println!("Error updating action: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[delete("/api/admin/actions/<id>")]
pub async fn delete_action(id: i32, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let query = "DELETE FROM action_buttons WHERE id = $1";
    
    match sqlx::query(query)
        .bind(id)
        .execute(sqlxPool.inner()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(json!({
                    "success": true,
                    "message": "Action deleted successfully"
                }).to_string())
            } else {
                Err(Status::NotFound)
            }
        }
        Err(error) => {
            println!("Error deleting action: {}", error);
            Err(Status::InternalServerError)
        }
    }
}