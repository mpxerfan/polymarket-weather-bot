# Integração da Aviation Weather API - Guia Técnico

## Visão Geral da API

A Aviation Weather API (https://aviationweather.gov/api) fornece dados meteorológicos de aviação, incluindo METARs (relatórios meteorológicos de aeroportos).

## Endpoints Utilizados

### 1. GET METAR Raw
```
GET https://aviationweather.gov/api/data/metar?ids={AIRPORT_CODE}&format=raw
```

**Parâmetros:**
- `ids`: Código ICAO do aeroporto (ex: KLGA, NZWN)
- `format`: Formato da resposta (raw para texto puro METAR)

**Exemplo de Requisição:**
```bash
curl "https://aviationweather.gov/api/data/metar?ids=KLGA&format=raw"
```

**Exemplo de Resposta:**
```
KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK
```

## Estrutura do METAR

Um METAR segue este padrão:

```
ICAO DATA/HORA VENTO VISIBILIDADE TEMP/ORVALHO ALTÍM CATEGORIAS
KLGA  151856Z   27015G25KT 9999   15/10        Q1012 CAVOK
```

| Campo | Exemplo | Descrição |
|--------|---------|-----------|
| ICAO | KLGA | Código do aeroporto |
| Data/Hora | 151856Z | Dia 15, hora 18:56 UTC |
| Vento | 27015G25KT | 270° a 15 kt, rajadas 25 kt |
| Visibilidade | 9999 | 9999 metros |
| Temp/Orvalho | 15/10 | Temperatura 15°C, orvalho 10°C |
| Altímetro | Q1012 | 1012 milibares |
| Categoria | CAVOK | Condições visuais, visibilidade boa |

## Parsing de METAR

### Como extrair informações:

1. **Temperatura**: Primeiro número antes da barra (15 em 15/10)
2. **Ponto de Orvalho**: Número depois da barra (10 em 15/10)
3. **Umidade Relativa**: Calculada usando a fórmula de Magnus

### Fórmula de Magnus para Umidade:

```
a = 17.27
b = 237.7
alpha = (a*Td/(b+Td) + a*T/(b+T)) / 2
RH = 100 * (exp(alpha) / (1 + exp(alpha)^2))
```

Onde:
- T = Temperatura (°C)
- Td = Ponto de Orvalho (°C)
- RH = Umidade Relativa (%)

## Categorias de Voo

| Categoria | Significado | Limites |
|-----------|-------------|---------|
| CAVOK | Clouds And Visibility OK | Céu claro, visibilidade >10km |
| VFR | Visual Flight Rules | Boas condições visuais |
| MVFR | Marginal VFR | Condições marginais |
| IFR | Instrument Flight Rules | Voo por instrumentos necessário |
| LIFR | Low IFR | Baixas condições de instrumento |

## Conversão de Unidades

- **Knots para km/h**: knots × 1.852
- **Ft para m**: pés × 0.3048
- **Milibares para hPa**: 1 mbar = 1 hPa

## Exemplo de Fórmula em Código Rust

```rust
// Extraindo temperatura e ponto de orvalho
let parts: Vec<&str> = "15/10".split('/').collect();
let temp = parts[0].parse::<f64>()?;    // 15.0
let dewpoint = parts[1].parse::<f64>()?; // 10.0

// Calculando umidade relativa
let a = 17.27;
let b = 237.7;
let alpha = ((a * dewpoint) / (b + dewpoint) + (a * temp) / (b + temp)).ln() / 2.0;
let humidity = 100.0 * (alpha.exp() / (1.0 + alpha.exp().powi(2)));
```

## Códigos de Aeroporto Monitorados

| ICAO | IATA | Cidade | País |
|------|------|--------|------|
| KLGA | LGA | Nova York (LaGuardia) | 🇺🇸 EUA |
| NZWN | WLG | Wellington | 🇳🇿 Nova Zelândia |
| SAEZ | AEP | Buenos Aires (Ministro Pistarini) | 🇦🇷 Argentina |
| KATL | ATL | Atlanta (Hartsfield-Jackson) | 🇺🇸 EUA |
| EGLL | LHR | Londres (Heathrow) | 🇬🇧 Reino Unido |

## Freqüência de Atualizações

- METARs são tipicamente atualizados a cada 1 hora
- Em caso de condições especiais, podem ser emitidos mais frequentemente
- O bot verifica a cada 5 minutos

## Tratamento de Erros

A API pode retornar:
- Respostas vazias se o aeroporto não existir
- Atualizações antigas se o aeroporto não reportou recentemente
- Erros HTTP 4xx/5xx em caso de indisponibilidade

## Próximas Melhorias Possíveis

1. Adicionar suporte para dados contínuos (SIGMET, PIREP)
2. Integrar gráficos de previsão
3. Alertas de mudanças significativas de tempo
4. Historical data analysis
5. Integração com APIs de previsão (NOAA, OpenMeteo)

## Referências

- [Aviation Weather API Docs](https://aviationweather.gov/api)
- [METAR Format Standard](https://www.weather.gov/media/epz/metar/metar_info.php)
- [ICAO METAR Standards](https://www.icao.int/)
