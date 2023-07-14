use crate::models::like::*;
use sqlx::{Error, Pool, Postgres};

pub async fn get_likes(executor: &Pool<Postgres>) -> Result<Vec<Like>, Error> {
    sqlx::query_as!(Like, "select * from likes")
        .fetch_all(executor)
        .await
}

pub async fn get_like(like_id: i32, executor: &Pool<Postgres>) -> Result<Option<Like>, Error> {
    sqlx::query_as!(Like, "select * from likes where like_id = $1", like_id)
        .fetch_optional(executor)
        .await
}

pub async fn add_like(
    user_id: i32,
    article_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Like, Error> {
    sqlx::query_as!(
        Like,
        "insert into likes ( user_id, article_id ) values ( $1, $2 ) returning *",
        user_id,
        article_id
    )
    .fetch_one(executor)
    .await
}

pub async fn delete_like(like_id: i32, executor: &Pool<Postgres>) -> Result<Option<Like>, Error> {
    sqlx::query_as!(
        Like,
        "delete from likes where like_id = $1 returning *",
        like_id
    )
    .fetch_optional(executor)
    .await
}
