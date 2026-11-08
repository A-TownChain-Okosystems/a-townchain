---
standard:
  id: ATC-STD-NET-009
  title: "Network Release Maturity & Versioning Standard (NETWORK x MATURITY Reifestufenmodell, 6 Release Gates, Versionslogik, Release State SSOT)"
  version: "1.0.0"
  status: approved
  category: net
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf 14.09.)"
  created: "2026-09-14"
  updated: "2026-09-14"
  normative: true
  supersedes: null
  superseded_by: null
  effective_date: null
  review_date: null
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Netzwerk-Releases (Devnet/Testnet/Mainnet)"
---

# ATC-STD-NET-009 — Network Release Maturity & Versioning Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (Sammelfreigabe SCR-0124) — Owner-Entwurf 14.09. 09:42 (ATC Network Release Lifecycle); SCR-0122. Wartet auf §9-Freigabe (ATC-STD-000).
> **Reihe:** ATC-STD-NET-001…009 · **Abgrenzung:** NET-004 bleibt SSOT der Promotion-Pipeline (GATE-011…013); dieser Standard definiert die MATURITY-Dimension INNERHALB der Umgebungen plus Versionssemantik.

## 1. Zweck (Purpose)

Ein verbindliches Reifestufenmodell fuer Netzwerk-Releases: Jede Freigabe deklariert
netzwerk und reifezustand als zwei unabhaengige Dimensionen, und jeder Stufenuebergang
erfolgt ausschliesslich durch ein Evidence-Gate — No Evidence, No Trust gilt auch fuer
Netzwerk-Releases.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle Releases und Reifestufenuebergaenge von ATC-DEVNET, ATC-TESTNET, ATC-MAINNET
(registry/networks.yaml). **Nicht im Gelt:** die Promotion-Mechanik zwischen den Umgebungen
(NET-004), Software-SemVer ausserhalb von Netzwerk-Releases (VERSION-001), Upgrades im
laufenden Netz (NET-006), Security-Anforderungen als solche (NET-007).

## 3. Zwei orthogonale Release-Dimensionen

Die Versionierung allein bestimmt nicht den Netzwerkstatus; der Netzwerkname ist eine
eigene Release-Dimension:

```
NETWORK  = DEVNET | TESTNET | MAINNET
MATURITY = ALPHA  | BETA    | STABLE   (+ RELEASE CANDIDATE als Gate-Zustand vor Mainnet)
```

## 4. Reifestufen-Matrix

| Netzwerk | Reifestufe | Zweck | Produktionsstatus |
|---|---|---|---|
| Devnet | Alpha | Fruehe Entwicklung, Architektur, neue Features | NICHT produktionsfaehig |
| Devnet | Beta | Stabilisierung, Integration, interne Belastungstests | NICHT produktionsfaehig |
| Testnet | Alpha | Oeffentliche/oekosystemweite Erprobung neuer Funktionen | NICHT produktionsfaehig |
| Testnet | Beta | Release Candidate, Security-/Performance-/Kompatibilitaetstests | NICHT produktionsfaehig |
| Mainnet | Full / Stable | Produktionsbetrieb | PRODUKTIONSFAEHIG |

Alpha und Beta sind NIE produktionsfaehig, ungeachtet der Umgebung (REQ-NET-083).

## 5. Versionslogik

```
DEVNET:  v0.x.0-alpha.1, v0.x.0-alpha.2, …  ->  v0.x.0-beta.1, v0.x.0-beta.2, …
TESTNET: v0.x.0-alpha.1, …                  ->  v0.x.0-beta.1, v0.x.0-beta.2, …
MAINNET: v1.0.0, v1.1.0, v1.2.0, … (nur STABLE)
```

- Prereleases (0.x mit -alpha.N/-beta.N) sind auf Devnet/Testnet beschraenkt (REQ-NET-084).
- Versionen >= 1.0.0 setzen MAINNET und MATURITY = STABLE voraus.
- Rueckstufungen (Stable -> Beta) sind verboten; ein Rollback folgt NET-008/MAINT-018, nicht einer Rueckstufung der Reifestufe.

