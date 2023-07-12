use crate::{
    models::article::{Article, NewArticle, UpdateArticle},
    AppState,
};
use actix_web::{delete, get, post, put, web, web::Path, HttpResponse, Responder};

#[get("")]
pub async fn get_articles(app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("")]
pub async fn add_article(app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("{article_id}")]
pub async fn get_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[put("{article_id}")]
pub async fn update_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[delete("{article_id}")]
pub async fn delete_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/articles")
        .service(get_articles)
        .service(add_article)
        .service(get_article)
        .service(update_article)
        .service(delete_article);
    cfg.service(scope);
}
