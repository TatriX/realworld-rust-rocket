use schema::articles;
use chrono::NaiveDateTime;
use models::user::User;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub tag_list: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorites_count: i32,
}

impl Article {
    pub fn to_json(self, author: User) -> ArticleJson {
        ArticleJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorites_count: self.favorites_count,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: User,
    pub tag_list: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorites_count: i32,
}
