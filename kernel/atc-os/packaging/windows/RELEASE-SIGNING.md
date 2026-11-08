# KAI-OS Release-Signierung (SignPath)

**Prinzip: Fail-Closed.** Die Release-Pipeline veröffentlicht **keine unsignierte**
Produkt-EXE (Governance: REQ-ENG-006 CI-Quality-Gates Fail Closed, ATC-STD-309
Supply Chain Security). Fehlt die SignPath-Konfiguration, bricht der Release-Job
abgelöst ab — die Artefakte bleiben als Workflow-Artifacts prüfbar, es entsteht
kein GitHub Release.

## Einrichtung (einmalig)

1. SignPath-Konto/Organisation anlegen: https://about.signpath.io
   (Free-Tier für Open Source verfügbar.)
2. Projekt `kai-os` anlegen, Signierungszertifikat (Authenticode, Code Signing) hinterlegen.
3. Signing Policy definieren (z. B. `release`) und das GitHub-Repository
   `A-TownChain-Okosystems/a-townchain-os` als Source verknüpfen.
4. API-Token erstellen (berechtigt auf das Projekt).
5. Im Repository → Settings → Secrets and variables → Actions:

| Typ | Name | Wert |
|---|---|---|
| Secret | `SIGNPATH_API_TOKEN` | API-Token aus Schritt 4 |
| Variable | `SIGNPATH_ORGANIZATION_ID` | SignPath-Organisations-ID |
| Variable | `SIGNPATH_PROJECT_SLUG` | z. B. `kai-os` |
| Variable | `SIGNPATH_SIGNING_POLICY_SLUG` | z. B. `release` |

## Ablauf in der Pipeline

```
kai-os-node.exe (Build, --locked)
        │
        ├── SignPath-Signierung (sign-signpath.ps1)
        ├── Authenticode-Verify (Get-AuthenticodeSignature -> muss 'Valid' sein)
        ├── Portable-ZIP (enthaelt die signierte exe)
        ├── SBOM (CycloneDX)
        ├── Inno-Setup-Installer (bettet die signierte exe ein)
        ├── Installer-Signierung + Verify
        ├── SHA256SUMS + Build-Provenance (Artifact Attestation)
        └── Release-Verification (alle Assets vorhanden + signiert)
                │
                v
        GitHub Release (erst nach PASS)
```

## Erste Validierung

Der SignPath-REST-Flow (`sign-signpath.ps1`) ist gegen die dokumentierte API v1
implementiert, aber noch nicht gegen ein Live-Konto getestet. **Der erste
Tag-Release (`v0.1.0` von kai-os/node) ist der Validierungslauf** — bei
Abweichungen die Schrittnummern (1)–(3) im Skript prüfen. Vorher KEIN
Produktions-Tag setzen.

## Upgrade-Pfad (P1)

- Windows-Service-Installation (kai-os-node als Dienst, systemd-Gegenstück)
- Signierte Mercury-Updates / automatisierte Update-Prüfung
- Zweite Plattform (linux-x86_64 tarball) im selben Workflow
