# 🔍 Sistema de Detecção por Timestamps - Documentação Técnica

## Resumo Executivo

O bot foi atualizado para usar **timestamps oficiais das APIs** como identificadores únicos de boletins, em vez de comparar o conteúdo completo ou usar hashing. Isso garante:

✅ **Nenhuma duplicação** - Mesmo timestamp = mesmo boletim  
✅ **Resiliência a atrasos** - Funciona se o relatório sai 1-5 minutos atrasado  
✅ **Menor overhead** - Comparação simples de strings em vez de cálculos de hash  
✅ **Responsivo** - Intervalo reduzido de 2 segundos para **20 segundos** entre verificações

---

## 🏗️ Arquitetura da Mudança

### Antes (Baseado em Hash)

```
METAR recebido → Calcular hash da string inteira → Comparar hash
❌ Problema: Se o servidor retorna dados idênticos com microsegundo diferente, 
   pode haver duplicação ou perda de mensagens
```

### Depois (Baseado em Timestamps Oficiais)

```
METAR recebido → Extrair timestamp OFFICIAL (DDHHMMZ) → Comparar timestamp
✅ Garantia: Timestamp é o identificador único do boletim na aviação
✅ Resultado: Mudança apenas quando ATC publica novo METAR
```

---

## 📡 METAR (Aviation Weather API)

### Extração do Timestamp

O timestamp oficial METAR está **sempre na 2ª posição** do boletim raw:

```
KLGA 262330Z 27015G25KT 9999 15/10 Q1012 CAVOK
      ^^^^^^
      DDHHMMZ = Identificador único do boletim
      
Breakdown:
  DD = Dia do mês (26)
  HH = Hora UTC (23)
  MM = Minuto UTC (30)
  Z  = Zulu (UTC)
```

### Detecção de Novo Boletim

```rust
// Armazenado em memória
last_metar_timestamp["KLGA"] = "262330Z"

// Nova verificação
novo_metar_timestamp = extrair_regex_\d{6}Z(boletim_raw)
// nova_metar_timestamp = "270000Z" (um novo boletim, 30 min depois)

// Comparação
if last != novo {
    // Novo boletim oficial detectado!
    // Enviar para Telegram
}
```

### Garantia de Funcionamento com Atrasos

```
Cenário: ATC publica METAR às 23:30Z, mas API retorna 23:33Z

Sequência de tempo:
  23:30 UTC → ATC publica boletim KLGA 262330Z ...
  23:33 UTC → Nossa app pega do servidor (3 min de atraso)
  
Resultado:
  ✅ Timestamp extraído = "262330Z"
  ✅ Comparação = "262330Z" ≠ "262300Z" (anterior)
  ✅ Enviar mensagem

A lógica está imune a atrasos de até 59 minutos (próxima emissão)
```

---

## 🌐 Open-Meteo API

### Extração do Timestamp

O timestamp está no objeto `current.time` da resposta JSON:

```json
{
  "current": {
    "time": "2024-02-26T23:30",
    "temperature_2m": 15.5,
    "relative_humidity_2m": 65,
    ...
  }
}
```

O servidor retorna o timestamp **exato do momento da atualização** dos dados.

### Detecção de Nova Previsão

```rust
// Armazenado em memória
last_openmeteo_timestamp["KLGA"] = "2024-02-26T23:30"

// Nova verificação
novo_timestamp = resposta_json["current"]["time"]
// novo_timestamp = "2024-02-26T23:45" (atualizado 15 min depois)

// Comparação
if last != novo {
    // Novos dados de previsão detectados!
    // Enviar para Telegram
}
```

### Intervalo de Atualização

A Open-Meteo **atualiza dados a cada 5-15 minutos**. Comparar timestamps detecta qualquer mudança real no conjunto de dados.

---

## ⏱️ Conversão de Timezone

### Por que Auckland?

Wellington (NZWN) é um dos aeroportos monitorados, logo a zona mais relevante é **Pacific/Auckland (NZDT)**.

### Implementação

```rust
// Entrada METAR: "262330Z"
// Saída formatada: "26/02 23:30 NZDT"

// Entrada Open-Meteo: "2024-02-26T23:30"
// Saída formatada: "26/02 23:30 NZDT"
```

Ambos os formatos são convertidos para Auckland usando `chrono-tz`:

```rust
use chrono_tz::Tz;

let auckland_tz = "Pacific/Auckland".parse::<Tz>()?;
let auckland_time = utc_time.with_timezone(&auckland_tz);
// Formatação: "26/02 23:30 NZDT"
```

---

## 🔄 Estrutura de Memória (HashMap)

Rastreamento eficiente de últimos timestamps:

```rust
struct LastReport {
    metar_timestamp: String,          // Ex: "262330Z"
    openmeteo_timestamp: String,      // Ex: "2024-02-26T23:30"
}

// Chave: "{airport_code}_metar" ou "{airport_code}_openmeteo"
// Exemplo: "KLGA_metar" → "262330Z"
//          "KLGA_openmeteo" → "2024-02-26T23:30"

last_reports: HashMap<String, LastReport>
```

**Benefício**: Comparação O(1) com string equality (não precisa recalcular hash)

---

## 📊 Intervalo de Verificação

### Mudança: 2 segundos → 20 segundos

| Aspecto | Antes | Depois |
|---------|-------|--------|
| Intervalo | 2 seg | 20 seg |
| Ciclos por hora | 1.800 | 180 |
| Requisições/hora | 10.800 (6 airports × 2 APIs) | 1.080 |
| Latência detecção | 0-2 seg | 0-20 seg |
| Overhead servidor | Alto | 90% reduzido ✅ |

