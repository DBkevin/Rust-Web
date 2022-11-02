use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../db_access.rs"]
mod db_access;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../errors.rs"]
mod errors;
use routers::*;
use state::AppState;
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL IS not set...");
    let db_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I m ok".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        println!("注入了路由");
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind(("127.0.0.1", 3000))?.run().await
}
