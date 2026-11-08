# Implementierungs-Roadmap und Prioritäten

**Status:** PLANNED — keine Ausführungsevidenz. Maschinenlesbare Quelle: `docs/architecture/implementation-roadmap.v1.json`.

## Prioritäten

- **P0 – Blockierende Grundlagen/Sicherheit:** SSOT-Zuständigkeiten, Kernel-Sicherheitsinvarianten, normative Schnittstellen, Sicherheitsgrenzen und Evidence-Infrastruktur.
- **P1 – Kanonische Laufzeit:** ShivaCore, ATC-VM, Blockchain-Kern, Konsensentscheidung und reproduzierbare Toolchain.
- **P2 – Integration:** ATC-Lang, GlobusOS, Node-Netzwerk/Operations und systemübergreifende Tests.
- **P3 – Produktfunktionen:** Aurora-AI und Genesis Engine hinter Policy-, Capability- und Service-Grenzen.
- **P4 – Härtung/Release:** Fuzzing, Security Review, Traceability, SBOM, Rollback und Release-Evidenz.

## Reihenfolge

1. **P0:** SSOT-Konflikte auflösen, CI/Evidence absichern, kritische ShivaCore-TCB-Blocker schließen und normative Schnittstellen festlegen.
2. **P1:** Kanonischen Kernel, ATC-VM, Blockchain-Kern und reproduzierbare Toolchain nachweisen; Konsensstatus explizit entscheiden.
3. **P2:** ATC-Lang an den kanonischen VM-Target anbinden, ShivaCore in GlobusOS integrieren und End-to-End-Tests ausführen.
4. **P3:** Aurora-AI und Genesis Engine gemäß ihren Abnahmekriterien umsetzen; optionale Integration nur über least-privilege Service-APIs.
5. **P4:** Fuzzing, Security Review, Traceability, Provenance/SBOM und Rollback-Evidenz abschließen.

## Verbindliche Regeln

- Abhängigkeitsgesteuerte Planung; keine Kalendertermine ohne gemessene Baseline.
- Parallelisierung nur bei geklärten Schnittstellen und unabhängigen Abhängigkeiten.
- Kanonische Implementierungen bleiben in `a-townchain/components/vm/`, `a-townchain/components/algorithm/` und `globus-os/modules/atc-shivacore/kernel/`.
- Anforderungen bleiben REQUIRED/PLANNED, bis Code und Integration nachgewiesen sind.
- VERIFIED benötigt exakten SHA plus Run → Job → Step → Exit/Logs/Artefakte.
- Keine automatischen Merges und keine abgeschwächten Gates.
- Sicherheitskritische Residuals blockieren abhängige Phasen.
