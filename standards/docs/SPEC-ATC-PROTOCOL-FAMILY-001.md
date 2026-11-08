---
document:
  id: SPEC-ATC-PROTOCOL-FAMILY-001
  title: "ATC Protocol Specification — Family Draft (Konsolidierte Protokoll-Familien-Spezifikation: Systemdefinition, Nachrichtenmodell, Konsens)"
  version: "0.1.0-DRAFT"
  status: draft
  scr: SCR-0130
  chain_id: 658467
  authority: A-TownChain-Okosystems
  owner: "Alexander (Owner-Entscheidungen OEP-1..OEP-6) / ausarbeitende Instanz Aurora"
  created: "2026-10-06"
  updated: "2026-10-06"
  classification: PUBLIC
  language: de-DE
  umbrella: ATC-STD-PROTOCOL-001
  normative_base:
    - "ATC-PROTO-P2P-001 (§9, v1.0.0, approved)"
    - "ATC-STD-PROTOCOL-001"
  draft_references:
    - "ATC-CONSENSUS-301..307 + DET + ENC (0.1.0-DRAFT, F-067)"
    - "ATC-HASH-001 (TownHash-256)"
    - "Signatur-Welt-Draft (secp256k1/RFC-6979/low-S für TX; Ed25519 ausschließlich P2P/DID)"
  note: 'Dokument-ID, KEIN Registry-Protokoll (SCR-Komposition, keine neue Protokoll-Identitaet). OEP-1..OEP-6 offen — nicht einfrierbar bis Owner-Entscheidungen. Evidence-Gate F-069 bindend.'
---

# ATC Protocol Specification — Family Draft

Dokumentklasse: Protocol Family Specification (SCR-Komposition)
Bezug: ATC-PROTO-P2P-001 §9 (normativ), ATC-CONSENSUS-301..307 + DET + ENC (0.1.0-DRAFT, F-067), TownHash-256, atc-vm
Status: 0.1.0-DRAFT — nicht APPROVED, unterliegt Evidence-Gate F-069
Regel: Solange ein Abschnitt OEP-Blöcke enthält, ist er nicht einfrierbar. Kein Diagramm, keine Referenzimplementierung, kein Message-Flow vor Schließen der OEPs.

---

## Abschnitt 1 — Formale Systemdefinition & Notation

### 1.1 Scope dieses Abschnitts

Abschnitt 1 legt die formale Basis fest, auf die alle nachfolgenden Familien (BLOCK/TX/CONSENSUS/VM/ATS-Policy) referenzieren. Er definiert keine neuen Primitive; er bindet die bestehenden Anker (TownHash-256, Envelope nach P2P-001 §9, Signatur-Welt nach Draft) ein und macht ihre Verwendung im ATC Protocol explizit.

### 1.2 Begriffe (normativ)

- Ledger-Zustand (State) — die Menge aller deterministisch adressierbaren Zustandsgrößen, die durch Transitionen verändert werden. Adressierung erfolgt über den State-Layer; die konkrete Datenstruktur ist OEP-3 (Merkle-Patricia vs. Verkle).
- Transition — eine signierte, deterministisch ausführbare Abbildung δ: S × TX → S' ∪ {⊥}. ⊥ bedeutet „invalide"; kein partieller State.
- Runde (Round) — die kleinste, konsensrelevante Zeiteinheit. Eine Runde endet mit Finalization-Commit oder Timeout.
- Epoch — eine geordnete Menge von Runden mit gemeinsamem Validator-Set und gemeinsamem ATS-Parametersatz. Epoch-Grenzen sind Replay-Schutz-Grenzen (siehe §2.4).
- Final — ein Block oder State-Commit ist final, wenn er von einer qualifizierten Mehrheit gemäß §4 quittiert wurde und nicht mehr rücknehmbar ist.
- Kanonisch — die eindeutige Serialisierung gemäß §2.3. Nicht-kanonische Serialisierungen sind ungültig, auch wenn sie semantisch äquivalent sind.

