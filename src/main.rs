use std::{collections::HashMap, sync::Arc};

use axum::http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::{Mutex, broadcast}};

mod route;
mod mail;
mod handlers;
mod db;
mod dtos;
mod config;
mod models;
mod state;
mod utils;
mod errors;

use config::Config;
use tower_http::cors::{Any, CorsLayer};

use crate::{route::create_router, state::{AppState, DbClient}};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST])
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    .allow_credentials(true);

    dotenv::dotenv().ok();
    //Creating a simple server in rust using axum
    // let (group_tx, _rx) = broadcast::channel(100);
    let tx = Mutex::new(HashMap::new());
    let config = Config::init_config();
    //Establish a db connection instance
    let pool = PgPoolOptions::new().max_connections(10).connect(&config.database_url).await.unwrap();
    //create and declare a router
    let db_client = DbClient{pool};
    let app_state = AppState{
        db_client,
        envs:config,
        tx
    };

    let port = app_state.envs.port.clone();

    let  app = create_router(Arc::new(app_state)).layer(cors.clone());

    // establish a listener
    let listener = TcpListener::bind(format!("localhost:{}", port)).await.expect("listener error");
    println!("Server is running on port {}", port);
    // intergrate the router and listener to finally build a server
    axum::serve(listener, app).await.expect("server error");

}

pub async fn return_server_response() -> String {
    "Server Established!!".to_string()
}
