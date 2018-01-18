#![feature(match_default_bindings)]
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

use rocket_contrib::{Json, Value};
use rocket::fairing::AdHoc;

use users::*;
use validator::{Validate, ValidationError, ValidationErrors};
use diesel::prelude::*;
use errors::Errors;

#[derive(Deserialize)]
struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = "8"))]
    password: String,
}

#[post("/users", format = "application/json", data = "<new_user>")]
fn post_user(new_user: Json<NewUser>, conn: db::Conn) -> Result<Json<Value>, Errors> {
    use schema::users;

    let mut errors = Errors {
        errors: new_user
            .user
            .validate()
            .err()
            .unwrap_or_else(ValidationErrors::new),
    };

    let NewUserData {
        username,
        email,
        password,
    } = &new_user.user;

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
        .mount("/api", routes![post_user])
        .manage(pool)
        .attach(rocket_cors::Cors::default())
        .attach(AdHoc::on_response(|_req, resp| {
            println!("{:?}", resp);
        }))
        .catch(errors![not_found])
        .launch();
}