### 1.3 Notation

- H(·) — TownHash-256 über kanonisch serialisierte Bytes.
- Sig_sk(·), Ver_vk(·,·) — Signaturverfahren gemäß Signatur-Welt-Draft (secp256k1/RFC-6979/low-S für TX; Ed25519 ausschließlich P2P/DID).
- S_r — State-Root am Ende von Runde r.
- B_r — Block der Runde r, enthält mindestens: parent_root, state_root_prev, state_root_new, tx_set, epoch_id, round_id, proposer_id, ats_param_hash.
- V_e — Validator-Set der Epoch e.
- Q(V_e) — Quorum gemäß Konsens-Familie (siehe OEP-4).

### 1.4 Determinismus-Invarianten (DET)

Diese Invarianten sind normativ und gelten für alle Familien. Verstöße sind Spec-Fehler, nicht Implementierungsfehler.

- DET-1 — Keine nichtdeterministischen Quellen. VM-Ausführung, ATS-Policy-Auswertung und Consensus-Validierung DÜRFEN NICHT abhängen von: Wall-Clock, Floating-Point, Speicheradressen, Iterationsreihenfolge nicht-geordneter Container, externem I/O, Zufall ohne Commit-Reveal.
- DET-2 — Kanonizität vor Ausführung. Jede TX MUSS vor Ausführung kanonisch serialisiert und ihr Hash MUSS verifiziert sein.
- DET-3 — Totalität. Für jede wohlgeformte TX und jeden wohldefinierten State liefert δ entweder S' oder ⊥. Es gibt kein „pending", kein „unknown", kein „partial".
- DET-4 — Idempotente Validierung. Ver_vk(vk, Sig_sk(m), m) = true ⟺ die TX ist in jeder Runde mit identischem Epoch-Kontext gültig. Kontextabhängige Gültigkeit ist auf epoch_id und round_id beschränkt, beide explizit im Envelope.
- DET-5 — State-Root-Bindung. state_root_new MUSS über TownHash-256 aus dem vollständigen post-execution State berechnet werden. Zwei Nodes mit gleichem state_root_new haben denselben State — per Definition, nicht per Annahme.

### 1.5 Systemzustand

Der globale Systemzustand ist das Tupel

```
Σ = (LedgerState, ValidatorSet, ATSParams, EpochMeta)
```

- LedgerState — gemäß OEP-3.
- ValidatorSet — gemäß OEP-4 und §4.
- ATSParams — der aktuell gebundene Parametervektor der ATS-Policy-Engine; seine Hash-Bindung im Block (ats_param_hash) ist normativ.
- EpochMeta — epoch_id, Grenzen, Slashing-Register-Ref, Replay-Epoch-Bindung.

Invariante SYS-1: Innerhalb einer Epoch ändern sich ValidatorSet und ATSParams nicht außer durch expliziten ATS-Governance-Akt, der selbst als TX in einem Block finalisiert wird.

### OEP-Block nach §1

**OEP-1 — Account- vs. UTXO-Modell.**

- Option A: Account-basiert (Balance/Nonce/Code pro Adresse).
- Option B: UTXO-basiert (Output-basiert, explizite Inputs).
- Option C: Hybrid (Account für VM, UTXO für Settlement).
- Auswirkung: Adressierung, Nonce-Replay-Schutz, Parallelisierbarkeit der Ausführung, State-Proof-Größe.
- Blockiert: §3 (TX-Modell), §5 (VM/Gas), §6 (Storage).
- Owner: offen.

**OEP-2 — Formale ATS-Policy-Sprache.**

- Option A: DSL mit statischem Typsystem, in Rust/WASM-Compiler überführt.
- Option B: WASM-Module als Policy-Träger, deterministisch sandboxed.
- Option C: Regel-Engine mit deklarativem Format (z. B. Datalog-Variante).
- Auswirkung: Auditierbarkeit, Upgrade-Pfad, Beweisbarkeit, Angriffsfläche.
- Blockiert: §5.4 (Policy-Hooks in VM), §6 (ATS-Policy Family).
- Owner: offen.

