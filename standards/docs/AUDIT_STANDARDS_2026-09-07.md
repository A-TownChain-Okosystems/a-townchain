# Standards-Voll-Audit 2026-09-07 (81 Standards)

**Prüfklassen:** Struktur (Registry/Dateien/Versionshistorie), Verknüpfung
(ID- und Abschnittsreferenzen, Abhängigkeitsgraph), Vollständigkeit
(Pflichtabschnitte, Platzhalter, leere Abschnitte), Beschreibungsqualität
(Titel-/Status-Konsistenz, Tippfehler).

## Ergebnisse (alle behoben, Commit 07.09.2026)

| Fund | Klasse | Schwere | Behebung |
|------|--------|---------|----------|
| Kein ID-Muster für ATC-STD-ZKP-NNN | Schema/S11 | S2 | zkpStandardId ergänzt |
| 28 Standards ohne Frontmatter-Fences | Struktur/S2 | S2 | Fences ergänzt, 81/81 konsistent |
| 3 Tippfehler (AI-DEV-005, AI-DEV-012) | Beschreibung | S4 | korrigiert |
| AAS-008 §3 redundant/unklar | Beschreibung | S3 | Abschnitt ersetzt |
| 49 fehlende Dependency-Kanten | Verknüpfung/R4 | S3 | ergänzt (zyklische bewusst ausgenommen) |

## Fehlalarm-Klassen (keine Änderung)

- Beispiel-Platzhalter: ATC-STD-XXX/NNN, ATC-STD-042/043, ZKP-001-999,
  REQ-XXX, ROLE-XXX, SCR-XXXX (Schema-Beispiele).
- supersedes-Legacy-IDs: ATC-STD-REPO-001/002/003 (historische Namen).
- Provenance-Marker „§N des Owner-Entwurfs" in AI-DEV-001 Abschnittstiteln.
- Referenz ≠ Abhängigkeit: Peer-/Spiegelreferenzen (BUG-Familie unter
 einander, ZKP-001↔ZKP-010, 202↔203) bewusst NICHT als Kante deklariert
  (Zyklusverbot, ATC-STD-204 §2).

## Offen (nicht audit-behebbar)

- F-009/F-010: workflow-Scope für CI-Workflow-Änderungen (Owner-Aktion).
- Nach-Freigabe-Rollouts: org-units.yaml, repositories.yaml, risks.yaml
  (ENT-008/009/011), Repo-Manifeste (AAS-025, #111), Commit-Trailer (#112),
  Interface-Test-Suiten IFC-0001..0010 (P0).
