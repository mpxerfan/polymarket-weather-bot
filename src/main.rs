use anyhow::Result;
use log::info;

mod weather;
mod config;

use weather::noaa::NOAAClient;
use config::Config;

##[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("🌤️ Polymarket Weather Bot iniciando...");

    let config = Config::from_env()?;
    let noaa_client = NOAAClient::new();

    // Teste básico do NOAA
    let forecast = noaa_client
        .get_forecast("40.7128", "-74.0060") // NYC
        .await?;

    info!("📊 Previsão obtida: {}", forecast.temperature);
    info!("🎯 Chance de chuva: {}%", forecast.precipitation_chance);

    Ok(())
}
