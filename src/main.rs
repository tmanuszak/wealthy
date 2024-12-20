use tower_http::trace::TraceLayer;
use tracing::info;

mod config;
mod db;
mod models;
mod routes;
mod services;
mod state;
mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    // Setup error reporting
    color_eyre::install()?;

    // Load env vars
    dotenvy::dotenv()?;

    // Setup tracing
    tracing_subscriber::fmt::init();

    let app_state = state::init_state().await?;

    // Build the app
    let app = routes::create_routes()
        .with_state(app_state.clone())
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind((
        app_state.config.server.ip.as_str(),
        app_state.config.server.port,
    ))
    .await?;

    info!(
        "Serving the app at {}:{}",
        app_state.config.server.ip, app_state.config.server.port
    );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
