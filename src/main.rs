mod models;
mod routes;

use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_config = read_configuration();
    println!("Successfully read the .env config");
    let db = db_connect(db_config).await;
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Should be able to perform migrations");
    println!("Successfully connected to the database");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::<AppState>::new(AppState {
                db_pool: db.clone(),
            }))
            .configure(routes::users::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

struct DbConfig {
    connection_string: String,
    max_connections: u32,
}

fn read_configuration() -> DbConfig {
    dotenv().expect("Should be able to read environment variables");
    let conn_str = env::var("DATABASE_URL").expect("Should be able to read connection string");
    // TODO nadji lepsi nacin za parsiranje
    let max_conn: u32 = match env::var("MAX_CONNECTIONS") {
        Ok(val) => val.parse().unwrap_or(100),
        Err(_) => 100,
    };

    DbConfig {
        connection_string: conn_str,
        max_connections: max_conn,
    }
}

async fn db_connect(cfg: DbConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .connect(&cfg.connection_string)
        .await
        .expect("Should be able to connect to db")
}
