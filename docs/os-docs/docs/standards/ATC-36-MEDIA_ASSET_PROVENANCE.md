# ATC-36 — Decentralized Media Asset & Content Provenance Protocol
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 3.0 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-36
> **Tier:** 5 (User & Application Layer)
> **Referenzen:** ATC-03 (Identity), ATC-04 (DAG), ATC-26 (XAI), ATC-29 (Marketplace), ATC-90 (NFT Standard), Issue #11 (NFT/Breeding), #13 (Marketplace)
> **Quelldatei:** Atc-36.docx (urspruengliche Spezifikation)
> **Kategorie:** Applications & Privacy  

---

## Abstract

ATC-36 definiert das Decentralized Media Asset & Content Provenance Protocol.
Wir befinden uns hier im Tier 5 (User & Application Layer). In einer Welt, in
der KI-generierte Inhalte (Text, Bild, Video) taeuschend echt sein koennen, ist
ATC-36 das Protokoll, das die Herkunft, Authentizitaet und Urheberschaft von
Medieninhalten innerhalb des KAI-OS-Oekosystems sicherstellt.

> **ATC-36 = Echtheitsnachweis fuer Medien — Schutz vor Deepfakes.**
> ATC-35 = Daten anonymisieren. ATC-36 = Inhalte verifizieren.

---

## 1. Kernkonzepte

### 1.1 Content-Hashing & On-Chain Signaturen
Jedes Medien-Asset erhaelt bei der Erstellung oder dem Upload eine digitale
Signatur, die mit der Identitaet (DID nach ATC-03) des Erstellers verknuepft
ist. Dieser Fingerabdruck wird unveraenderlich im Ledger gespeichert.

**Implementation:** PARTIAL — Basis da
- SHA-256 Hashing fuer Datei-Integritaet
- ECDSA (secp256k1) Signaturen fuer Authentifizierung
- ATC-04 DAG fuer On-Chain Speicherung
- ATC-90 NFT Standard fuer digitale Assets
- **Geplant:** Medien-Asset-spezifische Content-Hashing Pipeline

### 1.2 KI-Provenienz-Tagging
ATC-36 schreibt vor, dass Inhalte, die durch KI-Agenten (Tier 4) generiert
wurden, ein obligatorisches "AI-Generated"-Tag im Metadaten-Header tragen.
Dieses Tag enthaelt zudem den kryptografischen Hash des verwendeten KI-Modells
(aus der Registry nach ATC-29).

**Implementation:** Nicht implementiert. Konzept fuer AI-Generated-Tag geplant.

```python
# GEPLANT: Content Provenance Metadata
{
  asset_hash: "sha256(...)",
  creator_did: "ATC-DID:...",
  signature: "ECDSA(...)",
  ai_generated: true,
  model_hash: "ATC-MODEL-abc123...",  # ATC-29 Registry
  created_at: "2026-07-04T22:51:00Z",
  parent_asset: "sha256(parent...)",  # Provenance
  modifications: [...]
}
```

### 1.3 Content-Weg-Verfolgung (Provenance Tracking)
Sollte ein Inhalt modifiziert werden (z. B. eine KI bearbeitet ein Bild),
speichert ATC-36 die Historie dieser Transformationen. Das Ergebnis ist eine
Art "digitaler Stammbaum", der zeigt, welcher Agent welche Aenderungen an
welchem Original-Asset vorgenommen hat.

**Bezug:** ATC-04 DAG — Content-Historie als Transaktionskette.
ATC-26 (XAI) — KI-Entscheidungen bei Content-Generierung nachvollziehbar.

---

## 2. Warum ATC-36 fuer KAI-OS essenziell ist

### 2.1 Schutz vor Deepfakes
Nutzer koennen sich innerhalb des KAI-OS darauf verlassen, dass Medieninhalte
mit einem validen ATC-36-Nachweis authentisch sind. Faelschungen ohne diesen
Nachweis koennen sofort als "unverifiziert" markiert werden.

