#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub comment_id: i32,
    pub user_id: i32,    // TODO user: User
    pub article_id: i32, //      article: Article
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct NewComment {
    pub user_id: i32,
    pub article_id: i32,
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UpdateComment {
    pub content: String,
}
