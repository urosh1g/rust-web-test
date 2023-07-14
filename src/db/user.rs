use crate::models::user::{NewUser, UpdateUser, User};
use sqlx::{Error, Pool, Postgres};

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<User>, Error> {
    sqlx::query_as!(User, "select * from users")
        .fetch_all(pool)
        .await
}

pub async fn get_user(user_id: i32, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    sqlx::query_as!(User, "select * from users where user_id = $1", user_id)
        .fetch_optional(pool)
        .await
}

pub async fn add_user(user: NewUser, pool: &Pool<Postgres>) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "insert into users ( email, password ) values ($1, $2) returning *",
        user.email,
        user.password,
    )
    .fetch_one(pool)
    .await
}

pub async fn update_user(
    user_id: i32,
    update_fields: UpdateUser,
    pool: &Pool<Postgres>,
) -> Result<Option<User>, Error> {
    sqlx::query_as!(User,
        "update users set email = coalesce($1, email), password = coalesce($2, password) where user_id = $3 returning *",
        update_fields.email,
        update_fields.password,
        user_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_user(user_id: i32, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    sqlx::query_as!(
        User,
        "delete from users where user_id = $1 returning *",
        user_id
    )
    .fetch_optional(pool)
    .await
}
