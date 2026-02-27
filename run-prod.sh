#!/bin/bash

# Script para rodar a versão production-ready do Polymarket Weather Bot

set -a
source .env
set +a

RUST_LOG=${RUST_LOG:-info} ./target/release/polymarket-weather-bot "$@"
