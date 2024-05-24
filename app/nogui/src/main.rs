#[macro_use]
extern crate log;

use logger::set_up_logging;
use rqs_lib::RQS;

mod logger;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    set_up_logging()?;

    // Start the RQuickShare service
    let mut rqs = RQS::default();
    rqs.run().await?;

    // Still need to add everything here
    todo!();

    info!("Stopping service.");
    rqs.stop().await;

    Ok(())
}
