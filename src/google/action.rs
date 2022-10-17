use actix_session::Session;
use actix_web::{Error, HttpResponse};

// Functions define
pub fn add_token(_token: &str, session: Session) -> Result<HttpResponse, Error>{
    session.insert("token", _token);
    Ok(HttpResponse::Ok().json("Session value Added"))
}