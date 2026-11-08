# Rollback-Runbook: atc-standards Registry (MAINT-018 §1/§2, MAINT-000 §7.2)

> **Zustand:** GETESTET (2026-10-07, SCR-0130) — `rollback.available: true`,
> `rollback.tested: true` (MAINT-018 §2 Regel 1).

## Geltungsbereich

Rollback des Registry-Stands (Standards, Records, Views, Lock) von `atc-standards`
bei fehlerhafter Freigabe, Korruptem Registry-Zustand oder Fehl-Commit auf main.

## Verfahren A — Rollback des letzten Registry-Commits (Standardfall)

```bash
git fetch origin && git checkout main && git rebase origin/main   # sauberer Start
git checkout -b rb-<datum>-<szenario>                              # Scratch-Branch
git revert --no-edit <commit-sha>          # Content-Commit; Merge-Commit: -m 1
python3 tools/atc-std-validator/validate_all.py                    # RESULT pruefen
# Bei gruen: PR mit Revert-Commit oder Direktpush (Owner-Bypass) auf main
```

**Test 2026-10-07 (Evidenz):** Revert von `493df45` (Draft-Standard-Zufügung)
konfliktfrei (1 Datei, 157 Löschungen); Validator lief mit grünen
Selbsttests; Restore verifiziert (main: `RESULT: ALL COMPLIANT`).

## Verfahren B — Tiefe Rollbacks (>1 Monat / über mehrere Merges)

**Befund (Test 2026-10-07):** Revert von `9e3ea0f` (SCR-0124, ~3 Wochen zurück,
überlagert von Folgeschritten) erzeugt Konflikte in AGENT_MANIFEST, README,
STATUS, registry.lock, standards.yaml. Konflikt-Auflösung ist NICHT
mechanisierbar.

Deshalb verbindlich:
1. Kein Tiefe-Revert ohne Review jedes Konflikts (M1-Klassifizierung).
2. Alternativpfad: `git reset --hard <tag>` auf Scratch-Branch, Validator,
   dann als Revert-Commit dokumentieren; anschließend Folgeschritte
   selektiv re-applizieren (`git cherry-pick`).
3. Danach zwingend: Regeneration der Views (`tools/gen_views/`) und
   `registry.lock`-Aktualisierung, erneut validieren.

## Verfahren C — Restore-Zustand verifizieren

Nach jedem Rollback:
```bash
python3 tools/atc-std-validator/validate_all.py        # muss ALL COMPLIANT sein
grep -c "status: approved" registry/standards.yaml    # erwarteter Zaehler pruefen
```

## Verantwortung

- Ausführung: Engineering (Aurora) · Freigabe Rollback-Commit: Owner
- Klassifizierung: M1-Operational; bei Konsens-/Governanz-Korruption: M3.
