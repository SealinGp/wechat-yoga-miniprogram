use sqlx::{Pool as sPool, Postgres};
use rocket::http::Status;
use rocket::State;
#[get("/yoga/teacher/lessons?<start_time>&<end_time>&<open_id>&<class_type>&<teacher_id>")]
pub async fn teacher_lessons(
    start_time: i32,
    end_time: i32,
    open_id: String,
    class_type: i32,
    teacher_id: i32,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    let query = "select * from fn_teacher_lessons($1,$2,$3,$4,$5)";
    
    match sqlx::query_scalar::<_, serde_json::Value>(query)
        .bind(start_time)
        .bind(end_time)
        .bind(open_id)
        .bind(class_type)
        .bind(teacher_id)
        .fetch_one(sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}