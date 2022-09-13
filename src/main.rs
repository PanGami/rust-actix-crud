
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate actix_cors;
extern crate actix_rt;
extern crate bcrypt;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
extern crate serde;
extern crate uuid;


use actix_web::{http, middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::{env, io};
use actix_cors::Cors;
use actix_service::Service;
use futures::FutureExt;
use std::default::Default;


// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod tweets;
mod schema;
mod config;
mod auth;
mod constants;
mod users;
mod utils;
mod services;
mod error;

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
        .wrap(
            Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allowed_origin("http://127.0.0.1:8080")
                .allowed_origin("http://localhost:8080")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(crate::auth::auth::Authentication)
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::config;
    use actix_cors::Cors;
    use actix_service::Service;
    use actix_web::{http, App, HttpServer};
    use futures::FutureExt;

    #[actix_rt::test]
    async fn test_startup_ok() {
        let pool = config::db::migrate_and_config_db(":memory:");

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        // .allowed_origin("http://127.0.0.1:8080")
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .wrap(actix_web::middleware::Logger::default())
                .wrap(crate::auth::auth::Authentication)
                .wrap_fn(|req, srv| srv.call(req).map(|res| res))
                .configure(config::app::config_services)
        })
        .bind("localhost:8000".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }

    #[actix_rt::test]
    async fn test_startup_without_auth_middleware_ok() {
        let pool = config::db::migrate_and_config_db(":memory:");

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default() // allowed_origin return access-control-allow-origin: * by default
                        // .allowed_origin("http://127.0.0.1:8080")
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .wrap(actix_web::middleware::Logger::default())
                .wrap_fn(|req, srv| srv.call(req).map(|res| res))
                .configure(config::app::config_services)
        })
        .bind("localhost:8001".to_string())
        .unwrap()
        .run();

        assert_eq!(true, true);
    }
}
