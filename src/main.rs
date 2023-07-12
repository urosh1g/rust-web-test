use dotenv::dotenv;
use std::env;

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

fn main() {
    let db_config = read_configuration();
    println!("Successfully read the .env config");
}
