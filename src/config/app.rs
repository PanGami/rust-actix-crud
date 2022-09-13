use actix_web::{web, HttpRequest};
use crate::tweets;
use crate::users;

async fn index(_req: HttpRequest) -> String {
    format!("TEST!")
}

pub fn config_services(config: &mut web::ServiceConfig) { 
    config.service(
        web::scope("") // Sudah automated terisi '/'            
            // Anggap disini API login logout dan auth
            .route("/", web::get().to(index)) //default endpoint
            .route("/signup",web::post().to(users::routes::signup))
            .route("/login",web::post().to(users::routes::login))
            .route("logout",web::post().to(users::routes::logout))
            .service(
                web::scope("api")
                .route("", web::get().to(index)) //default api endpoint                        
                .service(
                    web::scope("tweets") // http://localhost:8080/api/tweets/
                        .route("", web::get().to(tweets::index))     
                        .route("", web::post().to(tweets::create))
                        .route("/{id}", web::get().to(tweets::show))
                        .route("/{id}", web::delete().to(tweets::destroy))
                        .route("/{id}", web::patch().to(tweets::update))       
                )                
                .service( //Another Exaple Endpoints
                    web::scope("test") //http://localhost:8080/api/test/
                        .route("", web::get().to(index))     
                        // .route("", web::post().to(tweets::create))
                        // .route("/{id}", web::get().to(tweets::show))
                        // .route("/{id}", web::delete().to(tweets::destroy))
                        // .route("/{id}", web::patch().to(tweets::update))     
                )
            )
    );
}