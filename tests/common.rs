//! This file contains utility functions used by all tests.

use realworld;
use rocket::http::ContentType;
use rocket::local::{Client, LocalResponse};
use serde_json::{json, Value};

pub const USERNAME: &'static str = "rust-diesel-rocket";
pub const EMAIL: &'static str = "rust-diesel-rocket@example.com";
pub const PASSWORD: &'static str = "qweasdzxc";

/// Utility macro for turning `json!` into string.
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&json!($value)).expect("cannot json stringify")
    };
}

type Token = String;

pub fn test_client() -> Client {
    let rocket = realworld::rocket();
    Client::new(rocket).expect("valid rocket instance")
}

/// Try logging in extracting access Token
pub fn login(client: &Client) -> Token {
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let wrapper = response_json_value(response);
    wrapper
        .get("user")
        .expect("must have a 'user' field")
        .get("token")
        .expect("user has token")
        .as_str()
        .expect("token must be a string")
        .to_string()
}

/// Helper function for converting response to json value.
pub fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}
