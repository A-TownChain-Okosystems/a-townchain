#!/bin/bash
echo "📊 DataAnalyst Deployment..."
cargo build --target wasm32-unknown-unknown --release
cargo run --bin shiva-deploy-agent -- \
    --wasm target/wasm32-unknown-unknown/release/shiva_data_analyst.wasm \
    --stake 15 --price 3 --service data-analysis \
    --description "Analysiert Daten, erstellt Berichte, erkennt Muster"
