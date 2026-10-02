mod articles;
mod auth;
mod config;
mod db;
mod errors;
mod state;
mod users;
use std::sync::Arc;

use crate::config::Config;
use axum::{
    Router,
    routing::{get, patch, post},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from_env());
    let database = db::connect(&config).await;
    let state = state::AppState::new(config.clone(), database);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/users", post(users::handlers::register))
        .route("/api/users/login", post(users::handlers::login))
        .route("/api/users/me", get(users::handlers::me))
        .route(
            "/api/articles",
            post(articles::handlers::create).get(articles::handlers::list),
        )
        .route(
            "/api/articles/{id}",
            get(articles::handlers::get_one)
                .patch(articles::handlers::update)
                .delete(articles::handlers::remove),
        )
        .with_state(state);

    let listener = TcpListener::bind((config.db_host(), config.db_port()))
        .await
        .unwrap();
    println!("Listening on {}:{}", config.db_host(), config.db_port());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "API ok"
}