---

## Abschnitt 2 — Netzwerk- und Nachrichtenmodell

Abschnitt 2 ist ganz vorne, weil der Envelope aus ATC-PROTO-P2P-001 §9 normativ ist und alle Familien daran hängen. Dieser Abschnitt dupliziert §9 nicht — er bindet ihn ein und ergänzt nur die konsens- und VM-spezifischen Message-Typen.

### 2.1 Normative Basis

Es gilt ATC-PROTO-P2P-001 §9 vollständig, insbesondere:

- 9+1-Feld-Envelope (9 Pflichtfelder + 1 Erweiterungsfeld).
- Kanonische Serialisierung mit deterministischer Feldsortierung.
- Chain-ID-Bindung im Envelope.
- Replay-Schutz durch Nonce + Epoch-Bindung.
- Signatur-Welt gemäß Draft: secp256k1/RFC-6979/low-S für TX; Ed25519 ausschließlich P2P/DID — nicht für TX.

Verstöße gegen §9 sind keine ATC-Protokoll-Fehler, sondern ungültige Nachrichten und MÜSSEN verworfen werden, ohne State-Änderung.

### 2.2 Message-Klassen

Die Spec unterscheidet drei normative Message-Klassen:

1. P2P-Klasse — ausschließlich durch P2P-001 §9 definiert: Peer-Discovery, Gossip-Kontrolle, Sync-Frames. Keine State-Semantik.
2. Konsens-Klasse — definiert in §4 und in den CONSENSUS-3xx-Drafts: BLOCK_PROPOSAL, BLOCK_VOTE, STATE_COMMIT, FINALITY_CERT.
3. Anwendungs-Klasse — definiert durch TX- und VM-Familie: TX_BROADCAST, STATE_QUERY, PROOF_REQUEST, PROOF_RESPONSE.

Der Envelope trägt in Feld 3 (msg_type) die Klasse und den Subtyp. Invariante MSG-1: Jede Nachricht MUSS genau einer Klasse angehören; Mischformen sind ungültig.

### 2.3 Kanonische Serialisierung

Die kanonische Serialisierung MUSS:

- deterministisch sein (gleiche Semantik ⟹ gleiche Bytes),
- integer-Encoding mit expliziter Endianness und Breite verwenden,
- keine optionalen Felder ohne explizite Längen-/Präsenzmarkierung enthalten,
- Reihenfolge von Sammlungen kanonisch definieren (Sortierung nach kanonischem Feldwert),
- unbekannte Felder nicht tolerieren (fail-closed).

Invariante SER-1: H(serialize(m)) ist die einzige zulässige Identität einer Nachricht. Kein alternativer Hash, kein Hash über nicht-kanonische Bytes.

### 2.4 Replay-Schutz

Replay-Schutz erfolgt dreifach:

- Nonce pro Sender — monoton, im Ledger-State verankert (Account) oder pro Output-Kette (UTXO). Abhängig von OEP-1.
- Epoch-Bindung — der Envelope trägt epoch_id; TXs mit abgelaufener oder zukünftiger epoch_id sind ungültig.
- Chain-ID-Bindung — Replays über Chain-Grenzen werden durch Chain-ID im Envelope und Signatur-Domain-Trennung ausgeschlossen.

Invariante RPL-1: Eine TX, die in Epoch e finalisiert wurde, ist in jeder Epoch e' ≠ e ungültig — unabhängig von der Nonce.

### 2.5 Sync-Modell

Sync verwendet SYNC_REQUEST/SYNC_RESPONSE mit folgenden Eigenschaften:

