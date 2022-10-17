extern crate google_signin;

use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use std::env;
use super::action::{add_token};

struct GoogleLogin {
  token: String,
}

pub async fn login(_req: HttpRequest) {
    dotenv::dotenv().ok();

    let audiences = env::var("CLIENT_ID").expect("CLIENT ID Must be set");
    let domain = env::var("HOSTED_DOMAIN").expect("HOSTED DOMAIN Must be set");
    let mut client = google_signin::Client::new();
    client.audiences.push(audiences);
    client.hosted_domains.push(domain);
}
pub async fn create(
  payload: web::Json<GoogleLogin>,
  session: Session
) -> Result<HttpResponse, Error> {
  dotenv::dotenv().ok();
  let audiences = env::var("CLIENT_ID").expect("CLIENT ID Must be set");
  let mut client = google_signin::Client::new();
  client.audiences.push(audiences);

  let token = web::block(move || {
    add_token(&payload.token, session);

    let token = session.get::<&str>("token");

    let id_info = client.verify(&payload.token).expect("Expected token to be valid");
    println!("Success! Signed-in as {}", id_info.sub);
  })
  .await?;

  Ok(HttpResponse::Ok().json(token))
}
pub async fn test(session: Session) -> Result<HttpResponse, Error> {
    if let Some(info) = session.get::<String>("info")? {
      Ok(HttpResponse::Ok().body(format!("Here is your Session! {info}")))
  } else {
      Ok(HttpResponse::Ok().json("No Session data Found!"))
    }
  }
