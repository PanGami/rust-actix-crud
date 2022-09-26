use anyhow::Result;
use google_signin::Client;

use crate::users::{AuthenticationSource, User};

// TODO: provide a warp route for the user authentication to hit.
// Need to decide where mountpoints should be defined (alternatives are either to define them
// inline where they are created, or to have a single place where all routes are defined together).

pub fn authenticate_google_user(client: &Client, id_token: &str) -> Result<User> {
    let res = client.verify(id_token)?;
    let name = res.name.unwrap_or("Traveler".to_string());
    let user = User::new(name, AuthenticationSource::Google, res.sub);
    // TODO: Insert user into database if necessary, otherwise populate id
    // That functionality should probably be implemented in the auth module
    // as it can be shared across all authentication sources in the future.
    Ok(user)
}