use crate::models::comment::*;
use sqlx::{Error, Pool, Postgres};

pub async fn get_comments(executor: &Pool<Postgres>) -> Result<Vec<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"
        select c.comment_id, c.article_id,
        c.content, u.user_id, u.email, u.password from comments c
        inner join users u on u.user_id = c.user_id"#
    )
    .fetch_all(executor)
    .await
}

pub async fn get_comment(
    comment_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"
        select c.comment_id, c.article_id,
        c.content, u.user_id, u.email, u.password from comments c
        inner join users u on u.user_id = c.user_id
        where c.comment_id = $1"#,
        comment_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn add_comment(
    comment: NewComment,
    executor: &Pool<Postgres>,
) -> Result<Option<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"with inserted_comment as (
            insert into comments 
            ( user_id, article_id, content ) 
            values ( $1, $2, $3 ) returning *)
        select c.article_id, c.content, c.comment_id, 
        u.user_id, u.email, u.password
        from inserted_comment c inner join
        users u on c.user_id = u.user_id"#,
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
) -> Result<Option<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"with updated_comment as 
        (update comments set content = $1 where comment_id = $2 returning *)
        select comment_id, article_id, content, u.user_id, u.email, u.password
        from updated_comment c inner join users u
        on u.user_id = c.user_id
        "#,
        update_fields.content,
        comment_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn delete_comment(
    comment_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"with deleted_comment as 
        (delete from comments where comment_id = $1 returning *)
        select comment_id, article_id, content, u.user_id, u.email, u.password
        from deleted_comment c inner join users u on
        u.user_id = c.user_id"#,
        comment_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn from_article(
    article_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Vec<UserCommentJoin>, Error> {
    sqlx::query_as!(
        UserCommentJoin,
        r#"select comment_id, c.content, c.article_id, u.user_id, u.email, u.password
        from comments c
        inner join users u on c.user_id = u.user_id
        inner join articles a on 
        a.article_id = $1"#,
        article_id
    )
    .fetch_all(executor)
    .await
}
