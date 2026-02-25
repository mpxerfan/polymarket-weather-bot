use anyhow::{Context, Result};
use chrono::Utc;
use log::info;
use reqwest::Client;

use super::types::{NOAAResponse, WeatherForecast};

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
        info!("Obtendo previsão para {}, {}", lat, lon);

        let point_url = format!("{}/points/{},{}", self.base_url, lat, lon);
        let point_response: serde_json::Value = self
            .client
            .get(&point_url)
            .header("User-Agent", "PolymarketWeatherBot/1.0")
            .send()
            .await?
            .json()
            .await?;

        let forecast_url = point_response["properties"]["forecast"]
            .as_str()
            .context("URL de forecast não encontrada em /points")?;

        let forecast_response: NOAAResponse = self
            .client
            .get(forecast_url)
            .header("User-Agent", "PolymarketWeatherBot/1.0")
            .send()
            .await?
            .json()
            .await?;

        let first_period = forecast_response
            .properties
            .periods
            .get(0)
            .context("Sem períodos no forecast")?;

        let precipitation_chance =
            self.extract_precipitation_chance(&first_period.detailed_forecast);

        Ok(WeatherForecast {
            location: format!("{}, {}", lat, lon),
            temperature: first_period.temperature as f64,
            precipitation_chance,
            condition: first_period.short_forecast.clone(),
            timestamp: Utc::now(),
            forecast_date: Utc::now(),
        })
    }

    fn extract_precipitation_chance(&self, detailed_forecast: &str) -> f64 {
        if let Ok(re) = regex::Regex::new(r"(\d+)%") {
            if let Some(caps) = re.captures(detailed_forecast) {
                if let Some(m) = caps.get(1) {
                    return m.as_str().parse().unwrap_or(0.0);
                }
            }
        }

        let s = detailed_forecast.to_lowercase();
        if s.contains("rain") || s.contains("showers") {
            return 70.0;
        }
        if s.contains("partly cloudy") {
            return 20.0;
        }
        0.0
    }
}
