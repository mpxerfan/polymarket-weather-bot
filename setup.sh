#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}Polymarket Weather Bot - Setup${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""

# Check if .env already exists
if [ -f .env ]; then
    echo -e "${YELLOW}Arquivo .env já existe. Deseja sobrescrever? (s/n)${NC}"
    read -r response
    if [ "$response" != "s" ]; then
        echo -e "${GREEN}Setup cancelado.${NC}"
        exit 0
    fi
fi

echo ""
echo -e "${YELLOW}Configure as variáveis de ambiente:${NC}"
echo ""

# Get Telegram Bot Token
echo -n "📱 Token do Bot Telegram: "
read -r TELEGRAM_BOT_TOKEN

if [ -z "$TELEGRAM_BOT_TOKEN" ]; then
    echo -e "${RED}Erro: Token do bot não pode estar vazio!${NC}"
    exit 1
fi

# Get Chat ID
echo -n "💬 ID do Chat/Grupo Telegram: "
read -r TELEGRAM_CHAT_ID

if [ -z "$TELEGRAM_CHAT_ID" ]; then
    echo -e "${RED}Erro: Chat ID não pode estar vazio!${NC}"
    exit 1
fi

# Get Polymarket API Key
echo -n "🔑 Chave API do Polymarket (opcional, pressione Enter para pular): "
read -r POLYMARKET_API_KEY

# Get Private Key
echo -n "🔐 Private Key (opcional, pressione Enter para pular): "
read -r PRIVATE_KEY

# Get RPC URL
echo -n "🌐 RPC URL (padrão: https://polygon-rpc.com): "
read -r RPC_URL
RPC_URL=${RPC_URL:-https://polygon-rpc.com}

# Get NOAA API Key (optional)
echo -n "🌦️ Chave API NOAA (opcional, pressione Enter para pular): "
read -r NOAA_API_KEY

# Create .env file
cat > .env << EOF
# Telegram Configuration
TELEGRAM_BOT_TOKEN=$TELEGRAM_BOT_TOKEN
TELEGRAM_CHAT_ID=$TELEGRAM_CHAT_ID

# Polymarket Configuration
POLYMARKET_API_KEY=${POLYMARKET_API_KEY:-seu_polymarket_api_key}
PRIVATE_KEY=${PRIVATE_KEY:-sua_private_key}

# RPC URL
RPC_URL=$RPC_URL

# NOAA API Key (optional)
NOAA_API_KEY=${NOAA_API_KEY:-sua_noaa_api_key}
EOF

echo ""
echo -e "${GREEN}✅ Arquivo .env criado com sucesso!${NC}"
echo ""
echo -e "${YELLOW}Próximos passos:${NC}"
echo "1. Verifique se o bot foi adicionado ao seu grupo Telegram"
echo "2. Execute: ${GREEN}cargo run${NC}"
echo "3. Ou compile a versão release: ${GREEN}cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Para logs detalhados, execute:${NC}"
echo "   ${GREEN}RUST_LOG=info cargo run${NC}"
echo ""
echo -e "${YELLOW}Aeroportos que serão monitorados:${NC}"
echo "   ✈️ KLGA (Nova York)"
echo "   ✈️ NZWN (Wellington)"
echo "   ✈️ SAEZ (Buenos Aires)"
echo "   ✈️ KATL (Atlanta)"
echo "   ✈️ EGLL (Londres)"
echo ""