## 6. Release Gates (6, fail-closed)

```
DEVNET ALPHA -> [GATE-DEV-ALPHA] -> DEVNET BETA -> [GATE-DEV-BETA] -> TESTNET ALPHA
 -> [GATE-TEST-ALPHA] -> TESTNET BETA -> [GATE-TEST-BETA] -> MAINNET RELEASE CANDIDATE
 -> [GATE-MAINNET-RC] -> MAINNET v1.0.0 STABLE [GATE-MAINNET-STABLE]
```

| Gate | Mindest-Eintrittskriterien (Evidence-geprueft) |
|---|---|
| GATE-DEV-ALPHA | Architektur dokumentiert, Genesis-Faehigkeit (NET-005), Basis-Tests gruen, Devnet-Konfiguration (NET-001) |
| GATE-DEV-BETA | Integration abgeschlossen, interne Belastungstests bestanden, keine offenen P0-Defekte |
| GATE-TEST-ALPHA | Oeffentliche Erprobung freigegeben, Telemetrie aktiv, Incident-Prozess (NET-007) verfuegbar |
| GATE-TEST-BETA | Security-Audit (NET-007), Performance-/Lasttests, Kompatibilitaetspruefung (COMPAT-001), Recovery-Uebung (NET-008) bestanden |
| GATE-MAINNET-RC | Vollstaendige Evidence-Kette, Rollback getestet, Governance-Freigabe (Owner), Release-Kandidat deklariert |
| GATE-MAINNET-STABLE | Maintenance Readiness (MAINT-000 §7, PR #25), Monitoring live, Produktionsbetrieb, v1.0.0 |

Kein Uebergang erfolgt automatisch; Gates werden nie uebersprungen (REQ-NET-082,
konsistent mit REQ-NET-031/NET-004). Fehlende Evidence blockiert den Uebergang (REQ-NET-086).

## 7. Maschinenlesbarer Release State

SSOT ist `registry/networks.yaml`; jeder Gate-Durchgang erzeugt einen maschinenlesbaren
Evidence-Record nach dem MAINT-019-Schema (PR #25; bis zur Freigabe analog ATC-AI-GOV
Evidence-Records):

```yaml
network: testnet
maturity: beta
version: 0.9.0-beta.2
gate_history: [GATE-DEV-ALPHA@v0.8.0, GATE-TEST-ALPHA@v0.9.0-alpha.1]
```

## 8. REQ-Matrix (normativ)

| REQ | Anforderung | Pflicht |
|---|---|---|
| REQ-NET-081 | Jeder Netzwerk-Release-Zustand deklariert network, maturity und version im SSOT (registry/networks.yaml) | MUST |
| REQ-NET-082 | MATURITY-Uebergange erfolgen nur ueber die zugehoerigen Gates; nie automatisch, nie uebersprungen | MUST |
| REQ-NET-083 | Alpha/Beta sind niemals produktionsfaehig; Mainnet verlangt MATURITY = STABLE | MUST |
| REQ-NET-084 | Prereleases (0.x-alpha/beta) nur auf Devnet/Testnet; >= 1.0.0 nur Mainnet STABLE | MUST |
| REQ-NET-085 | Jeder Gate-Durchgang erzeugt einen maschinenlesbaren Evidence-Record | MUST |
| REQ-NET-086 | Gates sind fail-closed: fehlende Evidence blockiert den Uebergang | MUST |

## 9. Implementierungsstatus (ehrlich)

**SPECIFICATION_ONLY.** Live-Befund 14.09.: alle drei Netzwerke status=planned (keine
Genesis); registry/networks.yaml enthaelt noch keine maturity/version-Felder. Die
Gate-Kriterien verweisen teilweise auf Standards, die selbst auf §9-Freigabe warten
(MAINT-Familie, PR #25). CLAIMED != PASS.
