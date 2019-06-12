//! Test registration and login

mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::LocalResponse;

#[test]
/// Register new user, handling repeated registration as well.
fn test_register() {
    let client = test_client();
    let response = &mut client
        .post("/api/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"username": USERNAME, "email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let status = response.status();
    // If user was already created we should get an UnprocessableEntity or Ok otherwise.
    //
    // As tests are ran in an indepent order `login()` probably has already created smoketest user.
    // And so we gracefully handle "user already exists" error here.
    match status {
        Status::Ok => check_user_response(response),
        Status::UnprocessableEntity => check_user_validation_errors(response),
        _ => panic!("Got status: {}", status),
    }
}

#[test]
/// Login with wrong password must fail.
fn test_incorrect_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": "foo"}}))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let login_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("email or password")
        .expect("must have 'email or password' errors")
        .get(0)
        .expect("must have non empty 'email or password' errors")
        .as_str();

    assert_eq!(login_error, Some("is invalid"));
}

#[test]
/// Try logging checking that access Token is present.
fn test_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"email": EMAIL, "password": PASSWORD}}))
        .dispatch();

    let value = response_json_value(response);
    value
        .get("user")
        .expect("must have a 'user' field")
        .get("token")
        .expect("user has token")
        .as_str()
        .expect("token must be a string");
}

#[test]
/// Check that `/user` endpoint returns expected data.
fn test_get_current_user() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .get("/api/user")
        .header(token_header(token))
        .dispatch();

    check_user_response(response);
}

// Utility functions

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
    let value = response_json_value(response);
    let username_error = value
        .get("errors")
        .expect("must have a 'errors' field")
        .get("username")
        .expect("must have 'username' errors")
        .get(0)
        .expect("must have non-empty 'username' errors")
        .as_str();

    assert_eq!(username_error, Some("has already been taken"))
}
