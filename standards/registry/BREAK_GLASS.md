# BREAK_GLASS.md — Notfall-Bypass-Verfahren Branch Protection (atc-standards main)

```yaml
document_id: ATC-DOC-GOV-004
status: draft
standard: ATC-STD-MD-001
owner_review: erforderlich (dieser PR)
date: 2026-10-06
```

## Zweck

Dokumentiertes, kontrolliertes Ausnahme-Verfahren, wenn die Branch Protection
von main (enforce_admins + Required-PR-Reviews + 14 Required Checks) in einem
Notfall ausser Kraft gesetzt werden MUSS. Ersetzt den bisherigen
undokumentierten Admin-Bypass (F-160, Push 96d5f12).

## Verfahren (ausschliesslich Owner)

1. **Anfrage:** Begruendung mit Was/Warum/Warum-kein-PR (Eskalation E2).
2. **Dokumentation VORHER:** Expected Result und begrenztes Zeitfenster als AUD-Record (ATC-ENT-014).
3. **Ausnahme:** Owner deaktiviert enforce_admins temporär (GitHub UI oder Protection-API).
   Agenten-Token ist auf der Protection-API bewusst ohne Administration-Rechte (403 by design) — Agenten koennen kein Break-Glass.
4. **Ausnahme-Push** durch den Owner mit Break-Glass-Kennzeichnung im Commit-Trailer (Finding-ID: F-160, Action-ID: BG-NNN).
5. **Reaktivierung:** enforce_admins sofort wieder aktivieren; Verifikation GET enabled:true.
6. **AUD-Record:** WHO/WHAT/WHEN/WHERE/WHY/VERSION/RESULT.
7. **Post-hoc:** Finding (BUG-001) je Vorfall; Praeventiv-Kontrolle pruefen/verschaerfen.

## Pflichten

- Kein Agent darf Break-Glass ausloesen, durchfuehren oder vorbereiten.
- Jeder Break-Glass-Vorfall wird zwingend als Finding registriert und im naechsten Audit bewertet.
