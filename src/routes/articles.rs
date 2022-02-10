use crate::auth::Auth;
use crate::database::articles::{FeedArticles, FindArticles};
use crate::database::{self, Db};
use crate::errors::{Errors, FieldValidator};
use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewArticle {
    article: NewArticleData,
}

#[derive(Deserialize, Validate)]
pub struct NewArticleData {
    #[validate(length(min = 1))]
    title: Option<String>,
    #[validate(length(min = 1))]
    description: Option<String>,
    #[validate(length(min = 1))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

#[post("/articles", format = "json", data = "<new_article>")]
pub async fn post_articles(
    auth: Auth,
    new_article: Json<NewArticle>,
    db: Db,
) -> Result<Value, Errors> {
    let new_article = new_article.into_inner().article;

    let mut extractor = FieldValidator::validate(&new_article);
    let title = extractor.extract("title", new_article.title);
    let description = extractor.extract("description", new_article.description);
    let body = extractor.extract("body", new_article.body);
    extractor.check()?;

    let article = db
        .run(move |conn| {
            database::articles::create(
                conn,
                auth.id,
                &title,
                &description,
                &body,
                &new_article.tag_list,
            )
        })
        .await;
    Ok(json!({ "article": article }))
}

/// return multiple articles, ordered by most recent first
#[get("/articles?<tag>&<author>&<favorited>&<limit>&<offset>")]
pub async fn get_articles(
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    auth: Option<Auth>,
    db: Db,
) -> Value {
    let params = FindArticles {
        tag,
        author,
        favorited,
        limit,
        offset,
    };
    let user_id = auth.map(|x| x.id);
    let articles = db
        .run(move |conn| database::articles::find(conn, &params, user_id))
        .await;
    json!({ "articles": articles.0, "articlesCount": articles.1 })
}

#[get("/articles/<slug>")]
pub async fn get_article(slug: String, auth: Option<Auth>, db: Db) -> Option<Value> {
    let user_id = auth.map(|x| x.id);
    db.run(move |conn| database::articles::find_one(conn, &slug, user_id))
        .await
        .map(|article| json!({ "article": article }))
}

#[delete("/articles/<slug>")]
pub async fn delete_article(slug: String, auth: Auth, db: Db) {
    db.run(move |conn| {
        database::articles::delete(conn, &slug, auth.id);
    })
    .await;
}

#[post("/articles/<slug>/favorite")]
pub async fn favorite_article(slug: String, auth: Auth, db: Db) -> Option<Value> {
    db.run(move |conn| database::articles::favorite(conn, &slug, auth.id))
        .await
        .map(|article| json!({ "article": article }))
}

#[delete("/articles/<slug>/favorite")]
pub async fn unfavorite_article(slug: String, auth: Auth, db: Db) -> Option<Value> {
    db.run(move |conn| database::articles::unfavorite(conn, &slug, auth.id))
        .await
        .map(|article| json!({ "article": article }))
}

#[derive(Deserialize)]
pub struct UpdateArticle {
    article: database::articles::UpdateArticleData,
}

#[put("/articles/<slug>", format = "json", data = "<article>")]
pub async fn put_articles(
    slug: String,
    article: Json<UpdateArticle>,
    auth: Auth,
    db: Db,
) -> Option<Value> {
    // TODO: check auth
    db.run(move |conn| {
        database::articles::update(conn, &slug, auth.id, article.into_inner().article)
    })
    .await
    .map(|article| json!({ "article": article }))
}

#[derive(Deserialize)]
pub struct NewComment {
    comment: NewCommentData,
}

#[derive(Deserialize, Validate)]
pub struct NewCommentData {
    #[validate(length(min = 1))]
    body: Option<String>,
}

#[post("/articles/<slug>/comments", format = "json", data = "<new_comment>")]
pub async fn post_comment(
    slug: String,
    new_comment: Json<NewComment>,
    auth: Auth,
    db: Db,
) -> Result<Value, Errors> {
    let new_comment = new_comment.into_inner().comment;

    let mut extractor = FieldValidator::validate(&new_comment);
    let body = extractor.extract("body", new_comment.body);
    extractor.check()?;

    let comment = db
        .run(move |conn| database::comments::create(conn, auth.id, &slug, &body))
        .await;
    Ok(json!({ "comment": comment }))
}

#[delete("/articles/<slug>/comments/<id>")]
pub async fn delete_comment(slug: String, id: i32, auth: Auth, db: Db) {
    db.run(move |conn| database::comments::delete(conn, auth.id, &slug, id))
        .await
}

#[get("/articles/<slug>/comments")]
pub async fn get_comments(slug: String, db: Db) -> Value {
    let comments = db
        .run(move |conn| database::comments::find_by_slug(conn, &slug))
        .await;
    json!({ "comments": comments })
}

#[get("/articles/feed?<limit>&<offset>")]
pub async fn get_articles_feed(
    limit: Option<i64>,
    offset: Option<i64>,
    auth: Auth,
    db: Db,
) -> Value {
    let params = FeedArticles { limit, offset };
    let articles = db
        .run(move |conn| database::articles::feed(conn, &params, auth.id))
        .await;
    let articles_count = articles.len();
    json!({ "articles": articles, "articlesCount": articles_count })
}
