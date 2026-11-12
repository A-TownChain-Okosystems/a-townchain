# P1b — Node-Import: Trait-Position anhand des echten Abhängigkeitsgraphen

> **Status:** `DESIGN` (Entwurf, keine Umsetzung). **Datum:** 2026-11-12.
> **Anlass:** Design-Review des Owners. **Nicht** implementiert, solange die
> Trait-Position nicht entschieden ist. **Nicht** Teil von P1b: eine zweite
> Payload-Prüfung, eine eigene ATCE/ATCB-Dekodierung im Knoten.

## 1 Befund am Graphen (gemessen, nicht angenommen)

`cargo metadata --no-deps` im Workspace `a-townchain`:

| Crate | hängt ab von |
|---|---|
| `atc-node` | `atc-algorithm`, `atc_wallet` |
| `atc-system` | `atc-core`, `atc-exec-admission`, `atc-node`, `atc-vm`, `atc_wallet` |
| `atc_cli` | `atc-node` |

Damit gilt: **`atc-system -> atc-node` existiert bereits.**

## 2 Korrektur einer Annahme im Review

Im Review stand, ein Trait in `atc-node`, den `atc-system` implementiert, erzeuge
einen Zyklus, weil `atc-system` dafür `atc-node` kennen müsste. `atc-system`
kennt `atc-node` bereits (Tabelle oben), und ein Trait wird in der Kiste
**implementiert, die ihn sieht** — das ist keine neue Kante. Es entsteht also
**kein** Zyklus: die Kante zeigt weiter `system -> node`.

Der Zyklus entstünde in der **umgekehrten** Richtung: Trait in `atc-system`, den
`atc-node` konsumiert, verlangt `node -> system`. Genau diese Variante ist die im
Review bevorzugte Variante A — sie ist mit dem heutigen Graphen **nicht** baubar.

## 3 Die drei Varianten, gegen den Graphen geprüft

| Variante | Aufbau | Kante | Bewertung |
|---|---|---|---|
| A | Trait in `atc-system`, `atc-node` konsumiert | `node -> system` (neu) | **nicht möglich**: Zyklus mit `system -> node` |
| B | Trait in einem neutral vorhandenen Contract-Crate, beide konsumieren | zwei bestehende Kanten | Durchschnitt der Abhängigkeiten von `node` und `system` ist `{atc-algorithm, atc_wallet}`. Beide sind fachlich nicht der Ort für eine Container-/Payload-Grenze (`atc_wallet` = Tx/Signatur nach WAL-SIGN-001, `atc-algorithm` = Hash-/PoH-Bausteine). Ein **neues** Crate nur für diesen Port ist durch die Review-Vorgabe ausgeschlossen. |
| C | Port (Trait) in `atc-node`, Implementierung in `atc-system`, Injektion beim Start | keine neue Kante | **empfohlen**: Richtung bleibt, Abhängigkeitsumkehr (Ports & Adapters), Knoten kennt nur den Vertrag |

Damit ist die im Review als „bevorzugt, sofern fachlich passend" genannte
Variante A am Graphen ausgeschlossen und Variante B ohne passendes Crate; übrig
bleibt C — der ursprüngliche Vorschlag, jetzt mit Beleg statt Behauptung.

## 4 Entwurf (Variante C)

```rust
// components/node/src/import.rs (Entwurf, keine Implementierung)
pub struct PayloadRejection {
    pub stage: &'static str,   // "block_payload" | "container" | "admission"
    pub code: &'static str,    // "P-002", "E-007", ...
    pub offset: Option<usize>,
}

pub trait ContainerValidator {
    fn validate_block_payload(&self, payload: &str) -> Result<(), PayloadRejection>;
}

impl Chain {
    /// Prüft Hash, dann Payload über den Validator, dann erst anhängen.
    pub fn import_block(&mut self, block: Block, v: &dyn ContainerValidator)
        -> Result<(), ImportError>;
}
```

* Der Trait nennt **kein** ATCE, ATCB oder Opcode: der Knoten kennt nur die
  Frage „darf diese Payload als ausführbarer Code angenommen werden?".
* `atc-system` implementiert ihn über `block_payload::validate_payload`
  (eine Dekodierung, `System::check_container`); optional `admit_payload` für die
  Freigabe im selben Schritt.
* Kein Validator verdrahtet: **expliziter Fehler** `ImportError::NoValidator` für
  ausführbare Payloads, kein stiller Durchlauf. Kein Startabbruch.
* `atc_cli` hängt an `atc-node` und kann die Implementierung aus `atc-system`
  einsetzen (die Kante existiert dort ohnehin).

## 5 Invarianten des Owners, je mit Umsetzungsort

| Invariante | Ort |
|---|---|
| `atc-node` implementiert keine ATCE-/ATCB-Dekodierung | Trait enthält keine Formatbegriffe; Dekodierung nur in `atc-vm`/`atc-system` |
| `atc-system` bleibt Eigentümer von Validierung und Admission | Implementierung des Traits ruft `validate_payload`/`admit_payload` |
| Import prüft vor der Blockannahme | `Chain::import_block`: Hash, Payload, dann Anhängen |
| Ohne Validator kein Fremdblock | `ImportError::NoValidator`, fail-closed |
| Fehlender Validator = expliziter Fehler | eigener Fehlervariante, kein `Option::unwrap_or` |
| Derselbe `code_hash` unabhängig vom Pfad | Test: Import gegen `deploy_container` |
| Abgelehnter Block ändert weder Chain-State noch Registry | Prüfung vor jeder Mutation; Test vergleicht Höhe, Best-Hash und Registry |
| Keine zyklische Crate-Abhängigkeit | Trait liegt in `atc-node`, die Kante `system -> node` bleibt die einzige |

## 6 Testmatrix (vor Umsetzung festgelegt)

| Fall | Erwartung |
|---|---|
| Fremdblock mit `atce:`-Payload | angenommen, `code_hash` identisch zum lokalen Deployment |
| Fremdblock mit `atcb:`-Payload | abgelehnt `P-002`, Höhe/Best-Hash unverändert |
| Fremdblock mit manipulierter Hülle | abgelehnt `E-006`/`E-007`, kein Zustand, keine Registry-Änderung |
| Fremdblock mit falschem Blockhash | abgelehnt vor der Payload-Prüfung |
| kein Validator verdrahtet | `NoValidator`, Block nicht angenommen |
| Datenpayload (Zustandswurzel) | unverändert angenommen |
| zwei Importe desselben Blocks | zweiter Import ohne Zustandsänderung (idempotent oder abgelehnt, festzulegen) |

## 7 Offene Entscheidung

Variante C ist am Graphen die einzige, die alle Invarianten erfüllt. Soll der
Port dennoch nicht in `atc-node` liegen, bleibt nur, den Knoten von
`atc-system` zu entkoppeln (Validierung in ein eigenes Crate ziehen). Das wäre
ein eigener Architekturschnitt mit eigenem Nachweis, nicht Teil von P1b.
