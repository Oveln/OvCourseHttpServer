use std::{io, sync::Mutex};

#[path = "../handlers/mod.rs"]
mod handlers;

#[path ="../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models/mod.rs"]
mod models;

#[path = "../dbaccess/mod.rs"]
mod db_access;

#[path = "../errors.rs"]
mod errors;


use errors::MyError;
use routers::*;
use state::{AppState};
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[actix_rt::main]
async fn main()-> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL NOT FOUND");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(
        AppState {
            health_check_response: "I'm OK".to_string(),
            visit_count: Mutex::new(0),
            // courses: Mutex::new(vec![]),
            db: db_pool
        }
    );
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req|{
                MyError::InvalidInput("Please provide valid Json input".into()).into()
            }))
            .configure(course_routes)
            .configure(general_routes)
            .configure(teacher_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}