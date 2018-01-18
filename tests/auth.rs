extern crate reqwest;

#[macro_use]
extern crate serde_derive;

const API_URL: &'static str = "http://localhost:8000/api";

const USERNAME: &'static str = "rust-diesel-rocket";
const EMAIL: &'static str = "rust-diesel-rocket@example.com";
const PASSWORD: &'static str = "qweasdzxc";

fn make_url(path: &str) -> String {
    format!("{}/{}", API_URL, path)
}

#[derive(Serialize)]
struct SignUp<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}


#[test]
fn register() {
    let response = reqwest::Client::new()
        .post(&make_url("users"))
        .json(&SignUp{
            username: USERNAME,
            email: EMAIL,
            password: PASSWORD,
        })
        .send()
        .expect("register should work");
    println!("{:?}", response);
    panic!();
}
