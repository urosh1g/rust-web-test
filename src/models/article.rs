#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Article {
    pub id: i32,
    pub author_id: i32, // TODO replace with author: User
    pub title: String,
    pub content: String,
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
