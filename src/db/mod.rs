use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init_db(db_url: &str) -> color_eyre::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create db pool: {}", e))
}
