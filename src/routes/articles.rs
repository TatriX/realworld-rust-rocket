use crate::auth::Auth;
use crate::db;
use crate::db::articles::{FeedArticles, FindArticles};
use crate::errors::{Errors, FieldValidator};
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewArticle {
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
pub fn post_articles(
    auth: Auth,
    new_article: Json<NewArticle>,
    conn: db::Conn,
) -> Result<Json<JsonValue>, Errors> {
    let new_article = new_article.into_inner().article;

    let mut extractor = FieldValidator::validate(&new_article);
    let title = extractor.extract("title", new_article.title);
    let description = extractor.extract("description", new_article.description);
    let body = extractor.extract("body", new_article.body);
    extractor.check()?;

    let article = db::articles::create(
        &conn,
        auth.id,
        &title,
        &description,
        &body,
        &new_article.tag_list,
    );
    Ok(Json(json!({ "article": article })))
}

/// return multiple articles, ordered by most recent first
#[get("/articles?<params..>")]
pub fn get_articles(
    params: Form<FindArticles>,
    auth: Option<Auth>,
    conn: db::Conn,
) -> Json<JsonValue> {
    let user_id = auth.map(|x| x.id);
    let articles = db::articles::find(&conn, &params, user_id);
    Json(json!({ "articles": articles, "articlesCount": articles.len() }))
}

#[get("/articles/<slug>")]
pub fn get_article(slug: String, auth: Option<Auth>, conn: db::Conn) -> Option<Json<JsonValue>> {
    let user_id = auth.map(|x| x.id);
    db::articles::find_one(&conn, &slug, user_id).map(|article| Json(json!({ "article": article })))
}

#[delete("/articles/<slug>")]
pub fn delete_article(slug: String, auth: Auth, conn: db::Conn) {
    db::articles::delete(&conn, &slug, auth.id);
}

#[post("/articles/<slug>/favorite")]
pub fn favorite_article(slug: String, auth: Auth, conn: db::Conn) -> Option<Json<JsonValue>> {
    db::articles::favorite(&conn, &slug, auth.id).map(|article| Json(json!({ "article": article })))
}

#[delete("/articles/<slug>/favorite")]
pub fn unfavorite_article(slug: String, auth: Auth, conn: db::Conn) -> Option<Json<JsonValue>> {
    db::articles::unfavorite(&conn, &slug, auth.id)
        .map(|article| Json(json!({ "article": article })))
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    article: db::articles::UpdateArticleData,
}

#[put("/articles/<slug>", format = "application/json", data = "<article>")]
pub fn put_articles(
    slug: String,
    article: Json<UpdateArticle>,
    auth: Auth,
    conn: db::Conn,
) -> Option<Json<JsonValue>> {
    // TODO: check auth
    db::articles::update(&conn, &slug, auth.id, article.into_inner().article)
        .map(|article| Json(json!({ "article": article })))
}

#[derive(Deserialize)]
pub struct NewComment {
    comment: NewCommentData,
}

#[derive(Deserialize, Validate)]
pub struct NewCommentData {
    #[validate(length(min = "1"))]
    body: Option<String>,
}

#[post(
    "/articles/<slug>/comments",
    format = "application/json",
    data = "<new_comment>"
)]
pub fn post_comment(
    slug: String,
    new_comment: Json<NewComment>,
    auth: Auth,
    conn: db::Conn,
) -> Result<Json<JsonValue>, Errors> {
    let new_comment = new_comment.into_inner().comment;

    let mut extractor = FieldValidator::validate(&new_comment);
    let body = extractor.extract("body", new_comment.body);
    extractor.check()?;

    let comment = db::comments::create(&conn, auth.id, &slug, &body);
    Ok(Json(json!({ "comment": comment })))
}

#[delete("/articles/<slug>/comments/<id>")]
pub fn delete_comment(slug: String, id: i32, auth: Auth, conn: db::Conn) {
    db::comments::delete(&conn, auth.id, &slug, id);
}

#[get("/articles/<slug>/comments")]
pub fn get_comments(slug: String, conn: db::Conn, auth: Option<Auth>) -> Json<JsonValue> {
    let auth_id = auth.and_then(|auth| Some(auth.id));
    let comments = db::comments::find_by_slug(&conn, &slug, auth_id);
    Json(json!({ "comments": comments }))
}

#[get("/articles/feed?<params..>")]
pub fn get_articles_feed(
    params: Form<FeedArticles>,
    auth: Auth,
    conn: db::Conn,
) -> Json<JsonValue> {
    let articles = db::articles::feed(&conn, &params, auth.id);
    let articles_count = articles.len();
    Json(json!({ "articles": articles, "articlesCount": articles_count }))
}
