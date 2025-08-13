mod errors;
mod handlers;
mod models;
mod utils;

use std::env;
use dotenv::dotenv;

use deadpool_postgres::{ManagerConfig, Runtime};
use models::settings::Settings;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use sqlx::postgres::PgPoolOptions;
use tokio_postgres::NoTls;
use crate::utils::content_disposition::ContentDisposition;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 加载 .env 文件
    dotenv().ok();
    
    // 配置 PostgreSQL 数据库
    let url= env::var("DB_URL").expect("DB_URL required");
    let pool = PgPoolOptions::new().max_connections(10).connect(&url).await.expect("connect db failed");

    // 通过环境变量设置数据库公网IP，端口，数据库名称，用户名，密码
    let mut config = deadpool_postgres::Config::new();
    config.host = Some(env::var("DB_HOST").expect("Please specify DB_HOST"));
    config.port = Some(
        env::var("DB_PORT")
            .expect("Please specify DB_PORT")
            .parse::<u16>()
            .expect("Couldn't parse"),
    );
    config.password = Some(env::var("DB_PASSWORD").expect("Please specify DB_PASSWORD"));
    config.dbname = Some("yoga".to_string());
    config.user = Some("psycho".to_string());
    config.manager = Some(ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    });

    let limits = Limits::default().limit("limits.file", 10.megabytes());

    let figment = Figment::from(rocket::Config::default())
        .merge((rocket::Config::ADDRESS, "127.0.0.1"))
        .merge((rocket::Config::PORT, 8002))
        .merge((rocket::Config::LIMITS, limits));
    // 实例化和启动 rocket
    rocket::build()
        .configure(figment)
        .attach(ContentDisposition)
        .manage(Settings {
            appid: env::var("APPID").expect("Couldn't find appid"),
            secret: env::var("SECRET").expect("Couldn't find secret"),
            image_dir: env::var("IMAGE_DIR").expect("Couldn't find image_dir"),
        })
        .manage(pool)
        .manage(
            config
                .create_pool(Some(Runtime::Tokio1), NoTls)
                .expect("Can't create pool"),
        )
        .mount(
            "/",
            routes![
                handlers::admin_auth::admin_login,
                handlers::admin_auth::admin_verify,
                handlers::admin_book::admin_lessons_update,
                handlers::admin_lessons::admin_lessons,
                handlers::admin_lessons::admin_lesson,
                handlers::admin_lessons::admin_lesson_hidden,
                handlers::admin_lessons::admin_lesson_delete,
                handlers::admin_lessons::admin_lessons_and_teachers,
                handlers::admin_lessons::admin_lesson_update,
                handlers::admin_user::admin_user_lessons,
                handlers::admin_user::admin_users_all,
                handlers::admin_user::admin_user,
                handlers::auth::auth,handlers::booking::lessons,
                handlers::booking::book,handlers::booking::unbook,
                handlers::debug::debug,handlers::favicon::favicon,
                handlers::index::index,handlers::index::index_without_openid,handlers::picture::picture,
                handlers::picture::avatar,handlers::schedule::admin_schedule,
                handlers::teacher::teacher_lessons,
                handlers::user::user_query,
                handlers::user::register_user,
                handlers::user::user_book_statistics,
                handlers::membership::get_plans,
                handlers::membership::get_user_cards,
                handlers::membership::purchase_card,
                handlers::membership::get_card_usage,
                handlers::admin_notices::get_notices,
                handlers::admin_notices::create_notice,
                handlers::admin_notices::update_notice,
                handlers::admin_notices::delete_notice,
                handlers::admin_posters::get_posters,
                handlers::admin_posters::create_poster,
                handlers::admin_posters::update_poster,
                handlers::admin_posters::delete_poster,
                handlers::admin_teachers::get_teachers,
                handlers::admin_teachers::create_teacher,
                handlers::admin_teachers::update_teacher,
                handlers::admin_teachers::delete_teacher,
                handlers::action_button::get_action_buttons,
                handlers::action_button::get_active_action_buttons,
                handlers::action_button::create_action_button,
                handlers::action_button::update_action_button,
                handlers::action_button::delete_action_button,
                handlers::admin_actions::get_actions,
                handlers::admin_actions::create_action,
                handlers::admin_actions::update_action,
                handlers::admin_actions::delete_action,
                handlers::admin_users::get_admin_users,
                handlers::admin_users::create_admin_user,
                handlers::admin_users::update_admin_user,
                handlers::admin_users::delete_admin_user,
                handlers::location::get_locations,
                handlers::location::get_available_locations,
                handlers::location::check_location_availability,
                handlers::location::get_location_stats,
                handlers::location::create_location,
                handlers::location::update_location,
                handlers::location::delete_location,
                handlers::admin_user::get_users,
                handlers::admin_user::create_user,
                handlers::admin_user::update_user,
                handlers::admin_user::delete_user
                ],
        )
        // Add API proxy routes (same handlers but with /api prefix)
        .mount(
            "/api",
            routes![
                handlers::admin_auth::admin_login,
                handlers::admin_auth::admin_verify,
                handlers::admin_notices::get_notices,
                handlers::admin_notices::create_notice,
                handlers::admin_notices::update_notice,
                handlers::admin_notices::delete_notice,
                handlers::admin_posters::get_posters,
                handlers::admin_posters::create_poster,
                handlers::admin_posters::update_poster,
                handlers::admin_posters::delete_poster,
                handlers::admin_teachers::get_teachers,
                handlers::admin_teachers::create_teacher,
                handlers::admin_teachers::update_teacher,
                handlers::admin_teachers::delete_teacher,
                handlers::admin_actions::get_actions,
                handlers::admin_actions::create_action,
                handlers::admin_actions::update_action,
                handlers::admin_actions::delete_action,
                handlers::admin_users::get_admin_users,
                handlers::admin_users::create_admin_user,
                handlers::admin_users::update_admin_user,
                handlers::admin_users::delete_admin_user,
                handlers::location::get_locations,
                handlers::location::get_available_locations,
                handlers::location::check_location_availability,
                handlers::location::get_location_stats,
                handlers::location::get_admin_locations,
                handlers::location::create_location,
                handlers::location::update_location,
                handlers::location::delete_location,
                handlers::admin_user::get_users,
                handlers::admin_user::create_user,
                handlers::admin_user::update_user,
                handlers::admin_user::delete_user
                ],
        )
        .register(
            "/",
            catchers![
                errors::not_found::not_found,
                errors::internal_error::internal_error
            ],
        )
        .launch()
        .await?;
    Ok(())
}
