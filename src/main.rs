#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;

#[macro_use]
extern crate diesel;

extern crate validator;
#[macro_use]
extern crate validator_derive;

extern crate crypto;
extern crate dotenv;

extern crate chrono;
extern crate frank_jwt as jwt;

mod db;
mod schema;
mod models;
mod users;
mod errors;
mod auth;

use rocket_contrib::{Json, Value};

use users::*;
use auth::Auth;
use validator::{Validate, ValidationError, ValidationErrors};
use diesel::prelude::*;
use errors::Errors;

#[derive(Deserialize)]
struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = "1"))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = "8"))]
    password: Option<String>,
}

fn extract_string<'a>(
    maybe_string: &'a Option<String>,
    field_name: &'static str,
    errors: &mut Errors,
) -> &'a str {
    maybe_string
        .as_ref()
        .map(String::as_str)
        .unwrap_or_else(|| {
            errors.add(field_name, ValidationError::new("can't be blank"));
            ""
        })
}

#[post("/users", format = "application/json", data = "<new_user>")]
fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<Json<Value>, Errors> {
    use schema::users;

    let mut errors = Errors {
        errors: new_user
            .user
            .validate()
            .err()
            .unwrap_or_else(ValidationErrors::new),
    };

    let username = extract_string(&new_user.user.username, "username", &mut errors);
    let email = extract_string(&new_user.user.email, "email", &mut errors);
    let password = extract_string(&new_user.user.password, "password", &mut errors);

    let n: i64 = users::table
        .filter(users::username.eq(username))
        .count()
        .get_result(&*conn)
        .expect("count username");
    if n > 0 {
        errors.add("username", ValidationError::new("has already been taken"));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let user = create_user(&conn, &username, &email, &password);
    Ok(Json(json!({ "user": user.to_user_auth() })))
}

#[derive(Deserialize)]
struct LoginUser {
    user: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: Option<String>,
    password: Option<String>,
}

#[post("/users/login", format = "application/json", data = "<user>")]
fn post_users_login(user: Json<LoginUser>, conn: db::Conn) -> Result<Json<Value>, Errors> {
    let mut errors = Errors::new();
    let email = extract_string(&user.user.email, "email", &mut errors);
    let password = extract_string(&user.user.password, "password", &mut errors);
    match login_user(&conn, &email, &password) {
        Some(user) => Ok(Json(json!({ "user": user.to_user_auth() }))),
        None => {
            errors.add("email or password", ValidationError::new("is invalid"));
            Err(errors)
        }
    }
}

#[get("/user")]
fn get_user(auth: Auth, conn: db::Conn) -> Option<Json<Value>> {
    println!("Auth: {:?}", auth);
    find_user(&conn, 1).map(|user| Json(json!({ "user": user })))
}

#[derive(Deserialize)]
struct UpdateUser {
    user: UpdateUserData,
}

#[derive(Deserialize)]
struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    image: Option<String>,
    password: Option<String>,
}

#[put("/user", data = "<user>")]
fn put_user(user: Json<UpdateUser>, conn: db::Conn) -> Option<Json<Value>> {
    find_user(&conn, 1).map(|user| Json(json!({ "user": user })))
}

#[get("/profiles/<username>")]
fn get_profile(username: String) -> Option<Json<Value>> {
    None
}

#[get("/articles")]
fn get_articles() -> Json<Value> {
    Json(json!({"articles": []}))
}

#[derive(FromForm)]
struct FeedArticles {
    limit: Option<u32>,
    offset: Option<u32>,
}

#[get("/articles/feed?<params>")]
fn get_articles_feed(params: FeedArticles) -> Json<Value> {
    Json(json!({"articles": []}))
}

#[get("/tags")]
fn get_tags() -> Json<Value> {
    Json(json!({"tags": []}))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    let pool = db::init_pool();

    rocket::ignite()
        .mount(
            "/api",
            routes![
                post_users,
                post_users_login,
                get_user,
                get_articles,
                get_articles_feed,
                get_tags
            ],
        )
        .manage(pool)
        .attach(rocket_cors::Cors::default())
        .catch(errors![not_found])
        .launch();
}
