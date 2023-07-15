use crate::{
    db,
    models::user::{NewUser, UpdateUser},
    AppState,
};
use actix_web::{delete, get, post, put, web, web::Path, HttpResponse, Responder};

#[get("")]
pub async fn get_users(app_data: web::Data<AppState>) -> impl Responder {
    let res = db::user::get_users(&app_data.db_pool).await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/{user_id}")]
pub async fn get_user(path: Path<u32>, _app_data: web::Data<AppState>) -> impl Responder {
    let _user_id: u32 = path.into_inner();
    let res = db::user::get_user(_user_id as i32, &_app_data.db_pool).await;
    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(format!("user with id {} not found", _user_id)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("")]
pub async fn add_user(body: web::Json<NewUser>, app_state: web::Data<AppState>) -> impl Responder {
    let res = db::user::add_user(body.into_inner(), &app_state.db_pool).await;
    match res {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[put("{user_id}")]
pub async fn update_user(
    path: Path<i32>,
    app_data: web::Data<AppState>,
    body: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = path.into_inner();
    let res = db::user::update_user(user_id, body.into_inner(), &app_data.db_pool).await;

    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(format!("user with id {user_id} not found")),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("{user_id}")]
pub async fn delete_user(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let res = db::user::delete_user(user_id, &app_data.db_pool).await;

    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(format!("user with id {} not found", user_id)),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(add_user)
        .service(get_users)
        .service(get_user)
        .service(delete_user)
        .service(update_user);
    cfg.service(scope);
}
