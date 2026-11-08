---
standard:
  id: ATC-AURORA-SWARM-001
  title: "Aurora Swarm Network (ASN) — Neuronales KI-Schwarmnetzwerk: Neuron-Modell, Synapse-Protokoll, Schwarm-Kognition, Evidence-getriebene Verifikation und Authority-Integration"
  version: "0.1.0"
  status: candidate
  category: aurora-swarm
  authority: A-TownChain-Okosystems
  owner: Michael (Owner-Entwurf)
  created: "2026-10-07"
  updated: "2026-10-07"
  normative: false
  supersedes: []
  superseded_by: null
  effective_date: "offen — 0.1.0-DRAFT, §9-Freigabe ausstehend"
  review_date: "2027-10-07"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "aurora-ai, globus-os, atc-shivacore, atc-vm, a-townchain (Integration Layer) — Zielarchitektur"
---

# ATC-AURORA-SWARM-001 — Aurora Swarm Network (ASN) v0.1.0-DRAFT

> **Status:** CANDIDATE (Owner-Entwurf 07.10.2026, SCR-0138) — **nicht normativ**; §9-Freigabe nach ATC-STD-000 ausstehend.
> **Reifeklasse:** RESEARCH (Zielarchitektur). Keine Aussage dieses Standards ist IMPLEMENTED; sämtliche Komponenten unterliegen der Evidence-Klassifikation NORMATIVE -> IMPLEMENTED -> RESEARCH -> LEGACY/UNVERIFIED gemäß ATC_ARCHITECTURE_REFERENCE_MODEL.
> **Block:** ATC-AURORA-SWARM (Aurora Swarm Network) · **Versionierung:** unabhängig von Protokoll- und Implementierungs-Versionen (ATC-STD-VERSION-001).

## Abstract

ATC-AURORA-SWARM-001 spezifiziert das **Aurora Swarm Network (ASN)**: eine verteilte kognitive Infrastruktur, in der viele unabhängige KI-Neuronen sich dynamisch zu überprüfbaren Expertenschwärmen zusammenschalten. Der Standard definiert Neuron-Modell, Synapse-Protokoll, Discovery, Routing, Expert-Koalitionen, verteiltes Reasoning, Schwarm-Konsens, Evidence-Protokoll, Verified Memory, Reputation, Selbstheilung, Privacy, TEE/Attestation, die ABI-Ebene (Model/Agent/Swarm) sowie die Authority-Integration über GlobusOS und ShivaCore. Vorbild-Architekturprinzipien (Bittensor-Subnets/Routing, Gensyn P2P/Verifikation) sind als Forschungsreferenz gekennzeichnet, nicht als normative Abhängigkeit.

## Scope

**Gilt für:** Zielarchitektur des aurora-ai-Repository (Runtime-Kern) und dessen Weiterentwicklung zum Schwarmnetzwerk; Schnittstellen zu globus-os (Authority Plane), atc-shivacore (Hardware), atc-vm (Model ABI-Kandidat), A-TownChain (optionale Settlement/Trust-Ebene).
**Nicht-Gilt:** Mainnet-Betrieb (ATC-STD-NET); bestehende AI-Services- und Agent-Runtime-Teile von aurora-ai in ihrem aktuellen Zustand (die bleiben aurora-ai-eigene Roadmap); biologische Verhaltensimitation — übernommen werden Architekturprinzipien, kein Imitat.
Normative Aussagen sind erst nach §9-Freigabe verbindlich (RFC 2119: MUSS/SOLLTE/DARF).

## 0. Schichtmodell und Abgrenzung

| Ebene | Name | Inhalt |
|-------|------|--------|
| L4 | SWARM COGNITION | Consensus, Reasoning, Planning, Coalition |
| L3 | AI EXECUTION | Models, Agents, Tools, Inference, Training |
| L2 | NEURAL NETWORK | P2P, Synapses, Routing, Discovery, Messaging |
| L1 | TRUST & EVIDENCE | Identity, Attestation, Capability, Verification |
| L0 | HARDWARE | CPU, GPU, NPU, TEE, Storage, Network |

Separater **Control Plane**: Identity, Capability, Policy, Governance, Audit, Evidence, Observability — konsistent zur X-Control-Plane-Struktur des Ecosystems.

Terminologie (verbindlich nach Freigabe): **Neural Node** = AI-Teilnehmer (Neuron); **Neural Network** = Gesamtheit der kooperierenden Teilnehmer (Schwarm). Ein Neuron ist nicht zwingend ein künstliches Neuron im Deep-Learning-Sinn.

## 1. Neuron Model

