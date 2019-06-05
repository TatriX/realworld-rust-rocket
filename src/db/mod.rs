use rocket_contrib::databases::diesel;

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
