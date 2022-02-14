#![allow(unused)]

//! This file contains utility functions used by all tests.

use once_cell::sync::OnceCell;
use realworld;
use rocket::http::{ContentType, Header, Status};
use rocket::local::blocking::{Client, LocalResponse};
use serde_json::Value;
use std::sync::Mutex;

pub const USERNAME: &'static str = "smoketest";
pub const EMAIL: &'static str = "smoketest@realworld.io";
pub const PASSWORD: &'static str = "qweasdzxc";

/// Utility macro for turning `json!` into string.
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("cannot json stringify")
    };
}

pub type Token = String;

pub fn test_client() -> &'static Mutex<Client> {
    static INSTANCE: OnceCell<Mutex<Client>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let rocket = realworld::rocket();
        Mutex::from(Client::tracked(rocket).expect("valid rocket instance"))
    })
}

/// Retrieve a token registering a user if required.
pub fn login(client: &Client) -> Token {
    try_login(client).unwrap_or_else(|| {
        register(client, USERNAME, EMAIL, PASSWORD);
        try_login(client).expect("Cannot login")
    })
}

/// Make an authorization header.
pub fn token_header(token: Token) -> Header<'static> {
    Header::new("authorization", format!("Token {}", token))
}

/// Helper function for converting response to json value.
pub fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}

// Internal stuff

/// Login as default user returning None if login is not found
fn try_login(client: &Client) -> Option<Token> {
    let response = client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    if response.status() == Status::UnprocessableEntity {
        return None;
    }

    let value = response_json_value(response);
    let token = value
        .get("user")
        .and_then(|user| user.get("token"))
        .and_then(|token| token.as_str())
        .map(String::from)
        .expect("Cannot extract token");
    Some(token)
}

/// Register user for
pub fn register(client: &Client, username: &str, email: &str, password: &str) {
    let response = client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"username": username, "email": email, "password": password}}))
        .dispatch();

    assert!(response.status() == Status::Ok || response.status() == Status::UnprocessableEntity);
}
