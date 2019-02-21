#![feature(proc_macro_hygiene, decl_macro)]
// silence diesel warning untill its new version is released
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;

#[macro_use]
extern crate diesel;

use validator;
#[macro_use]
extern crate validator_derive;

use frank_jwt as jwt;

mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod util;

use rocket_contrib::json::{Json, JsonValue};

#[catch(404)]
fn not_found() -> Json<JsonValue> {
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
                routes::users::post_users,
                routes::users::post_users_login,
                routes::users::put_user,
                routes::users::get_user,
                routes::articles::post_articles,
                routes::articles::put_articles,
                routes::articles::get_article,
                routes::articles::delete_article,
                routes::articles::favorite_article,
                routes::articles::unfavorite_article,
                routes::articles::get_articles,
                routes::articles::get_articles_feed,
                routes::articles::post_comment,
                routes::articles::get_comments,
                routes::articles::delete_comment,
                routes::tags::get_tags,
                routes::profiles::get_profile,
                routes::profiles::follow,
                routes::profiles::unfollow,
            ],
        )
        .manage(pool)
        .attach(rocket_cors::Cors::default())
        .register(catchers![not_found])
        .launch();
}
