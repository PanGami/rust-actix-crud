extern crate google_signin;

use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;
use super::action::{add_token};

#[derive(Debug,Deserialize,Serialize)]
pub struct GoogleLogin {
  token: String,
}
// pub async fn login(_req: HttpRequest) {
//     dotenv::dotenv().ok();

//     let audiences = env::var("CLIENT_ID").expect("CLIENT ID Must be set");
//     let domain = env::var("HOSTED_DOMAIN").expect("HOSTED DOMAIN Must be set");
//     let mut client = google_signin::Client::new();
//     client.audiences.push(audiences);
//     client.hosted_domains.push(domain);
// }
pub async fn login(
  payload: web::Json<GoogleLogin>,
  session: Session
) -> Result<HttpResponse, Error> {
  dotenv::dotenv().ok();
  let audiences = env::var("CLIENT_ID").expect("CLIENT ID Must be set");
  let mut client = google_signin::Client::new();
  client.audiences.push(audiences.to_string());

  add_token(&payload.token, session.clone());

  let token = session.get::<String>("token")?.unwrap();

  let id_info = client.verify(&payload.token).expect("Expected token to be valid");
  println!("Success! Signed-in as {}", id_info.sub);

  // let token = web::block(move || {

  // })
  // .await?;

  Ok(HttpResponse::Ok().json("test"))
}

// pub async fn test(session: Session) -> Result<HttpResponse, Error> {
//     if let Some(info) = session.get::<String>("info")? {
//       Ok(HttpResponse::Ok().body(format!("Here is your Session! {info}")))
//   } else {
//       Ok(HttpResponse::Ok().json("No Session data Found!"))
//     }
//   }
