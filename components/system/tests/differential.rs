// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Gate A — Compiler und VM fuehren dieselbe Semantik aus.
//!
//! Verglichen werden **zwei unabhaengige Ausfuehrungswege derselben Quelle**:
//! der Referenz-Interpreter in `atc-core` (ueber das verifizierte ATCA-Artefakt)
//! und der Kettenpfad ueber Uebersetzer, ATCB-Artefakt, Freigabe und `atc-vm`.
//!
//! WICHTIGE EINSCHRAENKUNG, die den Vergleich erst aussagekraeftig macht:
//! Ein Programm **ohne** eigenes `return` (Top-Level-Form `let r = ...;`) liefert
//! im Referenzweg die implizite Rueckgabe der Einstiegsfunktion `__main__`, und
//! die ist konstant 0. Ein Vergleich solcher Programme beweist nichts. Die
//! tragenden Tests hier erwarten deshalb entweder einen von Hand gerechneten
//! Wert oder ein ausdrueckliches `return`.
//!
//! Was **nicht** verglichen werden kann, ist der Speicher: der Referenz-
//! Interpreter kennt nur Rahmen-lokale Variablen und gibt sie nicht heraus, die
//! VM kennt nur Speicherplaetze. Dass Lokale auf Speicherplaetze abgebildet
//! werden, ist eine Festlegung des Backends und wird innerhalb des Kettenpfads
//! geprueft (`zustand_wird_gelesen`).

use atc_core::artifact::ArtifactMetadata;
use atc_system::{compile, translate, ContractState, System};

const NOW_US: i64 = 1_800_000_000_000_000;
const GAS: u64 = 200_000;

/// Referenzweg: atc-core-Interpreter ueber ein verifiziertes ATCA-Artefakt.
fn referenz(source: &str) -> Result<i64, String> {
    let compiled = compile(source).map_err(|e| e.message())?;
    let metadata = ArtifactMetadata {
        language_version: "atclang-0.1.0".into(),
        compiler_version: "atc-core-0.1.0".into(),
        target_profile: "atc-vm".into(),
        capabilities: vec![],
    };
    let artifact = atc_core::artifact::Artifact::from_program(&compiled, metadata)
        .map_err(|e| format!("{e:?}"))?;
    let verified = artifact.verify().map_err(|e| format!("{e:?}"))?;
    atc_core::vm::execute_verified(&verified).map_err(|e| format!("{e:?}"))
}

/// Kettenweg: Quelle -> Backend -> ATCB -> Freigabe -> VM -> Block.
fn kettenweg(source: &str) -> Result<u64, String> {
    kettenweg_mit_zustand(source, Vec::new())
}

fn kettenweg_mit_zustand(source: &str, seed: Vec<u64>) -> Result<u64, String> {
    let mut system = System::devnet().map_err(|e| e.to_string())?;
    let deployment = system.deploy(source, NOW_US).map_err(|e| e.to_string())?;
    let mut state = ContractState::from_words(seed);
    let receipt = system
        .execute(&deployment, &mut state, GAS, NOW_US, 1)
        .map_err(|e| e.to_string())?;
    receipt.result().ok_or_else(|| "leerer Stapel".to_string())
}

/// Beide Wege muessen **denselben** von Hand gerechneten Wert liefern.
fn gleich_mit_erwartung(source: &str, erwartet: i64) {
    let reference =
        referenz(source).unwrap_or_else(|e| panic!("Referenz scheiterte: {e}\n{source}"));
    assert_eq!(
        reference, erwartet,
        "Referenz weicht von der Erwartung ab: {source}"
    );
    let chain = kettenweg(source).unwrap_or_else(|e| panic!("Kettenweg scheiterte: {e}\n{source}"));
    assert_eq!(chain, erwartet as u64, "Kettenweg weicht ab: {source}");
}

#[test]
fn arithmetik_mit_praezedenz_stimmt() {
    gleich_mit_erwartung(
        "fn main() -> i64 { let r = 1 + 2 * 3 - 4 / 5; return r; }",
        7,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 7; let b = 3; let r = a * b - a / b; return r; }",
        19,
    );
    gleich_mit_erwartung("fn main() -> i64 { return (1 + 2) * (3 + 4); }", 21);
    gleich_mit_erwartung("fn main() -> i64 { return 100 / 7; }", 14);
}

