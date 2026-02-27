# 🔧 Troubleshooting & FAQ

## ❓ Perguntas Frequentes

### P: Como obtenho meu token do Telegram?
**R:** 
1. Procure por **@BotFather** no Telegram
2. Envie `/newbot`
3. Siga as instruções
4. Você receberá algo como: `123456:ABC-DEF1234ghIkl...`
5. Copie e salve este token no `.env` como `TELEGRAM_BOT_TOKEN`

### P: Como descubro meu Chat ID?
**R:**
```bash
# Substitua seu token
curl "https://api.telegram.org/botSEU_TOKEN/getUpdates" | grep -o '"id":-[0-9]*'
```
ou crie Python script:
```python
import requests
token = "seu_token"
r = requests.get(f"https://api.telegram.org/bot{token}/getUpdates").json()
for msg in r['result']:
    chat_id = msg['message']['chat']['id']
    print(f"Chat ID: {chat_id}")
```

### P: Por que o bot não envia mensagens?
**R: Verificação rápida:**
1. ✅ Bot foi adicionado ao grupo?
2. ✅ Bot tem permissão para enviar mensagens?
3. ✅ Chat ID começa com `-` (é negativo)?
4. ✅ Token está correto (sem espaços)?
5. ✅ Arquivo `.env` existe na pasta certa?

### P: Como vejo os logs?
**R:**
```bash
# Logs simples
RUST_LOG=info cargo run

# Logs detalhados
RUST_LOG=debug cargo run

# Apenas um módulo
RUST_LOG=polymarket_weather_bot::telegram=debug cargo run
```

### P: Como rodar em background?
**R:**
```bash
# Opção 1: nohup
nohup ./target/release/polymarket-weather-bot > bot.log 2>&1 &

# Opção 2: screen
screen -S bot
./target/release/polymarket-weather-bot
# Ctrl+A depois D para sair

# Opção 3: systemd (Linux)
sudo systemctl edit --force --full polymarket-bot.service
```

### P: O bot consome muita rede/CPU?
**R:** Não! Ele:
- Faz 5 requisições HTTP a cada 5 minutos
- Processa dados localmente (parsing simples)
- Usa < 1% CPU quando idle
- Consome ~2-5 KB por update

---

## 🐛 Erros Comuns e Soluções

### "`TELEGRAM_BOT_TOKEN não encontrada`"

```
❌ Erro:
thread 'main' panicked at 'TELEGRAM_BOT_TOKEN não encontrada'

✅ Solução:
1. Crie arquivo .env na raiz do projeto
2. Adicione: TELEGRAM_BOT_TOKEN=seu_token
3. Salve e tente novamente
```

### "`TELEGRAM_CHAT_ID não encontrada`"

```
❌ Erro:
thread 'main' panicked at 'TELEGRAM_CHAT_ID não encontrada'

✅ Solução:
1. Obter chat ID: curl "https://api.telegram.org/botSEU_TOKEN/getUpdates"
2. Procure por "id": -123456789 no resultado
3. Adicione ao .env: TELEGRAM_CHAT_ID=-123456789
```

### "Falha ao enviar mensagem: Unauthorized"

```
❌ Erro:
Falha ao enviar mensagem: {"ok":false,"error_code":401}

✅ Solução:
- Token está errado ou expirado
- Obtenha novo token em @BotFather
- Ou copie corretamente sem espaços
```

### "Falha ao enviar mensagem: Bad Request"

```
❌ Erro:
Falha ao enviar mensagem: {"ok":false,"error_code":400}

✅ Solução:
- Chat ID está errado (precisa ser negativo)
- Bot não foi adicionado ao grupo
- Verifique com: curl "https://api.telegram.org/botSEU_TOKEN/getMe"
```

### "Connection refused" ou "Network error"

```
❌ Erro:
Error: Connection refused

✅ Solução:
1. Verifique sua internet: ping 8.8.8.8
2. Tente fazer curl direto:
   curl "https://aviationweather.gov/api/data/metar?ids=KLGA"
3. Se não funcionar, a internet está indisponível
```

### "Failed to compile"

```
❌ Erro:
error: could not compile...

✅ Solução:
1. Atualize Rust: rustup update
2. Limpe cache: cargo clean
3. Tentar novamente: cargo build
```

---

## 🧐 Como Debugar

### 1. Testar Manualmente a Aviation Weather API

```bash
# Obter METAR bruto
curl "https://aviationweather.gov/api/data/metar?ids=KLGA&format=raw"

# Deve retornar algo como:
# KLGA 151856Z 27015G25KT 9999 15/10 Q1012 CAVOK
```

