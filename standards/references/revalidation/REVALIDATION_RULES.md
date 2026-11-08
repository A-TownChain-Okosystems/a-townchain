# Revalidierungs-Regelkatalog (RV-Regeln)

> Regelträger für die Flotten-/Strang-Revalidierung. Geltung je Regel an
> SCR-Bindung (RV-001/RV-002: SCR-0129, PENDING Owner-Approve).
> Prüfmodus bis zur Implementierung des atc-toolchain-Scans: manueller
> Befund mit dokumentiertem Inventar — Regel ist damit benannt und
> testbar spezifiziert, Automatisierung folgt (nicht blockierend).

## RV-001 — ATCB-SSOT-Pfad (SCR-0129)

**Regel:** Existenz normativer ATCB-Geometrie (Magic `ATCB`, Container-Layout,
Opcode-Katalog, Kodier-/Reject-Regeln — unabhängig vom Dateiform) außerhalb
`atclang/specs/bytecode/SPEC.md` → **VIOLATION**.

**Prüfung:** Org-weiter Scan auf ATCB-Geometrie-Muster; Ausnahmen nur mit
expliziter Klassifizierung als ARCHIVE/MIRROR ohne normativen Anspruch.
Verstoßklassen: NORMATIVE_CLAIM (VIOLATION), UNCLASSIFIED (FUND — zu
klassifizieren), ARCHIVE/MIRROR (zulässig).

## RV-002 — Klassifizierungspflicht (SCR-0129)

**Regel:** Jeder ATCB-Geometrie-Fund außerhalb des SSOT-Pfads muss
klassifiziert sein (ARCHIVE/MIRROR vs NORMATIVE_CLAIM). Unklassifizierte
Funde sind OFFEN, keine Violation — aber Revalidierung läuft erst, wenn
alle Funds klassifiziert sind.

## Inventar (Initialbestand, 06.10., PENDING Mirror-Klassifikation nach Gate 0)

| Fundort | Vorabannahme | Status |
|---|---|---|
| a-townchain-ecosystem/components/atclang/… | MIRROR (Ökosystem-Spiegel) | OFFEN |
| a-townchain-ecosystem/components/atc-vm/… | MIRROR (Ökosystem-Spiegel) | OFFEN |
| a-townchain/…/vm/… | ARCHIVE/MIRROR | OFFEN |
| a-townchain-os-docs (docs/archive/, monorepo-legacy) | ARCHIVE (Vault, ohne normativen Anspruch) | OFFEN |

## Rollen (Owner-Benennung 06.10.2026 — ROLE_DEFINED / HUMAN_OWNER_UNASSIGNED)

### Gate-0-Executor
ROLE: ARCHITECTURE/GOVERNANCE EXECUTOR — Authority: Existing-First, SSOT,
Duplicate-Detection, Owner-/Layer-/Capability-Resolution. Kein persönlicher
Name; Rollen-Inhaberschaft beim Owner, Zuweisung offen.

### Mirror-Klassifikator
ROLE: SSOT/MIRROR CLASSIFIER — Authority: Klassifikationsraum canonical |
mirror | generated view | integration-only | duplicate | obsolete. Kein
persönlicher Name; Rollen-Inhaberschaft beim Owner, Zuweisung offen.
