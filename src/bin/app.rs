use futures::FutureExt;
use tracing::info;
use bitcoin_explorer::{configure, util};
use bitcoin_explorer::constant::CONFIG;
use bitcoin_explorer::error::AppResult;
use bitcoin_explorer::server::AppServer;
use bitcoin_explorer::server::bitcoin_indexer::BitcoinIndexer;
use bitcoin_explorer::server::worker::MessengerTask;

#[tokio::main]
async fn main() -> AppResult<()> {
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");
    let config = CONFIG.clone();
    info!("Reading the config file was successful.");
    info!("Create a new server.");
    let server = AppServer::new(config).await?;
    info!("Create a new messenger task.");
    let messenger = MessengerTask::new(server.state.clone());
    info!("Run the server.");
    let indexer_result = BitcoinIndexer::new(server.state.clone());

    match indexer_result {
        Ok(indexer) => {
            let tasks = vec![
                (true, server.run().boxed()),
                (true, indexer.run().boxed()),
                (true, messenger.run().boxed()),
            ];
            util::task::join_all(tasks).await?;
        }
        Err(err) => {
            eprintln!("Failed to create BitcoinIndexer: {}", err);
        }
    }

    Ok(())
}

