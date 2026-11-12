# P1b — Node-Importintegration: Entwurf ohne Gegenkante

> **Status:** `DESIGN` (Entwurf, keine Umsetzung). **Datum:** 2026-11-12.
> **Bedingung (Owner):** Die Abhängigkeitsrichtung darf nicht durch einen
> Zyklus beschädigt werden. **Nicht** Teil von P1b: eine zweite Payload-Prüfung.

## 1 Ausgangslage

`atc-system` hängt bereits von `atc-node` ab (`components/system/Cargo.toml`).
Eine Kante `atc-node -> atc-system` wäre ein Zyklus. P1a hat die Prüfung deshalb
in `atc-system` gelegt und die Verdrahtung offen gelassen.

## 2 Optionen für die Grenze

| Option | Form | Bewertung |
|---|---|---|
| A | Trait in `atc-node` (`ContainerValidator`), Implementierung in `atc-system`, Injektion beim Start | keine neue Kante, Richtung bleibt; der Knoten kennt nur den Vertrag, nicht die Umsetzung |
| B | Neues kleines Crate `atc-payload-boundary` unter beiden | sauber, aber ein drittes Crate für eine Funktion, die schon existiert; mehr Pflegefläche |
| C | Prüfung in `atc-node` duplizieren | verboten: zweite Wahrheit, zweite Dekodierung |

**Empfehlung: A.** Der Knoten bekommt eine Port-Definition, keinen Import.

## 3 Entwurf A

```rust
// components/node/src/import.rs (Entwurf)
pub trait ContainerValidator {
    /// Prüft eine Block-Payload; Fehleridentitaet bleibt beim Aufrufer sichtbar.
    fn validate_block_payload(&self, payload: &str) -> Result<(), String>;
}
```

* `Chain::import_block(&mut self, block: Block, v: &dyn ContainerValidator)`:
  1. Blockhash prüfen (`Block::compute_hash`, unverändert),
  2. `v.validate_block_payload(&block.payload)`,
  3. erst danach anhängen; bei Ablehnung bleibt der Chain-State **unverändert**
     (kein Teilzustand, keine Höhenänderung).
* `atc-system` implementiert den Trait über `block_payload::validate_payload`
  (eine Dekodierung, `System::check_container`), optional `admit_payload` für
  die Freigabe im selben Schritt.
* Der Devnet-Start verdrahtet die Implementierung; ein Knoten ohne Validator
  (**fail-closed**) lehnt ausführbare Payloads ab, statt sie durchzulassen.

## 4 Testmatrix (vor Umsetzung festgelegt)

| Fall | Erwartung |
|---|---|
| Fremdblock mit `atce:`-Payload | angenommen, `code_hash` identisch zum lokalen Deployment |
| Fremdblock mit `atcb:`-Payload | abgelehnt `P-002`, Höhe und letzter Blockhash unverändert |
| Fremdblock mit manipulierter Hülle | abgelehnt `E-006`/`E-007`, kein Zustand |
| Fremdblock mit falschem Blockhash | abgelehnt vor jeder Payload-Prüfung |
| kein Validator verdrahtet | ausführbare Payload abgelehnt (fail-closed) |
| Datenpayload (Zustandswurzel) | unverändert angenommen |

## 5 Reihenfolge

1. Trait und Import-Signatur in `atc-node` (ohne Verhaltensänderung an
   bestehenden Blöcken; Blockhash-Formel unberührt).
2. Implementierung in `atc-system`, Injektion im Devnet-Start.
3. Testmatrix, dann `IMPLEMENTED`/`TESTED`; `CI_VERIFIED` erst mit Actions-Lauf
   auf dem Default-Branch-HEAD.
