use crate::DbPool;

use actix_web::{web, Error, HttpResponse};
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
  .await?;

  Ok(HttpResponse::Ok().json(tweet))
}

pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweets = web::block(move || {
    let conn = pool.get()?;
    find_all(&conn)
  })
  .await?;

  Ok(HttpResponse::Ok().json(tweets))
}

pub async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    find_by_id(id.into_inner(), &conn)
  })
  .await?;

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
  .await?;

  Ok(HttpResponse::Ok().json(tweet))
}

pub async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let conn = pool.get()?;
    delete_tweet(id.into_inner(), &conn)
  })
  .await?;
  // .map(|tweet| HttpResponse::Ok().json(tweet))

  Ok(HttpResponse::Ok().json(result))
}