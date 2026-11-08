// ATC Windows Edition -- Sprint 1: minimales GUI-Grundgeruest
// Rust mit std, GUI via eframe/egui. Ziel: x86_64-pc-windows-msvc.
//
// Naechste Views (Kandidaten): Wallet, Explorer, Dashboard -- siehe README.md.

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("ATC Windows Edition", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ATC Windows Edition -- Sprint 1 Platzhalter");
            ui.label("Naechste View folgt: Wallet / Explorer / Dashboard (siehe README.md).");
        });
    })
}
