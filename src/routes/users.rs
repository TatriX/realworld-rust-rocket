use crate::auth::Auth;
use crate::db;
use crate::errors::Errors;
use crate::util::extract_string;
use diesel::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize)]
pub struct NewUser {
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

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<Json<JsonValue>, Errors> {
    use crate::schema::users;

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

    let user = db::users::create(&conn, &username, &email, &password);
    Ok(Json(json!({ "user": user.to_user_auth() })))
}

#[derive(Deserialize)]
pub struct LoginUser {
    user: LoginUserData,
}

#[derive(Deserialize)]
struct LoginUserData {
    email: Option<String>,
    password: Option<String>,
}

#[post("/users/login", format = "application/json", data = "<user>")]
pub fn post_users_login(user: Json<LoginUser>, conn: db::Conn) -> Result<Json<JsonValue>, Errors> {
    let mut errors = Errors::new();
    let email = extract_string(&user.user.email, "email", &mut errors);
    let password = extract_string(&user.user.password, "password", &mut errors);
    match db::users::login(&conn, &email, &password) {
        Some(user) => Ok(Json(json!({ "user": user.to_user_auth() }))),
        None => {
            errors.add("email or password", ValidationError::new("is invalid"));
            Err(errors)
        }
    }
}

#[get("/user")]
pub fn get_user(auth: Auth, conn: db::Conn) -> Option<Json<JsonValue>> {
    db::users::find(&conn, auth.id).map(|user| Json(json!({ "user": user.to_user_auth() })))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user: db::users::UpdateUserData,
}

#[put("/user", format = "application/json", data = "<user>")]
pub fn put_user(user: Json<UpdateUser>, auth: Auth, conn: db::Conn) -> Option<Json<JsonValue>> {
    db::users::update(&conn, auth.id, &user.user)
        .map(|user| Json(json!({ "user": user.to_user_auth() })))
}
