# Polymarket Weather Bot

Um bot em Rust que monitora METARs (relatórios de clima de aeroportos) da Aviation Weather API e envia atualizações formatadas em linguagem simples para um grupo do Telegram.

## Funcionalidades

- ✈️ Monitora METAR de 5 aeroportos internacionais: **KLGA** (Nova York), **NZWN** (Wellington), **SAEZ** (Buenos Aires), **KATL** (Atlanta), **EGLL** (Londres)
- 📊 Extrai informações meteorológicas: temperatura, umidade, velocidade do vento
- 💬 Envia relatórios formatados para Telegram em linguagem simples (ex: "15°C, 50% de umidade")
- 🔄 Monitora continuamente a cada 5 minutos
- 🔗 Integração com Aviation Weather API (METAR)

## Pré-requisitos

- Rust 1.70+ (instale em [rustup.rs](https://rustup.rs/))
- Um bot do Telegram (obtenha em [@BotFather](https://t.me/botfather))
- O ID do grupo onde enviar mensagens

## Configuração

### 1. Obter o Token do Bot Telegram

1. Converse com [@BotFather](https://t.me/botfather) no Telegram
2. Use `/newbot` para criar um novo bot
3. Copie o token fornecido

### 2. Obter o Chat ID do Grupo

1. Adicione o bot ao seu grupo
2. Execute em seu terminal:
```bash
curl "https://api.telegram.org/bot{SEU_TOKEN}/getUpdates" | grep chat
```
3. Procure pelo `"id"` do seu grupo (será um número negativo grande)

### 3. Configurar Variáveis de Ambiente

Crie um arquivo `.env` na raiz do projeto:

```bash
# Telegram Configuration
TELEGRAM_BOT_TOKEN=seu_token_aqui
TELEGRAM_CHAT_ID=seu_chat_id_aqui

# Polymarket Configuration (opcional para versão atual)
POLYMARKET_API_KEY=sua_chave_aqui
PRIVATE_KEY=sua_chave_privada_aqui

# RPC URL (padrão: Polygon RPC)
RPC_URL=https://polygon-rpc.com

# NOAA API Key (opcional)
NOAA_API_KEY=sua_chave_aqui
```

## Como Executar

### Desenvolvimento

```bash
# Compilar e executar em debug
cargo run

# Com logs detalhados
RUST_LOG=info cargo run

# Com logs muito detalhados
RUST_LOG=debug cargo run
```

### Release (Otimizado)

```bash
# Compilar versão otimizada
cargo build --release

# Executar
./target/release/polymarket-weather-bot
```

## Saída Esperada

O bot enviará mensagens assim ao seu grupo Telegram:

```
✈️ METAR - KLGA

🌡️ Temperatura: 15°C
💧 Umidade: 65%
💨 Vento: 27.8 km/h
📊 Categoria: VFR
🕐 Horário: 18:56 UTC

📝 Raw: KLGA 151856Z 27015G25KT 9999 15/10 Q1012 LFEW
```

## Estrutura do Código

```
src/
├── main.rs          # Programa principal com loop de monitoramento
├── config.rs        # Configuração de variáveis de ambiente
├── telegram.rs      # Cliente Telegram e formatação de mensagens
└── weather/
    ├── mod.rs       # Módulos de clima
    ├── aviation.rs  # Cliente Aviation Weather e parser METAR
    ├── types.rs     # Tipos de dados meteorológicos
    └── noaa.rs      # Cliente NOAA (futuro)
```

## Formatos de Mensagens

As mensagens são formatadas de forma amigável para leigos:

| Campo | Descrição |
|-------|-----------|
| Temperatura | Em Celsius (°C) |
| Umidade | Porcentagem relativa (%) |
| Vento | Em quilômetros por hora (km/h) |
| Horário | UTC (Tempo Universal Coordenado) |
| Categoria | VFR (Condições visuais), IFR (Controle por instrumentos), etc |

## Aeroportos Monitorados

| Código | Cidade | País |
|--------|--------|------|
| KLGA | Nova York (LaGuardia) | EUA 🇺🇸 |
| NZWN | Wellington | Nova Zelândia 🇳🇿 |
| SAEZ | Buenos Aires (Ministro Pistarini) | Argentina 🇦🇷 |
| KATL | Atlanta (Hartsfield-Jackson) | EUA 🇺🇸 |
| EGLL | Londres (Heathrow) | Reino Unido 🇬🇧 |

Para adicionar mais aeroportos, edite o array `AIRPORTS` em `src/main.rs`.

## Solução de Problemas

### "TELEGRAM_BOT_TOKEN não encontrada"
- Verifique se o arquivo `.env` existe na raiz do projeto
- Verifique se a variável está no formato correto

### Bot não envia mensagens
- Verifique se o bot foi adicionado ao grupo
- Verifique se tem permissão para enviar mensagens
- Confirme o Chat ID está correto (deve ser negativo)

### Erro ao obter METAR
- Verifique sua conexão com a internet
- Verifique se o código do aeroporto está correto
- A Aviation Weather API pode estar temporariamente indisponível

## Dependências

- `tokio` - Runtime assíncrono
- `reqwest` - Cliente HTTP
- `serde_json` - Serialização JSON
- `chrono` - Manipulação de datas/horas
- `log` - Logging
- `env_logger` - Logger configurável

## Licença

Veja o arquivo [LICENSE](LICENSE) para detalhes.

## Autor

Desenvolvido para monitoramento meteorológico de mercados Polymarket.