Jeder Teilnehmer ist ein Aurora-Neuron mit strukturierter Beschreibung (id: node_id/identity_key; cognition: model/model_hash/context_window/reasoning_capabilities; capabilities: vision/language/coding/mathematics/planning/tool_use; resources: cpu/gpu/npu/memory/tee; state: local_memory/swarm_memory/reputation; network: peers/protocols; verification: attestation/execution_evidence).

- REQ-ASW-001: Jedes Neuron MUSS eine eindeutige node_id und einen identity_key besitzen; Identität erfolgt nach ATC-AAS-001 (Agent Identity) bzw. dessen Schwarm-Erweiterung.
- REQ-ASW-002: Ein Neuron MUSS seine Capabilities deklarieren (vision/language/coding/mathematics/planning/tool_use); unvollständige Deklaration DARF nur mit Passiv-Status geschehen.
- REQ-ASW-003: Ein Neuron DARF seine Ressourcen (inkl. TEE-Verfügbarkeit) reklamieren; Falsch-Reklamation MUSS über Attestation (§14) erkennbar und mit Reputationsverlust (§11) sanktioniert sein.

## 2. Synapse Protocol

Die Synapse ist die typisierte Verbindung zwischen Neuronen — kein einfaches Agent-Messaging. Synapse-Felder: source, destination, task (task_id, type), context (references), payload (data), capability (required), policy (constraints), evidence (parent_hash).

- REQ-ASW-004: Synapsen MÜSSEN Task-, Context-, Payload-, Capability-, Policy- und Evidence-Felder transportieren; reine Text-Weitergabe ist keine Synapse.
- REQ-ASW-005: Synapse-Nachrichten MÜSSEN dem ATC-Protokoll-Envelope (ATC-STD-PROTOCOL-001, 9-Felder-Nachrichtenformat) folgen; die Familie erhält die Protokoll-ID `ATC-PROTO-SWARM-*` mit eigenständiger Nummerierung.
- REQ-ASW-006: Jede Synapse MUSS eine parent_hash-Evidence-Kette führen (Aufgaben-Ursprungsverfolgbarkeit).

## 3. Swarm Discovery & Capability Advertisement

- REQ-ASW-007: Discovery MUSS anhand strukturierter Capability-Advertisement erfolgen (Fähigkeit, Spezialisierung, Reputation, Kosten, Latenz).
- REQ-ASW-008: Advertisement MUSS widerrufbar sein; veraltete Advertisements MÜSSEN nach TTL als ungültig gelten.
- REQ-ASW-009: Discovery DARF dezentral (Gossip/DHT-artig) oder über Registry-Server geschehen; der Coordinator (§6) DARF ausfallen, ohne Discovery dauerhaft zu blockieren.

## 4. Swarm Routing

- REQ-ASW-010: Ein Swarm Router MUSS Anfragen anhand von Capability-Encoding an geeignete Neuronen oder Koalitionen routen (Mixture-of-Experts-Prinzip; Routingmodelle wie Bittensor als Forschungsreferenz).
- REQ-ASW-011: Routing MUSS neben Fachlichkeit Kosten, Latenz, Reputation und Auslastung berücksichtigen.
- REQ-ASW-012: Bei Ausfall eines Neurons MUSS die Aufgabe automatisch neu geroutet werden (Selbstheilung, §12); kein Single-Point-of-Failure im Routing-Pfad.

## 5. Expert Coalition

- REQ-ASW-013: Für komplexe Anfragen MUSS der Router eine Expert-Koalition (mehrere spezialisierte Neuronen) bilden können; die Koalitionsbildung MUSS dokumentiert werden (wer, warum, welcher Anteil).
- REQ-ASW-014: Koalitionen MÜSSEN parallele Inferenz und Cross-Check zwischen Mitgliedern unterstützen.

## 6. Coordinator-Rolle

- REQ-ASW-015: Der Aurora Coordinator IST KEIN Gottknoten: Er KANN sterben, ersetzt werden oder ausfallen; sein Verlust DARF nur Latenz, niemals Aufgabenverlust bedeuten (Task-Graph persistent).
- REQ-ASW-016: Coordinator MUSS austauschbar sein (keine nicht-replizierbare Zustände).

## 7. Distributed Reasoning

- REQ-ASW-017: Problemzerlegung MUSS als Task Graph erfolgen: Decomposition -> Task Graph -> Specialist Nodes -> Parallel Reasoning -> Aggregation.
- REQ-ASW-018: Jeder Reasoning-Schritt MUSS seinem Ursprungs-Neuron und deren Evidence-Eintrag zuordenbar sein.

## 8. Swarm Consensus

