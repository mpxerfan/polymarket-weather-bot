#!/bin/bash

# Script para rodar o Polymarket Weather Bot com variáveis do .env

set -a
source .env
set +a

RUST_LOG=${RUST_LOG:-info} cargo run "$@"
