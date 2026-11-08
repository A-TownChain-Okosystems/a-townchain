# Security Policy — atc-standards

**Vulnerability Reporting (PRIVATE DISCLOSURE, SCR-0052/P1-003):**
Security-Findings werden NICHT über GitHub Issues gemeldet (Issues sind
potenziell öffentlich). Stattdessen:

1. **GitHub Private Vulnerability Reporting** — Security-Tab →
   „Report a vulnerability" (privater Kanal, nur Owner/Maintainer sichtbar)
2. **Direkt an den Owner:** Michael Wroblewski (GitHub: ShivaCoreDev)

**Vulnerability-Lifecycle:**

```
Security Finding -> PRIVATE REPORT -> Owner/Maintainer -> Triage (P0-P3)
  -> Fix -> private Verifikation -> Disclosure-Entscheidung
  -> Security Advisory / CHANGELOG
```

**Supported Versions:** Nur der aktuelle main-Stand.
**Disclosure Policy:** Keine öffentlichen Exploit-Details vor Fix und
Disclosure-Entscheidung; Advisory ggf. mit CVSS und Mitigation.
**Security Scope:** Normative Regeln; Verstöße gegen ATC-STD-203 (Secrets,
unsichere Releases) werden als Governance-Verstoß behandelt.
**Known Limitations:** Markdown-Standards sind referenziell, nicht
maschinell erzwungen — Durchsetzung liegt in den Repositories/CI-Gates.
