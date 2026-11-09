# VM Technische Befunde (nicht sicherheitskritisch, aber nachverfolgt)

Stand: 2026-11-09 · Basis `f4b0d99`
Diese Liste sammelt technische Schulden, die kein Sicherheitsblocker sind und
deshalb nicht in `VM-SECURITY-BLOCKERS.md` stehen. Kein Eintrag rechtfertigt eine
Aenderung an Ausfuehrungsverhalten.

## TF-01 — Doppelte Modulbaeume: vier statt zwei Dead-Code-Warnungen im Bin-Ziel

**Status:** OFFEN · **Prioritaet:** P3 · **Ausloeser:** Paket C1

**Ist:** `src/main.rs` deklariert `mod artifact; mod assembler; mod vm; mod ops;
mod context;` und kompiliert diese Module ein zweites Mal ein. Der bin-Target
sieht damit eine eigene Kopie der Bibliothek und meldet toten Code fuer alles,
was nur die Bibliothek braucht. Nach C1 sind es vier Warnungen statt zwei:
`GAS_TABLE_ID` und `GAS_TABLE_V1` kommen hinzu, neben `caller` und
`execute_state_transition`.

**Einordnung:** Kein Fehler der Gastabelle. Die Konstanten sind in der
Bibliothek oeffentlich und genutzt; die Warnung entsteht ausschliesslich durch
den zweiten Modulbaum. Die Struktur ist aelter als C1.

**Risiko bei einer Bereinigung:** Wer die Warnungen beseitigt, indem er Module
umbaut oder `#[allow(dead_code)]` streut, kann versehentlich den Runner oder die
oeffentliche Ausfuehrungs-API veraendern. Beides ist heute nicht durch Tests
abgedeckt, die den Runner als Ganzes pruefen.

**Empfehlung:** Eigener Schritt, nicht Teil eines Sicherheitspakets. Optionen:
(a) der bin nutzt die Bibliothek (`use atc_vm::...`) statt eigener `mod`-Baeume;
(b) die ungenutzten Eintraege werden tatsaechlich verwendet; (c) bewusste,
begruendete `allow`-Attribute statt stiller Duldung. Vorher einen Rauchtest fuer
den Runner, der Exit-Status und Ausgabe prueft.

## TF-02 — Doppelte Kostenrepraesentation (gas_cost und GAS_TABLE_V1)

**Status:** BEWUSST AKZEPTIERT (C1) · **Prioritaet:** P3
· **Owner-Entscheidung:** Umstellung nicht Teil von C1

**Ist:** `gas_cost` ist die ausfuehrende Quelle mit erschoepfendem `match`
(compilerseitige Vollstaendigkeit), `GAS_TABLE_V1` ist der benannte,
unveraenderliche Spiegel. Ein Test haelt beide in beide Richtungen deckungsgleich.

**Warum nicht sofort vereinfacht:** Eine reine Lookup-Tabelle verliert die
compilerseitige Vollstaendigkeit aus G-01; ein neuer Befehl ohne Eintrag wuerde
still zu einem Laufzeitproblem statt zu einem Build-Fehler.

**Moeglicher Weg, spaeter:** Befehlscode als `const fn` auf einen Index abbilden
(erschoepfend, compilergeprueft) und die Kosten aus einem `const`-Array lesen;
damit gibt es genau eine Pflegestelle und die Vollstaendigkeit bleibt erzwungen.
Das ist eine eigene Aenderung mit eigener Abnahme.

**Schutz bis dahin:** Der Konsistenztest ist zwingend und darf nicht entfallen.
