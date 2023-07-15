use super::user::User;
use std::convert::From;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct DbArticle {
    pub article_id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub article_id: i32,
    pub author: User, // TODO replace with author: User
    pub title: String,
    pub content: String,
}

impl From<ArticleUserJoin> for Article {
    fn from(value: ArticleUserJoin) -> Article {
        Article {
            article_id: value.article_id,
            author: User {
                user_id: value.user_id,
                email: value.email,
                password: value.password,
            },
            title: value.title,
            content: value.content,
        }
    }
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ArticleUserJoin {
    pub article_id: i32,
    pub author_id: i32,
    pub title: String,
    pub content: String,

    pub user_id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct NewArticle {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UpdateArticle {
    pub title: Option<String>,
    pub content: Option<String>,
}
