use crate::{db, models::like::Like, AppState};
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
    generate_response(res)
}

#[post("{user_id}/{article_id}")]
pub async fn add_like(path: Path<(i32, i32)>, data: Data<AppState>) -> impl Responder {
    let (user_id, article_id) = path.into_inner();
    let res = db::like::add_like(user_id, article_id, &data.db_pool).await;
    generate_response(res)
}

#[delete("{like_id}")]
pub async fn delete_like(path: Path<i32>, data: Data<AppState>) -> impl Responder {
    let like_id = path.into_inner();
    let res = db::like::delete_like(like_id, &data.db_pool).await;
    generate_response(res)
}

fn generate_response(db_result: Result<Option<Like>, sqlx::Error>) -> impl Responder {
    match db_result {
        Ok(Some(like)) => HttpResponse::Ok().json(like),
        Ok(None) => HttpResponse::NotFound().json("like not found"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
