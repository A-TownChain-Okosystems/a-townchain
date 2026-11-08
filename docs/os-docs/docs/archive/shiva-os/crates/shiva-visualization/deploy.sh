#!/bin/bash
echo "📊 Visualization Deployment..."
cargo build --target wasm32-unknown-unknown --release
cargo run --bin shiva-deploy-agent -- \
    --wasm target/wasm32-unknown-unknown/release/shiva_visualization.wasm \
    --stake 10 --price 2 --service visualization \
    --description "Erstellt Diagramme, Grafiken und Dashboards"
