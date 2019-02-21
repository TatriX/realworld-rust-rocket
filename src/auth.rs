use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::jwt;
use serde_json;

use crate::config;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    pub username: String,
}

impl Auth {
    pub fn token(&self) -> String {
        let headers = json!({});
        let payload = json!(self);
        jwt::encode(
            headers.0,
            &config::SECRET.to_string(),
            &payload,
            jwt::Algorithm::HS256,
        ).expect("jwt")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        if let Some(auth) = extract_auth_from_request(request) {
            Outcome::Success(auth)
        } else {
            Outcome::Forward(())
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Auth> {
    let header = request.headers().get("authorization").next();
    if let Some(token) = header.and_then(extract_token_from_header) {
        match jwt::decode(
            &token.to_string(),
            &config::SECRET.to_string(),
            jwt::Algorithm::HS256,
        ) {
            Err(err) => {
                println!("Auth decode error: {:?}", err);
            }
            Ok((_, payload)) => match serde_json::from_value::<Auth>(payload) {
                Ok(auth) => return Some(auth),
                Err(err) => println!("Auth serde decode error: {:?}", err),
            },
        };
    }
    None
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}
