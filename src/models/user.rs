use crate::auth::Auth;
use chrono::{Duration, Utc};
use diesel::PgConnection;
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
    pub fn to_user_auth(&self) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Auth {
            id: self.id,
            username: self.username.clone(),
            exp: exp.timestamp(),
        }
        .token();

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
    /// Return a `Profile` adding the `following` propertyfor a given `user_id`. If `None`
    /// `user_id` is given, the following option of profile always take `false`
    ///
    /// # Examples
    ///
    /// When `Some` `user_id` is given, the following is checked at the database
    ///
    /// ```rust
    /// # use diesel::PgConnection;
    /// let user_profile: Profile = user.to_profile_for(conn, Some(7));
    /// assert_eq!(user_profile.following, true);
    /// ```
    ///
    /// When `None` `user_id` is given, always the `following` property of the `Profile` returned
    /// is false
    ///
    /// ```rust
    /// # use diesel::PgConnection;
    /// let user_profile: Profile = user.to_profile_for(conn, None);
    /// assert_eq!(user_profile.following, false);
    /// ```
    pub fn to_profile_for(self, conn: &PgConnection, user_id: Option<i32>) -> Profile {
        use crate::db::profiles::is_following;
        let following = user_id.map_or(false, |user_id| is_following(conn, &self, user_id));
        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}
