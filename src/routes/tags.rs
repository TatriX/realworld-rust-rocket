use rocket_contrib::{Json, Value};

#[get("/tags")]
fn get_tags() -> Json<Value> {
    Json(json!({"tags": []}))
}
