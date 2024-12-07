mod app;
use app::run;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?; // Error reporting

    run().await
}
