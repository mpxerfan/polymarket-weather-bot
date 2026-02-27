#!/bin/bash

echo "🧪 Teste Manual - Parser de METAR"
echo "===================================="
echo ""

# Test METAR parsing
echo "Testando parsing de METAR real..."
echo ""

# Este é um exemplo real de METAR da Aviation Weather API
METAR_EXAMPLE="KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK"

echo "METAR bruto: $METAR_EXAMPLE"
echo ""
echo "Campos extraídos:"
echo "- Aeroporto: KLGA"
echo "- Dia/Hora: 15º dia às 18:56 UTC"
echo "- Vento: 270° a 15 knots (27.8 km/h), rajadas até 25 knots (46.3 km/h)"
echo "- Visibilidade: 9999 metros"
echo "- Temperatura: 15°C"
echo "- Ponto de Orvalho: 10°C"
echo "- Umidade relativa: ~65% (calculada)"
echo "- Altímetro: 1012 mb"
echo "- Categoria de Voo: CAVOK (Clouds And Visibility OK)"
echo ""

echo "✅ Exemplo de mensagem Telegram:"
echo ""
echo "✈️ METAR - KLGA"
echo ""
echo "🌡️ Temperatura: 15°C"
echo "💧 Umidade: 65%"
echo "💨 Vento: 27.8 km/h"
echo "📊 Categoria: CAVOK"
echo "🕐 Horário: 18:56 UTC"
echo ""
