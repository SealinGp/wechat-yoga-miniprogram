use crate::utils::data::query_json_with_params;
use deadpool_postgres::Pool;
use rocket::http::Status;
use rocket::State;
use serde_json;
#[get("/yoga/admin/user/lessons?<id>&<start>&<end>&<open_id>")]
pub async fn admin_user_lessons(
    id: i32,
    start: i64,
    end: i64,
    open_id: String,
    pool: &State<Pool>,
) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
            match query_json_with_params(
                &conn,
                "select * from fn_admin_user_lessons($1,$2,$3)",
                &[&id, &start, &end],
            )
            .await
            {
                Ok(v) => {
                    return match String::from_utf8(v.0) {
                        Ok(v) => Ok(v),
                        Err(_) => Err(Status::InternalServerError),
                    };
                }
                Err(error) => {
                    println!("Error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/admin/users/all?<open_id>")]
pub async fn admin_users_all(open_id: String, pool: &State<Pool>) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
            // 直接查询用户列表，不使用存储过程
            let query = r#"
                SELECT json_agg(jsonb_build_object(
                    'id', id,
                    'open_id', open_id,
                    'nick_name', nick_name,
                    'avatar_url', avatar_url,
                    'phone', phone,
                    'created_at', extract(epoch from created_at)::bigint,
                    'updated_at', extract(epoch from updated_at)::bigint,
                    'is_admin', is_admin
                )) as result
                FROM users
                ORDER BY created_at DESC
            "#;
            
            match conn.query_one(query, &[]).await {
                Ok(row) => {
                    let result: Option<serde_json::Value> = row.get("result");
                    match result {
                        Some(json_data) => Ok(json_data.to_string()),
                        None => Ok("[]".to_string()) // 返回空数组如果没有用户
                    }
                }
                Err(error) => {
                    println!("Error querying users: {}", error);
                    Ok("[]".to_string()) // 返回空数组作为降级处理
                }
            }
        }
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/yoga/admin/user?<open_id>&<id>")]
pub async fn admin_user(id:i32,
                        open_id:String,
                        pool: &State<Pool>,
                        ) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
            match query_json_with_params(
                &conn,
                "select * from fn_admin_user($1)",
                &[&id],
            )
            .await
            {
                Ok(v) => {
                    return match String::from_utf8(v.0) {
                        Ok(v) => Ok(v),
                        Err(_) => Err(Status::InternalServerError),
                    };
                }
                Err(error) => {
                    println!("Error: {}", error);
                    Err(Status::InternalServerError)
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}