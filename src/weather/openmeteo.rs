use anyhow::Result;
use chrono::Utc;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenMeteoData {
    pub airport_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub temperature_celsius: f64,
    pub humidity_percent: f64,
    pub wind_speed_kmh: f64,
    pub wind_direction: u32,
    pub weather_code: u32,
    pub weather_description: String,
    pub timestamp_utc: String,
}

pub struct OpenMeteoClient {
    client: Client,
    base_url: String,
}

// Coordenadas dos aeroportos para Open-Meteo
const AIRPORT_COORDINATES: &[(&str, f64, f64)] = &[
    ("KLGA", 40.7769, -73.8740),   // LaGuardia, Nova York
    ("NZWN", -41.3272, 174.8860),  // Wellington
    ("SAEZ", -34.8222, -58.5358),  // Buenos Aires
    ("KATL", 33.6407, -84.4277),   // Atlanta
    ("EGLL", 51.4700, -0.4543),    // Heathrow, Londres
    ("SBGR", -23.4356, -46.4731),  // Guarulhos, São Paulo
];

impl OpenMeteoClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.open-meteo.com/v1/forecast".to_string(),
        }
    }

    pub async fn get_weather(&self, airport_code: &str) -> Result<OpenMeteoData> {
        info!("Obtendo dados do Open-Meteo para {}", airport_code);

        // Encontrar coordenadas do aeroporto
        let (lat, lon) = AIRPORT_COORDINATES
            .iter()
            .find(|(code, _, _)| code == &airport_code)
            .map(|(_, lat, lon)| (*lat, *lon))
            .ok_or_else(|| anyhow::anyhow!("Aeroporto não encontrado: {}", airport_code))?;

        let url = format!(
            "{}?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,weather_code,wind_speed_10m,wind_direction_10m&timezone=UTC",
            self.base_url, lat, lon
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Erro na resposta da Open-Meteo: {}", response.status());
        }

        let data: serde_json::Value = response.json().await?;
        self.parse_response(airport_code, lat, lon, data)
    }

    fn parse_response(
        &self,
        airport_code: &str,
        lat: f64,
        lon: f64,
        data: serde_json::Value,
    ) -> Result<OpenMeteoData> {
        let current = data["current"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("Resposta inválida do Open-Meteo"))?;

        let temperature = current["temperature_2m"]
            .as_f64()
            .unwrap_or(0.0);
        let humidity = current["relative_humidity_2m"]
            .as_i64()
            .unwrap_or(0) as f64;
        let wind_speed = current["wind_speed_10m"]
            .as_f64()
            .unwrap_or(0.0);
        let wind_direction = current["wind_direction_10m"]
            .as_i64()
            .unwrap_or(0) as u32;
        let weather_code = current["weather_code"]
            .as_i64()
            .unwrap_or(0) as u32;

        let weather_description = self.get_weather_description(weather_code);

        // Extrair timestamp oficial da API Open-Meteo (formato ISO 8601)
        let timestamp_utc = current["time"]
            .as_str()
            .unwrap_or(&Utc::now().to_rfc3339())
            .to_string();

        Ok(OpenMeteoData {
            airport_code: airport_code.to_string(),
            latitude: lat,
            longitude: lon,
            temperature_celsius: temperature,
            humidity_percent: humidity,
            wind_speed_kmh: wind_speed,
            wind_direction,
            weather_code,
            weather_description,
            timestamp_utc,
        })
    }

    fn get_weather_description(&self, code: u32) -> String {
        match code {
            0 => "Céu Claro",
            1 | 2 => "Parcialmente Nublado",
            3 => "Nublado",
            45 | 48 => "Névoa",
            51 | 53 | 55 => "Chuva Leve",
            61 | 63 | 65 => "Chuva",
            71 | 73 | 75 => "Neve",
            77 => "Granizo",
            80 | 81 | 82 => "Chuva Forte",
            85 | 86 => "Neve com Chuva",
            95 | 96 | 99 => "Tempestade",
            _ => "Tempo Desconhecido",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_description() {
        let client = OpenMeteoClient::new();
        assert_eq!(client.get_weather_description(0), "Céu Claro");
        assert_eq!(client.get_weather_description(61), "Chuva");
        assert_eq!(client.get_weather_description(95), "Tempestade");
    }
}
