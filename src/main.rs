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

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL Must be set");
    let app_url = env::var("APP_URL").expect("APP_URL Must be set");
    let app_port = env::var("APP_PORT").expect("APP_PORT Must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(tweets::index)
            .service(tweets::create)
            .service(tweets::show)
            .service(tweets::update)
            .service(tweets::destroy)
    })
    .bind(format!("{}:{}", app_url, app_port))?
    .run()
    .await
}