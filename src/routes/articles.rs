use crate::{
    db,
    models::article::{Article, NewArticle, UpdateArticle},
    models::comment::UserComment,
    models::like::UserLike,
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Path},
    HttpResponse, Responder,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/articles")
        .service(get_articles)
        .service(add_article)
        .service(get_article)
        .service(update_article)
        .service(delete_article)
        .service(get_article_comments)
        .service(get_article_likes);
    cfg.service(scope);
}

#[get("")]
pub async fn get_articles(app_data: web::Data<AppState>) -> impl Responder {
    let res = db::article::get_articles(&app_data.db_pool).await;

    match res {
        Ok(vec) => HttpResponse::Ok().json(
            vec.into_iter()
                .map(|item| Article::from(item))
                .collect::<Vec<Article>>(),
        ),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("")]
pub async fn add_article(
    body: web::Json<NewArticle>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let res = db::article::add_article(1, body.into_inner(), &app_data.db_pool).await;

    match res {
        Ok(Some(flat_article)) => HttpResponse::Created().json(Article::from(flat_article)),
        Ok(None) => HttpResponse::InternalServerError().json("row not inserted"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{article_id}")]
pub async fn get_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = db::article::get_article(article_id, &app_data.db_pool).await;

    match res {
        Ok(Some(flat_article)) => HttpResponse::Ok().json(Article::from(flat_article)),
        Ok(None) => {
            HttpResponse::NotFound().json(format!("article with id {article_id} not found"))
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{article_id}/comments")]
pub async fn get_article_comments(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = db::comment::from_article(article_id, &data.db_pool).await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(
            vec.into_iter()
                .map(|flat_comment| UserComment::from(flat_comment))
                .collect::<Vec<UserComment>>(),
        ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("{article_id}/likes")]
pub async fn get_article_likes(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = db::like::from_article(article_id, &data.db_pool).await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(
            vec.into_iter()
                .map(|flat_like| UserLike::from(flat_like))
                .collect::<Vec<UserLike>>(),
        ),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("{article_id}")]
pub async fn update_article(
    body: web::Json<UpdateArticle>,
    path: Path<i32>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let article_id = path.into_inner();

    let res = db::article::update_article(article_id, body.into_inner(), &app_data.db_pool).await;

    match res {
        Ok(Some(flat_article)) => HttpResponse::Ok().json(Article::from(flat_article)),
        Ok(None) => HttpResponse::NotFound().json("article not found"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("{article_id}")]
pub async fn delete_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();

    let res = db::article::delete_article(article_id, &app_data.db_pool).await;
    match res {
        Ok(Some(flat_article)) => HttpResponse::Ok().json(Article::from(flat_article)),
        Ok(None) => HttpResponse::NotFound().json("article not found"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
