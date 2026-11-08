# Monitoring & DevOps — Architektur

> **Stand:** 05.07.2026 | **Sprint:** 3.0 | **Standards:** ATC-24

## Übersicht

Das Monitoring-System umfasst Prometheus-Metriken, System-Health-Checks, Alerting und BigQuery-Analytics-Pipeline — alles in ATCLang.

## Module

| Modul | Datei | Zeilen | Beschreibung |
|-------|-------|--------|--------------|
| Monitor | monitoring/monitor.atc | 212 | System health, metrics, alerting |
| Prometheus | monitoring/prometheus_metrics.atc | 201 | Prometheus + Grafana export |
| BigQuery Pipeline | tools/bigquery_pipeline.atc | 134 | GitHub metrics, blockchain stats |
| Issue Summary | tools/atc_issues_summary.atc | 211 | GitHub issue tracking, sprint summary |

## Monitoring Stack

### Prometheus + Grafana
- Gauges: Active nodes, block height, TPS, memory
- Counters: Total transactions, blocks mined, slashes
- Histograms: Block time distribution, latency
- Alert Rules: Node down, high latency, low TPS
- Dashboard Builder: Auto-generates Grafana JSON

### System Health Checks
- CPU, Memory, Disk, Network
- Node connectivity, peer count
- Consensus participation rate
- Block propagation latency

## CI/CD Pipeline (Sprint 2.7 — Blocked)

- **#79:** CI/CD Fix — `npm ci` vs `npm install` (Branch-Protection blockiert)
- CodeQL Workflow — repariert
- GitHub Pages Deploy — ausstehend
- **Lösung:** Michael muss `.github/workflows/ci-cd.yml` manuell pushen

---

*Monitoring & DevOps Architecture · Sprint 3.0 · 05.07.2026 · Aurora*
