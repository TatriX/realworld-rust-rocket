use crate::db;
use rocket::serde::json::{json, Value};

#[get("/tags")]
pub fn get_tags(conn: db::Conn) -> Value {
    json!({ "tags": db::articles::tags(&conn) })
}
