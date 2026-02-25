use anyhow::Result;
use log::info;

mod config;
mod weather;

use config::Config;
use weather::noaa::NOAAClient;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Polymarket Weather Bot iniciando...");

    let _config = Config::from_env()?;
    let noaa_client = NOAAClient::new();

    let forecast = noaa_client.get_forecast("40.7128", "-74.0060").await?;
    info!("Previsão obtida: {}°", forecast.temperature);
    info!("Chance de chuva: {}%", forecast.precipitation_chance);

    Ok(())
}
