//! Test registration and login

use realworld;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};
use rocket::http::{ContentType, Header};
use serde_json::{json, Value};

const USERNAME: &'static str = "rust-diesel-rocket";
const EMAIL: &'static str = "rust-diesel-rocket@example.com";
const PASSWORD: &'static str = "qweasdzxc";

/// Utility macro for turning `json!` into string.
macro_rules! json_string {
    ($value:tt) => (serde_json::to_string(&json!($value)).expect("cannot json stringify"));
}

type Token = String;

#[test]
fn test_auth() {
    let rocket = realworld::rocket();
    let client = &Client::new(rocket).expect("valid rocket instance");
    register(client);
    let token = login(client);
    get_current_user(client, token);
}

///  Register new user, handling repeated registration as well.
fn register(client: &Client) {
    let response = &mut client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"username": USERNAME, "email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let status = response.status();
    // If user was already created we should get an UnprocessableEntity or Ok otherwise.
    match status {
        Status::Ok => check_user_response(response),
        Status::UnprocessableEntity => check_user_validation_errors(response),
        _ => panic!("Got status: {}", status),
    }
}

/// Try logging in extracting access Token
fn login(client: &Client) -> Token {
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let wrapper = response_json_value(response);
    let user = wrapper.get("user").expect("must have a 'user' field");
    user
        .get("token")
        .expect("user has token")
        .as_str()
        .expect("token must be a string")
        .to_string()
}

/// Check that `/user` endpoint returns expected data.
fn get_current_user(client: &Client, token: Token) {
    let response = &mut client
        .get("/api/user")
        .header(Header::new("authorization", format!("Token {}", token)))
        .dispatch();
    check_user_response(response);
}

// Utility functions

/// Helper function for converting response to json value.
fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}

/// Assert that body contains "user" response with expected fields.
fn check_user_response(response: &mut LocalResponse) {
    let value = response_json_value(response);
    let user = value.get("user").expect("must have a 'user' field");

    assert_eq!(user.get("email").expect("user has email"), EMAIL);
    assert_eq!(user.get("username").expect("user has username"), USERNAME);
    assert!(user.get("bio").is_some());
    assert!(user.get("image").is_some());
    assert!(user.get("token").is_some());
}

fn check_user_validation_errors(response: &mut LocalResponse) {
    let validation_errors = response_json_value(response);
    let errors = validation_errors.get("errors").expect("no 'errors' field");
    let username_errors = errors.get("username").expect("no 'username' errors");
    let username_error = username_errors.get(0).expect("'username' errors are missing");
    if username_error.as_str().unwrap() != "has already been taken" {
        panic!("Got validation errors: {:#?}", validation_errors);
    }
}
