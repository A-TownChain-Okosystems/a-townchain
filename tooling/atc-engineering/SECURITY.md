# Security Policy — atc-engineering

## Scope

atc-engineering verarbeitet Org-Metadaten, CI-Ergebnisse und Policies. Es
führt **keine Secrets**, keine Keys, keine Chain-Assets. Sicherheitsklasse S2,
Criticality C2 (Registry: ATC-REPO-ENG-001).

## Prinzipien

- Fail-closed: Policy-/Audit-Engine verweigern bei unklarem Zustand (BLOCK),
  niemals stillschweigendes PASS.
- Kein `unwrap()` in Policy-/Audit-/Evidence-kritischem Code (Owner-Regel).
- Secrets ausschließlich über Environment/GitHub Secrets — nie committet,
  nie geloggt.
- GitHub-Tokens: minimalster Scope (read:org, repo für Audit-Zugriff);
  Schreib-Token nur für den Owner.

## Meldungen

Security-Findings über Issues mit Label `security` oder direkt an
<michael.worob@gmail.com>. Severity-Modell und Reaktionszeiten: ATC-AI-GOV-001
(P0 BLOCK bis P3 TRACK).

## Audit

Dieses Repo unterliegt dem Org-Audit (V-01..V-16, GATE ab Score 85).
Cargo-Audit-Gate analog atc-vm (RustSec) ist in Phase 1 vorgesehen.
