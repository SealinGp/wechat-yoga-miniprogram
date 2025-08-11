use deadpool_postgres::Pool;
use rocket::http::Status;
use rocket::State;
use serde_json::json;

#[get("/yoga/index")]
pub async fn index(pool: &State<Pool>) -> Result<String, Status> {
    match pool.get().await {
        Ok(conn) => {
            // 直接执行 SQL 查询而不是调用存储过程
            let query = r#"
                SELECT jsonb_build_object(
                    'poster', '[]'::json,
                    'booked', (
                        SELECT COALESCE(json_agg(jsonb_build_object(
                            'id', b.id,
                            'nick_name', u.nick_name,
                            'avatar_url', u.avatar_url
                        )), '[]'::json)
                        FROM bookings b
                        JOIN users u ON u.id = b.user_id
                        WHERE b.status = 'confirmed'
                        ORDER BY b.booking_time DESC
                        LIMIT 10
                    ),
                    'actions', '[]'::json,
                    'teachers', (
                        SELECT COALESCE(json_agg(jsonb_build_object(
                            'id', id,
                            'name', name,
                            'thumbnail', avatar_url,
                            'introduction', description
                        )), '[]'::json)
                        FROM teachers
                        WHERE is_active = true
                    ),
                    'market', jsonb_build_object('id', 1, 'slogan', '欢迎来到瑜伽馆'),
                    'notices', (
                        SELECT COALESCE(json_agg(jsonb_build_object(
                            'id', id,
                            'title', title,
                            'updated_time', extract(epoch from created_at)::bigint
                        )), '[]'::json)
                        FROM notices
                        WHERE is_active = true
                        ORDER BY created_at DESC
                        LIMIT 3
                    )
                ) as result
            "#;
            
            match conn.query_one(query, &[]).await {
                Ok(row) => {
                    let result: serde_json::Value = row.get("result");
                    Ok(result.to_string())
                }
                Err(error) => {
                    println!("Error executing index query: {}", error);
                    // 返回默认数据，避免完全失败
                    let default_data = json!({
                        "poster": [],
                        "booked": [],
                        "actions": [],
                        "teachers": [],
                        "market": {"id": 1, "slogan": "欢迎来到瑜伽馆"},
                        "notices": []
                    });
                    Ok(default_data.to_string())
                }
            }
        }
        Err(error) => {
            println!("Database connection error: {}", error);
            Err(Status::InternalServerError)
        }
    }
}