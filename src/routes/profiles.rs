use rocket_contrib::{Json, Value};
use auth::Auth;
use db;

#[derive(Serialize)]
struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

#[get("/profiles/<username>")]
fn get_profile(username: String, auth: Option<Auth>, conn: db::Conn) -> Option<Json<Value>> {
    db::users::find_by_name(&conn, &username).map(|user| {
        Json(json!({
            "profile": Profile{
                username: user.username,
                bio: user.bio,
                image: user.image,
                following: false, //TODO: get following
            }
        }))
    })
}
