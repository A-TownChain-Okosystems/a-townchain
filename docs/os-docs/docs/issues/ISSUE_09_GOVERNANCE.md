> **Status (05.07.2026):** Implementiert in ATCLang (.atc). Siehe STANDARDS_REGISTRY.md für Details.

# 📄 Issue #9 — Governance Contract (ATC-9900 DAO)

> **Labels:** enhancement · blockchain · governance · priority:medium
> **Priorität:** 🟡 Medium · **Milestone:** v2.2.0
> **Referenz:** [GitHub Issue #9](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/9)

---

## Ziel

Implementierung des **ATC-9900 Governance Standards** — dezentrale Abstimmung über Protokoll-Änderungen, Parameter-Updates und Ökosystem-Entscheidungen.

---

## Governance-Mechanismus

```
ATC-Holder reicht Proposal ein
          │
          ▼
Voting-Phase (default: 7 Tage)
  └─ Jeder ATC-Holder kann abstimmen
  └─ 1 ATC = 1 Stimme (oder gestakt: 1.5x Gewicht)
          │
          ▼
Quorum erreicht? (mind. 10% der Total Supply)
  ├─ Nein → Proposal FAILED (Quorum nicht erreicht)
  └─ Ja   → Mehrheit > 50%?
              ├─ Ja  → Proposal PASSED → Execute
              └─ Nein→ Proposal REJECTED
```

---

## Datenmodell

```python
@dataclass
class Proposal:
    proposal_id:  str         # "GOV-" + SHA-256[:12]
    creator:      str         # ATC-Adresse
    title:        str
    description:  str
    options:      list[str]   # z.B. ["Ja", "Nein", "Enthaltung"]
    votes:        dict        # option → total_atc
    voters:       dict        # address → option (verhindert Doppelabstimmung)
    created_at:   int
    deadline:     int         # Unix-Timestamp
    quorum:       float       # Mindest-Beteiligung (default: 0.10 = 10%)
    status:       str         # "active" | "passed" | "failed" | "rejected"
    executed_at:  int | None
```

---

## Contract-Methoden

```python
class GovernanceContract:

    def create_proposal(self, creator, title, description,
                        options, duration_days=7) -> dict: ...
    # Erstellt neuen Proposal, zieht 100 ATC Einreichungsgebühr

    def vote(self, voter, proposal_id, option) -> dict: ...
    # Stimmt ab — Voting Power = ATC Balance

    def execute_proposal(self, proposal_id) -> dict: ...
    # Nach Deadline: wertet aus, setzt Status

    def get_proposals(self, status=None) -> list: ...
    def get_proposal(self, proposal_id) -> dict: ...
    def get_voting_power(self, address) -> dict: ...
    # Gibt Balance + Stake-Bonus zurück
```

---

## Aufgaben

- [ ] `blockchain/contracts/governance/governance_contract.py`
- [ ] `backend/api/routes/governance_routes.py` — REST API
- [ ] Automatische Proposal-Auswertung nach Deadline (Cron)
- [ ] Delegated Voting: Stimmrecht übertragen
- [ ] Frontend: Governance-Seite (Sidebar `🏛 Governance`)
- [ ] Proposal erstellen, abstimmen, Ergebnisse als Chart
- [ ] Gateway Route `/api/governance/` registrieren
- [ ] Tests: `tests/test_governance.py`

---

## Akzeptanzkriterien

- [ ] Proposal erstellen und abstimmen funktioniert
- [ ] Quorum-Check korrekt (10% der Total Supply)
- [ ] Doppelabstimmung verhindert
- [ ] Status nach Deadline automatisch gesetzt
