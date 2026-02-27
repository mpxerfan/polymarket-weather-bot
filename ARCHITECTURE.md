# 🏗️ Arquitetura do Bot

```
┌─────────────────────────────────────────────────────────────┐
│                  POLYMARKET WEATHER BOT                     │
└─────────────────────────────────────────────────────────────┘

                            ┌──────────────┐
                            │   main.rs    │
                            │  (Executor)  │
                            └───────┬──────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
                    ▼               ▼               ▼
            ┌────────────────┐ ┌──────────────┐ ┌──────────────┐
            │   config.rs    │ │ weather/     │ │  telegram.rs │
            │ (Variáveis de  │ │ aviation.rs  │ │   (Cliente   │
            │  Ambiente)     │ │ (Aviation    │ │   Telegram)  │
            └────────────────┘ │  Weather)    │ └──────────────┘
                    │          └──────────────┘        │
                    │                │                 │
                    └────────┬───────┴──────┬──────────┘
                             │              │
                             ▼              ▼
                    ┌─────────────────────────────┐
                    │   Loop de Monitoramento     │
                    │   (5 minutos)               │
                    └─────────────────────────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
    [KLGA]  [NZWN]  [SAEZ]  [KATL]  [EGLL]
    (NY)    (NZ)    (AR)    (ATL)   (UK)
        │                    │                    │
        └────────────────────┼────────────────────┘
                             │
                    ┌────────▼─────────┐
                    │  Aviation        │
                    │  Weather API     │
                    │  (METAR)         │
                    └────────┬─────────┘
                             │
                    ┌────────▼──────────┐
                    │  Parse METAR      │
                    │  - Temperatura    │
                    │  - Umidade        │
                    │  - Vento          │
                    │  - Visibilidade   │
                    └────────┬──────────┘
                             │
                    ┌────────▼──────────┐
                    │  Formatar         │
                    │  Mensagem         │
                    │  Telegram         │
                    └────────┬──────────┘
                             │
                    ┌────────▼──────────┐
                    │  Enviar para      │
                    │  Bot Telegram     │
                    │  (Grupo)          │
                    └────────┬──────────┘
                             │
                    ┌────────▼──────────────────┐
                    │  Seu Grupo Telegram       │
                    │                           │
                    │  ✈️ METAR - KLGA          │
                    │  🌡️ Temperatura: 15°C    │
                    │  💧 Umidade: 65%         │
                    │  💨 Vento: 27.8 km/h     │
                    │  📊 Categoria: VFR       │
                    │  🕐 Horário: 18:56 UTC   │
                    └───────────────────────────┘
```

## 📦 Estrutura de Módulos

```rust
polymarket_weather_bot
│
├── main.rs
│   ├── Config (config.rs)
│   ├── AviationWeatherClient (weather/aviation.rs)
│   ├── TelegramClient (telegram.rs)
│   └── Loop de 5 minutos
│
├── config.rs
│   └── Config { telegram_bot_token, telegram_chat_id, ... }
│
├── telegram.rs
│   ├── TelegramClient::new()
│   ├── send_metar_report(&METARData)
│   └── format_metar_message(&METARData) -> String
│
└── weather/
    ├── mod.rs
    │   ├── pub mod aviation
    │   └── pub mod types
    │
    ├── aviation.rs
    │   ├── AviationWeatherClient
    │   │   ├── get_metar(airport_code)
    │   │   ├── parse_metar(raw_metar) -> METARData
    │   │   └── extract_humidity(temp, dewpoint)
    │   │
    │   └── METARData {
    │       ├── airport_code: String
    │       ├── temperature_celsius: f64
    │       ├── humidity_percent: Option<f64>
    │       ├── wind_speed_kmh: f64
    │       ├── wind_direction: Option<u32>
    │       ├── altimeter_mb: f64
    │       ├── raw_metar: String
    │       ├── timestamp_utc: String
    │       └── flight_category: String
    │   }
    │
    └── types.rs (futuro)
        └── WeatherForecast, NOAAResponse, etc
```

## 🔄 Fluxo de Dados

```
1. Inicialização
   └─> Ler .env -> Config
   └─> Criar clients (Aviation, Telegram)

2. Loop Principal (a cada 5 min)
   └─> Para cada aeroporto em [KLGA, NZWN, SAEZ, KATL, EGLL]
       └─> GET https://aviationweather.gov/api/data/metar?ids=KLGA
           └─> Raw METAR String: "KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK"
               └─> Parser METAR
                   ├─> Extrair Temperatura: 15°C
                   ├─> Extrair Orvalho: 10°C
                   ├─> Calcular Umidade: 65%
                   ├─> Extrair Vento: 27.8 km/h
                   ├─> Extrair Altímetro: 1012 mb
                   └─> Extrair Categoria: CAVOK
                       └─> METARData struct
                           └─> Formatar para Telegram
                               └─> POST /sendMessage
                               └─> Enviar para Grupo
```

## 📡 Integrações Externas

```
┌─────────────────────────────┐
│  Seu Código (Bot)           │
│  /polymarket-weather-bot    │
└────────────┬────────────────┘
             │
    ┌────────┴────────┐
    │                 │
    ▼                 ▼
┌─────────────────┐  ┌──────────────────┐
│ Aviation        │  │ Telegram         │
│ Weather API     │  │ Bot API          │
│ (Pública)       │  │ (Requer Token)   │
│                 │  │                  │
│ GET /api/data/  │  │ POST /sendMessage│
│   metar         │  │                  │
└─────────────────┘  └──────────────────┘
```

## 🔐 Fluxo de Autorização Telegram

```
setup.sh / .env
    │
    ├─> TELEGRAM_BOT_TOKEN (de @BotFather)
    │   └─> API: https://api.telegram.org/bot{TOKEN}/...
    │
    └─> TELEGRAM_CHAT_ID (seu grupo)
        └─> Só envia para este grupo
            └─> POST /sendMessage
                ├─> chat_id: -123456789
                ├─> text: "✈️ METAR..."
                └─> parse_mode: "Markdown"
```

## 🧪 Parsing METAR Line by Line

```
Input: "KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK"
       │     │        │          │    │    │     │
       │     │        │          │    │    │     └─ Categoria: CAVOK
       │     │        │          │    │    └─ Altímetro: 1012 mb
       │     │        │          │    └─ Temp/Orvalho: 15°C / 10°C
       │     │        │          └─ Visibilidade: 9999m
       │     │        └─ Vento: 270° 15kt com rajadas 25kt
       │     └─ Data/Hora: Dia 15 às 18:56 UTC
       └─ Código Aeroporto: KLGA

Output METARData:
{
  airport_code: "KLGA",
  temperature_celsius: 15.0,
  humidity_percent: Some(65.0),        ← Calculado
  wind_speed_kmh: 27.8,                ← 15 knots * 1.852
  wind_direction: Some(270),
  altimeter_mb: 1012.0,
  raw_metar: "KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK",
  timestamp_utc: "2026-02-27T18:56:00Z",
  flight_category: "CAVOK"
}
```

## 🎯 Ciclo Completo (Timings)

```
[00:00] Start Loop
├─ [00:00] KLGA: Requisição + Parse 1s
├─ [00:02] NZWN: Requisição + Parse 1s
├─ [00:04] SAEZ: Requisição + Parse 1s
├─ [00:06] KATL: Requisição + Parse 1s
└─ [00:08] EGLL: Requisição + Parse 1s

[04:52] Aguardando próximo ciclo...
[05:00] Reinicia o loope com todos os aeroportos novamente
```

