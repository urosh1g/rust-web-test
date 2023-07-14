#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Like {
    pub like_id: i32,
    pub user_id: i32,
    pub article_id: i32,
}
