use rocket_contrib::{Json, Value};
use auth::Auth;
use db;
use models::user::Profile;

fn to_profile_json(profile: Profile) -> Json<Value> {
    Json(json!({ "profile": profile }))
}

#[get("/profiles/<username>")]
fn get_profile(username: String, auth: Option<Auth>, conn: db::Conn) -> Option<Json<Value>> {
    let user_id = auth.map(|auth| auth.id);
    db::profiles::find(&conn, &username, user_id).map(to_profile_json)
}

#[post("/profiles/<username>/follow")]
fn follow(username: String, auth: Auth, conn: db::Conn) -> Option<Json<Value>> {
    db::profiles::follow(&conn, &username, auth.id).map(to_profile_json)
}

#[delete("/profiles/<username>/follow")]
fn unfollow(username: String, auth: Auth, conn: db::Conn) -> Option<Json<Value>> {
    db::profiles::unfollow(&conn, &username, auth.id).map(to_profile_json)
}
