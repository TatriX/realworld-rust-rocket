//! Test profiles

mod common;

use common::*;
use rocket::http::Status;

#[test]
/// Test profile getting.
fn test_get_profile() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .get(format!("/api/profiles/{}", USERNAME))
        .header(token_header(token))
        .dispatch();

    let value = response_json_value(response);
    let username = value
        .get("profile")
        .and_then(|profile| profile.get("username"))
        .and_then(|username| username.as_str())
        .expect("must have 'username' field");

    assert_eq!(username, USERNAME);
}

#[test]
/// Test follow and unfollow
fn test_follow_and_unfollow() {
    let client = test_client();
    let token = login(&client);

    let author = "author";
    register(&client, author, "author@realworld.io", PASSWORD);

    let response = &mut client
        .post(format!("/api/profiles/{}/follow", author))
        .header(token_header(token.clone()))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response = &mut client
        .delete(format!("/api/profiles/{}/follow", author))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
