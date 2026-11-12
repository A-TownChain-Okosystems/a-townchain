// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! E1 (Audit 2026-11-12, F1), Abnahmekriterium 3: `atc run` darf fuer einen
//! Vertrag keinen falschen Exit 0 liefern. Vor dem Fix gab `atc run token.atc`
//! `0` aus und endete mit 0, obwohl der Vertrag verworfen wurde.

use std::path::Path;
use std::process::Command;

#[test]
fn atc_run_lehnt_vertrag_mit_fehlerstatus_ab() {
    let exe = env!("CARGO_BIN_EXE_atc");
    let token = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../apps/atc-contracts/contracts/token.atc");

    let out = Command::new(exe).arg("run").arg(&token).output().unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "atc run darf fuer einen Vertrag nicht erfolgreich sein; stdout={:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "Kompilierfehler ist Exit 1, nicht {}",
        out.status.code().unwrap_or(-1)
    );
    assert!(
        out.stdout.is_empty(),
        "kein Ergebniswert auf stdout, war {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        stderr.contains("nicht ausfuehrbar") && stderr.contains("FranchiseToken"),
        "Fehler nennt Grund und Vertrag: {stderr}"
    );
}

#[test]
fn atc_run_bleibt_fuer_programme_erfolgreich() {
    let exe = env!("CARGO_BIN_EXE_atc");
    let dir = std::env::temp_dir().join(format!("atc-e1-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("main42.atc");
    std::fs::write(&file, "fn main() -> i64 { return 42; }\n").unwrap();

    let out = Command::new(exe).arg("run").arg(&file).output().unwrap();
    assert!(
        out.status.success(),
        "Programm ohne Vertrag muss laufen: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "42");
    let _ = std::fs::remove_dir_all(&dir);
}
