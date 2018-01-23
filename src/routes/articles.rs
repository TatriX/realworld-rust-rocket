use rocket_contrib::{Json, Value};
use auth::Auth;
use validator::Validate;

#[derive(Deserialize)]
struct NewArticle {
    article: NewArticleData,
}

#[derive(Deserialize, Validate)]
struct NewArticleData {
    #[validate(length(min = "1"))]
    title: String,
    #[validate(length(min = "1"))]
    description: String,
    #[validate(length(min = "1"))]
    body: String,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

#[post("/articles", format = "application/json", data = "<new_article>")]
fn post_articles(auth: Auth, new_article: Json<NewArticle>) -> Json<Value> {
    Json(json!({}))
}

#[get("/articles")]
fn get_articles() -> Json<Value> {
    Json(json!({"articles": []}))
}

#[derive(FromForm)]
struct FeedArticles {
    limit: Option<u32>,
    offset: Option<u32>,
}

#[get("/articles/feed?<params>")]
fn get_articles_feed(params: FeedArticles) -> Json<Value> {
    Json(json!({"articles": []}))
}