### 2.2 Urheberrecht fuer Franchise-KI
Im Bereich der "Social-Media-KI fuer Franchise" sorgt ATC-36 dafuer, dass die
Rechte an den von der KI generierten Inhalten sauber dem jeweiligen Franchise-
Inhaber zugewiesen werden koennen.

**Bezug:** ATC-90 NFT Standard — Urheberschaft als NFT.
ATC-13 (Fractional) — Co-Ownership von Content.

### 2.3 Transparenz
Werbe-Netzwerke und Plattform-Adapter nutzen ATC-36, um sicherzustellen, dass
nur lizenzierte und authentische Inhalte veroeffentlicht werden, was die
Qualitaet der Netzwerke massiv steigert.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-03 (Decentralized Identity)
Die DID des Erstellers ist das Fundament fuer die Signatur nach ATC-36.

### 3.2 ATC-29 (Marktplatz)
Assets, die in den Marktplatz eingestellt werden, muessen ATC-36-konform sein,
um als "verifiziertes Asset" gehandelt werden zu duerfen.

> ATC-29 = Wo Modelle gehandelt werden. ATC-36 = Wo deren Outputs verifiziert werden.

### 3.3 ATC-26 (Explainable AI)
Der XAI-Trace einer KI kann als Teil der ATC-36-Metadaten hinterlegt werden, um
genau zu erklaeren, wie ein spezifischer Content-Asset entstanden ist.

> ATC-26 = Warum die KI entschied. ATC-36 = Was die KI produzierte + Stammbaum.

### 3.4 ATC-90 (NFT Standard)
ATC-90 definiert NFTs. ATC-36 erweitert dies um Content-Provenance und
AI-Generated-Tags.

### 3.5 ATC-35 (Data Privacy)
ATC-35 anonymisiert Daten. ATC-36 verifiziert Content-Echtheit. Beides
ergaenzt sich: Privacy fuer Inputs, Provenance fuer Outputs.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| Content-Hashing | SHA-256 + DID Signatur | SHA-256 + ECDSA | OK Implementiert |
| On-Chain Signatur | DAG-verankert | ATC-04 DAG | OK Implementiert |
| NFT-Verknuepfung | Asset als NFT | ATC-90 implementiert | OK Implementiert |
| AI-Generated-Tag | Obligatorisch fuer KI | Nicht implementiert | PARTIAL Geplant |
| Model-Hash-Referenz | ATC-29 Registry Hash | ATC-29 konzeptionell | PARTIAL Geplant |
| Provenance Tracking | Digitaler Stammbaum | Nicht implementiert | PARTIAL Geplant |
| Modifikations-Historie | Transformations-Log | Nicht implementiert | PARTIAL Geplant |
| Deepfake-Schutz | Unverifiziert markieren | Nicht implementiert | PARTIAL Geplant |
| Urheberrecht-Zuweisung | Franchise-Inhaber | Nicht implementiert | PARTIAL Geplant |
| XAI-Metadaten | ATC-26 Trace im Asset | ATC-26 konzeptionell | PARTIAL Geplant |

> **Fazit:** Die kryptografische Basis (SHA-256, ECDSA, DAG, NFT) ist voll
> implementiert. Die ATC-36-spezifischen Features (AI-Generated-Tag,
> Provenance Tracking, Modifikations-Historie, Deepfake-Schutz) sind geplant.

---

## 5. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #6 | ECDSA Implementation | Done | ATC-36 Signatur-Basis |
| #11 | NFT/Breeding | Done | ATC-36 NFT-Verknuepfung |
| #13 | Marketplace | Done | ATC-36 Verifizierte Assets |
| #69 | Security-Audit | Open | ATC-36 Provenance-Sicherheit |
| Sprint 3.0 | AI-Generated-Tag | Geplant | ATC-36 KI-Provenienz |
| Sprint 3.0 | Provenance Tracking | Geplant | ATC-36 Digitaler Stammbaum |
| Sprint 3.0 | Deepfake-Schutz | Geplant | ATC-36 Unverifiziert-Markierung |
| Sprint 3.0 | Urheberrecht-Franchise | Geplant | ATC-36 IP-Zuweisung |

