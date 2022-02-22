use realworld;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    realworld::rocket().launch().await
}
