use std::future::Future;
use bitcoincore_rpc::{Auth, Client};
use log::info;
use crate::configure::bitcoin::BitcoinConfig;
use crate::error::AppResult;

pub type BitcoinClient = Client;

pub trait BitcoinClientExt: Sized {
    fn build_from_config(config: &BitcoinConfig) -> impl Future<Output=AppResult<Self>>;
}

impl BitcoinClientExt for BitcoinClient {
    async fn build_from_config(config: &BitcoinConfig) -> AppResult<Self> {
        info!("get host {}", &config.get_host());
        let rpc = Client::new(&config.get_host(), Auth::UserPass(config.username.to_string(), config.password.to_string())).unwrap();
        Ok(rpc)
    }
}