---

## 6. Verbesserungsvorschlaege (Zukunft)

- [ ] AI-Generated-Tag: Obligatorisch fuer alle KI-produzierten Inhalte
- [ ] Model-Hash-Referenz: ATC-29 Registry Hash in Metadaten
- [ ] Provenance Tracking: Digitaler Stammbaum aller Modifikationen
- [ ] Modifikations-Historie: Transformations-Log im DAG
- [ ] Deepfake-Schutz: Unverifizierte Inhalte markieren
- [ ] Urheberrecht-Zuweisung: Franchise-Inhaber als IP-Owner
- [ ] XAI-Metadaten: ATC-26 Trace im Content-Asset
- [ ] Content-Lizenz: On-Chain Nutzungsrechte (ATC-29 Marketplace)
- [ ] External-Verification: Echtheit auch ausserhalb KAI-OS beweisbar
- [ ] Content-Explorer: Visualisierung des Stammbaums im Explorer (#5)
- [ ] Watermark-Integration: Unsichtbare Wasserzeichen + On-Chain Hash
- [ ] Content-Revocation: Erststeller kann Content zurueckziehen

---

## 7. Content Provenance Pipeline

```
ERSTELLER             ATC-36                    ATC-04 (DAG)
  |                     |                          |
  | Upload Medien-Asset |                          |
  | (Bild/Text/Video)   |                          |
  |-------------------->|                          |
  |                     | 1. Content-Hash (SHA-256)|
  |                     | 2. DID-Signatur (ECDSA)  |
  |                     | 3. AI-Generated? -> Tag  |
  |                     | 4. Model-Hash (ATC-29)   |
  |                     | 5. Parent-Asset? -> Tree |
  |                     |                          |
  |                     | On-Chain Verankerung     |
  |                     |------------------------->|
  |                     |                          | Block N
  |                     |    Asset-Proof           |
  |                     |<-------------------------|
  | Verifizierungs-ID   |                          |
  |<--------------------|                          |
```

### Digitaler Stammbaum:
```
Original-Asset (User A, SHA-256, DID-A)
  |
  +-- Modifikation 1 (AI-Agent, Model-Hash-X)
  |   |-- AI-Generated-Tag: true
  |   |-- XAI-Trace: "Filter applied: Neon"
  |   |-- Parent: Original-Asset
  |
  +-- Modifikation 2 (AI-Agent, Model-Hash-Y)
      |-- AI-Generated-Tag: true
      |-- XAI-Trace: "Style transfer: Cyberpunk"
      |-- Parent: Modifikation 1
```

---

## 8. Tier 5 Fortschritt

```
TIER 5 — USER & APPLICATION (ATC-32+)
├── ATC-32  UX & Interface Abstraction    PARTIAL
├── ATC-33  AI Feedback & RLHF            PARTIAL KONZEPTIONELL
├── ATC-34  Cross-Layer Interop (CLIP)    PARTIAL
├── ATC-35  Data Privacy & Anonymization  PARTIAL KONZEPTIONELL
└── ATC-36  Media Asset & Provenance      PARTIAL
```

> KAI-OS hat nun ein robustes Framework gegen Desinformation und fuer den
> Schutz des geistigen Eigentums.

---

*Dieses Dokument wurde aus der urspruenglichen Atc-36.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 22:51 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| register_asset | 5000 |
| verify_provenance | 200 |
| transfer_right | 1000 |

### Sprint-Zuweisung

- **Sprint 3.1** — geplant
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** MEDIUM

### Testing

4+ Unit-Tests: Register, Verify, Transfer, Edge-Cases

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
