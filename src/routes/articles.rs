use crate::{
    auth, db,
    models::{
        article::{Article, NewArticle, UpdateArticle},
        comment::Comment,
        like::Like,
    },
    AppState,
};
use actix_web::{
    delete, get, post, put,
    web::{self, Path},
    HttpRequest, HttpResponse, Responder,
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
    let res = db::article::add_article(body.into_inner(), &app_data.db_pool).await;

    match res {
        Ok(article) => HttpResponse::Created().json(article),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{article_id}")]
pub async fn get_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = db::article::get_article(article_id, &app_data.db_pool).await;

    match res {
        Ok(optional) => match optional {
            Some(article) => HttpResponse::Ok().json(article),
            None => HttpResponse::NotFound().json("article not found"),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{article_id}/comments")]
pub async fn get_article_comments(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = sqlx::query_as!(
        Comment,
        "select * from comments where article_id = $1",
        article_id
    )
    .fetch_all(&data.db_pool)
    .await;
    match res {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("{article_id}/likes")]
pub async fn get_article_likes(path: Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = sqlx::query_as!(
        Like,
        "select * from likes where article_id = $1",
        article_id
    )
    .fetch_all(&data.db_pool)
    .await;
    match res {
        Ok(likes) => HttpResponse::Ok().json(likes),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[put("{article_id}")]
pub async fn update_article(
    body: web::Json<UpdateArticle>,
    path: Path<i32>,
    app_data: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let article_id = path.into_inner();

    let auth_cookie = req.cookie("token");

    let res = match auth_cookie {
        Some(token) => auth::verify_token(token),
        None => return HttpResponse::Unauthorized().json("asd"),
    };

    let claims = match res {
        Err(e) => {
            eprintln!("{}", e);
            return HttpResponse::Unauthorized().json("invalid token");
        }
        Ok(data) => data,
    };

    let user_id: i32 = claims.claims.sub;
    let article = db::article::get_article(article_id, &app_data.db_pool).await;

    match article {
        Ok(opt) => match opt {
            Some(article) => {
                if article.author_id != user_id {
                    return HttpResponse::Unauthorized()
                        .json("you dont have the permission to change other peoples articles");
                }
            }
            None => return HttpResponse::NotFound().json("Article not found"),
        },
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    }

    let res = db::article::update_article(article_id, body.into_inner(), &app_data.db_pool).await;
    match res {
        Ok(option) => match option {
            Some(article) => HttpResponse::Ok().json(article),
            None => {
                HttpResponse::NotFound().json(format!("article with id {} not found", article_id))
            }
        },
        Err(error) => HttpResponse::InternalServerError().json(error.to_string()),
    }
}

#[delete("{article_id}")]
pub async fn delete_article(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let article_id = path.into_inner();
    let res = db::article::delete_article(article_id, &app_data.db_pool).await;
    match res {
        Ok(option) => match option {
            Some(article) => HttpResponse::Ok().json(article),
            None => {
                HttpResponse::NotFound().json(format!("article with id {} not found", article_id))
            }
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
