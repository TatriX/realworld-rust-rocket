use diesel;
use diesel::prelude::*;
use schema::articles;
use schema::users;
use schema::favorites;
use diesel::pg::PgConnection;
use models::article::{Article, ArticleJson};
use models::user::User;
use slug;
use rand::{self, Rng};

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;

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
        .attach(author, false)
}

fn slugify(title: &str) -> String {
    if cfg!(feature = "random_suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    rand::thread_rng()
        .gen_ascii_chars()
        .take(len)
        .collect::<String>()
}

#[derive(FromForm, Default)]
pub struct FindArticles {
    tag: Option<String>,
    author: Option<String>,
    /// favorited by user
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn find(conn: &PgConnection, params: FindArticles, user_id: i32) -> Vec<ArticleJson> {
    let mut query = articles::table
        .inner_join(users::table)
        .left_join(
            favorites::table.on(articles::id
                .eq(favorites::article)
                .and(favorites::user.eq(user_id))),
        )
        .select((
            articles::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .into_boxed();
    if let Some(author) = params.author {
        query = query.filter(users::username.eq(author))
    }
    if let Some(tag) = params.tag {
        query = query.or_filter(articles::tag_list.contains(vec![tag]))
    }
    if let Some(favorited) = params.favorited {
        let id = users::table
            .select(users::id)
            .filter(users::username.eq(favorited))
            .get_result::<i32>(conn)
            .expect("Cannot load favorited user id");
        query = query.filter(diesel::dsl::sql(&format!(
            "article.id IN (SELECT favorites.article FROM favorites WHERE favorites.user = {})",
            id
        )));
    }
    // println!("{}", diesel::debug_query(&query).to_string());

    let result = query
        .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
        .offset(params.offset.unwrap_or(0))
        .load::<(Article, User, bool)>(conn)
        .expect("Cannot load articles");

    result
        .into_iter()
        .map(|(article, author, favorited)| article.attach(author, favorited))
        .collect()
}

pub fn find_one(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
    let result = articles::table
        .filter(articles::slug.eq(slug))
        .first::<Article>(conn);

    match result {
        Err(err) => {
            println!("articles::find_one: {}", err);
            None
        }
        Ok(article) => {
            let favorited = is_favorite(conn, &article, user_id);
            Some(populate(conn, article, favorited))
        }
    }
}

pub fn favorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count + 1))
            .get_result::<Article>(conn)?;

        diesel::insert_into(favorites::table)
            .values((
                favorites::user.eq(user_id),
                favorites::article.eq(article.id),
            ))
            .execute(conn)?;

        Ok(populate(conn, article, true))
    }).map_err(|err| println!("articles::favorite: {}", err))
        .ok()
}

pub fn unfavorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
            .set(articles::favorites_count.eq(articles::favorites_count - 1))
            .get_result::<Article>(conn)?;

        diesel::delete(favorites::table.find((user_id, article.id))).execute(conn)?;

        Ok(populate(conn, article, false))
    }).map_err(|err| println!("articles::unfavorite: {}", err))
        .ok()
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

pub fn update(
    conn: &PgConnection,
    slug: &str,
    user_id: i32,
    data: &UpdateArticleData,
) -> Option<ArticleJson> {
    let mut data = data.clone();
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    // TODO: check for not_found
    let article = diesel::update(articles::table.filter(articles::slug.eq(slug)))
        .set(&data)
        .get_result(conn)
        .expect("Error loading article");

    let favorited = is_favorite(conn, &article, user_id);
    Some(populate(conn, article, favorited))
}

pub fn delete(conn: &PgConnection, slug: &str) {
    let result = diesel::delete(articles::table.filter(articles::slug.eq(slug))).execute(conn);
    if let Err(err) = result {
        println!("articles::delete: {}", err);
    }
}

fn is_favorite(conn: &PgConnection, article: &Article, user_id: i32) -> bool {
    use diesel::select;
    use diesel::dsl::exists;

    select(exists(favorites::table.find((user_id, article.id))))
        .get_result(conn)
        .expect("Error loading favorited")
}

fn populate(conn: &PgConnection, article: Article, favorited: bool) -> ArticleJson {
    let author = users::table
        .find(article.author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    article.attach(author, favorited)
}
