use diesel;
use diesel::prelude::*;
use schema::articles;
use schema::users;
use diesel::pg::PgConnection;
use models::article::{Article, ArticleJson};
use models::user::User;
use slug;
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
) -> ArticleJson {
    let new_article = &NewArticle {
        title,
        description,
        body,
        author,
        tag_list,
        slug: &slugify(title),
    };

    let author = users::table
        .find(author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    diesel::insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(conn)
        .expect("Error creating article")
        .attach(author)
}

fn slugify(title: &str) -> String {
    format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
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
        Ok(article) => Some(populate(conn, article)),
    }
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "articles"]
pub struct UpdateArticleData {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
    #[serde(skip)]
    slug: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

pub fn update(conn: &PgConnection, slug: &str, data: &UpdateArticleData) -> Option<ArticleJson> {
    let mut data = data.clone();
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    // TODO: check for not_found
    let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
        .set(&data)
        .get_result(conn)
        .expect("Error loading article");

    Some(populate(conn, article))
}

fn populate(conn: &PgConnection, article: Article) -> ArticleJson {
    let author = users::table
        .find(article.author)
        .get_result::<User>(conn)
        .expect("Error loading author");
    article.attach(author)
}
