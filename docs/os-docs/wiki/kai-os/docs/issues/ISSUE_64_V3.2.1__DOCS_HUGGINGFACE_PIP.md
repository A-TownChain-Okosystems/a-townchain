# Issue #64 — v3.2.1 — Docs: HuggingFace Pipeline GitHub Actions Workflow

> **Status:** CLOSED | **Erstellt:** 2026-06-12 | **Labels:** enhancement, ai, priority:medium, devops
> **GitHub:** https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/64

---

## Beschreibung

## Beschreibung
`tools/hf_review_pipeline.py` ist fertig, aber es fehlt der GitHub Actions Workflow, der
automatisch bei jedem PR einen Code-Review durchführt.

## Aufgaben
- [ ] `.github/workflows/hf_code_review.yml` erstellen
- [ ] Trigger: `pull_request` auf `main` Branch
- [ ] Job: `python tools/hf_review_pipeline.py --pr $PR_NUMBER --post`
- [ ] Secret: `HUGGING_FACE_ACCESS_TOKEN` in GitHub A

---

## Aufgaben

- [ ] Implementierung
- [ ] Tests
- [ ] Dokumentation
- [ ] Code-Review

---

## Notion-Querverweis

- **Master Roadmap:** [Notion Roadmap](https://app.notion.com/p/Master-Roadmap-Synced-67-Issues-All-Closed-373b826db85c8125ba83f04995191bf0)

---

*Auto-generiert von Aurora v3.2 · 05.07.2026*
