use anyhow::Result;
use chrono::Utc;
use log::info;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct METARData {
    pub airport_code: String,
    pub temperature_celsius: f64,
    pub temperature_fahrenheit: f64,
    pub humidity_percent: Option<f64>,
    pub wind_speed_kmh: f64,
    pub wind_direction: Option<u32>,
    pub altimeter_mb: f64,
    pub raw_metar: String,
    pub timestamp_utc: String,
    pub flight_category: String,
}

pub struct AviationWeatherClient {
    client: Client,
    base_url: String,
}

impl AviationWeatherClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://aviationweather.gov/api/data/metar".to_string(),
        }
    }

    pub async fn get_metar(&self, airport_code: &str) -> Result<METARData> {
        info!("Obtendo METAR para {}", airport_code);

        let url = format!("{}?ids={}&format=raw", self.base_url, airport_code);

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "PolymarketWeatherBot/1.0")
            .send()
            .await?
            .text()
            .await?;

        self.parse_metar(airport_code, &response)
    }

    fn parse_metar(&self, airport_code: &str, raw_metar: &str) -> Result<METARData> {
        let metar = raw_metar.trim();
        let parts: Vec<&str> = metar.split_whitespace().collect();

        let mut temp_celsius = 0.0;
        let mut humidity_percent: Option<f64> = None;
        let mut wind_speed_kmh = 0.0;
        let mut wind_direction: Option<u32> = None;
        let mut altimeter_mb = 1013.25;
        let mut flight_category = "UNKNOWN".to_string();

        for part in &parts {
            // Temperatura e ponto de orvalho: 15/10
            if part.contains('/') && part.len() == 5 {
                let temp_parts: Vec<&str> = part.split('/').collect();
                if let Ok(temp) = temp_parts[0].parse::<f64>() {
                    temp_celsius = temp;
                }
                // Calcular umidade aproximada a partir de temperatura e ponto de orvalho
                if let (Ok(t), Ok(td)) = (
                    temp_parts[0].parse::<f64>(),
                    temp_parts[1].parse::<f64>(),
                ) {
                    // Fórmula aproximada de Magnus para umidade relativa
                    let a = 17.27;
                    let b = 237.7;
                    let alpha = ((a * td) / (b + td) + (a * t) / (b + t)).ln() / 2.0;
                    let rh = 100.0 * (alpha.exp() / (1.0 + alpha.exp().powi(2)));
                    humidity_percent = Some(rh.max(0.0).min(100.0));
                }
            }

            // Vento: 27015KT ou 27015G25KT
            if part.ends_with("KT") {
                let wind_str = part.trim_end_matches("KT");
                if wind_str.contains('G') {
                    let wind_parts: Vec<&str> = wind_str.split('G').collect();
                    if let Ok(dir) = wind_parts[0].parse::<u32>() {
                        wind_direction = Some(dir);
                    }
                    if let Ok(spd) = wind_parts[1].parse::<f64>() {
                        wind_speed_kmh = spd * 1.852; // Converter knots para km/h
                    }
                } else {
                    let (dir_str, spd_str) = wind_str.split_at(3);
                    if let Ok(dir) = dir_str.parse::<u32>() {
                        wind_direction = Some(dir);
                    }
                    if let Ok(spd) = spd_str.parse::<f64>() {
                        wind_speed_kmh = spd * 1.852;
                    }
                }
            }

            // Altímetro: A3012
            if part.starts_with('A') && part.len() == 5 {
                if let Ok(alt) = part[1..].parse::<f64>() {
                    altimeter_mb = alt / 100.0;
                }
            }

            // Categoria de voo
            if part.contains("CAVOK") {
                flight_category = "CAVOK".to_string();
            } else if *part == "VFR" {
                flight_category = "VFR".to_string();
            } else if *part == "MVFR" {
                flight_category = "MVFR".to_string();
            } else if *part == "IFR" {
                flight_category = "IFR".to_string();
            } else if *part == "LIFR" {
                flight_category = "LIFR".to_string();
            }
        }

        // Extrair timestamp oficial do METAR usando regex
        // Padrão DDHHMMZ pode estar em qualquer posição, geralmente logo após o código do aeroporto
        let timestamp_regex = Regex::new(r"\d{6}Z").expect("Regex válido");
        let timestamp_utc = if let Some(caps) = timestamp_regex.find(raw_metar) {
            caps.as_str().to_string()
        } else {
            // Fallback: se não encontrar o padrão, usar um formato único que nunca vai ser duplicado
            // para evitar problemas imediatamente
            format!("UNKNOWN_{}", Utc::now().timestamp_millis())
        };

        // Converter Celsius para Fahrenheit: F = (C * 9/5) + 32
        let temp_fahrenheit = (temp_celsius * 9.0 / 5.0) + 32.0;

        Ok(METARData {
            airport_code: airport_code.to_string(),
            temperature_celsius: temp_celsius,
            temperature_fahrenheit: temp_fahrenheit,
            humidity_percent,
            wind_speed_kmh,
            wind_direction,
            altimeter_mb,
            raw_metar: metar.to_string(),
            timestamp_utc,
            flight_category,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metar() {
        let client = AviationWeatherClient::new();
        let raw = "NZWN 151856Z 27015G25KT 9999 15/10 Q1012 LFEW";
        let result = client.parse_metar("NZWN", raw).unwrap();
        
        assert_eq!(result.airport_code, "NZWN");
        assert_eq!(result.temperature_celsius, 15.0);
        assert!(result.humidity_percent.is_some());
        assert!(result.wind_speed_kmh > 0.0);
    }
}
