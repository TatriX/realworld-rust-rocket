//! Test articles

mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::{Client, LocalResponse};

const ARTICLE_TITLE: &str = "Test article";
const ARTICLE_BODY: &str = "This is obviously a test article!";

#[test]
/// Test article creation.
fn test_post_articles() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token);

    let value = response_json_value(response);
    let title = value
        .get("article")
        .expect("must have an 'article' field")
        .get("title")
        .expect("must have a 'title' field")
        .as_str();

    assert_eq!(title, Some(ARTICLE_TITLE));
}

#[test]
/// Test article retrieval.
fn test_get_article() {
    let client = test_client();
    let response = &mut create_article(&client, login(&client));

    let slug = article_slug(response);
    // Slug can contain random prefix, thus `start_with` instead of `assert_eq`!
    assert!(slug.starts_with(&ARTICLE_TITLE.to_lowercase().replace(' ', "-")));

    let response = &mut client.get(format!("/api/articles/{}", slug)).dispatch();

    let value = response_json_value(response);
    let body = value
        .get("article")
        .and_then(|article| article.get("body"))
        .and_then(|body| body.as_str());

    assert_eq!(body, Some(ARTICLE_BODY));
}

#[test]
/// Test article update.
fn test_put_articles() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let new_desc = "Well, it's an updated test article";
    let response = &mut client
        .put(format!("/api/articles/{}", slug))
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
            "article": {
                "description": new_desc,
                "tagList": ["test", "foo"]
            }
        }))
        .dispatch();

    let value = response_json_value(response);
    let description = value
        .get("article")
        .and_then(|article| article.get("description"))
        .and_then(|description| description.as_str());

    assert_eq!(description, Some(new_desc));
}

#[test]
/// Test article deletion.
fn test_delete_article() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client
        .delete(format!("/api/articles/{}", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test that it's not possible to delete article anonymously.
fn test_delete_article_anonymously() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client.delete(format!("/api/articles/{}", slug)).dispatch();

    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
/// Test putting article to favorites.
fn test_favorite_article() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client
        .post(format!("/api/articles/{}/favorite", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test removing article from favorites .
fn test_unfavorite_article() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client
        .delete(format!("/api/articles/{}/favorite", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test getting multiple articles.
fn test_get_articles() {
    let client = test_client();
    let token = login(&client);
    create_article(&client, token);

    let response = &mut client.get("/api/articles").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let num = value
        .get("articlesCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'articlesCount' field");

    assert!(num > 0);
}

#[test]
/// Test getting multiple articles with params.
fn test_get_articles_with_params() {
    let client = test_client();
    let token = login(&client);
    create_article(&client, token);

    let url = "/api/articles?tag=foo&author=smoketest&favorited=smoketest&limit=1&offset=0";
    let response = &mut client.get(url).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    value
        .get("articlesCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'articlesCount' field");
}

#[test]
/// Test getting articles feed.
fn test_get_articles_fedd() {
    let client = test_client();
    let token = login(&client);

    let url = "/api/articles/feed?limit=1&offset=0";
    let response = &mut client.get(url).header(token_header(token)).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    value.get("articles").expect("must have 'articles' field");
}

#[test]
/// Test posting and deleteing of comments.
fn test_commenting() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client
        .post(format!("/api/articles/{}/comments", slug))
        .header(ContentType::JSON)
        .header(token_header(token.clone()))
        .body(json_string!({
            "comment": {
                "body": "Like!",
            }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let comment_id = value
        .get("comment")
        .and_then(|comment| comment.get("id"))
        .and_then(|id| id.as_i64())
        .expect("must have comment 'id' field");

    let response = client
        .delete(format!("/api/articles/{}/comments/{}", slug, comment_id))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test getting comments.
fn test_get_comment() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_article(&client, token.clone());

    let slug = article_slug(response);

    let response = &mut client
        .get(format!("/api/articles/{}/comments", slug))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let comments_num = value
        .get("comments")
        .and_then(|comments| comments.as_array())
        .map(|comments| comments.len())
        .expect("must have 'comments' field");
    // Newly created article must have no comments
    assert_eq!(comments_num, 0);
}

// Utility functions

fn article_slug(response: &mut LocalResponse) -> String {
    response_json_value(response)
        .get("article")
        .and_then(|article| article.get("slug"))
        .and_then(|slug| slug.as_str())
        .map(String::from)
        .expect("Cannot extract article slug")
}

fn create_article(client: &Client, token: Token) -> LocalResponse {
    let response = client
        .post("/api/articles")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
                "article": {
                    "title": ARTICLE_TITLE,
                    "description": "Well, it's a test article",
                    "body": ARTICLE_BODY,
                    "tagList": ["test", "foo", "bar"]
                }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    response
}
