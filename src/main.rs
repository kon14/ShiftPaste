mod common;
mod db;
mod domain;
mod http;
mod prelude;

use dotenv::dotenv;
use env_logger;
use sqlx::postgres::PgPoolOptions;

use crate::common::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));
    let db_url = common::utils::get_database_url();
    let api_port = common::utils::get_api_port();
    let _ = common::utils::get_api_base_url();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database!");

    let state = AppState { db };
    let app = http::build_router().with_state(state);

    let server_addr = format!("0.0.0.0:{api_port}");
    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    println!("Server listening on: http://{server_addr}");
    axum::serve(listener, app).await.unwrap();
}
