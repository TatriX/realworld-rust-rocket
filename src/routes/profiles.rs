use rocket_contrib::{Json, Value};

#[get("/profiles/<username>")]
fn get_profile(username: String) -> Option<Json<Value>> {
    None
}