#[test]
fn schleife_stimmt() {
    gleich_mit_erwartung(
        "fn main() -> i64 { let i = 1; let f = 1; while i < 6 { let f = f * i; let i = i + 1; } return f; }",
        120,
    );
    // Schleife, die nie laeuft.
    gleich_mit_erwartung(
        "fn main() -> i64 { let i = 9; let s = 0; while i < 5 { let s = s + 1; let i = i + 1; } return s; }",
        0,
    );
}

#[test]
fn alle_sechs_vergleiche_stimmen() {
    // Die vier erweiterten Vergleiche (>, <=, >=, !=) sind die Stellen, an denen
    // das Backend mehrere Operationen erzeugt statt einer.
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 9; return a < b; }",
        1,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 9; return a > b; }",
        0,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 9; let b = 4; return a > b; }",
        1,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 9; return a <= b; }",
        1,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 9; let b = 4; return a <= b; }",
        0,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 9; let b = 4; return a >= b; }",
        1,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 9; return a >= b; }",
        0,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 4; return a == b; }",
        1,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 4; return a != b; }",
        0,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let a = 4; let b = 9; return a != b; }",
        1,
    );
}

#[test]
fn bedingungen_stimmen() {
    gleich_mit_erwartung(
        "fn main() -> i64 { let x = 7; if x < 5 { return 1; } else { return 2; } }",
        2,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let x = 7; if x < 5 { return 1; } else { if x < 9 { return 2; } else { return 3; } } }",
        2,
    );
    gleich_mit_erwartung(
        "fn main() -> i64 { let x = 2; if x < 5 { return 1; } else { return 2; } }",
        1,
    );
}

#[test]
fn parameter_starten_in_beiden_wegen_bei_null() {
    // Ohne Aufrufkonvention gibt es keine Argumente: beide Wege starten mit 0.
    // Der verkettete Vergleich folgt der Referenz (SSOT), auch wenn das
    // Ergebnis ueberrascht: 0 ist hier die richtige Antwort, nicht 1.
    let source = "fn f(a: i64, b: i64) -> i64 { return a + 1 <= b == 0; }";
    let reference = referenz(source).unwrap();
    assert_eq!(reference, 0);
    assert_eq!(kettenweg(source).unwrap(), reference as u64);
}

#[test]
fn zustand_wird_gelesen() {
    // `let s = s + 1;` liest Speicherplatz 0, bevor es ihn schreibt. Der
    // Referenzweg startet seine Lokalen immer bei 0, der Kettenpfad nimmt den
    // Zustand der Kette. Beide Wege muessen deshalb identisch sein, wenn der
    // Zustand leer ist, und der Kettenpfad muss dem Zustand folgen.
    let source = "fn main() -> i64 { let s = s + 1; return s; }";
    gleich_mit_erwartung(source, 1);

    let mut system = System::devnet().unwrap();
    let deployment = system.deploy(source, NOW_US).unwrap();
    let mut state = ContractState::from_words(vec![41]);
    let receipt = system
        .execute(&deployment, &mut state, GAS, NOW_US, 1)
        .unwrap();
    assert_eq!(
        receipt.result(),
        Some(42),
        "der Zustand der Kette ist der Ausgangswert"
    );
    assert_eq!(state.words(), &[42]);

    // Zweiter Lauf auf dem fortgeschriebenen Zustand: 43.
    let zweiter = system
        .execute(&deployment, &mut state, GAS, NOW_US, 1)
        .unwrap();
    assert_eq!(zweiter.result(), Some(43));
}

#[test]
fn ueberlauf_ist_eine_bekannte_divergenz() {
    // atc-core rechnet geprueft und lehnt ab, die VM laeuft um.
    // Diese Divergenz ist offen und steht maschinenlesbar in DIVERGENZEN.
    let source = "fn main() -> i64 { return 4611686018427387904 * 4; }";
    let reference = referenz(source);
    assert!(
        reference.is_err(),
        "Referenz muss den Ueberlauf ablehnen, war {reference:?}"
    );
    assert_eq!(
        kettenweg(source).unwrap(),
        0,
        "VM laeuft bei 2^62*4 auf 0 um"
    );
    assert!(atc_system::DIVERGENZEN
        .iter()
        .any(|d| d.contains("ueberlauf")));
}