- Sync ist zustandslos auf Seiten des Responders — er liest nur aus Storage.
- Sync MUSS Proofs liefern (Merkle-/Verkle-Proofs gemäß OEP-3), nicht nur Rohdaten.
- Sync-Frames sind selbst nicht finalisierend; sie transportieren nur bereits finalisierte Daten.
- Invariante SYNC-1: Ein Node MUSS einen Sync-Frame verwerfen, wenn der enthaltene Proof nicht gegen den finalen State-Root verifiziert.

### OEP-Block nach §2

**OEP-3 — State-Proof-Form.**

- Option A: Merkle-Patricia-Trie (Ethereum-Kompatibilität, breite Tooling-Unterstützung, größere Proofs).
- Option B: Verkle-Trie (kompaktere Proofs, neuer, weniger Bibliotheksreife).
- Option C: Hybrid (MPT für Settlement, Verkle für Light-Client-Pfade).
- Auswirkung: Light-Client-Größe, Sync-Bandbreite, TownHash-Integration, Cold-Storage-Format.
- Blockiert: §5 (Storage Family), §2.5 (Sync), §3 (TX-Modell).
- Owner: offen.

**OEP-4 — Finality-Runde und Reorg-Garantie.**

- Option A: Ein-Runden-Finality (Tendermint-artig), Reorg ausgeschlossen.
- Option B: Zwei-Runden-Finality (HotStuff-artig), Reorg ausgeschlossen.
- Option C: Ein-Runden-Finality mit explizitem Partitionsprotokoll (Reorg nur bei Partitionsheilung, deterministisch definiert).
- Auswirkung: Latenz, Sicherheitsbeweis-Struktur, Timeout-Mechanik, Slashing-Bedingungen.
- Blockiert: §4 (Consensus Family).
- Owner: offen.

**OEP-5 — Validator-Set-Dynamik und Slashing-Parameter.**

- Option A: Statisch pro Epoch, Übergang nur per Governance-TX.
- Option B: Dynamisch per Stake-weighted Wahl am Epoch-Ende.
- Option C: Hybrid (statische Top-K, dynamische Rotation).
- Auswirkung: Sybil-Resistenz, Slashing-Komplexität, Epoch-Länge, Finality-Beweise.
- Blockiert: §4.
- Owner: offen.

**OEP-6 — BFT-Variante.**

- PoH ist gesetzt (F-067). Offen ist die BFT-Schicht: welche der CONSENSUS-301..307-Drafts wird zur normativen Referenz erhoben, und wie interagiert sie mit PoH als Taktgeber?
- Blockiert: §4.
- Owner: offen.

---

## Konsens-Block — ATC-CONSENSUS-301..307 + DET + ENC

Status: Die Konsens-Familie existiert als 0.1.0-DRAFT (F-067). Dieser Block konsolidiert sie zu einer spec-fähigen Struktur. Er erhebt keinen der Drafts vorzeitig zu APPROVED — er markiert die Stellen, an denen die Owner-Entscheidungen einfließen.

### K.1 Modell

Der Konsens läuft in Runden. Jede Runde hat eine deterministische Proposer-Auswahl (abhängig von OEP-5 und OEP-6) und ein deterministisches Quorum Q(V_e). Die PoH-Schicht liefert die Taktung; die BFT-Schicht liefert Finalität.

### K.2 PoH-Integration

- PoH-Sequenz MUSS in jedem Block als Verkettung nachweisbar sein.
- PoH dient als Zeitgeber, nicht als Konsens-Ersatz.
- Invariante POH-1: Kein Block darf ohne gültige PoH-Verkettung finalisiert werden.

### K.3 Finality (abhängig von OEP-4)

- Finality wird durch FINALITY_CERT quittiert, das Q(V_e) Signaturen über (state_root_new, round_id, epoch_id) trägt.
- Invariante FIN-1: Ein finaler State-Root ist unveränderlich. Reorg ist ausgeschlossen, außer im explizit definierten Partitionsfall (OEP-4 Option C).

### K.4 Validator-Set (abhängig von OEP-5)

