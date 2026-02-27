use anyhow::Result;
use chrono::{DateTime, Timelike, Utc};
use chrono_tz::Tz;
use log::info;
use reqwest::Client;
use serde_json::json;

use crate::weather::aviation::METARData;
use crate::weather::openmeteo::OpenMeteoData;

pub struct TelegramClient {
    client: Client,
    bot_token: String,
    chat_id: String,
}

/// Converte timestamp Zulu (UTC) para Pacific/Auckland
/// Para METAR: formato "DDHHMMZ", ex: "262330Z"
/// Para Open-Meteo: formato ISO 8601, ex: "2024-02-26T23:30"
pub fn convert_to_auckland_time(timestamp_utc: &str) -> String {
    // Para METAR: formato DDHHMMZ, ex: "262330Z"
    if timestamp_utc.ends_with('Z') && timestamp_utc.len() == 7 {
        let day_str = &timestamp_utc[0..2];
        let hour_str = &timestamp_utc[2..4];
        let min_str = &timestamp_utc[4..6];

        if let (Ok(_day), Ok(hour), Ok(min)) = (
            day_str.parse::<u32>(),
            hour_str.parse::<u32>(),
            min_str.parse::<u32>(),
        ) {
            // Criar um DateTime UTC com o horário extraído
            let now: DateTime<Utc> = Utc::now();
            if let Some(utc_time) = now.with_hour(hour).and_then(|t| t.with_minute(min)) {
                if let Ok(auckland_tz) = "Pacific/Auckland".parse::<Tz>() {
                    let auckland_time = utc_time.with_timezone(&auckland_tz);
                    return format!("{} NZDT", auckland_time.format("%d/%m %H:%M"));
                }
            }
        }
    }

    // Para Open-Meteo: formato ISO 8601, ex: "2024-02-26T23:30"
    if let Ok(utc_time) = timestamp_utc.parse::<DateTime<Utc>>() {
        if let Ok(auckland_tz) = "Pacific/Auckland".parse::<Tz>() {
            let auckland_time = utc_time.with_timezone(&auckland_tz);
            return format!("{} NZDT", auckland_time.format("%d/%m %H:%M"));
        }
    }

    "Horário indisponível".to_string()
}

impl TelegramClient {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            client: Client::new(),
            bot_token,
            chat_id,
        }
    }

    pub async fn send_metar_report(&self, metar_data: &METARData) -> Result<()> {
        let message = self.format_metar_message(metar_data);
        self.send_message(&message).await
    }

    pub async fn send_openmeteo_report(&self, weather_data: &OpenMeteoData) -> Result<()> {
        let message = self.format_openmeteo_message(weather_data);
        self.send_message(&message).await
    }

    fn format_metar_message(&self, metar_data: &METARData) -> String {
        let airport = &metar_data.airport_code;
        let temp = metar_data.temperature_celsius;
        let humidity = metar_data
            .humidity_percent
            .map(|h| format!("{:.0}%", h))
            .unwrap_or_else(|| "N/A".to_string());
        let wind = format!("{:.1} km/h", metar_data.wind_speed_kmh);
        let category = &metar_data.flight_category;

        // Converter timestamp Zulu para Auckland
        let auckland_time = convert_to_auckland_time(&metar_data.timestamp_utc);

        format!(
            "📡 *AVIATION WEATHER METAR*\n\
            ✈️ *Aeroporto: {}*\n\n\
            🌡️ Temperatura: *{:.0}°C*\n\
            💧 Umidade: *{}*\n\
            💨 Vento: *{}*\n\
            📊 Categoria: *{}*\n\
            🕐 Horário Auckland: *{}*\n\
            ⏰ Zulu (UTC): *{}*\n\n\
            📝 Raw: `{}`",
            airport, temp, humidity, wind, category, auckland_time, 
            metar_data.timestamp_utc, metar_data.raw_metar
        )
    }

    fn format_openmeteo_message(&self, weather_data: &OpenMeteoData) -> String {
        let airport = &weather_data.airport_code;
        let temp = weather_data.temperature_celsius;
        let humidity = weather_data.humidity_percent;
        let wind = weather_data.wind_speed_kmh;
        let direction = weather_data.wind_direction;
        let description = &weather_data.weather_description;

        // Converter timestamp ISO 8601 para Auckland
        let auckland_time = convert_to_auckland_time(&weather_data.timestamp_utc);

        format!(
            "🌐 *OPEN-METEO FORECAST*\n\
            ✈️ *Aeroporto: {}*\n\n\
            🌡️ Temperatura: *{:.1}°C*\n\
            💧 Umidade: *{:.0}%*\n\
            💨 Vento: *{:.1} km/h* (Dir: {}°)\n\
            🌤️ Condição: *{}*\n\
            🕐 Horário Auckland: *{}*",
            airport, temp, humidity, wind, direction, description, auckland_time
        )
    }

    pub async fn send_message(&self, message: &str) -> Result<()> {
        info!("Enviando mensagem para Telegram (Chat ID: {})", self.chat_id);

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let payload = json!({
            "chat_id": self.chat_id,
            "text": message,
            "parse_mode": "Markdown"
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Mensagem enviada com sucesso!");
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Falha ao enviar mensagem: {}", error_text)
        }
    }
}
