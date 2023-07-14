use super::user::User;
use chrono::{self, offset::Utc};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserClaims {
    pub iat: usize,
    pub sub: i32,
    pub email: String,
    pub aud: String,
    pub exp: usize,
    pub iss: String,
}

impl UserClaims {
    pub fn from(user: &User) -> Self {
        Self {
            sub: user.user_id,
            email: user.email.clone(),
            exp: (Utc::now() + chrono::Duration::minutes(60)).timestamp() as usize,
            aud: String::from("audience"),
            iss: String::from("uros"),
            iat: Utc::now().timestamp() as usize,
        }
    }
}
