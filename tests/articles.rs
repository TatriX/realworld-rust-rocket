//! Test articles

mod common;

use common::*;
use rocket::http::ContentType;

#[test]
/// Test article creation.
fn test_post_articles() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .post("/api/articles")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
            "article": {
                "title": "Test article",
                "description": "Well, it's a test article",
                "body": "This is obviously a test article!",
                "tagList": ["test", "foo", "bar"]
            }
        }))
        .dispatch();

    let value = response_json_value(response);
    let title = value
        .get("article")
        .expect("must have an 'article' field")
        .get("title")
        .expect("must have a 'title' field")
        .as_str();

    assert_eq!(title, Some("Test article"));
}
