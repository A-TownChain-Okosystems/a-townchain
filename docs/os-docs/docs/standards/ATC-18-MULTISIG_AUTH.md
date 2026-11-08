# ATC-18 — Multi-Signature Transaction Authorization Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.3 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-18
> **Tier:** 2 (Logik & Oekonomie)
> **Referenzen:** ATC-03 (DID/Identity), ATC-04 (DAG), ATC-05 (Quantum-Resistant), ATC-13 (Fractional), ATC-17 (DAO), ATC-19 (AMM, geplant), Issue #6 (ECDSA), #12 (Solidity)
> **Quelldatei:** Atc-18.docx (urspruengliche Spezifikation)
> **Kategorie:** Governance  

---

## Abstract

ATC-18 definiert das Multi-Signature Transaction Authorization Protocol. In einem
dezentralen Betriebssystem (KAI-OS) ist es oft notwendig, dass fuer kritische
Aktionen — wie das Aendern von Protokoll-Parametern, das Verschieben grosser
Asset-Mengen oder das Upgraden von Smart Contracts — nicht ein einzelner Key
ausreicht, sondern eine Gruppe von autorisierten Akteuren zustimmen muss.

> **ATC-18 = Vier-Augen-Prinzip auf der Blockchain.**
> ATC-11 bis 17 = Oekonomie, Assets, Mining, Wachstum, Governance (Tier 2).
> ATC-18 = Sicherheits-Protokoll fuer autorisierte Transaktionen.

---

## 1. Kernkonzepte

### 1.1 m-aus-n-Signatur-Logik
ATC-18 legt fest, dass ein Transaktions-Event erst dann in den DAG (ATC-04)
aufgenommen wird, wenn eine definierte Anzahl (m) von autorisierten privaten
Schluesseln aus einer definierten Gruppe (n) die Transaktion signiert hat.

**Implementation:** OK Implementiert
- `blockchain/contracts/` — MultiSig Wallet Funktionalitaet
- ECDSA (secp256k1) Signaturen in `ecdsa_impl.py` / `ecdsa_final.py`
- Issue #6 (ECDSA) abgeschlossen
- Wallet (`wallet.py`) unterstuetzt MultiSig-Adressen

```python
# MultiSig Logik (m-of-n)
# Beispiel: 3 von 5 Schluesseln muessen signieren
# tx = { data, signatures: [sig1, sig2, sig3], required: 3, total: 5 }
# Erst wenn >= 3 gueltige Signaturen vorliegen -> TX in DAG
```

### 1.2 Kryptografische Zusammenfuehrung (Threshold Signatures)
Anstatt mehrere einzelne Transaktionen zu senden, werden diese durch ATC-18 zu
einer einzigen, kompakten "Threshold-Signatur" zusammengefuehrt. Das spart
Speicherplatz auf der Chain und beschleunigt die Validierung.

**Implementation:** Teilweise implementiert
- Aktuelle MultiSig sammelt einzelne Signaturen (nicht Threshold-kompakt)
- ECDSA-Signaturen werden einzeln verifiziert
- **Geplant:** Threshold Signature Scheme (TSS) mit Shamir's Secret Sharing

> **Geplant:** TSS — Teil-Keys werden nie im Netzwerk exponiert. Signatur-
> Erstellung erfolgt verteilt ohne Offenlegung privater Schluessel.

### 1.3 Governance-Integration
ATC-18 ist eng mit der DAO (ATC-17) verknuepft. Oftmals fungiert die DAO selbst
als eine dieser autorisierten "Parteien". Eine ATC-18-Transaktion kann
beispielsweise erfordern, dass ein Administrator und ein Teil der
Community-Governance zustimmen.

**Bezug:** `governance_contract.py` (ATC-17) kann als einer der n-Signer
fungieren. DAO-Proposal -> MultiSig-Execution.

---

## 2. Warum ATC-18 fuer KAI-OS essenziell ist

### 2.1 Sicherheit fuer "Hohe Werte"
Fuer die Verwahrung von Liquiditaet in AMM-Pools (ATC-19) oder fraktionierten
Assets (ATC-13) bietet ATC-18 den notwendigen Schutz, damit ein kompromittierter
einzelner Key nicht zum Totalverlust fuehrt.

### 2.2 Protokoll-Stabilitaet
Upgrades des KAI-OS-Kerns duerfen niemals durch einen einzelnen Entwickler-Key
initiiert werden koennen. ATC-18 erzwingt hier ein "Four-Eyes-Principle" (oder
mehr), was das Risiko von boesartigen Protokoll-Aenderungen minimiert.

### 2.3 Autonome Agenten-Sicherheit
KI-Agenten, die ueber ATC-17 an Abstimmungen teilnehmen oder Assets bewegen,
koennen ueber ATC-18 so konfiguriert werden, dass sie fuer bestimmte Aktionen
eine zusaetzliche Freigabe durch den Besitzer benoetigen.

**Bezug:** AI-Kernel (`ai_kernel.py`) — KI-Agenten mit ATC-03 Identity.
MultiSig-Anforderung fuer kritische KI-Aktionen als zukuenftige Erweiterung.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-05 (Quantensichere Signaturen)
ATC-18 nutzt die PQC-Verfahren aus ATC-05, um auch die Multi-Signature-Struktur
quantensicher zu machen.

