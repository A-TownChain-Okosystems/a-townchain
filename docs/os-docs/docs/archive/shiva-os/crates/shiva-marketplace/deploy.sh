#!/bin/bash
echo "💰 Marketplace Deployment..."
cargo build --target wasm32-unknown-unknown --release
cargo run --bin shiva-deploy-agent -- \
    --wasm target/wasm32-unknown-unknown/release/shiva_marketplace.wasm \
    --stake 30 --price 2 --service marketplace \
    --description "Bietet Dienste an, handelt Preise aus, schließt Verträge ab"
