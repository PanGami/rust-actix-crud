use crate::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use super::TweetPayload;
use super::action::{add_a_tweet,find_all,find_by_id,update_tweet,delete_tweet};

#[post("/tweets")]
async fn create(
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

#[get("/tweets")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweets = web::block(move || {
    let conn = pool.get()?;
    find_all(&conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweets))
}

#[get("/tweets/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    find_by_id(id.into_inner(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}

#[put("/tweets/{id}")]
async fn update(
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

#[delete("/tweets/{id}")]
async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let conn = pool.get()?;
    delete_tweet(id.into_inner(), &conn)
  })
  .await?
  .map(|tweet| HttpResponse::Ok().json(tweet))
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(result)
}