- V_e ist für die Dauer von Epoch e fix.
- Übergänge erfolgen ausschließlich an Epoch-Grenzen.
- Invariante VAL-1: Innerhalb einer Epoch ist V_e unveränderlich. Kein „fliegender" Validator-Wechsel.

### K.5 Slashing

- Slashing-Bedingungen, Höhe und Verfall MÜSSEN in der ATS-Policy-Engine als Parameter geführt werden (ats_param_hash-gebunden).
- Invariante SLSH-1: Slashing ist deterministisch aus dem finalen Block ableitbar. Kein Slashing ohne finalen Beweis.

### K.6 Sicherheitsargumente (Skizze)

Die Sicherheitsbeweise der Konsens-Familie MÜSSEN in §4 als separate Unterabschnitte geführt werden:

- Safety: Zwei finale Blöcke in derselben Runde implizieren ≥⅓ byzantinische Validatoren — Widerspruch zur Quorum-Annahme.
- Liveness: Unter synchronen Annahmen und Q(V_e)-ehrlicher Mehrheit endet jede Runde mit Finalization.
- Partition: Gemäß OEP-4 Option C explizit definiert; sonst ausgeschlossen.

### K.7 Beweisverpflichtung

Jede Invariante aus K.2–K.5 MUSS im finalen SCR-Draft mit einem Testfall in mindestens einer der Kategorien versehen werden:

- UNIT — Einzelfall.
- PROP — Property-basiert über Zufallsverteilung.
- FUZZ — Envelope-/Serialisierungs-Robustheit.
- ADV — adversariale Partition/Slashing/BFT-Verletzung.

---

## Evidence-Gate (F-069)

Kein Abschnitt dieses Drafts wandert nach APPROVED, bevor:

1. alle OEPs 1–6 mit Owner-Entscheidung und Begründung geschlossen sind,
2. alle DET-, MSG-, SER-, RPL-, SYNC-, POH-, FIN-, VAL-, SLSH-Invarianten je einen Testfall in einer der vier Kategorien haben,
3. die Konsens-Familie in einen einzelnen normativen SCR-Draft konsolidiert ist, der auf die P2P-001-§9-Norm verweist statt sie zu duplizieren,
4. die Signatur-Welt (secp256k1/RFC-6979/low-S für TX, Ed25519 P2P/DID) als normative Referenz im Draft zitiert ist — nicht neu erfunden.

Erst danach: E2E-Message-Flow-Diagramm und Rust/Go-Referenzimplementierung.

---

## Nächster Schritt

Nach OEP-Schließung: Abschnitt 3 (TX- und State-Modell) plus Überführung der CONSENSUS-301..307-Drafts in den konsolidierten §4. Beide hängen an OEP-1, OEP-3, OEP-4, OEP-5, OEP-6.

---

## Anhang A — Beratungsempfehlungen der ausarbeitenden Instanz (NICHT normativ, keine Entscheidung)

Diese Empfehlungen sind Beratung, keine Festlegung. Jede OEP-Entscheidung bleibt Owner-Aktion mit Begründung im Entscheidungsprotokoll.

### OEP-1: Empfehlung Option A (Account-basiert)

Begründung: atc-vm und ATCLang-Verträge benötigen per-Adresse Code und Storage — Account-Semantik ist bereits die implizite Annahme in §2.4 (Nonce im Ledger-State) und DET-4. UTXO brächte Parallelisierungs-Vorteile, aber für v1 eine zweite State-Wahrheit und aufwendigere Replay-Logik pro Output-Kette. Wallet-/Explorer-Consumer (F-140-Kette) erwarten Account-Semantik. Hybrid (C) wäre für v1 die teuerste Variante (zwei Modelle spezifizieren, testen, synchronisieren).

### OEP-2: Empfehlung Option B (WASM-Module als Policy-Träger)

