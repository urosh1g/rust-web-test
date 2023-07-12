use crate::{
    models::article::{Article, NewArticle, UpdateArticle},
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Path},
    HttpResponse, Responder,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/articles")
        .service(get_articles)
        .service(add_article)
        .service(get_article)
        .service(update_article)
        .service(delete_article);
    cfg.service(scope);
}

#[get("")]
pub async fn get_articles(app_data: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as!(Article, "select * from articles")
        .fetch_all(&app_data.db_pool)
        .await;
    match res {
        Ok(vec) => HttpResponse::Ok().json(vec),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("")]
pub async fn add_article(
    body: web::Json<NewArticle>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    //TODO proveri da li autor postoji
    let res = sqlx::query!(
        "insert into articles ( author_id, title, content ) values ( $1, $2, $3 )",
        body.author_id,
        body.title,
        body.content
    )
    .execute(&app_data.db_pool)
    .await;
    match res {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("{article_id}")]
pub async fn get_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = sqlx::query_as!(Article, "select * from articles where id = $1", article_id)
        .fetch_optional(&app_data.db_pool)
        .await;
    match res {
        Ok(optional) => match optional {
            Some(article) => HttpResponse::Ok().json(article),
            None => HttpResponse::NotFound().json("article not found"),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[put("{article_id}")]
pub async fn update_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

#[delete("{article_id}")]
pub async fn delete_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}
