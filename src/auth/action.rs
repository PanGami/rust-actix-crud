use std::sync::Arc;

use google_signin::Client;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use warp::Filter;

use crate::users::action::authenticate_google_user;

pub fn auth_google<'a>(
    client: Arc<Client>,
) -> impl warp::Filter<Extract = (WithStatus<Json>,), Error = warp::Rejection> + 'a + Clone {
    warp::path!("auth" / "google" / String)
        .and(warp::post())
        .map(
            move |id_token: String| match authenticate_google_user(&client, &id_token) {
                Ok(user) => warp::reply::with_status(warp::reply::json(&user), StatusCode::OK),
                Err(_) => {
                    warp::reply::with_status(warp::reply::json(&"Bad"), StatusCode::NOT_FOUND)
                }
            },
        )
}