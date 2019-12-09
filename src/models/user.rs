use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::Serialize;

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

#[derive(Serialize)]
pub struct Profile {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            username: &self.username,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            image: self.image.as_ref().map(String::as_str),
            token,
        }
    }

    pub fn to_profile(self, following: bool) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}