- REQ-ASW-019: Der Schwarm MUSS für Ergebnisse einen Konsensprozess definieren; einfache Mehrheitsentscheidung reicht NICHT.
- REQ-ASW-020: Ergebnisbewertung MUSS mindestens gewichten: Accuracy, Evidence, Reputation, Specialization, Verification, Cost, Latency.
- REQ-ASW-021: Dissens MUSS sichtbar bleiben (Minderheitenergebnisse werden nicht verworfen, sondern als Dissens protokolliert).

## 9. Evidence Protocol (Evidence-Weighted Cognition)

- REQ-ASW-022: Jedes Ergebnis MUSS einen Evidence Score erhalten aus mindestens: Model Identity, Input Hash, Execution Hash, Tool Evidence, Source Evidence, Verification, Node Reputation.
- REQ-ASW-023: Aus der rohen AI-Antwort wird erst über Evidence-Evaluation eine Verified Intelligence; unverifizierte Antworten DÜRFEN nicht als final zertifiziert werden.
- REQ-ASW-024: Execution-Evidence-Verifikation SOLLTE den Gensyn-Verde-Forschungsansatz nutzen (divergierenden Berechnungsschritt identifizieren statt Gesamtberechnung wiederholen) — RESEARCH, bis ein konkreter Mechanismus normativ festgelegt ist.
- REQ-ASW-025: Evidence-Records folgen ATC-AAS-010 (Evidence-Pflicht) und AUD-Record-Kopplung (AAS-018 Audit Trail).

## 10. Verified Memory

Drei Memory-Ebenen: Local Memory (ein Node), Swarm Memory (geteilt im Schwarm), Network Memory (globale, verifizierte Wissenseinträge).

- REQ-ASW-026: NICHT jede AI-Ausgabe darf globales Wissen werden: Candidate -> Evidence -> Validation -> Consensus -> Verified Memory.
- REQ-ASW-027: Network-Memory-Einträge MÜSSEN einen Evidence-Record und Provenance (Erzeuger-Neuron, Eingaben) tragen.

## 11. Node Reputation

- REQ-ASW-028: Reputation MUSS aus messbarer Historie entstehen (Erfolg/Failure, Evidence-Qualität, Attestation-Ergebnisse); manuelle Reputationsvergabe ist unzulässig.
- REQ-ASW-029: Reputation MUSS in Routing (§4) und Konsens-Gewichtung (§8) einfließen.

## 12. Failure & Recovery (Selbstheilender Schwarm)

- REQ-ASW-030: Neuron-Ausfall MUSS automatisch zur Neu-Routung führen (Capability Discovery -> Ersatz-Auswahl -> Fortsetzung).
- REQ-ASW-031: Der Schwarm MUSS Timeouts, Retry-Strategien und Circuit-Breaker je Synapse unterstützen (gemäß ATC-STD-PROTOCOL-001 Fehlerbehandlung).

## 13. Privacy & Data Locality

- REQ-ASW-032: Datennähe VOR Transport: Aufgaben SOLLTEN dorthin routen, wo die Daten liegen, wenn Policies dies erfordern.
- REQ-ASW-033: Policy-Constraints der Synapse MÜSSEN vom Ziel-Neuron erzwungen werden können; Verstöße MÜSSEN auditierbar sein.

## 14. TEE & Attestation

- REQ-ASW-034: Neuronen MIT TEE MÜSSEN Remote-Attestation vor Critical-Task-Ausführung unterstützen; Attestationsergebnis ist Teil des Evidence-Records.
- REQ-ASW-035: Nicht-attestierte Neuronen DÜRFEN an nicht-kritischen Aufgaben teilnehmen, MÜSSEN aber im Evidence-Record als nicht-attestiert gekennzeichnet sein.

## 15. Model ABI / Agent ABI / Swarm ABI

- REQ-ASW-036: Die ABI-Ebene MUSS drei Schnittstellen definieren: Model ABI (Modellausführung, atc-vm als Kandidat), Agent ABI (Agenten-Runtime, kompatibel zur AAS-Familie), Swarm ABI (Schwarm-Kontrakte: Discovery, Routing, Konsens, Memory).
- REQ-ASW-037: Änderungen an einer ABI MÜSSEN via SCR/Versionierung erfolgen; die drei ABIs versionieren unabhängig voneinander und unabhängig von dieser Spezifikation.

## 16. Hierarchical Swarms (Swarm-to-Swarm)

- REQ-ASW-038: Ein Schwarm DARF selbst als Neuron eines größeren Schwarms auftreten (Micro-Swarm -> Domain-Swarm -> Global-Swarm); hierarchische Schwarm-Topologie MUSS explizit spezifiziert sein.
- REQ-ASW-039: Swarm-Grenzen MÜSSEN im Synapse-Routing erkennbar sein (keine versehentliche Ebenen-Vermischung).

