use sqlx::PgPool;
use std::sync::Arc;
use tap::Pipe;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<crate::config::Config>,
    pub db_pool: Arc<PgPool>,
}

pub async fn init_state() -> color_eyre::Result<AppState> {
    let config = crate::config::get_config().await?;
    let db_pool = crate::db::init_db(&config.database.url).await?;
    AppState {
        config: Arc::new(config),
        db_pool: Arc::new(db_pool),
    }
    .pipe(Ok)
}
