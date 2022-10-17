use actix_web::{web, HttpRequest};
use crate::tweets;
use crate::session;

async fn index(_req: HttpRequest) -> String {
    format!("TEST!")
}

pub fn config_path(config: &mut web::ServiceConfig) { 
    config.service(
        web::scope("")            
            // Anggap disini API login logout dan auth
            .route("/", web::get().to(index)) //default endpoint
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
                .service( 
                    web::scope("session") //http://localhost:8080/api/session/
                        .route("", web::get().to(index))                         
                        .route("/add", web::post().to(session::add_session))   
                        .route("/get", web::get().to(session::get_session))   
                        .route("/update", web::put().to(session::update_session))  
                        .route("/delete", web::delete().to(session::delete_data_session))
                        .route("/clear", web::get().to(session::clear_data_session))
                        .route("/renew", web::get().to(session::renew_key_session))
                        .route("/entries", web::get().to(session::entries_session))      
                        .route("/status", web::get().to(session::status_session))                             
                        .route("/end", web::get().to(session::end_session))      
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