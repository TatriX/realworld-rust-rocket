use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorites_count: i32,
}
