use rocket_contrib::{Json, Value};
use auth::Auth;
use validator::{Validate, ValidationErrors};
use db;
use errors::Errors;
use util::extract_string;

#[derive(Deserialize)]
struct NewArticle {
    article: NewArticleData,
}

#[derive(Deserialize, Validate)]
pub struct NewArticleData {
    #[validate(length(min = "1"))]
    title: Option<String>,
    #[validate(length(min = "1"))]
    description: Option<String>,
    #[validate(length(min = "1"))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

#[post("/articles", format = "application/json", data = "<new_article>")]
fn post_articles(
    auth: Auth,
    new_article: Json<NewArticle>,
    conn: db::Conn,
) -> Result<Json<Value>, Errors> {
    let mut errors = Errors {
        errors: new_article
            .article
            .validate()
            .err()
            .unwrap_or_else(ValidationErrors::new),
    };

    let title = extract_string(&new_article.article.title, "title", &mut errors);
    let description = extract_string(&new_article.article.description, "description", &mut errors);
    let body = extract_string(&new_article.article.body, "body", &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }

    let article = db::articles::create(
        &conn,
        auth.id,
        &title,
        &description,
        &body,
        &new_article.article.tag_list,
    );
    Ok(Json(json!({ "article": article })))
}

#[get("/articles/<slug>")]
fn get_articles(slug: String, conn: db::Conn) -> Option<Json<Value>> {
    db::articles::find(&conn, &slug).map(|article| Json(json!({ "article": article })))
}

// #[get("/articles")]
// fn get_articles() -> Json<Value> {
//     Json(json!({"articles": []}))
// }

#[get("/articles/<slug>/comments")]
fn get_articles_comments(slug: String) -> Json<Value> {
    Json(json!({"comments": []}))
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