### 2. Testar Telegram Bot Manualmente

```bash
# Verificar se bot existe
curl "https://api.telegram.org/botSEU_TOKEN/getMe"

# Enviar mensagem de teste
curl -X POST "https://api.telegram.org/botSEU_TOKEN/sendMessage" \
  -d "chat_id=SEU_CHAT_ID" \
  -d "text=Teste 🎉"

# Ver atualizações (para obter chat_id)
curl "https://api.telegram.org/botSEU_TOKEN/getUpdates"
```

### 3. Testar Parsing de METAR

```bash
# No código Rust, adicione debug print:
fn parse_metar(&self, airport_code: &str, raw_metar: &str) {
    println!("DEBUG: Parsing METAR: {}", raw_metar);
    // ...
    println!("DEBUG: Temperatura: {}", temp_celsius);
    println!("DEBUG: Umidade: {:?}", humidity_percent);
}
```

### 4. Verificar Logs em Tempo Real

```bash
# Terminal 1: Rodar bot com logs
RUST_LOG=debug cargo run

# Terminal 2: Ver arquivo de logs (se rodar em background)
tail -f bot.log
```

---

## 🎯 Performance Tips

### Para rodar em VPS/Server

```bash
# Compilar para o servidor (ex: Linux x86_64)
cargo build --release

# Enviar para servidor
scp target/release/polymarket-weather-bot user@server:/opt/bot/

# No servidor
cd /opt/bot
nohup ./polymarket-weather-bot > bot.log 2>&1 &
```

### Para economizar recursos

```bash
# Mudar intervalo de verificação em src/main.rs
// Linha ~40
sleep(Duration::from_secs(300)).await; // 5 minutos
// Para 10 minutos:
sleep(Duration::from_secs(600)).await;
// Depois: cargo build --release
```

### Para monitorar uso

```bash
# Em outro terminal
top -p $(pgrep polymarket)

# Ou
ps aux | grep polymarket
```

---

## 🔒 Segurança

### Proteger seu Token

```bash
# ❌ NUNCA faça isso:
export TELEGRAM_BOT_TOKEN=seu_token_aqui  # Visível no histórico!

# ✅ Use arquivo .env:
echo "TELEGRAM_BOT_TOKEN=seu_token" > .env
echo ".env" >> .gitignore
```

### Se Token For Comprometido

```bash
# 1. No @BotFather: /mybots -> seu bot -> /revoke
# 2. Espere confirmação
# 3. /newbot para criar novo
# 4. Atualize .env e reinicie
```

---

## 📊 Monitoramento de Saúde

### Verificar Status do Bot

```bash
# Crie script check_bot.sh
#!/bin/bash
if pgrep -x "polymarket-weather-bot" > /dev/null
then
    echo "✅ Bot está rodando"
else
    echo "❌ Bot parou! Reiniciando..."
    ./target/release/polymarket-weather-bot &
fi
```

### Alertas (Cron Job)

```bash
# Ejecutar verificação a cada 5 minutos
*/5 * * * * /home/user/polymarket-bot/check_bot.sh

# Ou a cada hora
0 * * * * /home/user/polymarket-bot/check_bot.sh
```

---

## 🚀 Deployment em Docker (Futuro)

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/polymarket-weather-bot /usr/local/bin/
ENV RUST_LOG=info
CMD ["polymarket-weather-bot"]
```

```bash
docker build -t polymarket-bot .
docker run -d --env-file .env polymarket-bot
```

---

## 📞 Suporte

Se ainda tiver problemas:

1. Verifique [QUICK_START.md](QUICK_START.md)
2. Leia [SETUP_PT_BR.md](SETUP_PT_BR.md)
3. Revise [ARCHITECTURE.md](ARCHITECTURE.md)
4. Ativar logs: `RUST_LOG=debug`
5. Fazer testes manuais com curl

---

## ✅ Checklist de Troubleshooting Rápido

- [ ] `.env` existe na pasta correta?
- [ ] `TELEGRAM_BOT_TOKEN` tem o :124_nome format?
- [ ] `TELEGRAM_CHAT_ID` é negativo (ex: -123456789)?
- [ ] Bot foi adicionado ao grupo?
- [ ] Teste curl funcionou?
- [ ] Compilação foi bem-sucedida?
- [ ] Não há erros nos logs?
- [ ] Internet está funcionando?

Se todas estiverem OK e ainda não funciona, ativar `RUST_LOG=debug` e verificar saída!
