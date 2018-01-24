use diesel;
use diesel::prelude::*;
use schema::articles;
use schema::tags;
// use schema::article_tag;
use diesel::pg::PgConnection;
use models::article::Article;
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
}

pub fn create<'a>(
    conn: &PgConnection,
    author: i32,
    title: &'a str,
    description: &'a str,
    body: &'a str,
    tags: &Vec<String>,
) -> Article {
    let new_article = &NewArticle {
        title,
        description,
        body,
        author,
        slug: &format!("{}-{}", slugify(title), generate_suffix(SUFFIX_LEN)),
    };

    conn.transaction(|| {
        let tag_values: Vec<_> = tags.iter().map(|name| tags::name.eq(name)).collect();
        // TODO: use insert_or_ignore_into
        diesel::insert_into(tags::table)
            .values(&tag_values)
            .execute(conn)
            .expect("Error creatings tags");

        diesel::insert_into(articles::table)
            .values(new_article)
            .get_result::<Article>(conn)
    }).expect("Error creating article")
}

fn generate_suffix(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(len)
        .collect::<String>()
}

pub fn find(conn: &PgConnection, slug: &str) -> Option<Article> {
    let result = articles::table.filter(articles::slug.eq(slug)).first(conn);
    match result {
        Err(err) => {
            println!("find_article: {}", err);
            None
        }
        Ok(article) => Some(article),
    }
}
