use crate::config::Config;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
}

impl AppState {
    pub fn new(config: Arc<Config>, db: PgPool) -> Self {
        AppState { config, db }
    }
}