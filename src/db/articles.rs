use diesel;
use diesel::prelude::*;
use schema::articles;
use schema::users;
use diesel::pg::PgConnection;
use models::article::{Article, ArticleJson};
use models::user::User;
use slug::slugify;
use rand::{self, Rng};

const SUFFIX_LEN: usize = 6;

#[derive(Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
    title: &'a str,
    description: &'a str,
    body: &'a str,
    slug: &'a str,
    author: i32,
    tag_list: &'a Vec<String>,
}

pub fn create<'a>(
    conn: &PgConnection,
    author: i32,
    title: &'a str,
    description: &'a str,
    body: &'a str,
    tag_list: &'a Vec<String>,
) -> Article {
    let new_article = &NewArticle {
        title,
        description,
        body,
        author,
        tag_list,
        slug: &format!("{}-{}", slugify(title), generate_suffix(SUFFIX_LEN)),
    };

    diesel::insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(conn)
        .expect("Error creating article")
}

fn generate_suffix(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(len)
        .collect::<String>()
}

pub fn find(conn: &PgConnection, slug: &str) -> Option<ArticleJson> {
    let result = articles::table
        .filter(articles::slug.eq(slug))
        .first::<Article>(conn);
    match result {
        Err(err) => {
            println!("find_article: {}", err);
            None
        }
        Ok(article) => {
            let author = users::table
                .find(article.author)
                .get_result::<User>(conn)
                .expect("Error loading author");
            Some(article.to_json(author))
        }
    }
}
