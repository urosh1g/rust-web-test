use crate::{
    models::comment::{Comment, NewComment, UpdateComment},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Path},
    HttpResponse, Responder,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .service(get_comments)
            .service(add_comment)
            .service(get_comment)
            .service(update_comment)
            .service(delete_comment),
    );
}

#[get("")]
pub async fn get_comments(data: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as!(Comment, "select * from comments")
        .fetch_all(&data.db_pool)
        .await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(vec),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{comment_id}")]
pub async fn get_comment(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let comment_id = path.into_inner(); // proveri dal handluje pogresne vrednosti
    let res = sqlx::query_as!(Comment, "select * from comments where id = $1", comment_id)
        .fetch_optional(&data.db_pool)
        .await;
    match res {
        Ok(opt) => match opt {
            Some(comment) => HttpResponse::Ok().json(comment),
            None => {
                HttpResponse::NotFound().json(format!("comment with id {} not found", comment_id))
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("")]
pub async fn add_comment(body: web::Json<NewComment>, data: web::Data<AppState>) -> impl Responder {
    let user = sqlx::query!("select id from users where id = $1", body.user_id)
        .fetch_one(&data.db_pool)
        .await;
    if let Err(_) = user {
        return HttpResponse::NotFound().json(format!("user with id {} not found", body.user_id));
    }
    let article = sqlx::query!("select id from articles where id = $1", body.article_id)
        .fetch_one(&data.db_pool)
        .await;
    if let Err(_) = article {
        return HttpResponse::NotFound()
            .json(format!("article with id {} not found", body.article_id));
    }
    let res = sqlx::query_as!(
        Comment,
        "insert into comments ( user_id, article_id, content ) values ( $1, $2, $3 ) returning *",
        body.user_id,
        body.article_id,
        body.content
    )
    .fetch_optional(&data.db_pool)
    .await;
    match res {
        Ok(opt) => match opt {
            Some(row) => HttpResponse::Created().json(row),
            _ => HttpResponse::NotFound().json("nesto nije nadjeno???????"),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[put("{comment_id}")]
pub async fn update_comment(
    path: Path<i32>,
    body: web::Json<UpdateComment>,
    data: web::Data<AppState>,
) -> impl Responder {
    let comment_id = path.into_inner();
    let res = sqlx::query_as!(
        Comment,
        "update comments set content = $1 where id = $2 returning *",
        body.content,
        comment_id
    )
    .fetch_optional(&data.db_pool)
    .await;
    match res {
        Ok(opt) => match opt {
            Some(comment) => HttpResponse::Ok().json(comment),
            _ => HttpResponse::NotFound().json(format!("comment with id {comment_id} not found")),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[delete("{comment_id}")]
pub async fn delete_comment(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let comment_id = path.into_inner();
    let res = sqlx::query_as!(
        Comment,
        "delete from comments where id = $1 returning *",
        comment_id
    )
    .fetch_optional(&data.db_pool)
    .await;
    match res {
        Ok(opt) => match opt {
            Some(comment) => HttpResponse::Ok().json(comment),
            _ => HttpResponse::NotFound().json(format!("comment with id {comment_id} not found")),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
