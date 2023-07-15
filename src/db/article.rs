use crate::models::article::{ArticleUserJoin, NewArticle, UpdateArticle};
use sqlx::{Error, Pool, Postgres};

pub async fn get_articles(executor: &Pool<Postgres>) -> Result<Vec<ArticleUserJoin>, Error> {
    sqlx::query_as!(
        ArticleUserJoin,
        "select * from articles inner join users on author_id = user_id"
    )
    .fetch_all(executor)
    .await
}

pub async fn get_article(
    article_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<ArticleUserJoin>, Error> {
    sqlx::query_as!(
        ArticleUserJoin,
        "select * from articles inner join users on author_id = user_id where article_id = $1",
        article_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn add_article(
    article: NewArticle,
    executor: &Pool<Postgres>,
) -> Result<Option<ArticleUserJoin>, Error> {
    sqlx::query_as!(
        ArticleUserJoin,
        r#"with inserted_article as (
            insert into articles 
            ( author_id, title, content ) 
            values ( $1, $2, $3 ) returning *)
        select * from inserted_article inner join users
        on author_id = user_id"#,
        article.author_id,
        article.title,
        article.content
    )
    .fetch_optional(executor)
    .await
}

pub async fn update_article(
    article_id: i32,
    update_fields: UpdateArticle,
    executor: &Pool<Postgres>,
) -> Result<Option<ArticleUserJoin>, Error> {
    sqlx::query_as!(
        ArticleUserJoin,
        r#"update articles 
        set title = coalesce($1, title), 
        content = coalesce($2, content) 
        from users where article_id = $3 and author_id = user_id 
        returning *"#,
        update_fields.title,
        update_fields.content,
        article_id
    )
    .fetch_optional(executor)
    .await
}

pub async fn delete_article(
    article_id: i32,
    executor: &Pool<Postgres>,
) -> Result<Option<ArticleUserJoin>, Error> {
    sqlx::query_as!(
        ArticleUserJoin,
        "delete from articles using users where article_id = $1 returning *",
        article_id,
    )
    .fetch_optional(executor)
    .await
}
