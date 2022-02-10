use crate::database::{self, Db};
use rocket::serde::json::{json, Value};

#[get("/tags")]
pub async fn get_tags(db: Db) -> Value {
    let tags = db.run(move |conn| database::articles::tags(conn)).await;
    json!({ "tags": tags })
}
