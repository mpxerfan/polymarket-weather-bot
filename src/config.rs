use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub noaa_api_key: Option<String>,
    pub polymarket_api_key: String,
    pub private_key: String,
    pub rpc_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            noaa_api_key: env::var("NOAA_API_KEY").ok(),
            polymarket_api_key: env::var("POLYMARKET_API_KEY")
                .context("POLYMARKET_API_KEY não encontrada")?,
            private_key: env::var("PRIVATE_KEY").context("PRIVATE_KEY não encontrada")?,
            rpc_url: env::var("RPC_URL").unwrap_or_else(|_| "https://polygon-rpc.com".to_string()),
        })
    }
}
