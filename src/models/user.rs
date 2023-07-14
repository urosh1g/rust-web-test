#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
}
