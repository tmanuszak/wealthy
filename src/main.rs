use axum::Router;

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

    // Setup Config
    let config = config::get_config();

    // Initialize the db
    let db = db::init_db(&config.db_url).await?;

    // Build the app
    let app = Router::new()
        .merge(routes::create_routes())
        .layer(axum::extract::Extension(db));

    let listener = tokio::net::TcpListener::bind(&config.app_url)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
