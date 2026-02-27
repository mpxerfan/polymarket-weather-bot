# 🚀 Guia Completo: Deploy no Render

## ✅ O que você precisa

- ✅ Conta do GitHub com seu repositório
- ✅ Conta no [Render.com](https://render.com) (gratuita)
- ✅ Token do Telegram Bot
- ✅ Chat ID do seu grupo Telegram

---

## 📋 Passo 1: Preparar o Repositório

Certifique-se que tudo está commitado:

```bash
cd /workspaces/polymarket-weather-bot
git add .
git commit -m "Preparando para deploy no Render"
git push
```

✅ Seu repositório já tem:
- `Dockerfile` (para build automático)
- `.env.example` (configuração de exemplo)
- `Cargo.toml` e `Cargo.lock` (dependências)

---

## 🔑 Passo 2: Acessar Render e Conectar GitHub

1. Acesse [render.com](https://render.com)
2. Clique em **Sign up** (ou faça login com GitHub)
3. **Autorize Render a acessar seu GitHub**
   - GitHub aparecerá pedindo autorização
   - Clique em "Authorize render"

---

## ⚙️ Passo 3: Criar um Novo Web Service

1. No dashboard do Render, clique em **"New +"** (canto superior direito)
2. Selecione **"Web Service"**
3. Clique em **"Deploy existing repo"**
4. Procure por `polymarket-weather-bot` e selecione
5. Clique em **"Connect"**

---

## 🎯 Passo 4: Configurar o Web Service

### Preenchimento Automático:
- **Name**: `polymarket-weather-bot` (ou qualquer outro nome)
- **Region**: `Oregon` (ou mais perto de você)
- **Branch**: `teste` (ou `main` se preferir)
- **Runtime**: `Docker`

### Importante! Mude esses campos:

| Campo | Valor |
|-------|-------|
| **Root Directory** | Deixe vazio |
| **Dockerfile path** | `Dockerfile` |
| **Docker Command** | `/app/polymarket-weather-bot` |

### Plano:
- Selecione **"Free"** (primeira vez)
- Depois pode atualizar para **Paid** se precisar de uptime 100%

---

## 🔐 Passo 5: Adicionar Variáveis de Ambiente

**MUITO IMPORTANTE!** Antes de fazer deploy, configure as variáveis:

1. Clique em **"Environment"** (abaixo da tela de configuração)
2. Clique em **"Add Environment Variable"** e preencha:

```
TELEGRAM_BOT_TOKEN = seu_token_aqui
TELEGRAM_CHAT_ID = seu_chat_id_aqui
POLYMARKET_API_KEY = dummy
PRIVATE_KEY = dummy
RPC_URL = https://polygon-rpc.com
NOAA_API_KEY = dummy
RUST_LOG = info
```

**Onde conseguir cada valor:**

- **TELEGRAM_BOT_TOKEN**: Obtém com @BotFather no Telegram
  ```bash
  # Ou copie desta conversa anterior
  123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
  ```

- **TELEGRAM_CHAT_ID**: Seu Chat ID (número negativo)
  ```bash
  # Se precisar obter novamente
  curl "https://api.telegram.org/botSEU_TOKEN_AQUI/getUpdates" | grep -o '"id":-[0-9]*'
  ```

---

## 🚀 Passo 6: Fazer Deploy

1. Clique em **"Create Web Service"**
2. Render vai começar o build automaticamente
3. Veja os logs em tempo real (deve levar 3-5 minutos)

### Logs esperados:
```
Building Docker image...
Step 1/14 : FROM rust:latest as builder
...
Successfully built ...
Starting service...
[INFO] Polymarket Weather Bot iniciando...
[INFO] Bot configurado para monitorar: ["KLGA", "NZWN", "SAEZ", "KATL", "EGLL"]
```

✅ Quando ver `Service started`, seu bot está rodando!

---

## ✨ Passo 7: Testar o Bot

1. Acesse seu grupo no Telegram
2. Espere até 5 minutos (primeiro ciclo)
3. Você deve receber a primeira mensagem:
   ```
   ✈️ METAR - KLGA
   🌡️ Temperatura: 15°C
   ...
   ```

---

## 📊 Passo 8: Monitorar (Opcional)

No dashboard do Render:

### Logs em Tempo Real:
- Clique em **"Logs"** para ver o que está acontecendo
- Procure por erros ou mensagens importantes

### Métricas:
- **CPU**: Deve estar bem baixo (< 10%)
- **Memory**: Entre 50-150MB
- **Status**: "Live" = tudo OK

---

## 🔄 Auto-Reload (Importante!)

Render **automaticamente**:
- ✅ Faz deploy de novos commits
- ✅ Reinicia o bot se ele cair
- ✅ Roda 24/7
- ✅ Cancela builds antigos

---

## ⚠️ Limitações do Plano Grátis

| Limitação | Detalhes |
|-----------|----------|
| **Uptime** | 99% (pode cair 1 vez/mês por 30 min) |
| **Hibernação** | Serviço dorme após 15 min sem requisição HTTP (seu bot ativa a cada 5 min, então OK) |
| **Memória** | 512 MB (mais que suficiente) |

### Se precisar de 99.99% uptime:
- Upgrade para **Railway** ou plano pago do Render

---

## 🆘 Troubleshooting

### Problema: Build falha
```
error: could not find Cargo.toml
```
**Solução**: Certifique-se que fez commit de todos os arquivos
```bash
git status
git add -A
git commit -m "Arquivos faltando"
git push
```

### Problema: Serviço fica "Deploy in Progress"
**Solução**: Pode estar compilando. Aguarde 5-10 minutos. Rust compila devagar.

### Problema: Bot não envia mensagens
**Solução**:
1. Verifique logs no Render (aba "Logs")
2. Confira se TELEGRAM_BOT_TOKEN está correto
3. Verifique se TELEGRAM_CHAT_ID é negativo

### Problema: "invalid deployment error"
**Solução**: Provavelmente erro no `Cargo.toml`. Verifique:
```bash
cargo check
cargo build
```

---

## 🔄 Atualizar o Bot (Depois)

Para atualizar depois:
1. Faça mudanças no código localmente
2. Teste com `./run.sh`
3. Commit e push:
   ```bash
   git add .
   git commit -m "Melhorias XYZ"
   git push
   ```
4. Render detecta automaticamente e faz novo deploy!

---

## 📈 Próximos Passos (Opcional)

- Upgrade para plano pago se precisar de uptime 100%
- Adicionar mais APIs de monitoramento
- Criar alertas mais customizados

---

## ✅ Checklist Final

- [ ] Token do Telegram (com @BotFather)
- [ ] Chat ID do grupo
- [ ] Repositório com commits pushed
- [ ] Variáveis de ambiente configuradas no Render
- [ ] Deploy completo e bot rodando
- [ ] Primeira mensagem recebida no Telegram

---

🎉 **Pronto! Seu bot está rodando 24/7 no Render!**

Qualquer dúvida, cheque os logs em tempo real no dashboard ou volta aqui.
