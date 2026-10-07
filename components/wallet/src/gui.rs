// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Wallet UI rendering primitives. The host application owns the egui frame.

pub struct WalletUiState {
    pub address: String,
    pub available: u128,
    pub locked: u128,
    pub status: String,
}

impl Default for WalletUiState {
    fn default() -> Self {
        Self { address: String::new(), available: 0, locked: 0, status: "Ready".into() }
    }
}

pub fn render(ui: &mut egui::Ui, state: &WalletUiState) {
    ui.heading("A-TownChain Wallet");
    ui.label(format!("Address: {}", state.address));
    ui.label(format!("Available: {}", state.available));
    ui.label(format!("Locked: {}", state.locked));
    ui.label(format!("Status: {}", state.status));
}
