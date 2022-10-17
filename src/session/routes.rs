
use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct InputSession {
  key: String,
  value: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct GetSession {
  key: String
}

pub async fn add_session(payload: web::Json<InputSession>, session: Session) -> Result<HttpResponse, Error> {
    session.insert(&payload.key, &payload.value).expect("Error inserting session");
    Ok(HttpResponse::Ok().body(format!("Success! added {} : {}", &payload.key, &payload.value)))
  }
  
  pub async fn get_session(payload: web::Json<GetSession>, session: Session) -> Result<HttpResponse, Error> {
    if let Some(value) = session.get::<String>(&payload.key)? {
      Ok(HttpResponse::Ok().body(format!("Here is your Session data on key {} : {}", &payload.key, value)))
  } else {
      Ok(HttpResponse::Ok().json("No Session data in key Found!"))
    }
  }
  
  pub async fn update_session(payload: web::Json<InputSession>, session: Session) -> Result<HttpResponse, Error> {
    session.insert(&payload.key, &payload.value);
    Ok(HttpResponse::Ok().json("Session Value updated"))
  }
  
  pub async fn delete_data_session(payload: web::Json<GetSession>, session: Session) -> Result<HttpResponse, Error> {
    session.remove(&payload.key);
    Ok(HttpResponse::Ok().json(format!("Session Value deleted on key : {}",&payload.key)))
  }
  
  pub async fn clear_data_session(session: Session) -> Result<HttpResponse, Error> {
    session.clear();
    Ok(HttpResponse::Ok().json("Session Data Cleared"))
  }
  
  pub async fn renew_key_session(session: Session) -> Result<HttpResponse, Error> {
    session.renew();
    Ok(HttpResponse::Ok().json("Session Key Renewed"))
  }
  
  pub async fn entries_session(session: Session) -> Result<HttpResponse, Error> {
    let entries: Vec<_> = session.entries().clone().into_iter().collect();
    Ok(HttpResponse::Ok().json(entries))
  }
  
  pub async fn status_session(session: Session) -> Result<HttpResponse, Error> {
    //Bisa untuk check, testing purpose ataupun match dengan tambahan 
    // actix_session:SessionStatus;
  
    // Contoh seperti 
    // assert_eq!(session.status(), SessionStatus::Unchanged);
    // session.purge();
    // assert_eq!(session.status(), SessionStatus::Purged);
  
    session.status();
    Ok(HttpResponse::Ok().json("Session status"))
  }
  
  pub async fn end_session(session: Session) -> Result<HttpResponse, Error> {
    session.purge();
    Ok(HttpResponse::Ok().json("Session Ended"))
  }