use crate::utils::data::{query_int_with_params, query_json_with_params, Body};
use rocket::http::Status;
use rocket::State;
use sqlx::{Pool as sPool, Postgres};
#[post("/api/admin/lessons/update?<open_id>", data = "<obj>")]
pub async fn admin_lessons_update(
    open_id: String,
    obj: String,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = "SELECT * FROM fn_admin_lessons_update($1)";
    
    match sqlx::query_scalar::<_, i32>(query)
        .bind(obj)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => {
            Ok(result.to_string())
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}