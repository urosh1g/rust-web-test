use crate::{
    db,
    models::comment::{NewComment, UpdateComment},
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
    let res = db::comment::get_comments(&data.db_pool).await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(vec),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{comment_id}")]
pub async fn get_comment(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let comment_id = path.into_inner(); // proveri dal handluje pogresne vrednosti
    let res = db::comment::get_comment(comment_id, &data.db_pool).await;
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
    let res = db::comment::add_comment(body.into_inner(), &data.db_pool).await;
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
    let res = db::comment::update_comment(comment_id, body.into_inner(), &data.db_pool).await;
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
    let res = db::comment::delete_comment(comment_id, &data.db_pool).await;
    match res {
        Ok(opt) => match opt {
            Some(comment) => HttpResponse::Ok().json(comment),
            _ => HttpResponse::NotFound().json(format!("comment with id {comment_id} not found")),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
