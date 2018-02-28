use rocket_contrib::{Json, Value};
use db::Conn;
use db;

#[get("/tags")]
fn get_tags(conn: db::Conn) -> Json<Value> {
    Json(json!({ "tags": db::articles::tags(&conn) }))
}
