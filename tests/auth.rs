extern crate reqwest;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;

const API_URL: &'static str = "http://localhost:8000/api";

const USERNAME: &'static str = "rust-diesel-rocket";
const EMAIL: &'static str = "rust-diesel-rocket@example.com";
const PASSWORD: &'static str = "qweasdzxc";

#[derive(Debug, Deserialize)]
struct ValidationErrors {
    errors: HashMap<String, Vec<String>>,
}

fn make_url(path: &str) -> String {
    format!("{}/{}", API_URL, path)
}

fn post(path: &str, json: Value) -> reqwest::Response {
    reqwest::Client::new()
        .post(&make_url(path))
        .json(&json)
        .send()
        .expect(&format!("{} {:#?}: post error", path, json))
}

#[test]
fn register() {
    let mut resp: reqwest::Response = post(
        "users",
        json! ({"user": {"username": USERNAME, "email": EMAIL, "password": PASSWORD}}),
    );
    let status = resp.status();
    match status {
        reqwest::StatusCode::Ok => {
            // everything is fine
        }
        reqwest::StatusCode::UnprocessableEntity => {
            let body = resp.json::<ValidationErrors>().unwrap();
            if body.errors["username"] != vec!["has already been taken"] {
                panic!("Got validation errors: {:#?}", body);
            }
        }
        _ => panic!("Got status: {}", status),
    }
}
