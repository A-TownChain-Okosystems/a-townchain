//! MAINT-000 Par.7.2 Readiness-Gate gegen die ECHTEN Records.
//! Aufruf: ATC_READINESS_DIR=<pfad> cargo run -p atc-maintenance --example readiness_gate
//! Exit 0 = alles PASS · Exit 1 = mindestens ein BLOCK (Evidenz auf stdout).

use atc_maintenance::{evaluate_coverage, evaluate_record, ReadinessRecord};
use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(std::env::var("ATC_READINESS_DIR").expect("ATC_READINESS_DIR setzen"));
    // Live-Release-Komponenten (Org-Audit 14.09., SCR-0123 Welle 1)
    let live = ["atc-standards", "a-townchain-os"];

    let mut records = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("readiness dir lesbar")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "yaml").unwrap_or(false))
        .collect();
    files.sort(); // deterministisch

    let mut blocked = 0;
    for f in &files {
        let component = f.file_stem().unwrap().to_string_lossy().to_string();
        let rec =
            ReadinessRecord::load(&component, f).unwrap_or_else(|e| panic!("{component}: {e}"));
        let outcome = evaluate_record(&rec);
        println!(
            "[{}] {component}: {outcome}",
            if outcome.is_pass() { "PASS" } else { "BLOCK" }
        );
        if !outcome.is_pass() {
            blocked += 1;
        }
        records.push(rec);
    }

    let coverage = evaluate_coverage(&records, &live);
    println!("[COVERAGE] {coverage}");
    let ok = blocked == 0 && coverage.is_pass();
    println!(
        "{}",
        if ok {
            "GATE: PASS — Release freigegeben"
        } else {
            "GATE: BLOCK — naechstes Release gesperrt (MAINT-000 Par.7.2)"
        }
    );
    std::process::exit(if ok { 0 } else { 1 });
}
