# ⚡ Guia Rápido - Começar em 2 Minutos

## 🆕 ATUALIZAÇÃO: Agora com 2 APIs!

O bot foi **atualizado para monitorar continuamente** com:
- 🛩️ **Aviation Weather** (METAR)
- 🌐 **Open-Meteo** (Previsões)

E envia para Telegram **APENAS quando há novo relatório** (sem atrasos!)

Para detalhes completos: [DUAS_APIS_GUIA.md](DUAS_APIS_GUIA.md)

1. Abra Telegram e procure por **@BotFather**
2. Envie `/newbot`
3. Siga as instruções para criar seu bot
4. **Copie o token** fornecido (exemplo: `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`)

## Passo 2: Obter Chat ID do Grupo (1 min)

1. Crie um grupo novo ou use um existente
2. Adicione seu bot ao grupo
3. Em seu terminal, execute:
```bash
curl "https://api.telegram.org/bot{SEU_TOKEN_AQUI}/getUpdates" | grep -o '"id":-[0-9]*'
```
4. Procure pelo número negativo grande - esse é seu **Chat ID**

## Passo 3: Configurar e Rodar (muito fácil!)

### Opção A: Script Rápido (Recomendado!)
```bash
cd /workspaces/polymarket-weather-bot
./run.sh
```

Isso vai ler o arquivo `.env` e rodar o bot automaticamente!

### Opção B: Script de Produção
```bash
cd /workspaces/polymarket-weather-bot
./run-prod.sh
```

Versão otimizada e mais rápida.

### Opção B: Configurar Manualmente
```bash
cd /workspaces/polymarket-weather-bot

# Criar arquivo .env
cat > .env << EOF
TELEGRAM_BOT_TOKEN=seu_token_aqui
TELEGRAM_CHAT_ID=seu_chat_id_aqui
POLYMARKET_API_KEY=dummy
PRIVATE_KEY=dummy
RPC_URL=https://polygon-rpc.com
NOAA_API_KEY=dummy
EOF
```

### Opção C: Rodar Direto (mais rápido com cargo)
```bash
cd /workspaces/polymarket-weather-bot
TELEGRAM_BOT_TOKEN=seu_token TELEGRAM_CHAT_ID=seu_chat_id cargo run
```

## Passo 4: Verificar que Funciona

Se vir isso nos logs, está funcionando:
```
[INFO] Polymarket Weather Bot iniciando...
[INFO] Bot configurado para monitorar: ["KLGA", "NZWN", "SAEZ", "KATL", "EGLL"]
[INFO] Obtendo METAR para KLGA
[INFO] METAR obtido para KLGA: 15°C
[INFO] Enviando mensagem para Telegram
[INFO] Mensagem enviada com sucesso!
```

E seu grupo Telegram receberá mensagens assim:
```
✈️ METAR - KLGA

🌡️ Temperatura: 15°C
💧 Umidade: 65%
💨 Vento: 27.8 km/h
📊 Categoria: VFR
🕐 Horário: 18:56 UTC
```

---

## 📱 Exemplo de Teste Rápido

Para testar se o token está correto SEM rodar o bot:

```bash
# Replace com seu token
curl -X POST "https://api.telegram.org/botSEU_TOKEN_AQUI/sendMessage" \
  -d "chat_id=SEU_CHAT_ID_AQUI" \
  -d "text=Olá! Este é um teste 🎉"
```

---

## 🚀 Modo Produção

Para rodar o bot otimizado (8MB executable):

```bash
# Versão mais rápida e menor
./target/release/polymarket-weather-bot

# Ou com logs
RUST_LOG=info ./target/release/polymarket-weather-bot
```

---

## 🆘 Erros Comuns

| Erro | Solução |
|------|---------|
| `TELEGRAM_BOT_TOKEN não encontrada` | Use `./run.sh` ou exporte variáveis de ambiente |
| Bot não envia mensagens | Verifique se chat_id é negativo |
| `connection refused` | Verifique sua internet |
| `invalid token` | Copie token corretamente de @BotFather |
| Warnings sobre "never read" | Normal - são campos para futuro uso |

---

## 🎯 Aeroportos Monitorados

A cada 5 minutos, o bot verifica:

| Código | Cidade |
|--------|--------|
| 🗽 KLGA | **Nova York** |
| 🏔️ NZWN | **Wellington** |
| 🌆 SAEZ | **Buenos Aires** |
| 🏢 KATL | **Atlanta** |
| 🏰 EGLL | **Londres** |

---

## 📊 O que o Bot Envia

Para cada aeroporto, você recebe:
- 🌡️ **Temperatura** em Celsius (ex: 15°C)
- 💧 **Umidade** em % (ex: 65%)
- 💨 **Vento** em km/h (ex: 27.8 km/h)
- 📊 **Categoria** VFR/IFR/CAVOK
- 🕐 **Horário** UTC

---

## ✨ Pronto!

Seu bot está funcionando! 🎉

Para parar: `Ctrl + C`
Para rodar em background: `nohup ./target/release/polymarket-weather-bot > bot.log 2>&1 &`

---

Suporte: Verifique [SETUP_PT_BR.md](SETUP_PT_BR.md) para mais detalhes.
