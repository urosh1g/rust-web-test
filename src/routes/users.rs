use crate::{
    models::user::{NewUser, UpdateUser, User},
    AppState,
};
use actix_web::{delete, get, post, put, web, web::Path, HttpResponse, Responder};
//TODO
//mozda moze lepsi error handling

// ako se stvarno ovako pristupa konekciji ka bazi
// ( preko web::Data<AppState> )
// da li je moguce napraviti recimo
// trait DbHandler {
//     fn get_users() -> Vec<User>;
//     fn add_user(user_request: NewUser);
//     ...
// }
// da AppState ima db_pool + da implementira DbHandler
// i onda umesto pisanja sql query-a u svakom request handleru
// da se samo pozivaju metode objekta koji implementira DbHandler
// unutar AppState-a
#[get("")]
pub async fn get_users(app_data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as!(User, "select * from users")
        .fetch_all(&app_data.db_pool)
        .await
    {
        Ok(vec) => HttpResponse::Ok().json(vec),
        Err(_) => HttpResponse::InternalServerError().json("err"),
    }
}

#[get("/{user_id}")]
pub async fn get_user(path: Path<u32>, _app_data: web::Data<AppState>) -> impl Responder {
    let _user_id: u32 = path.into_inner();
    let res = sqlx::query_as!(User, "select * from users where id = $1", _user_id as i32)
        .fetch_one(&_app_data.db_pool)
        .await;
    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("")]
pub async fn add_user(body: web::Json<NewUser>, app_state: web::Data<AppState>) -> impl Responder {
    let res = sqlx::query("insert into users ( email, password ) values ($1, $2)")
        .bind(&body.email)
        .bind(&body.password)
        .execute(&app_state.db_pool)
        .await;
    match res {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("{user_id}")]
pub async fn update_user(
    path: Path<i32>,
    app_data: web::Data<AppState>,
    body: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = path.into_inner();
    //sqlx::Error::RowNotFound + fetch_optional
    //da li ima smisla?
    //ako ne postoji red ( RowNotFound ) onda fetch_optional
    //sigurno vraca None?
    let res = sqlx::query_as!(User, "select * from users where id = $1", user_id)
        .fetch_one(&app_data.db_pool)
        .await;
    let user = match res {
        Ok(option) => option,
        Err(err) => {
            return HttpResponse::InternalServerError().json(err.to_string());
        }
    };
    // puno to_owned() [u pozadini takodje clone()?] i clone() ako je veliki objekat
    // bljak?
    let res = sqlx::query!(
        "update users set email = $1, password = $2 where id = $3",
        body.email
            .to_owned()
            .unwrap_or_else(|| { user.email.clone() }),
        body.password
            .to_owned()
            .unwrap_or_else(|| { user.password.clone() }),
        user_id
    )
    .execute(&app_data.db_pool)
    .await;
    match res {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[delete("{user_id}")]
pub async fn delete_user(path: Path<i32>, app_data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let res = sqlx::query!("delete from users where id = $1", user_id)
        .execute(&app_data.db_pool)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().json(format!("Sucessfully removed user with id {user_id}")),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
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
