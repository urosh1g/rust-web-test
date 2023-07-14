use crate::auth;
use crate::{
    models::{
        claim::UserClaims,
        user::{NewUser, User},
    },
    AppState,
};
use actix_web::{
    cookie, post,
    web::{self},
    HttpResponse, Responder,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user).service(login_user);
}

#[post("/register")]
async fn register_user(body: web::Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as!(
        User,
        "insert into users (email, password) values ( $1, $2 ) returning *",
        body.email,
        body.password
    )
    .fetch_one(&data.db_pool)
    .await;

    let user = match res {
        Ok(usr) => usr,
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    };

    let claims = UserClaims::from(&user);

    let token = match auth::generate_token(claims) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().json("couldnt generate jwt"),
    };

    HttpResponse::Ok()
        .cookie(cookie::Cookie::new("token", token))
        .json(user)
}

#[post("/login")]
async fn login_user(body: web::Json<NewUser>, data: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query_as!(
        User,
        "select * from users where email = $1 and password = $2",
        body.email,
        body.password
    )
    .fetch_optional(&data.db_pool)
    .await;

    let user = match res {
        Ok(opt) => match opt {
            Some(user) => user,
            _ => return HttpResponse::NotFound().json("user not found"),
        },
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    };

    let claims = UserClaims::from(&user);

    let token = match auth::generate_token(claims) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().json("couldnt generate jwt"),
    };

    HttpResponse::Ok()
        .cookie(cookie::Cookie::new("token", token))
        .json(user)
}
