use anyhow::Result;
use log::info;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

mod config;
mod weather;
mod telegram;

use config::Config;
use weather::aviation::AviationWeatherClient;
use weather::openmeteo::OpenMeteoClient;
use telegram::TelegramClient;

// Aeroportos a monitorar
const AIRPORTS: &[&str] = &["KLGA", "NZWN", "SAEZ", "KATL", "EGLL", "SBGR"];

// Estrutura para rastrear último timestamp de cada aeroporto
#[derive(Clone, Debug)]
struct LastReport {
    metar_timestamp: String,
    openmeteo_timestamp: String,
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Polymarket Weather Bot iniciando...");

    let config = Config::from_env()?;
    let aviation_client = AviationWeatherClient::new();
    let openmeteo_client = OpenMeteoClient::new();
    let telegram_client = TelegramClient::new(
        config.telegram_bot_token.clone(),
        config.telegram_chat_id.clone(),
    );

    info!("Bot configurado para monitorar: {:?}", AIRPORTS);
    info!("Enviando para o chat Telegram: {}", config.telegram_chat_id);
    info!("Verificação contínua iniciada...");

    // Rastrear últimos relatórios para detectar mudanças
    let mut last_reports: HashMap<String, LastReport> = HashMap::new();

    // Loop infinito com verificação contínua a cada 20 segundos
    loop {
        for airport in AIRPORTS {
            // Verificar Aviation Weather METAR
            match aviation_client.get_metar(airport).await {
                Ok(metar_data) => {
                    // Usar timestamp oficial do METAR como identificador único
                    let metar_timestamp = metar_data.timestamp_utc.clone();

                    let key = format!("{}_metar", airport);
                    let should_send = last_reports
                        .get(&key)
                        .map(|r| r.metar_timestamp != metar_timestamp)
                        .unwrap_or(true);

                    if should_send {
                        info!("🆕 Novo METAR detectado para {} (timestamp: {})", airport, metar_timestamp);
                        if let Err(e) = telegram_client.send_metar_report(&metar_data).await {
                            log::error!("Erro ao enviar METAR para {}: {}", airport, e);
                        }

                        // Atualizar timestamp
                        let entry = last_reports.entry(key).or_insert_with(|| LastReport {
                            metar_timestamp: String::new(),
                            openmeteo_timestamp: String::new(),
                        });
                        entry.metar_timestamp = metar_timestamp;
                    }
                }
                Err(e) => {
                    log::error!("Erro ao obter METAR para {}: {}", airport, e);
                }
            }

            // Pequeno delay entre requisições
            sleep(Duration::from_millis(500)).await;

            // Verificar Open-Meteo
            match openmeteo_client.get_weather(airport).await {
                Ok(weather_data) => {
                    // Usar timestamp oficial da API Open-Meteo como identificador
                    let openmeteo_timestamp = weather_data.timestamp_utc.clone();

                    let key = format!("{}_openmeteo", airport);
                    let should_send = last_reports
                        .get(&key)
                        .map(|r| r.openmeteo_timestamp != openmeteo_timestamp)
                        .unwrap_or(true);

                    if should_send {
                        info!("🆕 Novo relatório Open-Meteo detectado para {} (timestamp: {})", airport, openmeteo_timestamp);
                        if let Err(e) = telegram_client.send_openmeteo_report(&weather_data).await {
                            log::error!("Erro ao enviar Open-Meteo para {}: {}", airport, e);
                        }

                        // Atualizar timestamp
                        let entry = last_reports.entry(key).or_insert_with(|| LastReport {
                            metar_timestamp: String::new(),
                            openmeteo_timestamp: String::new(),
                        });
                        entry.openmeteo_timestamp = openmeteo_timestamp;
                    }
                }
                Err(e) => {
                    log::error!("Erro ao obter Open-Meteo para {}: {}", airport, e);
                }
            }

            // Pequeno delay entre requisições
            sleep(Duration::from_millis(500)).await;
        }

        // Delay responsivo entre verificações (20 segundos)
        info!("Próxima verificação em 20 segundos...");
        sleep(Duration::from_secs(20)).await;
    }
}
