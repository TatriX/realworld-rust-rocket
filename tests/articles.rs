//! Test articles

mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};

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
fn test_get_articles() {
    let client = test_client();
    let response = &mut create_article(&client, login(&client));

    let slug = article_slug(response);
    // Slug can contain random prefix, this start_with, instead assert_eq!
    assert!(slug.starts_with(&ARTICLE_TITLE.to_lowercase().replace(' ', "-")));

    let response = &mut client
        .get(format!("/api/articles/{}", slug))
        .dispatch();

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
