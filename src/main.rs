#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod tweets;
mod schema;
mod config;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Setting .env to variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL Must be set");    
    let app_port = env::var("APP_PORT").expect("APP_PORT Must be set");
    let app_host = env::var("APP_HOST").expect("APP_PORT Must be set");
    let app_url =  format!("{}:{}", &app_host, &app_port);

    // set up database connection pool    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(config::app::config_path)
    })
    .bind(&app_url)?
    .run()
    .await
}