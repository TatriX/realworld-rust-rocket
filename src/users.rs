use schema::users;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel;
use models::User;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    email: &'a str,
    password: &'a str,
) -> User {
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

pub fn login_user<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> Option<User> {
    let result = users::table
        .filter(users::email.eq(email))
        .get_result::<User>(conn);

    // TODO: get rid of pyramid
    match result {
        Err(err) => {
            println!("login_user: {}", err);
            None
        }
        Ok(user) => match scrypt_check(password, &user.hash) {
            Ok(valid) => {
                if valid {
                    Some(user)
                } else {
                    None
                }
            }
            Err(err) => {
                println!("login_user scrypt_check: {}", err);
                None
            }
        },
    }
}

pub fn find_user(conn: &PgConnection, id: i32) -> Option<User> {
    let result = users::table.find(id).get_result::<User>(conn);
    match result {
        Err(err) => {
            println!("find_user: {}", err);
            None
        }
        Ok(user) => Some(user),
    }
}

pub fn update_user(conn: &PgConnection, id: i32) -> Option<User> {
    None
}
