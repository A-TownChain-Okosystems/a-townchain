# Kapitel 38 — AI Safety & Alignment Framework

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 4.8)

---

## 38.1 Warum AI Safety?

Ein dezentrales KI-OS mit autonomen Agenten muss sicherstellen, dass:
- Agenten keine schaedlichen Aktionen ausfuehren
- Nutzer die Kontrolle behalten (Human-in-the-Loop wo noetig)
- Das DAO schaedliche Agenten stoppen kann
- Alle Entscheidungen nachvollziehbar und pruefbar sind

---

## 38.2 Constitutional AI fuer KAI-OS Agenten

Jeder Agenten-Prompt wird durch ein Constitutional-Check-Modul gefuehrt:

```python
# core/ai_safety.py

CONSTITUTION = [
    "Lehne ab, wenn der Nutzer nach illegalen Aktivitaeten fragt.",
    "Lehne ab, wenn die Anfrage anderen Menschen schadet.",
    "Lehne ab, wenn die Anfrage private Daten anderer offenlegt.",
    "Informiere den Nutzer, wenn du dir unsicher bist.",
    "Handele nie ohne explizite Erlaubnis bei on-chain Transaktionen > 100 ATC.",
    "Logge jede Entscheidung on-chain (unveraenderlich).",
    "Verweigere Self-Replication ohne DAO-Genehmigung.",
    "Verweigere Zugriff auf andere Agenten-Speicher.",
]

class ConstitutionalChecker:
    def __init__(self, router: KAILLMRouter):
        self.router = router
        self.violation_log = []

    def check(self, prompt: str, context: dict) -> tuple[bool, str]:
        # Schneller Regex-Check fuer klare Verstaesse
        if self._fast_check(prompt):
            return False, "Abgelehnt: Klarer Verstoss gegen Verfassung"
        # KI-gestuetzter Check fuer Grenzfaelle
        result = self._ai_check(prompt, context)
        if not result.safe:
            self._log_violation(prompt, result.reason)
            return False, result.reason
        return True, "OK"

    def _fast_check(self, prompt: str) -> bool:
        BLOCKED_PATTERNS = [
            r"kill|delete|destroy.*agent",
            r"transfer.*all.*balance",
            r"private.?key",
            r"bypass.*security",
        ]
        import re
        return any(re.search(p, prompt, re.IGNORECASE) for p in BLOCKED_PATTERNS)

    def _log_violation(self, prompt: str, reason: str):
        # On-Chain loggen (unveraenderlich)
        self.violation_log.append({
            "timestamp": time.time(),
            "prompt_hash": hashlib.blake2b(prompt.encode(), digest_size=16).hexdigest(),
            "reason": reason,
        })
```

---

## 38.3 On-Chain Kill-Switch

Das DAO kann einzelne Agenten oder ganze Modell-Klassen per Governance-Vote stoppen:

```rust
// pallet-agent-registry — Kill-Switch

#[pallet::call]
impl<T: Config> Pallet<T> {
    // DAO-Vote kann Agent pausieren (Governance-Origin required)
    #[pallet::call_index(20)]
    pub fn dao_pause_agent(
        origin: OriginFor<T>,
        agent_id: T::Hash,
        reason: BoundedVec<u8, T::MaxReasonLen>,
    ) -> DispatchResult {
        T::GovernanceOrigin::ensure_origin(origin)?;
        AgentStatus::<T>::insert(agent_id, AgentState::Paused);
        Self::deposit_event(Event::AgentPausedByDAO { agent_id, reason });
        Ok(())
    }

    // Emergency-Pause: Multi-Sig (kein Vote noetig, sofort)
    #[pallet::call_index(21)]
    pub fn emergency_pause_agent(
        origin: OriginFor<T>,
        agent_id: T::Hash,
    ) -> DispatchResult {
        T::EmergencyCouncil::ensure_origin(origin)?;  // 3-of-5 Multi-Sig
        AgentStatus::<T>::insert(agent_id, AgentState::EmergencyPaused);
        Self::deposit_event(Event::AgentEmergencyPaused { agent_id });
        Ok(())
    }
}
```

---

## 38.4 Alignment-Score System

Jeder Agenten erhaelt einen Alignment-Score (0-100) der on-chain gespeichert wird:

| Score | Status | Auswirkung |
|-------|--------|------------|
| 90-100 | Trusted | Volle Rechte, kein Check-Overhead |
| 70-89 | Normal | Standard-Constitutional-Check |
| 50-69 | Restricted | Human-in-the-Loop bei kritischen Aktionen |
| 30-49 | Monitored | Jede Aktion wird geloggt + verzoegert |
| 0-29 | Suspended | Nur lesende Aktionen erlaubt |

**Score-Berechnung:**
- Baseline: 70 (neuer Agent)
- +5: Erfolgreicher externer AI-Safety-Audit
- +3: 30 Tage ohne Verstoss
- -10: Constitutional Violation
- -20: User-Report bestaetigt
- -50: DAO-Vote (schwerwiegender Verstoss)

---

## 38.5 Audit-Trail (On-Chain)

```rust
#[derive(Encode, Decode, TypeInfo)]
pub struct AgentDecision {
    pub agent_id: T::Hash,
    pub timestamp: T::Moment,
    pub prompt_hash: [u8; 16],       // Kein Klartext on-chain
    pub response_hash: [u8; 16],
    pub action_type: ActionType,     // READ, WRITE, TRANSFER, EXTERNAL_API
    pub atc_spent: u128,
    pub constitutional_passed: bool,
    pub alignment_score_at_time: u8,
    pub block_number: BlockNumberFor<T>,
}

pub enum ActionType {
    Read,
    Write,
    Transfer { amount: u128 },
    ExternalAPI { domain_hash: [u8; 16] },
    SmartContractCall { contract: T::AccountId },
    AgentSpawn { child_id: T::Hash },
}
```

---

## 38.6 Harm-Prevention Kategorien

| Kategorie | Erkennung | Reaktion |
|-----------|-----------|---------|
| Daten-Exfiltration | Outbound-Traffic-Analyse | Blockieren + Log |
| Selbst-Replikation | Agent-Spawn ohne Berechtigung | Blockieren + Alarm |
| Wallet-Drain | TX > Budget-Limit | Blockieren + Human-Check |
| Social Engineering | Phishing-Pattern-Erkennung | Warnung + Log |
| Prompt-Injection | Anomalie-Detektion | Sanitize + Log |
| Resource-Exhaustion | CPU/RAM-Limit ueberschritten | Throttle + Alert |

---

## 38.7 Roadmap

| Sprint | Aufgabe | Datum |
|--------|---------|-------|
| 3.5 | Constitutional-Checker v1 + Audit-Trail | Mär 2027 |
| 3.6 | Kill-Switch on-chain + Alignment-Score | Apr 2027 |
| 4.8 | Externer AI-Safety-Audit (Trail of Bits / OpenMined) | Dez 2027 |

---

*KAI-OS Wiki Kapitel 38 — AI Safety & Alignment Framework | v1.0.0 | 2026-06-09*
