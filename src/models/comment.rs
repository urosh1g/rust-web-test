use super::user::User;

// TODO UserComment -> umesto user_id, user: User
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Comment {
    pub comment_id: i32,
    pub user_id: i32,
    pub article_id: i32,
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UserComment {
    pub comment_id: i32,
    pub user: User,
    pub article_id: i32,
    pub content: String,
}

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct UserCommentJoin {
    pub comment_id: i32,
    pub article_id: i32,
    pub content: String,

    pub user_id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl From<UserCommentJoin> for UserComment {
    fn from(user_comment: UserCommentJoin) -> UserComment {
        UserComment {
            comment_id: user_comment.comment_id,
            article_id: user_comment.article_id,
            content: user_comment.content,
            user: User {
                user_id: user_comment.user_id,
                email: user_comment.email,
                password: user_comment.password,
            },
        }
    }
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
