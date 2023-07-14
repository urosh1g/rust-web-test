use crate::{db, AppState};
use actix_web::{
    delete, get, post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/likes")
            .service(get_likes)
            .service(get_like)
            .service(add_like)
            .service(delete_like),
    );
}

#[get("")]
pub async fn get_likes(data: Data<AppState>) -> impl Responder {
    let res = db::like::get_likes(&data.db_pool).await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(vec),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{like_id}")]
pub async fn get_like(path: Path<i32>, data: Data<AppState>) -> impl Responder {
    let like_id = path.into_inner();
    let res = db::like::get_like(like_id, &data.db_pool).await;
    match res {
        Ok(opt) => match opt {
            Some(like) => HttpResponse::Ok().json(like),
            _ => HttpResponse::NotFound().json("like not found"),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("{user_id}/{article_id}")]
pub async fn add_like(path: Path<(i32, i32)>, data: Data<AppState>) -> impl Responder {
    let (user_id, article_id) = path.into_inner();
    let res = db::like::add_like(user_id, article_id, &data.db_pool).await;
    match res {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[delete("{like_id}")]
pub async fn delete_like(path: Path<i32>, data: Data<AppState>) -> impl Responder {
    let like_id = path.into_inner();
    let res = db::like::delete_like(like_id, &data.db_pool).await;
    match res {
        Ok(opt) => match opt {
            Some(like) => HttpResponse::Ok().json(like),
            _ => HttpResponse::NotFound().json(format!("like with id {like_id} not found")),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
