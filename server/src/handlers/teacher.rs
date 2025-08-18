use sqlx::{Pool as sPool, Postgres};
use rocket::http::Status;
use rocket::State;
use crate::models::teacher;

#[get("/yoga/teacher/lessons?<start_time>&<end_time>&<open_id>&<class_type>&<teacher_id>")]
pub async fn teacher_lessons(
    start_time: i32,
    end_time: i32,
    open_id: String,
    class_type: i32,
    teacher_id: i32,
    sqlxPool: &State<sPool<Postgres>>,
) -> Result<String, Status> {
    match teacher::get_teacher_lessons(start_time, end_time, open_id, class_type, teacher_id, sqlxPool.inner()).await {
        Ok(result) => Ok(result.to_string()),
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}