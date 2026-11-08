# ATC Security & Technology Evidence Store (ATC-GATE-SEC-001 §4)

Zentraler Evidence-Bereich des Assurance-Systems (SCR-0094). Struktur:

security/ · technology/ · dependencies/ · vulnerabilities/ · fuzzing/ ·
audits/ · penetration-testing/ · releases/

Grundsätze (verbindlich, ATC-GATE-SEC-001):
- **Keine künstlichen PASS-Nachweise.** Ein fehlender Test bleibt NOT VERIFIED
  und wird nicht durch Dokumentation ersetzt (ATC-STD-018 §9).
- Evidence MUSS versioniert und einem Repository/Release zuordenbar sein.
- Schwachstellen-Details MÜSSEN zugriffsbeschränkt behandelt werden —
  öffentlich nur Metadaten/Aggregate.
- Evidence-Promotion erfolgt PR-basiert (SCR-0087-Muster) bzw. mit
  Task-Identity-Trailer.
