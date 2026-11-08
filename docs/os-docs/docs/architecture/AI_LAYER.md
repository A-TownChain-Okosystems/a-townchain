# AI Layer — aktuelle Architekturgrenze

> **Aktualisiert:** 2026-10-09  
> **Status:** Architektur-Referenz; keine Implementierungs- oder Produktionsfreigabe.

## Einordnung

Diese Datei ersetzt die früheren Sprint-3.2-Aussagen vom 2026-07-05 als aktuelle Statusquelle. Die früheren Modulnamen, Zeilenzahlen, „55 %“-Angabe und die Behauptung, die gesamte AI-Schicht sei in ATCLang implementiert, sind historische Angaben und müssen gegen die aktuellen Quell-Repositories geprüft werden.

Die kanonische AI-/Agenten-Schicht liegt in [`aurora-ai`](https://github.com/A-TownChain-Okosystems/aurora-ai). `atclang` ist die Sprache/Compiler-Schicht und keine Quelle für die gesamte Aurora-AI-Runtime. KI-, Federated-Learning-, Biometrie- oder „Consciousness“-Features gelten nicht als implementiert, nur weil sie in einem alten Architekturentwurf beschrieben sind.

## Sicherheits- und Autoritätsgrenzen

```text
Benutzerintention / Systemereignis
             ↓
Aurora AI — Vorschlag / Planung / Inferenz
             ↓
Policy-Prüfung
             ↓
Capability-Check + erforderliche Freigabe
             ↓
Tool-/Service-Aufruf mit begrenzten Rechten
             ↓
Autoritativer Service / Runtime
```

- Aurora AI ist **nicht Teil des ShivaCore TCB**.
- Aurora darf keine Kernel-Rechte, Chain-Regeln, Konsensentscheidungen oder VM-Semantik eigenmächtig überschreiben.
- Autoritative Aktionen müssen durch Policy, Capability-Prüfung und die zuständige Runtime-/Service-Grenze laufen.
- On-chain Audit, Federated Learning, biometrische Authentifizierung und Modell-Fallbacks sind nur dann als implementiert zu deklarieren, wenn die aktuelle Implementierung und passende Tests/Evidence das für einen exakten SHA belegen.

## Source of truth

- Aurora AI: [Repository](https://github.com/A-TownChain-Okosystems/aurora-ai)
- OS-/Service-Grenze: [GlobusOS](https://github.com/A-TownChain-Okosystems/globus-os)
- ShivaCore TCB: `globus-os/modules/atc-shivacore/kernel/`
- Normative Anforderungen: [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards)

## Verifikation

`IMPLEMENTED` bedeutet nicht `VERIFIED`. Verifikation erfordert relevante Evidenz auf dem exakt beanspruchten Commit-SHA (Run → Job → Step → Exit-Status/Logs). Alte Sprintzahlen und historische Testresultate sind keine aktuelle Freigabe.
