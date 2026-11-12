// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Differentialtest Stufe 1: die ATCE-Huelle gegen den Parser-Korpus C-1 bis C-8.
//!
//! Die Sollwerte stammen aus der Konstruktion des Korpus
//! (`tools/build_parser_corpus.py`), nicht aus dieser Implementierung. Der Korpus
//! wurde mit dem unabhaengigen Referenzdekoder `tools/atc_reference_decoder.py`
//! erzeugt. Eine Uebereinstimmung belegt, dass **zwei** Implementierungen dieselbe
//! Entscheidung und dieselbe Fehlerposition liefern (REQ-SCE-084).
//!
//! Grenze: gruen ist `TESTED`, nicht `CI_VERIFIED` und nicht `VERIFIED`. Der
//! Differentialtest bleibt bis zur vollstaendigen Integration und Pruefung
//! `BLOCKED` (Owner-Status 12.11.2026).

use atc_vm::envelope::Envelope;

#[path = "stage1_parser_corpus.rs"]
mod korpus;

fn aus_hex(text: &str) -> Vec<u8> {
    assert_eq!(text.len() % 2, 0, "ungerade Hexlaenge");
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("Hex"))
        .collect()
}

#[test]
fn huelle_stimmt_mit_dem_korpus_ueberein() {
    let mut fehler: Vec<String> = Vec::new();

    for (id, hex, entscheidung, code, offset) in korpus::FAELLE {
        let bytes = aus_hex(hex);
        let ergebnis = Envelope::parse(&bytes);

        match (entscheidung, ergebnis) {
            ("accept", Ok(env)) => {
                // Eine angenommene Huelle muss den Umlauf ueberstehen und einen
                // ausfuehrbaren ATCB-Container ergeben.
                let wieder = Envelope::parse(&env.to_bytes());
                if wieder.is_err() {
                    fehler.push(format!("{id}: angenommen, aber to_bytes nicht lesbar"));
                }
                if env.into_artifact().is_err() {
                    fehler.push(format!("{id}: angenommen, aber kein ATCB-Artefakt"));
                }
            }
            ("accept", Err(e)) => fehler.push(format!(
                "{id}: erwartet accept, erhalten {} bei {}",
                e.code(),
                e.offset()
            )),
            ("reject", Ok(_)) => fehler.push(format!(
                "{id}: erwartet {}, erhalten accept",
                code.unwrap_or("?")
            )),
            ("reject", Err(e)) => {
                if Some(e.code()) != code {
                    fehler.push(format!(
                        "{id}: erwartet {}, erhalten {}",
                        code.unwrap_or("?"),
                        e.code()
                    ));
                }
                if Some(e.offset()) != offset {
                    fehler.push(format!(
                        "{id}: erwartet Offset {:?}, erhalten {}",
                        offset,
                        e.offset()
                    ));
                }
            }
            _ => fehler.push(format!("{id}: unbekannte Entscheidung {entscheidung}")),
        }
    }

    if !fehler.is_empty() {
        panic!(
            "{} Abweichung(en) zwischen Huelle und Korpus:\n{}",
            fehler.len(),
            fehler.join("\n")
        );
    }
}

#[test]
fn korpus_deckt_alle_acht_klassen_ab() {
    let klassen: std::collections::BTreeSet<&str> =
        korpus::FAELLE.iter().map(|(id, ..)| &id[..3]).collect();
    assert_eq!(klassen.len(), 8, "Klassen C-1 bis C-8 erwartet");
}

#[test]
fn korpus_hat_mindestens_dreissig_eingaben() {
    assert!(
        korpus::FAELLE.len() >= 30,
        "REQ-SCE-084 verlangt mindestens 30 Eingaben"
    );
}
