#!/bin/bash
echo "🔒 Security Deployment..."
cargo build --target wasm32-unknown-unknown --release
cargo run --bin shiva-deploy-agent -- \
    --wasm target/wasm32-unknown-unknown/release/shiva_security.wasm \
    --stake 25 --price 8 --service security \
    --description "Überwacht Systeme auf Bedrohungen, erkennt Anomalien"
