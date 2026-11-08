//! Lädt die echte atc-standards-Registry und druckt Evidence-Zählungen.
//! Aufruf: ATC_STANDARDS_REGISTRY_DIR=<pfad> cargo run -p atc-standards --example load_registry

use atc_standards::{RepoRegistry, StandardsRegistry};
use std::path::PathBuf;

fn main() {
    let dir =
        std::env::var("ATC_STANDARDS_REGISTRY_DIR").expect("ATC_STANDARDS_REGISTRY_DIR setzen");
    let dir = PathBuf::from(dir);

    let std_reg = StandardsRegistry::load(&dir.join("standards.yaml"))
        .unwrap_or_else(|e| panic!("standards.yaml: {e}"));
    println!("standards.yaml: {} Einträge", std_reg.count());
    for (status, n) in std_reg.counts_by_status() {
        println!("  {status}: {n}");
    }
    // Spot-Checks gegen bekannte Einträge
    let s = std_reg
        .require_normative_released("ATC-STD-000")
        .unwrap_or_else(|e| panic!("resolve ATC-STD-000: {e}"));
    println!("  ATC-STD-000 -> v{} [{}]", s.version, s.status.as_str());

    let repo_reg = RepoRegistry::load(&dir.join("repositories.yaml"))
        .unwrap_or_else(|e| panic!("repositories.yaml: {e}"));
    println!(
        "repositories.yaml: {} Einträge, davon governed: {}",
        repo_reg.count(),
        repo_reg.governed().count()
    );
    let r = repo_reg
        .resolve_by_name("atc-engineering")
        .unwrap_or_else(|e| panic!("resolve atc-engineering: {e}"));
    println!("  atc-engineering -> {} [{}]", r.id, r.criticality);
}
