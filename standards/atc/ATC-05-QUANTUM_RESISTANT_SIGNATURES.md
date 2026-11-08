# ATC-05 — Quantum-Resistant Cryptographic Signatures

> **Issue:** #69 | **Wiki:** Kap.25 | **Sprint:** 3.3
> **Status:** 📐 FINAL — Spezifikation vollständig, Implementation geplant in Sprint 2.4 | **Version:** 1.0.0 | **Datum:** 04.07.2026> **Autor:** Michael Wroblewski, Aurora (Superagent)
> **Standard-ID:** ATC-05
> **Referenzen:** ATC-03 (DID & Zero-Trust), ATC-11 (Assets, geplant), ATC-12 (Assets, geplant), ATC-31/32 (KI-Agenten Signaturen, geplant), Issue #6 (ECDSA)
> **Quelldatei:** Atc-05.docx (urspruengliche Spezifikation)
> **Kategorie:** P2P Networking  
> **Tier:** Tier 1 — Blockchain Infrastructure  

---

## Abstract

ATC-05 widmet sich einem der kritischsten Sicherheitsaspekte im KAI-OS: den
Quantum-Resistant Cryptographic Signatures (quantenresistenten kryptografischen
Signaturen).

In einer Welt, in der die Rechenleistung von Quantencomputern exponentiell
zunimmt, koennten klassische Verschluesselungsverfahren (wie die weit
verbreiteten ECDSA-Signaturen, die aktuell in Standard-Blockchains verwendet
werden) in Zukunft theoretisch "geknackt" werden. ATC-05 ist die präventive
Antwort darauf.

> **Merksatz:** ATC-01 = Verbindung. ATC-02 = Gedaechtnis. ATC-03 = Identitaet.
> ATC-04 = Parallele Wahrheit. ATC-05 = Quantensichere Integritaet.

---

## 1. Kernkonzepte

### 1.1 Post-Quanten-Kryptografie (PQC)
ATC-05 schreibt den Einsatz von Algorithmen vor, die auch bei Verfuegbarkeit
leistungsfaehiger Quantencomputer als sicher gelten. Dies betrifft insbesondere
den Signatur-Algorithmus, der beweist, dass eine Transaktion vom rechtmässigen
Besitzer stammt.

**Kandidaten-Algorithmen:**
- **Lattice-based** (CRYSTALS-Dilithium, FALCON) — NIST PQC Standardisierung
- **Hash-based** (SPHINCS+) —_stateless, lange bewährt
- **Code-based** (Classic McEliece) — konservativ, grosse Keys
- **Multivariate** (Rainbow) — klein Signaturen, grosse Keys