> ATC-05 ist PARTIAL KONZEPTIONELL — PQC-Algorithmen als Zukunftssicherung.

### 3.2 ATC-03 (Decentralized Identity)
Die Akteure, die in der n-Gruppe fuer eine ATC-18-Transaktion definiert sind,
werden ueber ihre Identitaeten nach ATC-03 referenziert.

### 3.3 ATC-17 (DAO Governance)
ATC-18 ist eng mit der DAO verknuepft — die DAO kann als autorisierte Partei
fungieren.

### 3.4 Issue #12 (Solidity Contracts)
Die Implementierung von Multi-Signature-Wallets in der EVM-Kompatibilitaets-
schicht des KAI-OS basiert auf diesem Standard.

**Status:** Issue #12 abgeschlossen. Solidity MultiSig in
`blockchain/contracts/solidity/contracts/KAIGovernance.sol`.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| m-of-n MultiSig | m Signaturen von n Keys | MultiSig Wallet, ECDSA | OK Implementiert |
| ECDSA-Signaturen | secp256k1 | ecdsa_impl.py, ecdsa_final.py | OK Implementiert |
| Threshold Signatures | Kompakte TSS-Signatur | Einzelsignaturen (noch nicht TSS) | PARTIAL Geplant |
| Shamir's Secret Sharing | Verteilte Schluesselteile | Nicht implementiert | PARTIAL Geplant |
| Governance-Integration | DAO als Signer | governance_contract.py | OK Implementiert |
| Solidity MultiSig | EVM-kompatibel | KAIGovernance.sol (Issue #12) | OK Implementiert |
| KI-Agent-MultiSig | KI braucht Besitzer-Freigabe | Nicht implementiert | PARTIAL Geplant |
| PQC MultiSig | Quantensichere MultiSig | ATC-05 konzeptionell | PARTIAL Geplant |

> **Fazit:** Die Kern-Funktionalitaet (m-of-n MultiSig, ECDSA, Governance-
> Integration, Solidity) ist voll implementiert. Threshold Signature Scheme
> (TSS) und KI-Agent-MultiSig sind als Evolution geplant.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #6 | ECDSA Implementation | Done | ATC-18 Signatur-Basis |
| #9 | Governance DAO | Done | ATC-18 DAO als Signer |
| #12 | Solidity Contracts | Done | ATC-18 EVM MultiSig |
| #39 | DAO-Governance Live | Done | ATC-18 DAO-Execution |
| #69 | Security-Audit | Open | ATC-18 MultiSig-Audit |
| Sprint 2.3 | Threshold Signature Scheme | Geplant | ATC-18 TSS |
| Sprint 2.3 | Shamir's Secret Sharing | Geplant | ATC-18 SSS |
| Sprint 2.3 | KI-Agent-MultiSig | Geplant | ATC-18 KI-Freigabe |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] Threshold Signature Scheme (TSS): Kompakte Signatur statt einzelner Sammlung
- [ ] Shamir's Secret Sharing: Verteilte Schluesselteile ohne Exposition
- [ ] PQC MultiSig: Quantensichere Multi-Signaturen (ATC-05)
- [ ] KI-Agent-MultiSig: KI-Agent braucht Besitzer-Freigabe fuer kritische Aktionen
- [ ] MultiSig-Explorer: Visualisierung der Signatur-Status
- [ ] Time-Locked MultiSig: Zeitverzoegerte Freigabe fuer kritische TXs
- [ ] MultiSig-Template: Standardisierte m-of-n Konfigurationen
- [ ] DAO + MultiSig: Automatische DAO-Approval als Teil der Signer-Gruppe
- [ ] AMM-Pool-Schutz: ATC-19 Liquiditaet durch MultiSig gesichert
- [ ] Fraktionierungs-Schutz: ATC-13 Vault durch MultiSig gesichert

---

## 7. Threshold Signatures — Technischer Detail

### Aktuelle MultiSig (implementiert):
```
TX = { data, signatures: [sig_A, sig_B, sig_C] }
-> Validator prueft jede Signatur einzeln
-> Benoetigt Speicher fuer alle m Signaturen auf Chain
```

### Threshold Signature Scheme (geplant):
```
TX = { data, threshold_sig }
-> TSS erzeugt eine einzige Signatur aus m Teil-Signaturen
-> Teil-Schluessel werden nie im Netzwerk exponiert
-> Shamir's Secret Sharing: Schluessel in n Teile geteilt, m reichen aus
-> Kompakter auf Chain, schnellere Validierung
```

### Sicherheitsvorteile TSS:
1. Keine Offenlegung einzelner Signaturen auf Chain
2. Geringere Speicherplatz-Pro-Transaktion
3. Schnellere Validierung (eine Signatur statt m)
4. Schutz gegen Signatur-Replay-Angriffe

---

*Dieses Dokument wurde aus der urspruenglichen Atc-18.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:05 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| submit_tx | 30000 |
| confirm | 20000 |
| execute_multisig | 50000 |

### Sprint-Zuweisung

- **Sprint 2.6** — #78
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

6+ Unit-Tests: Submit, Confirm, Execute, Revoke, Threshold, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
