use crate::models::comment::*;
use sqlx::{Error, Pool, Postgres};

pub async fn get_comments(executor: &Pool<Postgres>) -> Result<Vec<Comment>, Error> {
    sqlx::query_as!(Comment, "select * from comments")
        .fetch_all(executor)
        .await
}

pub async fn get_comment(
    comment_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<Comment>, Error> {
    sqlx::query_as!(
        Comment,
        "select * from comments where comment_id = $1",
        comment_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn add_comment(
    comment: NewComment,
    executor: &Pool<Postgres>,
) -> Result<Option<Comment>, Error> {
    let _ = sqlx::query!(
        "select user_id from users where user_id = $1",
        comment.user_id
    )
    .fetch_one(executor)
    .await?;
    let _ = sqlx::query!(
        "select article_id from articles where article_id = $1",
        comment.article_id
    )
    .fetch_one(executor)
    .await?;

    sqlx::query_as!(
        Comment,
        "insert into comments ( user_id, article_id, content ) values ( $1, $2, $3 ) returning *",
        comment.user_id,
        comment.article_id,
        comment.content
    )
    .fetch_optional(executor)
    .await
}

pub async fn update_comment(
    comment_id: i32,
    update_fields: UpdateComment,
    executor: &Pool<Postgres>,
) -> Result<Option<Comment>, Error> {
    sqlx::query_as!(
        Comment,
        "update comments set content = $1 where comment_id = $2 returning *",
        update_fields.content,
        comment_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn delete_comment(
    comment_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<Comment>, Error> {
    sqlx::query_as!(
        Comment,
        "delete from comments where comment_id = $1 returning *",
        comment_id
    )
    .fetch_optional(executor)
    .await
}
