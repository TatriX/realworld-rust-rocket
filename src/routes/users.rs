use crate::auth::Auth;
use crate::db::{self, users::UserCreationError};
use crate::errors::{Errors, FieldValidator};

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

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

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<JsonValue, Errors> {
    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    db::users::create(&conn, &username, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth() }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
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

#[post("/users/login", format = "json", data = "<user>")]
pub fn post_users_login(user: Json<LoginUser>, conn: db::Conn) -> Result<JsonValue, Errors> {
    let user = user.into_inner().user;

    let mut extractor = FieldValidator::default();
    let email = extractor.extract("email", user.email);
    let password = extractor.extract("password", user.password);
    extractor.check()?;

    db::users::login(&conn, &email, &password)
        .map(|user| json!({ "user": user.to_user_auth() }))
        .ok_or_else(|| Errors::new(&[("email or password", "is invalid")]))
}

#[get("/user")]
pub fn get_user(auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::users::find(&conn, auth.id).map(|user| json!({ "user": user.to_user_auth() }))
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user: db::users::UpdateUserData,
}

#[put("/user", format = "json", data = "<user>")]
pub fn put_user(user: Json<UpdateUser>, auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::users::update(&conn, auth.id, &user.user).map(|user| json!({ "user": user.to_user_auth() }))
}
