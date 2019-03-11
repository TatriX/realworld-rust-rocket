use crate::models::user::User;
use crate::schema::users;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;
use diesel::dsl::{exists, select};

#[allow(unused_variables)]
pub fn username_exists(conn: &PgConnection, username: &str) -> bool {
    use self::users::dsl::*;
    let username_exists = select(exists(users.filter(username.eq(username))))
        .get_result(conn)
        .expect("exist username");
    username_exists
}

#[allow(unused_variables)]
pub fn email_exists(conn: &PgConnection, email: &str) -> bool {
    use self::users::dsl::*;
    select(exists(users.filter(email.eq(email))))
        .get_result(conn)
        .expect("exist email")
}



#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub fn create(conn: &PgConnection, username: &str, email: &str, password: &str) -> User {
    // see https://blog.filippo.io/the-scrypt-parameters
    let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

    let new_user = &NewUser {
        username,
        email,
        hash,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn)
        .expect("Error saving user")
}

pub fn login(conn: &PgConnection, email: &str, password: &str) -> Option<User> {
    let user = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn)
        .map_err(|err| println!("login_user: {}", err))
        .ok()?;

    let password_matches = scrypt_check(password, &user.hash)
        .map_err(|err| println!("login_user: scrypt_check: {}", err))
        .ok()?;

    if password_matches {
        Some(user)
    } else {
        println!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub fn find(conn: &PgConnection, id: i32) -> Option<User> {
    users::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

// TODO: remove clone when diesel will allow skipping fields
#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "users"]
pub struct UpdateUserData {
    username: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    image: Option<String>,

    // hack to skip the field
    #[column_name = "hash"]
    password: Option<String>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateUserData) -> Option<User> {
    let data = &UpdateUserData {
        password: None,
        ..data.clone()
    };
    diesel::update(users::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
