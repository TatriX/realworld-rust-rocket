use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde_json::Value;

use jwt;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Auth {
    /// timestamp
    exp: i64,
    /// user id
    id: i32,
    username: String,
}

const SECRET: &'static str = "secret123";
const TOKEN_PREFIX: &'static str = "Token ";

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        let header = request.headers().get("authorization").next();
        if let Some(token) = header.and_then(extract_token_from_header) {
            match jwt::decode(
                &token.to_string(),
                &SECRET.to_string(),
                jwt::Algorithm::HS256,
            ) {
                Err(err) => {
                    println!("Auth decode error: {:?}", err);
                    return Outcome::Forward(());
                }
                Ok((_, payload)) => match serde_json::from_value::<Auth>(payload) {
                    Ok(auth) => return Outcome::Success(auth),
                    Err(err) => println!("Auth serde decode error: {:?}", err),
                },
            };
        }
        Outcome::Failure((Status::BadRequest, ()))
    }
}

pub fn encode_payload(payload: Value) -> String {
    let header = json!({});
    jwt::encode(header, &SECRET.to_string(), &payload, jwt::Algorithm::HS256).expect("jwt")
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(TOKEN_PREFIX) {
        Some(&header[TOKEN_PREFIX.len()..])
    } else {
        None
    }
}
