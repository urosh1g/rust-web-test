use super::user::User;
// nema potrebe da like bude nested
// i da sadrzi ref na Article
// jer ce se lajkovi pribavljati samo
// iz Article-a
// moze umesto user_id ref na User
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Like {
    pub like_id: i32,
    pub user_id: i32,
    pub article_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserLike {
    pub like_id: i32,
    pub user: User,
    pub article_id: i32,
}

impl From<UserLikeJoin> for UserLike {
    fn from(val: UserLikeJoin) -> Self {
        Self {
            like_id: val.like_id,
            article_id: val.article_id,
            user: User {
                user_id: val.user_id,
                email: val.email,
                password: val.password,
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserLikeJoin {
    pub like_id: i32,
    pub user_id: i32,
    pub article_id: i32,
    pub email: String,
    pub password: String,
}