## 17. Aurora Authority Plane (GlobusOS / ShivaCore Integration)

Sicherheitsgrenze wird NICHT aufgeweicht: AURORA SWARM (AI Neurons / AI Agents) -> Authority Plane (Capability Check) -> GlobusOS (IPC / Policy) -> ShivaCore (Hardware).

- REQ-ASW-040: KEIN Schwarm-Neuron erhält direkten ShivaCore-Zugriff; ein Neuron erhält HÖCHSTENS eine Capability (z. B. network.read, filesystem.read, gpu.compute, camera.read) und NUR innerhalb der Policy.
- REQ-ASW-041: GlobusOS bleibt die Autoritätsschicht; der Schwarm ist Nutzer der Capability-API, nicht deren Umgehung. Verstoß = Governance-Finding (F-Serie).
- REQ-ASW-042: ShivaCore bleibt quelleigene Kernel-Ebene; Schwarm-Integration erfolgt ausschließlich über die bestehenden Schichten (globus-os -> atc-shivacore), keine direkte Kopplung (ATC-STD-201ff Repository-Trennung, ATC-STD-204 Dependency-Tracking).

## 18. Control Plane & Governance

- REQ-ASW-043: Identity, Capability, Policy, Governance, Audit, Evidence, Observability werden als separater Control Plane geführt und MÜSSEN mit der bestehenden Governance (ATC-STD-000, AAS, ENT, ai-gov) konsistent sein.
- REQ-ASW-044: Schwarm-Operationen von ATC-Agenten unterliegen ATC-AAS-014 (Security) und ATC-STD-SC-020 (AI-Assisted Development: kein Agent-Deploy ohne Gates).

## 19. A-TownChain Settlement / Trust (OPTIONAL)

- REQ-ASW-045: Value-Transport, Staking-artige Reputation-Bindung oder On-Chain-Settlement von Schwarm-Ergebnissen DARF als optionale Erweiterung spezifiziert werden; normative Pflicht besteht NICHT. Nutzung erst nach NET-Freigaben (ATC-STD-NET).

## 20. Swarm Learning

- REQ-ASW-046: Der Schwarm MUSS Routing-Feedback aus Verification-Ergebnissen lernen können (welche Neuronen eignen sich für welche Aufgaben) — als Meta-Lernen des Schwarms, getrennt vom Modelltraining der einzelnen Neuronen.
- REQ-ASW-047: Lern-Veränderungen am Routing MÜSSEN versioniert und auditierbar sein; kein unüberwachtes Selbst-Verhalten im produktiven Schwarm.

## Verweise & Abhängigkeiten

- Verfassung: ATC-STD-000 (§9 Freigabe, §30 Immutabilität, §37/38)
- Agentenbetrieb: ATC-AAS-001 (Identität), -005 (Discovery), -007 (Task), -010 (Evidence), -018 (Audit Trail), -024 (A2A), -025 (Repo-Manifest)
- Protokolle: ATC-STD-PROTOCOL-001 (Envelope, Fehler, Kompatibilität), ATC-STD-PROTOCOL-002 (Conformance), -003 (Security)
- Architektur/Governance: ATC_ARCHITECTURE_REFERENCE_MODEL (Evidence-Klassen), ATC-STD-204 (Dependency/Interface), ATC-STD-201ff
- Forschungsreferenzen (NICHT normativ): Bittensor (Subnets, Miner/Validator-Neuronen, Routing/Coalition-Ansätze), Gensyn (P2P, persistente Identität, kryptographische Verifikation, Verde-Forschung)

## Gates & Umsetzungsfolge

- **ASW-G0 (Spezifikations-Freeze):** §9-Freigabe durch Owner; danach §30-eingefroren, Änderungen nur via SCR.
- **ASW-G1 (Referenz-Neuron):** Eine Referenz-Implementierung eines Neurons mit Synapse-Minimalprototyp im aurora-ai-Repository (RESEARCH-Status, klar gekennzeichnet).
- **ASW-G2 (Conformance):** CONF-SWARM-001-Plan gemäß ATC-STD-PROTOCOL-002, Stufe BRONZE vor jedem produktiven Einsatz.
- **ASW-G3 (Authority-Integration):** Nachweis Capability-Durchgriff über globus-os; ShivaCore-isoliert.

## Änderungshistorie

| Version | Datum | Änderung |
|---------|-------|----------|
| 0.1.0 | 2026-10-07 | Owner-Entwurf Michael (SCR-0138): Erstfassung — Neuron/Synapse/Routing/Konsens/Evidence/Memory/Authority; Status CANDIDATE, §9-Freigabe offen |
