# Changelog — Approval Package ATC-STD-000

## 1.0.0 — 07.09.2026
- Formales Approval-Paket nach Owner-Vorgabe errichtet: Review-Snapshot
  (ATC-STD-000-v1.0.0.md, SHA-256 dffc547c…), drei Reviews gegen die
  tatsaechliche Fassung @ 8eda21a mit erweiterten Pruefkatalogen
  (Supply-Chain, Emergency Changes, Rollen/Berechtigungen,
  Parallelspruchs-Verfahren), REQUIREMENT-MATRIX.yaml (12 Eintraege,
  11 PASS / 1 PARTIAL), APPROVAL-DECISION.md (PENDING, Empfehlung APPROVE).
- Vorgaenger-Paket (governance/APPROVAL_PACKAGE_ATC-STD-000.md, 07.09.
  frueher) wird Zeiger auf dieses Verzeichnis — eine Quelle der Wahrheit.
- Evidenz: atc-std-validator COMPLIANT, atc-repo-audit GATE PASS,
  Snapshot byte-identisch zur geprueften Fassung.

## 1.1.0 — 07.09.2026 (Nachmittag)
- Review-Chain ERNEUT durchgefuehrt — gegen die Owner-Formalfassung
  (35 Abschnitte), die den Agent-Entwurf als verbindlichen v1.0.0-Text
  ersetzt hat. Sequenz sauber: Registry DRAFT → Reviews (3/3 PASS) →
  Statusuebergang CANDIDATE (Datei/Registry/Snapshot synchron).
- Findings aktualisiert: 3 fruehere Findings durch Formalfassung behoben
  (Emergency Changes §31, Konflikt-Resolution §30, L-Skalen); neu: 5
  nicht-blockierende (T-F01, S-F01, S-F02, S-F03-LOW, A-F01).
- Requirement-Matrix neu: 16/16 PASS. Schema/Validator um authority-Feld
  erweitert (§6 der Formalfassung); S-06 akzeptiert Purpose-Struktur (§8).
- APPROVAL-DECISION: PENDING — Approval bleibt beim Owner BLOCKED.

## 1.2.0 — 07.09.2026 (Naming Convention)
- §36 per Owner-Mandat ergänzt (ID-Tabelle, Repo-/Datei-Namen,
  ID-Immutabilität, maschinenprüfbares Schema naming-conventions.schema.json,
  Validator S-16, Findings-Registry registry/findings.yaml mit F-001…F-005
  als kanonische IDs). Snapshot aktualisiert; Matrix 17/17 PASS;
  Validator-Re-Lauf COMPLIANT.

## 1.4.0 — 07.09.2026 (normatives Naming-Hardening §7)
- Owner-Mandat: §7 "ATC Naming, Identification and Namespace Convention"
  (7.1-7.11) als normativer Abschnitt; Abschnitte 7-35 → 8-36; alte §36-
  Vorversion in §7 aufgegangen. Mapping für Alt-Referenzen: N→N+1 (7…35),
  36→7.
- naming-conventions.schema.json gehärtet (additionalProperties: false,
  6 Regelgruppen). Validator v0.2.0: S-16 lädt Regeln NUR aus dem Schema;
  S-17 Duplicate Detection im CI-Modus validate_all.py; CI-Workflow
  naming-governance.yml (§7.11). Requirement-Matrix 20/20 PASS.
