use diesel;
use diesel::prelude::*;
use schema::articles;
use schema::users;
use schema::comments;
use diesel::pg::PgConnection;
use models::comment::{Comment, CommentJson};
use models::user::User;

#[derive(Insertable)]
#[table_name = "comments"]
struct NewComment<'a> {
    body: &'a str,
    author: i32,
    article: i32,
}

pub fn create<'a>(conn: &PgConnection, author: i32, slug: &'a str, body: &'a str) -> CommentJson {
    let article_id = articles::table
        .select(articles::id)
        .filter(articles::slug.eq(slug))
        .get_result::<i32>(conn)
        .expect("Cannot find article id");
    let new_comment = &NewComment {
        body,
        author,
        article: article_id,
    };

    let author = users::table
        .find(author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    diesel::insert_into(comments::table)
        .values(new_comment)
        .get_result::<Comment>(conn)
        .expect("Error creating comment")
        .attach(author)
}

pub fn find_by_slug(conn: &PgConnection, slug: &str) -> Vec<CommentJson> {
    let result = comments::table
        .inner_join(articles::table)
        .inner_join(users::table)
        .select((comments::all_columns, users::all_columns))
        .filter(articles::slug.eq(slug))
        .get_results::<(Comment, User)>(conn)
        .expect("Cannot load comments");

    result
        .into_iter()
        .map(|(comment, author)| comment.attach(author))
        .collect()
}

pub fn delete<'a>(conn: &PgConnection, author: i32, slug: &'a str, comment_id: i32) {
    use diesel::select;
    use diesel::dsl::exists;

    let belongs_to_author_result = select(exists(
        articles::table.filter(articles::slug.eq(slug).and(articles::author.eq(author))),
    )).get_result::<bool>(conn);

    if let Err(err) = belongs_to_author_result {
        match err {
            diesel::result::Error::NotFound => return,
            _ => panic!("Cannot find article by author: {}", err),
        }
    }

    let result = diesel::delete(comments::table.filter(comments::id.eq(comment_id))).execute(conn);
    if let Err(err) = result {
        println!("comments::delete: {}", err);
    }
}
