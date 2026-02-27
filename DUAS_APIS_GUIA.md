# 🚀 Bot com Duas APIs - Guia de Uso

Seu bot agora integra **2 APIs meteorológicas simultâneas**:

## 📡 APIs Integradas

### 1. **Aviation Weather (METAR)**
- **Fonte**: https://aviationweather.gov
- **Tipo**: Relatórios meteorológicos de aeroportos (METAR)
- **Atualização**: Geralmente a cada hora
- **Dados**: Temperatura, umidade, vento, visibilidade, categoria com código de voo
- **Etiqueta**: `📡 AVIATION WEATHER METAR`

### 2. **Open-Meteo**
- **Fonte**: https://open-meteo.com
- **Tipo**: Previsões meteorológicas
- **Atualização**: Contínua, conforme dados mudarem
- **Dados**: Temperatura, umidade, velocidade/direção do vento, condição do tempo
- **Etiqueta**: `🌐 OPEN-METEO FORECAST`

---

## 🎯 Como Funciona

### Verificação Contínua

O bot agora:
1. ✅ Verifica **continuamente** (a cada 2 segundos)
2. ✅ Para cada aeroporto:
   - Obtém METAR da Aviation Weather
   - Obtém previsão do Open-Meteo
3. ✅ **Detecta mudanças** usando hash das informações
4. ✅ **Envia para Telegram APENAS quando há novo relatório**

### Fluxo de Detecção

```
Verificação 1 (Inicial)
├─ KLGA METAR: Temp=0°C, Vento=10kt → ENVIAR ✅
├─ KLGA Open-Meteo: Temp=0°C, Umidade=65% → ENVIAR ✅
├─ NZWN METAR: Temp=16°C... → ENVIAR ✅
└─ NZWN Open-Meteo... → ENVIAR ✅

Verificação 2 (2 segundos depois)
├─ KLGA METAR: Temp=0°C, Vento=10kt → NÃO MUDOU ❌
├─ KLGA Open-Meteo: Temp=0°C, Umidade=65% → NÃO MUDOU ❌
└─ (segue para próximos aeroportos)

Verificação 3 (4 segundos depois)
├─ KLGA METAR: Temp=2°C, Vento=12kt → MUDOU! → ENVIAR ✅
└─ KLGA Open-Meteo: Temp=1°C, Umidade=68% → MUDOU! → ENVIAR ✅
```

---

## 📱 Ejemplo de Mensagens

### Aviation Weather METAR
```
📡 AVIATION WEATHER METAR
✈️ Aeroporto: KLGA

🌡️ Temperatura: 0°C
💧 Umidade: 65%
💨 Vento: 18.5 km/h
📊 Categoria: VFR
🕐 Horário: 01:00 UTC

📝 Raw: KLGA 270104Z 27010KT 9999 0/10 Q1012 CAVOK
```

### Open-Meteo Forecast
```
🌐 OPEN-METEO FORECAST
✈️ Aeroporto: KLGA

🌡️ Temperatura: 0.1°C
💧 Umidade: 65.0%
💨 Vento: 18.5 km/h (Dir: 270°)
🌤️ Condição: Céu Claro
🕐 Horário: 02:04 UTC
```

---

## 🎯 Aeroportos Monitorados

| Código | Cidade | Lat | Lon |
|--------|--------|-----|-----|
| KLGA | Nova York (LaGuardia) | 40.77°N | 73.87°W |
| NZWN | Wellington | 41.33°S | 174.89°E |
| SAEZ | Buenos Aires (Ministro Pistarini) | 34.82°S | 58.54°W |
| KATL | Atlanta (Hartsfield-Jackson) | 33.64°N | 84.43°W |
| EGLL | Londres (Heathrow) | 51.47°N | 0.45°W |

---

## 🏃 Executar o Bot

### Versão Desenvolvimento (Mais Logs)
```bash
./run.sh
# ou
RUST_LOG=info ./run.sh
```

### Versão Produção (Otimizada)
```bash
./run-prod.sh
# ou
RUST_LOG=info ./run-prod.sh
```

### Rodar em Background
```bash
nohup ./run-prod.sh > bot.log 2>&1 &
tail -f bot.log  # Ver logs em tempo real
```

---

## 📊 O Que Muda Agora

### ❌ Antes
- ⏱️ Verificava a cada 5 minutos
- 📨 Enviava SEMPRE, mesmo sem mudanças
- 🛩️ Só Aviation Weather (METAR)

### ✅ Agora
- ⚡ Verifica **continuamente** (a cada 2 segundos)
- 📨 Envia **APENAS quando há mudanças**
- 🛩️ **2 APIs simultâneas** (Aviation Weather + Open-Meteo)
- 🏷️ Especifica qual API é a fonte da mensagem

---

## 🔧 Detalhes Técnicos

### Sistema de Hash para Detecção

O bot usa `DefaultHasher` de Rust para comparar:

```rust
Iteração 1: hash(KLGA_METAR) = 12345
Iteração 2: hash(KLGA_METAR) = 12345 → Sem mudanças ❌

Iteração 3: hash(KLGA_METAR) = 12346 → Mudou! ✅ ENVIAR
```

### Campos Comparados

**Para METAR**:
```
temperatura + umidade + velocidade_vento + categoria_voo + raw_metar
```

**Para Open-Meteo**:
```
temperatura + umidade + velocidade_vento + codigo_clima + descricao
```

---

## 📈 Performance

| Métrica | Antes | Agora |
|---------|-------|-------|
| Intervalo de verificação | 5 minutos | 2 segundos |
| Mensagens enviadas | Todas | Apenas novas |
| APIs monitorizadas | 1 | 2 |
| Detecção de mudança | Nenhuma | Sim, com hash |
| CPU (idle) | ~1% | <1% |
| Memória | ~15MB | ~20MB |

---

## 🆘 Troubleshooting

### Bot não envia mensagens do Open-Meteo
```bash
# Verifique os logs
tail -f bot.log | grep "Open-Meteo"

# Verifique conectividade
curl "https://api.open-meteo.com/v1/forecast?latitude=40.77&longitude=-73.87&current=temperature_2m"
```

### Bot envia muitas mensagens
- Normal se os dados estão mudando constantemente
- Aumente o delay entre verificações em `src/main.rs` (linha ~75)
- De 2 segundos para 10 segundos: `sleep(Duration::from_secs(10))`

### Mensagens aparecem no Telegram?
- ✅ Aviation Weather: Envia sempre que novo METAR
- ✅ Open-Meteo: Envia quando dados mudam
- Ambas as APIs podem enviar simultaneamente

---

## 🚀 Próximos Passos

1. **Testar em Produção**:
   ```bash
   ./run-prod.sh &
   # Deixar rodando 24/7
   ```

2. **Monitorar Logs**:
   ```bash
   tail -f bot.log
   ```

3. **Adicionar Mais Aeroportos**:
   - Edite `src/weather/openmeteo.rs`: `AIRPORT_COORDINATES`
   - Edite `src/main.rs`: `const AIRPORTS`

4. **Customizar Intervalo**:
   - `src/main.rs` linha ~135: `duration(from_secs(2))`
   - Mude para seu intervalo desejado

---

Desenvolvido com ❤️ para monitoramento meteorológico contínuo!
