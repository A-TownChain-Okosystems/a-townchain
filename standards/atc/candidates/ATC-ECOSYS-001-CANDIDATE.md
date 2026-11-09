# ATC-ECOSYS-001-CANDIDATE — Ecosystem Interaction Standard

**Status:** KANDIDAT · nicht normativ · keine Registry-Bindung
**Datum:** 2026-11-09 · **Geltung nach** Ratifizierung in `atc-standards`

## 1. Zweck

Regelt, wie die sieben Kern-Domänen und die Infrastruktur-Domänen miteinander
sprechen. Er erfindet **keine** neuen Schnittstellen: jeder Vertrag wird einer
bereits existierenden Norm oder einem Crate zugeordnet. Wo heute nichts
existiert, wird das als Lücke markiert, nicht als Entwurf.

## 2. Domänen (keine Hierarchie)

Kern: ShivaCore, GlobusOS, A-TownChain, ATCLang, ATC-VM, Aurora AI, Genesis.
Infrastruktur: Cryptography, Update, Identity, Security, Networking, Storage,
SDK, Model ABI. Governance: `atc-standards` (normativ), `a-townchain-ecosystem`
(Integration, Evidence).

## 3. Bestehende Verträge und ihre Besitzer

| Vertrag | Ort | Stand |
|---|---|---|
| Code-Admission | `ATC-SEC-EXEC-001` + `components/exec-admission` | Kandidat, 21 Tests |
| Update/Artefakt | `components/update-authority` (UPD-001…010) | gebaut, 17 Tests |
| Determinismus | `ATC-14` | Norm |
| Bytecode-Format | `ATC-BC-001` (BE eingefroren) | Norm |
| Hash/Primitive | `ATC-HASH-001`, `ATC-05` | Norm, F-067 offen |
| Syscall-ABI | ABI 2.0.0, `libshivacore` | Norm + Code |
| Boot | `SHIVA-BOOT-001` | Norm |
| Kernel-Reuse | `SHIVA-KERNEL-REUSE` | Norm |
| Identität | `ATC-03` + `node/src/did.rs` | Norm, Code im Node |
| Knotenprotokoll | `ATC-01` | Norm |

Lücken ohne Vertrag: **Model ABI**, **Cryptography-Fassade**, **Networking-Transport**.

## 4. Normative Regeln

- **ECO-001 Eine Autorität je Fähigkeit.** Wer Signaturprüfung braucht, ruft sie
  auf; er implementiert sie nicht neu. Doppelimplementierung ist ein Befund.
- **ECO-002 Kanten sind endlich.** Kommunikation nur über die in §5 gelisteten
  Kanten. Eine neue Kante ist eine Änderung, kein Nebeneffekt.
- **ECO-003 Kein transitives Vertrauen.** Signatur ≠ Autorisierung, Build ≠
  Ausführung, Inferenz ≠ Erlaubnis. Jede Kante prüft selbst.
- **ECO-004 Richtung ist deklariert.** Wer ruft, wer antwortet, wer besitzt die
  Entscheidung. Zyklen sind verboten, ausgenommen der dokumentierte
  Kernel↔Service-Space-Zyklus.
- **ECO-005 KI erhält keine Kernel-Rechte.** Aurora bekommt Capabilities, nie
  Kerneladressraum.
- **ECO-006 Konsens ist KI-frei.** Kein Konsenspfad darf Aurora, Genesis oder
  Modelle aufrufen.
- **ECO-007 Signierte Nutzlast ist kanonisch.** Domain-Präfix, Längenpräfixe je
  Feld, Ganzzahlen statt Gleitkomma. Muster: `ATC-EXEC-ADM-v1`, `ATC-RCT-v1`,
  `ATC-UPDATE-v1`.
- **ECO-008 Version vor Nutzlast.** Der Empfänger kennt die Version, bevor er
  fremde Bytes interpretiert (Muster: ABI 2.0, Größe vor Inhalt).
- **ECO-009 Fail-closed.** Kein Trust-on-first-use, kein Rückfall auf unsigniert.
- **ECO-010 Jede Kante braucht Evidence.** Test + Nachweis, Status nach
  IMPLEMENTED → TESTED → CI_VERIFIED → E2E_VERIFIED → VERIFIED.
- **ECO-011 Neue Schnittstelle = Änderungsverfahren.** SCR, kein stilles Verdrahten.
- **ECO-012 Der Mirror führt keine Autorität.** Kanonisch ist das Produkt-Repo.

## 5. Kantenkatalog (Ist-Zustand, gemessen)

Belegt: Kernel ← 15 system-Crates über `libshivacore`; Service Space → Kernel;
node → algorithm, wallet; sdk → node; ecosystem/integration → node, sdk,
atc-blockchain.

Fehlend und damit priorisiert: **node → atc-vm** (0 Treffer, Verträge laufen
nicht), **atclang → atc-vm** (kein CI-Aufruf), **OS → init → globus-runtime**
(Image ohne Nutzlast), **Aurora → Model ABI** (existiert nicht).

## 6. Security Lifecycle (Prüfreihenfolge)

Identität → kryptografische Prüfung → Capability → Admission → Isolation →
Audit. Die Tiefe ist objektabhängig: Treiber, Vertrag und KI-Agent haben
verschiedene Vertrauensmodelle; ein Vertrag braucht keinen Kernel-Cap-Check,
ein Treiber keine Gasprüfung.

## 7. Offene Entscheidungen vor Ratifizierung

1. Model ABI: eine Schnittstelle für alle Beschleuniger oder je Backend?
2. Cryptography: Fassade über bestehende Primitive oder Neuschnitt?
3. Networking: bleibt `net` im Kernel-Crate (AD-012-Verstoß) oder zieht es in
   den Service Space?
4. Wird ECO-002 mit Kantenliste oder mit Rollenmodell normiert?
5. Wo lebt der Model-ABI-Code: `aurora-ai`, `atc-standards` oder neues Modul?
