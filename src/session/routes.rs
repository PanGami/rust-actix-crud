
use actix_session::Session;
use actix_web::{Error, HttpResponse};

pub async fn add_session(session: Session) -> Result<HttpResponse, Error> {
    session.insert("value", "Hello There!");
    Ok(HttpResponse::Ok().json("Session value Added"))
  }
  
  pub async fn get_session(session: Session) -> Result<HttpResponse, Error> {
    if let Some(value) = session.get::<String>("value")? {
      Ok(HttpResponse::Ok().body(format!("Here is your Session! {value}")))
  } else {
      Ok(HttpResponse::Ok().json("No Session data Found!"))
    }
  }
  
  pub async fn update_session(session: Session) -> Result<HttpResponse, Error> {
    session.insert("value", "updated");
    Ok(HttpResponse::Ok().json("Session Value updated"))
  }
  
  pub async fn delete_data_session(session: Session) -> Result<HttpResponse, Error> {
    session.remove("value");
    Ok(HttpResponse::Ok().json("Session Value deleted"))
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