use rocket::http::Status;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use crate::models::membership;

// Using structs from model layer

// 获取会员卡套餐列表
#[get("/yoga/membership/plans")]
pub async fn get_plans(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match membership::get_membership_plans(sqlxPool.inner()).await {
        Ok(Some(plans)) => Ok(plans.to_string()),
        Ok(None) => Ok("[]".to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 获取用户的会员卡列表
#[get("/yoga/membership/cards?<openid>")]
pub async fn get_user_cards(openid: String, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    match membership::get_user_membership_cards(&openid, sqlxPool.inner()).await {
        Ok(Some(cards)) => Ok(cards.to_string()),
        Ok(None) => Ok("[]".to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}

// 购买会员卡
#[post("/yoga/membership/purchase?<openid>&<plan_id>&<paid_amount>")]
pub async fn purchase_card(
    openid: String, 
    plan_id: i32, 
    paid_amount: Option<f64>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    match membership::purchase_membership_card(&openid, plan_id, paid_amount, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok(json!({"success": false, "message": "Database error"}).to_string())
        }
    }
}

// 获取会员卡使用记录
#[get("/yoga/membership/usage?<openid>&<card_id>")]
pub async fn get_card_usage(
    openid: String, 
    card_id: Option<i32>,
    sqlxPool: &State<sPool<Postgres>>
) -> Result<String, Status> {
    match membership::get_card_usage(&openid, card_id, sqlxPool.inner()).await {
        Ok(Some(usage)) => Ok(usage.to_string()),
        Ok(None) => Ok("[]".to_string()),
        Err(error) => {
            println!("Database error: {}", error);
            Ok("[]".to_string())
        }
    }
}