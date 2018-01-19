use chrono::{Duration, Utc};
use jwt;

type Url = String;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<Url>,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    username: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    token: String,
}

impl User {
    pub fn to_user_auth(&self) -> UserAuth {
        let exp = Utc::now() + Duration::days(60);
        let payload = json!({
            "id" : self.id,
            "username" : &self.username,
            "exp": exp.timestamp(),
        });
        let header = json!({});
        let secret = "secret123";
        let token =
            jwt::encode(header, &secret.to_string(), &payload, jwt::Algorithm::HS256).expect("jwt");

        UserAuth {
            username: &self.username,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            image: self.image.as_ref().map(String::as_str),
            token,
        }
    }
}
