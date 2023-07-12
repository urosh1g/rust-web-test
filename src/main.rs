use actix_web;
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
    let _ = db_connect(db_config).await;
    println!("Successfully connected to the database");
    Ok(())
}

struct DbConfig {
    connection_string: String,
    max_connections: u32,
}

fn read_configuration() -> DbConfig {
    dotenv().expect("Should be able to read environment variables");
    let conn_str = env::var("DATABASE_URL").expect("Should be able to read connection string");
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
