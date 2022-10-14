use crate::DbPool;

use actix_web::{web, Error, HttpResponse};
use actix_session::Session;
use super::TweetPayload;
use super::action::{add_a_tweet,find_all,find_by_id,update_tweet,delete_tweet};

pub async fn create(
  pool: web::Data<DbPool>,
  payload: web::Json<TweetPayload>,
) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    add_a_tweet(&payload.message, &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweets = web::block(move || {
    let conn = pool.get()?;
    find_all(&conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweets))
}

pub async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    find_by_id(id.into_inner(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

pub async fn update(
  id: web::Path<i32>,
  payload: web::Json<TweetPayload>,
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    update_tweet(id.into_inner(), payload.message.clone(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

pub async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let conn = pool.get()?;
    delete_tweet(id.into_inner(), &conn)
  })
  .await?
  .map(|tweet| HttpResponse::Ok().json(tweet))
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(result)
}

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
  session.entries();
  Ok(HttpResponse::Ok().json("Session status"))
}

pub async fn end_session(session: Session) -> Result<HttpResponse, Error> {
  session.purge();
  Ok(HttpResponse::Ok().json("Session Ended"))
}