#[test]
fn negative_zwischenwerte_divergieren_ebenfalls() {
    // 0 - 1 ist in atc-core -1, in der VM 2^64 - 1. Der Vergleich kehrt sich um.
    let source = "fn main() -> i64 { let a = 0 - 1; return a < 1; }";
    assert_eq!(referenz(source).unwrap(), 1);
    assert_eq!(kettenweg(source).unwrap(), 0);
    assert!(atc_system::DIVERGENZEN
        .iter()
        .any(|d| d.contains("vorzeichen")));
}

#[test]
fn ungueltiges_programm_wird_in_beiden_wegen_abgelehnt() {
    use atc_core::bytecode::{Bytecode, Instruction};
    use atc_core::lower::{CompiledFunction, CompiledProgram};

    let broken = CompiledProgram {
        functions: vec![CompiledFunction {
            name: "__main__".into(),
            param_count: 0,
            local_count: 1,
            bytecode: Bytecode {
                instructions: vec![Instruction::ConstI64(1), Instruction::Jump(500)],
            },
        }],
        entry: 0,
    };
    assert!(broken.verify().is_err(), "Referenzweg muss ablehnen");
    assert!(translate(&broken).is_err(), "Kettenweg muss ablehnen");
}

#[test]
fn eingebetteter_korpus_ist_tragend() {
    // Eigene Faelle mit ausdruecklichem `return` und von Hand gerechnetem Wert.
    // Sie belegen, dass der Vergleich wirklich rechnet und nicht nur Nullen
    // gegeneinander haelt.
    let faelle: [(&str, i64); 6] = [
        ("fn main() -> i64 { return 2 + 3 * 4; }", 14),
        ("fn main() -> i64 { let a = 9; let b = 2; return a / b + a * b; }", 22),
        (
            "fn main() -> i64 { let i = 0; let s = 1; while i < 4 { let s = s * 2; let i = i + 1; } return s; }",
            16,
        ),
        (
            "fn main() -> i64 { let x = 5; if x == 5 { return 42; } else { return 0; } }",
            42,
        ),
        ("fn main() -> i64 { let a = 6; let b = 7; return a != b; }", 1),
        ("fn main() -> i64 { let a = 8; let b = 8; return a >= b; }", 1),
    ];
    for (source, erwartet) in faelle {
        assert_ne!(erwartet, 0, "Erwartung muss ein echtes Ergebnis sein");
        gleich_mit_erwartung(source, erwartet);
    }
}

#[test]
fn korpus_sweep_ist_gleich() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../ai/atclang/crates/atc-core/differential/corpus");
    if !dir.exists() {
        eprintln!(
            "Korpus nicht vorhanden, Test uebersprungen: {}",
            dir.display()
        );
        return;
    }
    let (mut verglichen, mut aussagekraeftig, mut abgelehnt) = (0usize, 0usize, 0usize);
    for entry in std::fs::read_dir(&dir).expect("Korpus lesbar") {
        let path = entry.expect("Eintrag lesbar").path();
        if path.extension().and_then(|e| e.to_str()) != Some("atc") {
            continue;
        }
        let source = std::fs::read_to_string(&path).expect("Datei lesbar");
        if compile(&source)
            .map(|c| translate(&c).is_err())
            .unwrap_or(true)
        {
            abgelehnt += 1;
            continue;
        }
        let reference = referenz(&source).expect("Referenz laeuft");
        let chain = kettenweg(&source).expect("Kettenweg laeuft");
        assert_eq!(
            reference as u64,
            chain,
            "Abweichung bei {}",
            path.file_name().unwrap().to_string_lossy()
        );
        verglichen += 1;
        // Top-Level-Programme liefern im Referenzweg die implizite 0; ein
        // Vergleich mit 0 ist deckend, aber nicht tragend.
        if reference != 0 {
            aussagekraeftig += 1;
        }
    }
    eprintln!("Korpus: {verglichen} verglichen ({aussagekraeftig} mit echtem Ergebnis), {abgelehnt} ausserhalb des Subsets");
    assert!(
        verglichen >= 10,
        "zu wenige verglichene Faelle: {verglichen}"
    );
    // BEFUND, nicht Deckmaengel: der ausgelieferte Korpus besteht ueberwiegend
    // aus Top-Level-Programmen. Deren Referenzwert ist die implizite 0 aus
    // `__main__`, ein Vergleich mit 0 belegt also keine Rechenregel. Erwartet
    // wird hier gar nichts; die tragenden Faelle stehen in den Tests oben.
    let _ = aussagekraeftig;
}
