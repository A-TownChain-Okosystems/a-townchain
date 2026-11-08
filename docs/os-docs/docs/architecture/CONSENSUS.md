# Konsens — kanonische Zuständigkeit und aktueller Evidenzstatus

> **Aktualisiert:** 2026-10-09  
> **Status:** Architekturhinweis; diese Datei ist keine normative Konsens-Spezifikation.

## Kanonische Quelle

Die kanonische Algorithmus-/Konsensimplementierung liegt in `a-townchain/components/algorithm` innerhalb des [`a-townchain`](https://github.com/A-TownChain-Okosystems/a-townchain)-Repositories. Das separate [`atc-algorithm`](https://github.com/A-TownChain-Okosystems/atc-algorithm)-Repository ist unterstützendes Spezifikations-/Governance-/Entwicklungsmaterial und darf nicht als konkurrierende Produktionsimplementierung dargestellt werden.

## Historische Angaben, nicht als aktuelle Protokollparameter verwenden

Die frühere Fassung behauptete eine finalisierte Kombination aus PoH + PoW + PoS und nannte unter anderem 10-Sekunden-Blöcke, Halving alle 210.000 Blöcke und 50 ATC Start-Reward. Diese Angaben stammen aus einem älteren Python-Beispiel und sind **nicht** als aktuelle, kanonische Protokollparameter zu behandeln.

Diese Datei entscheidet nicht, ob PoH, PoW, PoS, PoI oder eine Kombination davon finalisiert ist. Die tatsächlich geltenden Regeln müssen aus dem kanonischen Algorithmus-/Chain-Code und den genehmigten normativen Standards abgeleitet werden. Wo diese Quellen keinen eindeutig genehmigten Stand belegen, ist die Konsensauswahl als **nicht abschließend verifiziert** zu kennzeichnen.

## Implementierungs- und Verifikationsstatus

- Ein Python-Pseudocodebeispiel oder ein deterministisch wirkender lokaler Algorithmus belegt keine sichere, verteilte Konsensimplementierung.
- Determinismus, Fork-Choice, Validator-Auswahl, Sybil-/Grinding-Schutz, Slashing, Finalität, Reorg-Verhalten, Netzwerkpartitionen und Recovery benötigen jeweils geeignete Tests und Evidenz.
- `IMPLEMENTED` und `VERIFIED` sind getrennte Zustände.
- Eine grüne Einzelprüfung oder ein früherer Audit-Score belegt keine Produktionsreife oder Mainnet-Freigabe.

## Nachweisregel

Für einen Verifikationsclaim müssen Quell-SHA, Workflow-Run, Job, Step, Exit-Status und relevante Logs zusammenpassen. Fehlgeschlagene Determinism-Gates bleiben offen, bis die Ursache korrigiert und auf dem neuen SHA erneut geprüft wurde.

Normative SSOT: [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards).
