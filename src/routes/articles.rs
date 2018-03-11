use rocket_contrib::{Json, Value};
use auth::Auth;
use validator::{Validate, ValidationErrors};
use db;
use errors::Errors;
use util::extract_string;
use db::articles::{FeedArticles, FindArticles};

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

/// return multiple articles, ordered by most recent first
#[get("/articles")]
fn get_articles(auth: Option<Auth>, conn: db::Conn) -> Json<Value> {
    get_articles_with_params(FindArticles::default(), auth, conn)
}

/// return multiple articles, ordered by most recent first
#[get("/articles?<params>")]
fn get_articles_with_params(
    params: FindArticles,
    auth: Option<Auth>,
    conn: db::Conn,
) -> Json<Value> {
    let user_id = auth.map(|x| x.id);
    let articles = db::articles::find(&conn, params, user_id);
    Json(json!({ "articles": articles, "articlesCount": articles.len() }))
}

#[get("/articles/<slug>")]
fn get_article(slug: String, auth: Option<Auth>, conn: db::Conn) -> Option<Json<Value>> {
    let user_id = auth.map(|x| x.id);
    db::articles::find_one(&conn, &slug, user_id).map(|article| Json(json!({ "article": article })))
}

#[delete("/articles/<slug>")]
fn delete_article(slug: String, auth: Auth, conn: db::Conn) {
    db::articles::delete(&conn, &slug, auth.id);
}

#[post("/articles/<slug>/favorite")]
fn favorite_article(slug: String, auth: Auth, conn: db::Conn) -> Option<Json<Value>> {
    db::articles::favorite(&conn, &slug, auth.id).map(|article| Json(json!({ "article": article })))
}

#[delete("/articles/<slug>/favorite")]
fn unfavorite_article(slug: String, auth: Auth, conn: db::Conn) -> Option<Json<Value>> {
    db::articles::unfavorite(&conn, &slug, auth.id)
        .map(|article| Json(json!({ "article": article })))
}

#[derive(Deserialize)]
struct UpdateArticle {
    article: db::articles::UpdateArticleData,
}

#[put("/articles/<slug>", format = "application/json", data = "<article>")]
fn put_articles(
    slug: String,
    article: Json<UpdateArticle>,
    auth: Auth,
    conn: db::Conn,
) -> Option<Json<Value>> {
    // TODO: check auth
    db::articles::update(&conn, &slug, auth.id, &article.article)
        .map(|article| Json(json!({ "article": article })))
}

#[derive(Deserialize)]
struct NewComment {
    comment: NewCommentData,
}

#[derive(Deserialize, Validate)]
pub struct NewCommentData {
    #[validate(length(min = "1"))]
    body: Option<String>,
}

#[post("/articles/<slug>/comments", format = "application/json", data = "<new_comment>")]
fn post_comment(
    slug: String,
    new_comment: Json<NewComment>,
    auth: Auth,
    conn: db::Conn,
) -> Result<Json<Value>, Errors> {
    let mut errors = Errors {
        errors: new_comment
            .comment
            .validate()
            .err()
            .unwrap_or_else(ValidationErrors::new),
    };

    let body = extract_string(&new_comment.comment.body, "body", &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }

    let comment = db::comments::create(&conn, auth.id, &slug, &body);
    Ok(Json(json!({ "comment": comment })))
}

#[delete("/articles/<slug>/comments/<id>")]
fn delete_comment(slug: String, id: i32, auth: Auth, conn: db::Conn) {
    db::comments::delete(&conn, auth.id, &slug, id);
}

#[get("/articles/<slug>/comments")]
fn get_comments(slug: String, conn: db::Conn) -> Json<Value> {
    let comments = db::comments::find_by_slug(&conn, &slug);
    Json(json!({ "comments": comments }))
}

#[get("/articles/feed?<params>")]
fn get_articles_feed(params: FeedArticles, auth: Auth, conn: db::Conn) -> Json<Value> {
    let articles = db::articles::feed(&conn, params, auth.id);
    Json(json!({ "articles": articles }))
}