Begründung: ATS-Policies sind deterministisch on-chain ausführbar und über ats_param_hash bereits im Block-Header gebunden — WASM-Module integrieren sich in dieselbe Bindung (Hash = kanonische Modul-Bytes). Die Determinismus-Garantie fällt aus der VM-Sandbox heraus (DET-1 gilt ohne neue Beweislast). Option A (DSL + eigener Compiler) wäre die beste menschliche Auditierbarkeit, erfordert aber einen kompletten Compiler-Toolchain-Aufbau, bevor die erste Policy läuft — als spätere MINOR mit statischem Typsystem nachholbar. Option C (Datalog-artig) ist beweisfreundlich, aber Nischen-Tooling und schwächer bei parametrisierter Policy-Logik.

### OEP-3: Empfehlung Option A (Merkle-Patricia-Trie über TownHash-256)

Begründung: MPT ist bewiesen, Node-Hash = H(kanonisch serialisierte Kinder) integriert TownHash-256 direkt, Tooling ist breit, Proofs sind groß aber für Devnet/v1 akzeptabel und ehrlich dokumentierbar. Verkle liefert kompaktere Proofs, aber geringere Reife und ein späteres MAJOR mit COMPAT-001-Beweislast (Proof-Format-Änderung ist breaking). Hybrid (C) verzichtet auf den Hauptvorteil von Verkle, solange Light-Client-Pfade noch nicht existieren.

### OEP-4: Empfehlung Option A (Ein-Runden-Finality, Reorg ausgeschlossen)

Begründung: Passt zur FINALITY_CERT-Struktur (K.3) und zum Tendermint-artigen Beweisschema (K.6 Safety über Quorum-Intersect). Partition wird als Liveness-Stall behandelt: Ohne Quorum pausiert die Kette deterministisch — kein Reorg, kein Zustandsverlust, ehrlichste und einfachste Semantik. Option C (deterministischer Reorg bei Partitionsheilung) ist ein Forschungsthema mit erheblicher Beweislast und gehört nicht in v1. Option B (HotStuff, zwei Runden) verdoppelt die Latenz ohne Vorteil bei gesetzter PoH-Taktung.

### OEP-5: Empfehlung Option A (statisch pro Epoch, Übergang nur per Governance-TX)

Begründung: SYS-1 und VAL-1 sind bereits so formuliert — die Option ist strukturell vorbereitet. Statische Sets minimieren Slashing-Komplexität und Sybil-Design-Last für v1; dynamisches Staking (Option B) erfordert ein vollständiges Stake-/Abwahl- und Slashing-Design, das die Konsens-Familie vor dem ersten Freeze belasten würde. Devnet arbeitet ohnehin mit statischem Set. Rotation als spätere MINOR/MAJOR mit COMPAT-001 nachholbar.

### OEP-6: Empfehlung Tendermint-artige BFT als Referenz der CONSENSUS-3xx-Familie, PoH als Taktgeber

Begründung: Konsistent mit OEP-4-A (Ein-Runden-Finality), einfachster vollständiger Sicherheitsbeweis (Quorum-Intersect 2f+1), deterministische Proposer-Rotation aus V_e ableitbar. PoH bleibt gemäß K.2 Taktgeber und Verkettungsnachweis — die BFT-Schicht liefert Finalität. Dies erhebt den entsprechenden 3xx-Draft zur normativen Referenz für §4, ohne die übrigen Drafts zu verwerfen: sie bleiben als Varianten für spätere MINOR-Reviews dokumentiert.

### Kohärenzhinweis

Die Empfehlungen OEP-1/3/4/5/6-Option-A bilden einen kohärenten Pfad (Account + MPT + Tendermint-artige Ein-Runden-Finality + statisches Epoch-Set + PoH-Taktung), der mit SYS-1, DET-1..5, VAL-1, FIN-1 konsistent ist und die geringste Beweislast vor dem ersten Freeze trägt. OEP-2-Option-B nutzt dieselbe VM-Infrastruktur.
