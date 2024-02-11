#[macro_use]
extern crate log;

#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE")
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

	info!("Starting service");

    Ok(())
}