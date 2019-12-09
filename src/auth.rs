use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};

use jsonwebtoken as jwt;

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
        jwt::encode(&jwt::Header::default(), self, config::SECRET.as_bytes()).expect("jwt")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    /// Extract Auth token from the "Authorization" header.
    ///
    /// Handlers with Auth guard will fail with 503 error.
    /// Handlers with Option<Auth> will be called with None.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        if let Some(auth) = extract_auth_from_request(request) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(decode_token)
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    jwt::decode(
        token,
        &config::SECRET.as_bytes(),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}
