use axum::Router;
use tracing::info;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // Setup error reporting
    color_eyre::install()?;

    // Setup tracing
    tracing_subscriber::fmt::init();

    // Setup Config
    let config = config::get_config().await?;

    // Initialize the db
    let db = db::init_db(&config.database).await?;

    // Build the app
    let app = Router::new()
        .merge(routes::create_routes())
        .layer(axum::extract::Extension(db));

    let listener = tokio::net::TcpListener::bind((config.server.ip.clone(), config.server.port))
        .await
        .unwrap();

    info!(
        "Serving the app at {}:{}",
        config.server.ip, config.server.port
    );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
