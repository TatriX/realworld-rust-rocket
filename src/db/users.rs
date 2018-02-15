use schema::users;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use models::user::User;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub fn create<'a>(
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

pub fn login<'a>(conn: &PgConnection, email: &'a str, password: &'a str) -> Option<User> {
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

pub fn find(conn: &PgConnection, id: i32) -> Option<User> {
    let result = users::table.find(id).get_result::<User>(conn);
    match result {
        Err(err) => {
            println!("find_user: {}", err);
            None
        }
        Ok(user) => Some(user),
    }
}

pub fn find_by_name(conn: &PgConnection, name: &str) -> Option<User> {
    let result = users::table
        .filter(users::username.eq(name))
        .get_result::<User>(conn);
    match result {
        Err(err) => {
            println!("find_user_by_name: {}", err);
            None
        }
        Ok(user) => Some(user),
    }
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
