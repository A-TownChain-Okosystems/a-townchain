# Contributing — atc-engineering

1. Änderungen folgen dem SCR-Prozess (atc-standards) für normative Teile.
2. Implementation entlang ROADMAP-Phasen; Commits referenzieren die Phase.
3. Vor jedem Push: `cargo fmt`, `cargo clippy -D warnings`, `cargo test`.
4. Kein `unwrap()` in Policy-/Audit-/Evidence-kritischem Code.
5. Evidence-Regel: CLAIMED != PASS — Status-Änderungen nur mit Evidenz.
