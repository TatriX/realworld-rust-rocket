use rocket::config::{Config, Environment, Value};
use rocket_contrib::databases::diesel;
use std::collections::HashMap;
use std::env;

pub mod articles;
pub mod comments;
pub mod profiles;
pub mod users;

pub fn config() -> Config {
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let secret_key = env::var("SECRET_KEY").expect("No SECRET_KEY environment variable found");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_postgres_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .secret_key(secret_key)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);
