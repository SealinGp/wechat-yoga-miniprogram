use rocket::http::Status;
use rocket::State;
use serde_json::json;
use sqlx::{Pool as sPool, Postgres, FromRow};
use chrono::NaiveDateTime;

#[derive(FromRow)]
struct UserIdResult {
    id: i32,
}

#[derive(FromRow)]
struct JsonResult {
    posters: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct ActionsResult {
    actions: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct TeachersResult {
    teachers: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct NoticesResult {
    notices: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct BookedResult {
    booked: Option<serde_json::Value>,
}

#[derive(FromRow)]
struct MarketResult {
    market: Option<serde_json::Value>,
}

#[get("/yoga/index")]
pub async fn index_without_openid(sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    index_handler(None, sqlxPool).await
}

#[get("/yoga/index?<openid>")]
pub async fn index(openid: Option<String>, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    index_handler(openid, sqlxPool).await
}

async fn index_handler(openid: Option<String>, sqlxPool: &State<sPool<Postgres>>) -> Result<String, Status> {
    let user_id = if let Some(ref open_id) = openid {
        // Get user ID if openid is provided
        match sqlx::query_as::<_, UserIdResult>("SELECT id FROM users WHERE open_id = $1")
            .bind(open_id)
            .fetch_optional(sqlxPool.inner()).await {
            Ok(Some(row)) => Some(row.id),
            Ok(None) => None,
            Err(_) => None,
        }
    } else {
        None
    };

    // 获取轮播图数据
    let poster_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'image', image,
                'title', title,
                'href', link_url
            ) ORDER BY sort_order ASC
        ), '[]'::json) as posters
        FROM posters 
        WHERE is_active = true 
        AND (start_date IS NULL OR start_date <= CURRENT_TIMESTAMP)
        AND (end_date IS NULL OR end_date >= CURRENT_TIMESTAMP)
    "#;

    let posters = match sqlx::query_as::<_, JsonResult>(poster_query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => row.posters.unwrap_or(json!([])),
        Err(error) => {
            println!("Error querying posters: {}", error);
            json!([])
        }
    };

    // 获取功能按钮数据
    let actions_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'icon', icon,
                'link', link
            ) ORDER BY sort_order ASC
        ), '[]'::json) as actions
        FROM action_buttons 
        WHERE is_active = true
    "#;

    let actions = match sqlx::query_as::<_, ActionsResult>(actions_query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => row.actions.unwrap_or(json!([])),
        Err(error) => {
            println!("Error querying actions: {}", error);
            json!([])
        }
    };

    // 获取教师数据
    let teachers_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'name', name,
                'thumbnail', avatar_url,
                'introduction', description,
                'rating', COALESCE(average_rating, 0.0),
                'experience_years', experience_years
            ) ORDER BY average_rating DESC NULLS LAST, experience_years DESC
        ), '[]'::json) as teachers
        FROM teachers 
        WHERE is_active = true 
        LIMIT 5
    "#;

    let teachers = match sqlx::query_as::<_, TeachersResult>(teachers_query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => row.teachers.unwrap_or(json!([])),
        Err(error) => {
            println!("Error querying teachers: {}", error);
            json!([])
        }
    };

    // 获取通知数据（带时间显示）
    let notices_query = r#"
        SELECT COALESCE(json_agg(
            jsonb_build_object(
                'id', id,
                'title', title,
                'timeago', CASE
                    WHEN EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) < 3600 THEN 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 60) || '分钟前'
                    WHEN EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) < 86400 THEN 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 3600) || '小时前'
                    ELSE 
                        FLOOR(EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - created_at)) / 86400) || '天前'
                END
            ) ORDER BY priority DESC, created_at DESC
        ), '[]'::json) as notices
        FROM notices 
        WHERE is_active = true 
        LIMIT 5
    "#;

    let notices = match sqlx::query_as::<_, NoticesResult>(notices_query)
        .fetch_one(sqlxPool.inner()).await {
        Ok(row) => row.notices.unwrap_or(json!([])),
        Err(error) => {
            println!("Error querying notices: {}", error);
            json!([])
        }
    };

    // 获取用户预约数据（如果有用户ID）
    let booked = if let Some(uid) = user_id {
        let booked_query = r#"
            SELECT COALESCE(json_agg(
                jsonb_build_object(
                    'id', b.id,
                    'title', l.title,
                    'teacher', t.name,
                    'start_time', extract(epoch from l.start_time)::bigint,
                    'location', loc.name,
                    'lesson_id', l.id
                ) ORDER BY l.start_time ASC
            ), '[]'::json) as booked
            FROM bookings b
            JOIN lessons l ON b.lesson_id = l.id
            LEFT JOIN teachers t ON l.teacher_id = t.id
            LEFT JOIN locations loc ON l.location_id = loc.id
            WHERE b.user_id = $1
              AND b.status = 'confirmed'
              AND l.start_time > CURRENT_TIMESTAMP
            LIMIT 10
        "#;

        match sqlx::query_as::<_, BookedResult>(booked_query)
            .bind(uid)
            .fetch_one(sqlxPool.inner()).await {
            Ok(row) => row.booked.unwrap_or(json!([])),
            Err(error) => {
                println!("Error querying user bookings: {}", error);
                json!([])
            }
        }
    } else {
        json!([])
    };

    // 获取商城信息
    let market_query = r#"
        SELECT jsonb_build_object(
            'id', id,
            'slogan', slogan,
            'description', description
        ) as market
        FROM market_info 
        WHERE is_active = true 
        LIMIT 1
    "#;

    let market = match sqlx::query_as::<_, MarketResult>(market_query)
        .fetch_optional(sqlxPool.inner()).await {
        Ok(Some(row)) => row.market.unwrap_or(json!({"id": 1, "slogan": "积分兑换好礼，健康生活更精彩"})),
        Ok(None) => json!({"id": 1, "slogan": "积分兑换好礼，健康生活更精彩"}),
        Err(error) => {
            println!("Error querying market info: {}", error);
            json!({"id": 1, "slogan": "积分兑换好礼，健康生活更精彩"})
        }
    };

    let result = json!({
        "poster": posters,
        "actions": actions,
        "teachers": teachers,
        "notices": notices,
        "booked": booked,
        "market": market
    });

    Ok(result.to_string())
}