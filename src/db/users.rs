use crate::models::user::{Profile, User};
use crate::schema::users;
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;

/// Return a `Profile` for a given `user` adding the `following` property.
/// `following` take true if  `Some(user_id)` is given, and `user` is follower
/// of user with id `user_id`.
///
/// If  `user_id` is `None`, returned `Profile.following`  always take `false`
///
/// # Examples
///
/// When `Some` `user_id` is given, the `following` is checked at the database
///
/// ```rust
/// # use crate::db::to_profile_for;
/// let user_profile_follower: Profile = to_profile_for(conn, &user, Some(7) /* following */);
/// assert_eq!(user_profile.following, true);
/// let user_profile_not_follow: Profile = to_profile_for(conn, &user, Some(8) /* not following */);
/// assert_eq!(user_profile_not_follow.following, false);
/// ```
///
/// When `None` `user_id` is given, always the `following` property of the `Profile` returned
/// is false
///
/// ```rust
/// # use crate::db::to_profile_for;
/// let user_profile: Profile = to_profile_for(conn, &user,  None);
/// assert_eq!(user_profile.following, false);
/// ```
pub fn to_profile_for(conn: &PgConnection, user_from: &User, user_for_id: Option<i32>) -> Profile {
    use crate::db::profiles::is_following;
    let following = user_for_id.map_or(false, |user_id| is_following(conn, user_from, user_id));
    Profile::new(
        user_from.username.clone(),
        user_from.bio.clone(),
        user_from.image.clone(),
        following,
    )
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

pub fn username_exists(conn: &PgConnection, username: &str) -> bool {
    select(exists(users::table.filter(users::username.eq(username))))
        .get_result(conn)
        .expect("exist username")
}

pub fn email_exists(conn: &PgConnection, email: &str) -> bool {
    select(exists(users::table.filter(users::email.eq(email))))
        .get_result(conn)
        .expect("exist email")
}
