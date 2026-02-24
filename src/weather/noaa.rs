use anyhow::{Context, Result};
use chrono::Utc;
use log::{info, warn};
use reqwest::Client;

use super::types::{WeatherForecast, NOAAResponse};

pub struct NOAAClient {
    client: Client,
    base_url: String,
}

impl NOAAClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.weather.gov".to_string(),
        }
    }

    pub async fn get_forecast(&self, lat: &str, lon: &str) -> Result<WeatherForecast> {
        info!("🌍 Obtendo previsão para {}, {}", lat, lon);

        // Passo 1: Obter office e grid
        let point_url = format!("{}/points/{},{}", self.base_url, lat, lon);
        let point_response: serde_json::Value = self.client
            .get(&point_url)
            .header("User-Agent", "PolymarketWeatherBot/1.0")
            .send()
            .await?
            .json()
            .await?;

        let forecast_url = point_response["properties"]["forecast"]
            .as_str()
            .context("URL de forecast não encontrada")?;

        info!("📡 URL do forecast: {}", forecast_url);

        // Passo 2: Obter forecast
        let forecast_response: NOAAResponse = self.client
            .get(forecast_url)
            .header("User-Agent", "PolymarketWeatherBot/1.0")
            .send()
            .await?
            .json()
            .await?;

        // Passo 3: Processar dados
        let first_period = &forecast_response.properties.periods[0];
        
        let precipitation_chance = self.extract_precipitation_chance(&first_period.detailed_forecast);

        Ok(WeatherForecast {
            location: format!("{}, {}", lat, lon),
            temperature: first_period.temperature as f64,
            precipitation_chance,
            condition: first_period.short_forecast.clone(),
            timestamp: Utc::now(),
            forecast_date: Utc::now(), // Simplificado por agora
        })
    }

    fn extract_precipitation_chance(&self, detailed_forecast: &str) -> f64 {
        // Regex simples para extrair chance de chuva
        if let Some(captures) = regex::Regex::new(r"(\d+)%")
            .ok()
            .and_then(|re| re.captures(detailed_forecast))
        {
            if let Some(percent_str) = captures.get(1) {
                return percent_str.as_str().parse().unwrap_or(0.0);
            }
        }

        // Se não encontrar, analisar palavras-chave
        let lower_forecast = detailed_forecast.to_lowercase();
        if lower_forecast.contains("rain") || lower_forecast.contains("showers") {
            return 70.0; // Estimativa conservadora
        }
        if lower_forecast.contains("partly cloudy") {
            return 20.0;
        }

        0.0
    }
}
