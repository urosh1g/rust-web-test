//TODO
//posebni moduli za models
//mooozda posebni moduli (fajlovi) za svaki route scope ( 'users', 'articles', 'comments', 'likes' )
pub mod users {
    use crate::AppState;
    use actix_web::{get, post, web, web::Path, HttpResponse, Responder};
    use sqlx::FromRow;

    #[derive(FromRow, serde::Serialize, serde::Deserialize)]
    pub struct User {
        pub id: i32,
        pub email: String,
        pub password: String,
    }

    #[derive(FromRow, serde::Serialize, serde::Deserialize)]
    pub struct NewUser {
        pub email: String,
        pub password: String,
    }

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
    // da se samo pozivaju metode AppState-a
    #[get("/users")]
    pub async fn get_users(app_data: web::Data<AppState>) -> impl Responder {
        match sqlx::query_as!(User, "select * from users")
            .fetch_all(&app_data.db_pool)
            .await
        {
            Ok(vec) => HttpResponse::Ok().json(vec),
            Err(_) => HttpResponse::InternalServerError().json("err"),
        }
    }
    #[get("/users/{user_id}")]
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
    #[post("/users")]
    pub async fn add_user(
        body: web::Json<NewUser>,
        app_state: web::Data<AppState>,
    ) -> impl Responder {
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
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(add_user).service(get_users).service(get_user);
    }
}
