use schema::{follows, users};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::user::{Profile, User};

pub fn find(conn: &PgConnection, name: &str, user_id: Option<i32>) -> Option<Profile> {
    let result = users::table
        .filter(users::username.eq(name))
        .get_result::<User>(conn);
    match result {
        Err(err) => {
            println!("find_user_by_name: {}", err);
            None
        }
        Ok(user) => {
            let following = match user_id {
                Some(user_id) => is_following(conn, &user, user_id),
                None => false,
            };
            Some(user.to_profile(following))
        }
    }
}

fn is_following(conn: &PgConnection, user: &User, user_id: i32) -> bool {
    use diesel::select;
    use diesel::dsl::exists;

    select(exists(follows::table.find((user_id, user.id))))
        .get_result(conn)
        .expect("Error loading following")
}

pub fn follow(conn: &PgConnection, followed_name: &str, follower_id: i32) -> Option<Profile> {
    let followed = users::table
        .filter(users::username.eq(followed_name))
        .get_result::<User>(conn)
        .expect("Cannot load followed");

    diesel::insert_into(follows::table)
        .values((
            follows::followed.eq(followed.id),
            follows::follower.eq(follower_id),
        ))
        .execute(conn)
        .expect("Cannot follow");

    Some(followed.to_profile(true))
}

pub fn unfollow(conn: &PgConnection, followed_name: &str, follower_id: i32) -> Option<Profile> {
    let followed = users::table
        .filter(users::username.eq(followed_name))
        .get_result::<User>(conn)
        .expect("Cannot load followed");

    diesel::delete(follows::table.find((follower_id, followed.id)))
        .execute(conn)
        .expect("Cannot unfollow");

    Some(followed.to_profile(false))
}
