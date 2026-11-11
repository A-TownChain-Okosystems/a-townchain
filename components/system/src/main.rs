// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! CLI des zusammengesetzten Systems.
//!
//!   atc-system demo                — feste Demo-Quelle, vollstaendiger Durchlauf
//!   atc-system run <datei.atc>     — eigene Quelle, vollstaendiger Durchlauf
//!   --gas <budget>                 — Gasbudget (Standard 50 000)
//!
//! Die CLI ist ein **Bedienwerkzeug**, kein Konsenspfad: sie baut ein
//! Devnet-System im Speicher auf und zeigt jede Stufe mit ihren Belegen.

use atc_system::system::DEMO_SOURCE;
use atc_system::{hex, ContractState, Deployment, Receipt, System, DEFAULT_GAS};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(command) = args.first().map(String::as_str) else {
        eprintln!("Aufruf: atc-system demo | run <datei.atc> [--gas N]");
        return ExitCode::from(2);
    };

    let gas = args
        .iter()
        .position(|a| a == "--gas")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DEFAULT_GAS);

    let source = match command {
        "demo" => DEMO_SOURCE.to_string(),
        "run" => {
            let Some(path) = args.get(1) else {
                eprintln!("run braucht einen Dateipfad");
                return ExitCode::from(2);
            };
            match std::fs::read_to_string(path) {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("lesen {path}: {error}");
                    return ExitCode::from(1);
                }
            }
        }
        other => {
            eprintln!("unbekannter Befehl: {other}");
            return ExitCode::from(2);
        }
    };

    match run(&source, gas) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ABGEBROCHEN auf Stufe {}: {}", error.stage(), error);
            ExitCode::from(1)
        }
    }
}

fn run(source: &str, gas: u64) -> Result<(), atc_system::SystemError> {
    // Feste Uhr: der Durchlauf soll reproduzierbar sein, nicht "jetzt".
    const NOW_US: i64 = 1_800_000_000_000_000;

    let mut system = System::devnet()?;
    println!("Genesis      : {}", &system.genesis().genesis_id[..16]);
    println!(
        "Netz         : {} / {} (Protokoll {}, VM {})",
        system.genesis().chain_id,
        system.genesis().network_id,
        system.genesis().protocol_version,
        system.genesis().vm_version
    );
    println!("Freigabe     : {}", system.seal_address());
    println!("Gas          : {gas}");

    let deployment = system.deploy(source, NOW_US)?;
    print_deployment(&deployment);

    let mut state = ContractState::new();
    let receipt = system.execute(&deployment, &mut state, gas, NOW_US, 1)?;
    print_receipt(&receipt);

    // Zweite Ausfuehrung auf dem fortgeschriebenen Zustand: beweist, dass der
    // Zustand tatsaechlich ueber die Kette weitergegeben wird.
    let second = system.execute(&deployment, &mut state, gas, NOW_US, 1)?;
    println!(
        "Wiederholung : Ergebnis {:?}, Zustand vorher {}, nachher {}",
        second.result(),
        &hex(&second.state_root_before)[..16],
        &hex(&second.state_root_after)[..16]
    );

    system
        .chain()
        .verify()
        .map_err(atc_system::SystemError::Block)?;
    println!(
        "Kette        : Hoehe {}, {} Bloecke, Verifikation ok",
        system.chain().height(),
        system.chain().blocks().len()
    );
    println!("Freigaben    : {} Audit-Eintraege", system.audit_count());
    Ok(())
}

fn print_deployment(deployment: &Deployment) {
    println!(
        "Kompilat     : {} ATCLang-Instruktionen -> {} ATCB-Operationen",
        deployment.source_instructions, deployment.ops
    );
    println!("Artefakt     : {} Bytes", deployment.artifact_bytes.len());
    println!("Code-Hash    : {}", deployment.code_hash_hex());
    println!("Artefakt-Hash: {}", deployment.artifact_hash_hex());
    println!(
        "Freigabe     : Version {}, Rechte {:02b}",
        deployment.policy.version, deployment.policy.requested_capabilities
    );
    for divergence in deployment.divergences {
        println!("Offen        : {divergence}");
    }
}

fn print_receipt(receipt: &Receipt) {
    println!("Ergebnis     : {:?}", receipt.result());
    println!(
        "Zustand      : {} -> {}",
        &hex(&receipt.state_root_before)[..16],
        &hex(&receipt.state_root_after)[..16]
    );
    println!(
        "Block        : Hoehe {} ({})",
        receipt.block_height,
        &hex(&receipt.block_hash)[..16]
    );
    println!("Audit        : {} Eintraege", receipt.audit_entries);
}
