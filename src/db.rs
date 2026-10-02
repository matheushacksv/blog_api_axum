use crate::config::Config;
use sqlx::PgPool;

pub async fn connect(config: &Config) -> PgPool {
    // connect
    let pool = PgPool::connect(config.db_url())
        .await
        .expect("failed to connect to database");

    // migration
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to migrate to database");

    pool
}
