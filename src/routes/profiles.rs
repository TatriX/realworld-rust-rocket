use crate::auth::Auth;
use crate::db::{self, Db};
use crate::models::user::Profile;
use rocket::serde::json::{json, Value};

fn to_profile_json(profile: Profile) -> Value {
    json!({ "profile": profile })
}

#[get("/profiles/<username>")]
pub async fn get_profile(username: String, auth: Option<Auth>, db: Db) -> Option<Value> {
    let user_id = auth.map(|auth| auth.id);
    db.run(move |conn| db::profiles::find(conn, &username, user_id))
        .await
        .map(to_profile_json)
}

#[post("/profiles/<username>/follow")]
pub async fn follow(username: String, auth: Auth, db: Db) -> Option<Value> {
    db.run(move |conn| db::profiles::follow(conn, &username, auth.id))
        .await
        .map(to_profile_json)
}

#[delete("/profiles/<username>/follow")]
pub async fn unfollow(username: String, auth: Auth, db: Db) -> Option<Value> {
    db.run(move |conn| db::profiles::unfollow(conn, &username, auth.id))
        .await
        .map(to_profile_json)
}
