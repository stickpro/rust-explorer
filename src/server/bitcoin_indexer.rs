use bitcoincore_rpc::RpcApi;
use tracing::info;
use crate::error::AppResult;
use crate::server::state::AppState;


pub struct BitcoinIndexer {
    pub state: AppState,
}

impl BitcoinIndexer {
    pub fn new(state: AppState) -> AppResult<Self> {
        Ok(Self { state })
    }
    pub async fn run(self) -> AppResult<()> {
        info!("Start parse block");
        let best_block_hash = self.state.bitcoin.();
        println!("best block hash: {:?}", best_block_hash);
        Ok(())
    }
}