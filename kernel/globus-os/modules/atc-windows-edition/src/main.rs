// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ATC Windows Edition -- Sprint 0 Platzhalter
// Rust mit std, GUI via eframe/egui. eframe 0.36-API (run_native, SCR-0078).

use eframe::egui;

#[derive(Default)]
struct Sprint0App;

impl eframe::App for Sprint0App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("ATC Windows Edition -- Sprint 0 Platzhalter");
        ui.label("Naechste View folgt: Wallet / Explorer / Dashboard (siehe README.md).");
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ATC Windows Edition",
        options,
        Box::new(|_cc| Ok(Box::new(Sprint0App))),
    )
}
