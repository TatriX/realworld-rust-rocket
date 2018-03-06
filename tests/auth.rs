extern crate reqwest;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate hyper;

use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use hyper::header::{Authorization, Headers};
use std::sync::RwLock;

const API_URL: &'static str = "http://localhost:8000/api";

const USERNAME: &'static str = "rust-diesel-rocket";
const EMAIL: &'static str = "rust-diesel-rocket@example.com";
const PASSWORD: &'static str = "qweasdzxc";

lazy_static! {
    static ref TOKEN: RwLock<String> = RwLock::new("".to_string());
}

#[derive(Debug, Deserialize)]
struct ValidationErrors {
    errors: HashMap<String, Vec<String>>,
}

fn make_url(path: &str) -> String {
    format!("{}/{}", API_URL, path)
}

fn post(path: &str, json: Value) -> Response {
    reqwest::Client::new()
        .post(&make_url(path))
        .json(&json)
        .send()
        .expect(&format!("{} {:#?}: post error", path, json))
}

fn get(path: &str) -> Response {
    let token = TOKEN.read().unwrap().to_string();
    let mut headers = Headers::new();
    headers.set(Authorization(format!("Token {}", token)));

    reqwest::Client::new()
        .get(&make_url(path))
        .headers(headers)
        .send()
        .expect(&format!("{} get error", path))
}

fn check_response<F>(resp: &mut Response, f: F)
where
    F: FnOnce(&mut Response),
{
    let status = resp.status();
    match status {
        reqwest::StatusCode::Ok => f(resp),
        _ => panic!("Got status: {}", status),
    }
}

fn check_user_response(resp: &mut Response) {
    let wrapper = resp.json::<Value>().expect("Can't parse user");
    let user = wrapper.get("user").expect("Must have a 'user' field");

    assert_eq!(user.get("email").expect("User has email"), EMAIL);
    assert_eq!(user.get("username").expect("User has username"), USERNAME);
    assert!(user.get("bio").is_some());
    assert!(user.get("image").is_some());
    assert!(user.get("token").is_some());
}

// Run with:
// cargo test -- --test-threads=1
//
// We need to run the tests in order. It seems that default test
// harness run then in alpabet order, so I use letter as module names
// here.

mod a {
    use super::*;

    #[test]
    fn register() {
        let mut resp = post(
            "users",
            json! ({"user": {"username": USERNAME, "email": EMAIL, "password": PASSWORD}}),
        );
        let status = resp.status();
        match status {
            reqwest::StatusCode::Ok => check_user_response(&mut resp),
            reqwest::StatusCode::UnprocessableEntity => {
                let body = resp.json::<ValidationErrors>()
                    .expect("Can't parse validation errors");
                if body.errors["username"] != vec!["has already been taken"] {
                    panic!("Got validation errors: {:#?}", body);
                }
            }
            _ => panic!("Got status: {}", status),
        }
    }
}

mod b {
    use super::*;

    #[test]
    fn login() {
        check_response(
            &mut post(
                "users/login",
                json!({"user": {"email": EMAIL, "password": PASSWORD}}),
            ),
            check_user_response,
        );
    }

    #[test]
    fn login_and_save_credentials() {
        check_response(
            &mut post(
                "users/login",
                json!({"user": {"email": EMAIL, "password": PASSWORD}}),
            ),
            |resp| {
                let wrapper = resp.json::<Value>().expect("Can't parse user");
                let user = wrapper.get("user").expect("Must have a 'user' field");
                let mut token = TOKEN.write().unwrap();
                *token = user.get("token")
                    .expect("User has token")
                    .as_str()
                    .expect("Token must be a string")
                    .to_string();
            },
        );
    }
}

mod c {
    use super::*;

    #[test]
    fn current_user() {
        check_response(&mut get("user"), check_user_response)
    }
}
