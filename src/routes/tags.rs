use crate::db;
use rocket_contrib::json::{Json, JsonValue};

#[get("/tags")]
pub fn get_tags(conn: db::Conn) -> Json<JsonValue> {
    Json(json!({ "tags": db::articles::tags(&conn) }))
}