**20 segundos é suficiente** porque:
- METAR publica novo boletim a cada **29-60 minutos**
- Open-Meteo atualiza a cada **5-15 minutos**  
- Margem de segurança (20 seg) detecta qualquer mudança em tempo útil

---

## 🛡️ Garantias de Integridade

### 1. Sem Duplicação

```
Sim: Mesmo servidor retorna "262330Z" 3 vezes seguidas
  1ª chamada:  timestamp = "262330Z" → last = "" → ENVIAR
  2ª chamada:  timestamp = "262330Z" → last = "262330Z" → ❌ NÃO enviar
  3ª chamada:  timestamp = "262330Z" → last = "262330Z" → ❌ NÃO enviar
```

**Resultado**: Uma mensagem = um novo boletim. Garantido.

### 2. Resiliência a Atrasos

```
Cenário: ATC publica 23:30Z, API retorna com 30 seg de atraso

  23:30:00 → ATC publica KLGA 262330Z ...
  23:30:30 → Nossa app verifica (vê timestamp "262330Z")
  
❌ Detalhe: Se usássemos `Utc::now()` diferente seria problema
✅ Solução: Extrair timestamp OFICIAL do METAR (262330Z), não usar relógio local
```

### 3. Sem Perda de Mensagens

```
Cenário: App verificar a cada 20 seg, novo boletim a cada 30 min

Timeline:
  10:00 METAR 011000Z
  10:20 Verificar → ENVIOU (11:00Z ≠ "")
  
  10:30 METAR 011030Z (novo boletim)
  10:40 Verificar → ENVIOU (11:30Z ≠ 11:00Z) ✅
```

**Garantia**: Independente do intervalo de verificação, qualquer novo timestamp é detectado na próxima iteração.

---

## 📝 Fluxo Completo de Uma Verificação

```
┌─────────────────────────────────────────────────────────┐
│ INÍCIO DO LOOP (A cada 20 segundos)                    │
└─────────────────────────────────────────────────────────┘
                     ↓
        Para cada aeroporto (KLGA, NZWN, ...)
                     ↓
    ┌──────────────────────────────────────┐
    │ METAR (Aviation Weather)             │
    ├──────────────────────────────────────┤
    │ 1. GET /api/data/metar?ids=KLGA      │
    │ 2. Parse raw: KLGA 262330Z ...       │
    │ 3. Extrair timestamp: "262330Z"      │
    │ 4. Recuperar último do HashMap       │
    │ 5. Comparar: "262330Z" ≠ "262300Z"? │
    │    ✅ SIM → Enviar para Telegram     │
    │       - Converter "262330Z" → Auckland
    │       - Formatar mensagem com horário
    │       - Atualizar HashMap            │
    │    ❌ NÃO → Pular                    │
    └──────────────────────────────────────┘
              delay 500ms
    ┌──────────────────────────────────────┐
    │ Open-Meteo (Forecast)                │
    ├──────────────────────────────────────┤
    │ 1. GET /v1/forecast?lat=...&lon=...  │
    │ 2. Parse JSON: current.time           │
    │ 3. Extrair timestamp: "2024-02-26..." │
    │ 4. Recuperar último do HashMap       │
    │ 5. Comparar timestamps?              │
    │    ✅ SIM → Enviar para Telegram     │
    │       - Converter para Auckland      │
    │       - Formatar mensagem            │
    │       - Atualizar HashMap            │
    │    ❌ NÃO → Pular                    │
    └──────────────────────────────────────┘
              delay 500ms
        Próximo aeroporto...
                     ↓
        Fim de todos os aeroportos
                     ↓
        ┌──────────────────────┐
        │ AGUARDAR 20 segundos │
        └──────────────────────┘
                     ↓
        VOLTA ao início do loop
```

---

## 🚀 Benefícios Técnicos

| Problema Antigo | Solução Nova |
|-----------------|--------------|
| Hash de string inteira (overhead) | String simples (O(1)) |
| Sensibilidade a pequenas mudanças | Apenas timestamps oficiais |
| Falha se relatório atrasa | Imune a atrasos (até 1 hora+) |
| Comparação complexa | `last != novo` |
| 2 seg × 6 aeroportos × 2 APIs = alto overhead | 20 seg reduz 90% requisições |
| Possível duplicação por timing | Timestamp é único per boletim |

---

## 📋 Checklist de Funcionamento

- ✅ METAR: Extrai apenas `DDHHMMZ` (não usa Utc::now())
- ✅ Open-Meteo: Extrai `current.time` do JSON
- ✅ Comparação: String equality simples
- ✅ Intervalo: 20 segundos entre ciclos
- ✅ Timezone: Auckland para exibição
- ✅ HashMap: Rastreia timestamps anteriores
- ✅ Telegram: Mostra horário em Auckland
- ✅ Sem duplicação: Mesmo timestamp = uma mensagem
- ✅ Resiliência: Funciona com atrasos de minutos

---

## 🔧 Arquivo de Teste

Para testar a lógica manualmente:

```bash
# Verificar timestamps extraídos
./run.sh

# Logs mostrarão:
# [INFO] 🆕 Novo METAR detectado para KLGA (timestamp: 262330Z)
# [INFO] 🆕 Novo relatório Open-Meteo detectado para KLGA (timestamp: 2024-02-26T23:30)
```

Se rodar novamente sem mudanças nas APIs:
```bash
# Nenhuma nova mensagem = funcionando corretamente ✅
```

---

**Versão**: 2.0 (Timestamps Official-Based Detection)  
**Última atualização**: Fevereiro 2026