**Aktueller Stand:** Das System nutzt ECDSA (secp256k1) als Basis-Schicht
(Issue #6, `blockchain/wallet/ecdsa.py`). PQC-Algorithmen sind als
Hybrid-Signaturen (ECDSA + PQC) fuer die naechste Evolutionsstufe geplant.

### 1.2 Zukunftssicherheit
Da Betriebssysteme (wie KAI-OS) eine lange Lebensdauer haben, muss die
Sicherheit heute bereits gegen die Bedrohungen von morgen ausgelegt sein.
ATC-05 sorgt dafuer, dass die digitalen Identitaeten und Assets im Netzwerk
nicht durch zukuenftige technologische Durchbrueche entwertet werden.

### 1.3 Algorithmen-Agilitaet
Der Standard ist so definiert, dass das zugrundeliegende kryptografische
Verfahren bei Bedarf aktualisiert werden kann, ohne das gesamte Protokoll neu
schreiben zu muessen (sogenannte "Hybrid-Signaturen").

**Konzept:**
- Signatur-Typ-Feld in Transaction-Format (ATC-0003)
- Versionierbare Signatur-Algorithmen mit Fallback
- Hybrid-Modus: ECDSA (kompatibel) + PQC (zukunftsicher) parallel
- Migration ohne Protocol-Break

---

## 2. Warum ATC-05 fuer KAI-OS essenziell ist

### 2.1 Schutz der Assets
Alle Vermoegenswerte, die in KAI-OS (gemaess ATC-11 und ATC-12) abgebildet
werden, verlassen sich auf die Unveraenderlichkeit und Integritaet der
digitalen Signaturen. ATC-05 schuetzt diese Assets langfristig.

**Bezug:** ATCoin (`atcoin.py`, ATC-89 Standard, 21M Max Supply), NFTs
(ATC-90), MultiSig Wallet (`multisig.py`) — alle basieren auf ECDSA.

### 2.2 Vertrauen in das "KI-Gehirn"
Wenn KI-Agenten Inferenz-Auftraege signieren (siehe ATC-31/32), muss
sichergestellt sein, dass diese Signaturen faelschungssicher sind. Ein
Quanten-Angriff koennte sonst die Identitaet eines Agenten uebernehmen und das
gesamte System korrumpieren.

**Bezug:** AI-Kernel (`ai_kernel.py`) loggt KI-Entscheidungen mit Hashes
on-chain. Die Signatur dieser Entscheidungen muss langfristig sicher sein.

### 2.3 Regulatorische Anforderungen
Die Einhaltung von ATC-05 positioniert das A-TownChain-Oekosystem als
"Enterprise-Ready" fuer sicherheitskritische Anwendungsbereiche (Finanzen,
KI-Infrastruktur), wo PQC zunehmend zur Standardanforderung wird.

---

## 3. Zusammenhang mit anderen Standards

### 3.1 ATC-03 (Decentralized Identity)
Die Identitaet eines Nutzers ist nur so sicher wie die Signatur, die sie
legitimiert. ATC-05 ersetzt oder ergaenzt hier die schwaecheren, klassischen
Signatur-Algorithmen.

**Implementation:** `signature_verify.py` Middleware verifiziert aktuell
ECDSA-Signaturen. Im PQC-Modus wuerde zusaetzlich ein PQC-Verify-Step
eingefuegt.

### 3.2 Issue #6 (ECDSA Signatur)
Interessanterweise ist in der Roadmap aktuell der Uebergang zu ECDSA
(secp256k1) als Basis-Schritt priorisiert. ATC-05 stellt die langfristige
Evolution dar, um ueber ECDSA hinauszugehen, sobald die Standardisierung
fuer PQC in der Laufzeitumgebung (Tier 3) vollstaendig ausgereift ist.

**Status:** Issue #6 ist abgeschlossen (ECDSA implementiert). ATC-05 baut
darauf auf als naechste Sicherheits-Evolutionsstufe.

---

## 4. Spezifikation vs. Implementation

| Komponente | Spezifikation (docx) | Implementation | Status |
|------------|---------------------|----------------|--------|
| PQC-Algorithmus | Lattice-based / Hash-based | ECDSA (secp256k1) als Basis | PARTIAL Basis da, PQC geplant |
| Zukunftssicherheit | Schutz gegen Quanten-Angriffe | ECDSA anfaellig fuer Shor-Algorithm | PARTIAL Konzeptionell |
| Algorithmen-Agilitaet | Hybrid-Signaturen, austauschbar | ECDSA fix in TX-Format | PARTIAL Konzeptionell |
| KI-Agenten-Signaturen | Quantensichere Inferenz-Signaturen | On-Chain Hash-Logging (ECDSA) | PARTIAL Basis da |
| Asset-Schutz | Langfristige Integritaet | ATCoin/NFT/MultiSig mit ECDSA | PARTIAL Basis da |
| Regulatorische Compliance | Enterprise-Ready PQC | ECDSA (Standard) | PARTIAL Geplant |

> **Fazit:** ECDSA (secp256k1) ist als Basis voll implementiert (Issue #6).
> ATC-05 definiert den **Evolutionspfad zu Post-Quanten-Kryptografie**.
> Die NIST PQC-Standardisierung (CRYSTALS-Dilithium, FALCON, SPHINCS+) ist
> weit fortgeschritten — sobald stabile Python-Implementierungen verfuegbar
> sind, kann der Hybrid-Modus (ECDSA + PQC) aktiviert werden.

---

## 5. Evolutionspfad: ECDSA zu PQC

```
AKTUELL (v3.2.1)              ZUKUNFT (v4.0+)
ECDSA (secp256k1)        ->   Hybrid (ECDSA + Dilithium)
                            -> Voll-PQC (Dilithium/FALCON)

Schutz: Klassisch              Schutz: Klassisch + Quanten
Kompatibel: Ja                 Kompatibel: Ja (Fallback)
Geschwindigkeit: Schnell       Geschwindigkeit: Etwas langsamer
Key-Size: 64B (priv)           Key-Size: ~1-3KB (PQC)
```

### Migrationsstrategie:
1. **Phase 1:** Signatur-Typ-Feld in Transaction-Format (ATC-0003) ergaenzen
2. **Phase 2:** PQC-Bibliothek evaluieren (python-oqs / pyca PQCrypto)
3. **Phase 3:** Hybrid-Signatur: TX enthaelt ECDSA + PQC Signatur
4. **Phase 4:** PQC-Verify in Gateway Middleware ergaenzen
5. **Phase 5:** Optional Voll-PQC (ohne ECDSA-Fallback)

---

## 6. Roadmap-Referenzen

| Issue | Titel | Status | Verbindung |
|-------|-------|--------|------------|
| #6 | ECDSA-Implementation | Done | ATC-05 Basis-Schicht |
| #24 | MultiSig Wallet | Done | ATC-05 M-of-N + PQC-Potenzial |
| #69 | Security-Audit | Open | ATC-05 PQC-Bewertung |
| Sprint 2.4 | PQC-Hybrid-Signaturen | Geplant | ATC-05 Vollimplementation |
| Sprint 2.4 | Dilithium/SPHINCS+ Integration | Geplant | ATC-05 Algorithmus |
| Sprint 2.4 | Signatur-Typ-Feld in TX-Format | Geplant | ATC-05 Agilitaet |

---

## 7. Verbesserungsvorschlaege (Zukunft)

- [ ] Signatur-Typ-Feld (1 Byte) in ATC-0003 Transaction-Format aufnehmen
- [ ] python-oqs (Open Quantum Safe) als Abhaengigkeit evaluieren
- [ ] CRYSTALS-Dilithium3 als primaerer PQC-Algorithmus (NIST empfohlen)
- [ ] SPHINCS+ als konservative Alternative (hash-based, keine Annahmen)
- [ ] Hybrid-Signatur-Modus: ECDSA + PQC parallel in TX
- [ ] PQC-Verify in `signature_verify.py` Middleware ergaenzen
- [ ] Key-Generation fuer PQC in `keygen.py` (neben ECDSA)
- [ ] MultiSig mit PQC-Signaturen (M-of-N, PQC-basiert)
- [ ] Benchmark: ECDSA vs. PQC Sign/Verify Performance
- [ ] PQC-Migration-Tool: Bestehende ECDSA-Keys zu PQC-Keys ueberleiten

---

*Dieses Dokument wurde aus der urspruenglichen Atc-05.docx Spezifikation
abgeleitet und mit der tatsaechlichen Code-Implementation abgeglichen.*
*Letzte Aktualisierung: 04.07.2026 21:27 | Aurora (Superagent)*


---

## Ergänzung — Gas-Costs, Testing & Sprint-Zuweisung

> **Aktualisiert:** 05.07.2026 | **Roadmap v2.0**

### Gas-Cost Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| dilithium_sign | 8000 |
| dilithium_verify | 12000 |
| keygen | 15000 |

### Sprint-Zuweisung

- **Sprint 2.6** — #78
- **Roadmap v2.0** — Task zugewiesen
- **Priorität:** HIGH

### Testing

5+ Unit-Tests: KeyGen, Sign, Verify, Tampering-Detection

### Coverage-Ziel: 90%+

---

*Ergänzung: Aurora (MasterBrain) · 05.07.2026 · Roadmap v2.0*
