use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BitcoinConfig {
    pub host: String,
    pub username: String,
    pub password: String
}

impl BitcoinConfig {
    pub fn get_host(&self) -> String {
        format!("{}", self.host)
    }
}