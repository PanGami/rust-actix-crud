extern crate google_signin;

use actix_session::Session;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;
// use super::action::{add_token};

#[derive(Debug,Deserialize,Serialize)]
pub struct GoogleLogin {
  token: String,
}

pub async fn login(
  payload: web::Json<GoogleLogin>,
  session: Session
) -> Result<HttpResponse, Error> {
  dotenv::dotenv().ok();
  let audiences = env::var("CLIENT_ID").expect("CLIENT ID Must be set");
  let domain = env::var("HOSTED_DOMAIN").expect("HOSTED DOMAIN Must be set");
  let mut client = google_signin::Client::new();

  client.audiences.push(audiences.to_string());
  client.hosted_domains.push(domain.to_string());

  session.insert("token", &payload.token)?;

  let token = session.get::<String>("token")?.unwrap();

  // recommended
  let id_info = client.verify(&token).expect("Expected token to be valid");
  // println!("Success! Signed-in as {}", id_info.sub);

  // Inspect the ID before verifying it
  //let id_info = client.get_slow_unverified(&token).expect("Expected token to exist");
  // let ok = id_info.verify(&client).is_ok();
  // println!("Ok: {}, Info: {:?}", ok, id_info);

  // let id_info = "test";

  Ok(HttpResponse::Ok().json(format!("token : {}, info : {:?}", &token, &id_info)))
}
//rust jwk convert
