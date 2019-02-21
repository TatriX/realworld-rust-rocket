use rocket_contrib::json::{Json, JsonValue};
use crate::db;

#[get("/tags")]
pub fn get_tags(conn: db::Conn) -> Json<JsonValue> {
    Json(json!({ "tags": db::articles::tags(&conn) }))
}
