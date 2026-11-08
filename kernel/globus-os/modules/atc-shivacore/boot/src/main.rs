// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore Boot-Image-Builder.
// Nimmt den fertig gebauten ShivaCore-Kernel-ELF (Argument 1) und erzeugt
// daraus bootfähige BIOS- und UEFI-Disk-Images per `bootloader` 0.11.
// Übernimmt NUR das reine Boot-Protokoll — kein Betriebssystem-Code, kein Linux.

use std::path::PathBuf;

fn main() {
    let kernel_path = std::env::args()
        .nth(1)
        .expect("Nutzung: boot <pfad-zum-shivacore-kernel-elf> <ausgabe-verzeichnis>");
    let out_dir = std::env::args()
        .nth(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    std::fs::create_dir_all(&out_dir).unwrap();

    let bios_path = out_dir.join("shivacore-bios.img");
    bootloader::BiosBoot::new(&PathBuf::from(&kernel_path))
        .create_disk_image(&bios_path)
        .expect("BIOS-Image-Erstellung fehlgeschlagen");
    println!("BIOS-Image: {}", bios_path.display());

    let uefi_path = out_dir.join("shivacore-uefi.img");
    bootloader::UefiBoot::new(&PathBuf::from(&kernel_path))
        .create_disk_image(&uefi_path)
        .expect("UEFI-Image-Erstellung fehlgeschlagen");
    println!("UEFI-Image: {}", uefi_path.display());
}
