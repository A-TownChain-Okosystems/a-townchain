spec:
  id: ATC-SPEC-001
  title: "Prompt Engineering Theory: Das APOS/ACE-Framework"
  version: "1.0.0"
  status: draft
  category: ai
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: false
---

# ATC-SPEC-001 — Prompt Engineering Theory: Das APOS/ACE-Framework

> Ein guter Prompt ist keine Zauberformel, sondern eine praezise Beschreibung
> der Struktur kognitiver Operationen. Prompt Engineering ist kein
> Template-Handwerk, sondern eine Ingenieursdisziplin auf der Grundlage
> davon, wie Systeme denken lernen.

## 1. Positionierung: Von der Template-Krise zur Kognitionstechnik

Die meisten Prompt-Anleitungen sind Oberflaechen-Heuristiken: Rolle + Aufgabe +
Output-Format. Das produziert Templates, die wie magische Formeln weitergegeben
werden — ohne erklaerbaren Wirkmechanismus. Zwei Beobachtungen liefern den
Ausweg:

1. **Inhalt:** Gute Prompts folgen Mechanismen der Lernpsychologie — sie
   funktionieren, weil sie kognitive Prozesse gezielt anstossen, nicht weil
   die Formulierung "elegant" ist.
2. **Form:** Ein guter Prompt ist die Beschreibung einer *Struktur kognitiver
   Operationen* — Schritte, Reihenfolge, Reflexionsschleifen, Verifikation.

Das Paradigma verschiebt sich von "wie schreibe ich" zu "wie denkt das System
— und wie bringe ich es auf die naechste Denkstufe". Dafuer liefern die
Mathematik-Didaktik (APOS-Theorie, 40 Jahre erprobt) und ihr Lehrzyklus (ACE)
ein ausgereiftes theoretisches Fundament.

## 2. Theoretisches Fundament: APOS-Theorie (Dubinsky)

Die APOS-Theorie beschreibt, wie Wissensstrukturen reifen — vier Stufen:

| Stufe | Merkmal | Prompt-Modus |
|---|---|---|
| **Action** | Schrittweise, extern anweisungsgetrieben | Praezise Einzelkommandos |
| **Process** | Internalisierte Sequenz; als Ganzes beschreib- und reflektierbar | Verbundaufgaben + Review |
| **Object** | Encapsulated; kann selbst Gegenstand von Operationen sein | Modulbildung, Wiederverwendung |
| **Schema** | Kohärentes Rahmenwerk; wird automatisch aufgerufen und verallgemeinert | Rahmenwerk artikulieren + kreuzvalidieren |

Warum das auf LLMs uebertragbar ist: Ein Sprachmodell fuehrt kognitive
Operationen *auf Abruf* aus. Der Prompt ist die Steuervorschrift dafuer, auf
welcher Stufe und mit welcher Struktur das Modell operiert. Damit wird
Prompt Engineering zu dem, was es sein sollte: Steuerung von
Kognitionsstrukturen statt Formulierungsakrobatik.

## 3. Das ACE-Framework

ACE (Activities – Class discussion – Exercises) ist der Lehr-/Uebungszyklus,
mit dem die APOS-Theorie operationell umgesetzt wird. Auf Prompts uebertragen:

- **A (Activities/Lernen):** Prompts, die Vorwissen aktivieren und Konzepte
  einfuehren — bevor die eigentliche Aufgabe laeuft.
- **C (Construction/Konstruktion):** Prompts, die die Loesungskonstruktion
  anleiten — gefuehrte Zerlegung, Selbst-Erklaerung, Review.
- **E (Exercises/Uebung & Verifikation):** Prompts, die das Ergebnis pruefen,
  festigen und verallgemeinern — Edge-Cases, Fehleranalyse, Schema-Bildung.

Der Zyklus ist geschlossen: Verifikationsergebnisse werden zurueck in
Lernprompts gespeist (kognitive Schleife, Abschnitt 6).

## 4. Instruktionsdesign-Strategien als Prompt-Templates

| Strategie | ACE-Prompt-Template | Trigger |
|---|---|---|
| Vorwissen einfuehren (Advance Organizer) | „Vervollstaendige vor der Hauptaufgabe die noetige Wissensbasis: Definiere die beteiligten Schluesselbegriffe kurz und praegnant, starte dann die Aufgabe." | Neue Domaene / mehrkonzeptige Aufgabe |
| Konflikt-Detektion | „Es folgen zwei einander widersprechende Aussagen. Identifiziere den Widerspruch, bewerte beide und loese ihn." | Konsolidierung von Konzepten |
| Concept Mapping | „Liste alle an der Aufgabe beteiligten Konzepte, zeichne ihre Abhaengigkeiten als Struktur und markiere den Pfad der kritischen Konzepte." | Komplexe Aufgabe, Strukturvorab |
| Gefuehrte Inquiry | „Zerlege die Aufgabe in geordnete Teilfragen. Beantworte sie Schritt fuer Schritt; das Fazit kommt zuletzt." | Verhindert vorschnelle Schluesse |
| Kognitive Last verwalten | „Nummeriere alle Schlüsselinformationen. Zitiere beim Schliessen nach Nummer, ohne neue Details hinzuzufuegen." | Lange Kontexte / Vielrestriktionen |
| Worked Example | „Hier ist eine vollstaendig geloeste Beispielaufgabe samt Begründung. Loese die neue Aufgabe nach demselben Muster." | Methodentransfer |
| Umgekehrter Unterricht (Flipped) | „Äussere zunaechst dein Verständnis der Aufgabe und deinen Loesungsplan. Warte auf Korrektur, fuehre dann aus." | Missverstaendnisanfaellig / teuer |
| Deliberate Practice | „Hier sind 3 Aufgaben desselben Typs. Loese jede; gib nach jedem Feedback deine Verbeserung selbst an." | Internalisierung (Process-Ebene) |
| Selbst-Erklaerung | „Erlaeutere nach jedem Denkschritt in einem Satz, warum dieser Schritt gueltig ist." | Logikkritische Aufgaben |
| Boundary Probing | „Konstruiere zunaechst 2 Randfaelle zu dieser Anforderung und erklaere die jeweilige Behandlungsstrategie." | Robustheit / Spezifikation |
| Multi-Perspektive (Social Learning) | „Gib 3 Argumentationen unterschiedlicher Schulen mit je 1-Satz-Bewertung, dann dein eigenes Urteil." | Bewertungs-/Urteilsaufgaben |
| Fehleranalyse | „Hier ist eine Loesung mit typischen Fehlern. Diagnostiziere: konzeptuell / prozedural / logisch. Korrigiere." | Qualitaetssicherung, Lehren aus Fehlern |

## 5. Kognitive Leiter: APOS im Prompt-Betrieb

| Kognitive Aktion | APOS-Ebene | Prompt-Muster | Zweck |
|---|---|---|---|
| Kommando ausfuehren | Action | „Fuehre Schritte 1-5 aus; gib nach jedem Schritt das Ergebnis." | Zuverlaessige Einzeloperation |
| Prozess beschreiben | Process | „Revueiere den Ablauf: Welcher Schritt war entscheidend? Was aenderst du beim naechsten Mal?" | Internalisierung + Reflexion |
| Loesung einkapseln | Object | „Verdichte die Loesung dieses Aufgabentyps zu einem wiederverwendbaren Template/Tool mit Parametern und Anwendungsregel." | Wissen als Operand |
| Schema artikulieren | Schema | „State dein Verstaendnis-Framework dieser Domaene; validiere es gegen alle bekannten Faelle dieser Sitzung." | Verallgemeinerung, Abrufbarkeit |
| Umkehrung (Reverse) | Verifikation | „Stelle selbst eine Aufgabe und den erwarteten Loesungsweg; ich korrigiere." | Verifiziert echtes Verstaendnis |

**Leitregel:** Höher ist nicht automatisch besser. Deterministische Aufgaben
gehienen auf Action-/Process-Ebene (reproduzierbar!), kreative auf
Schema-Ebene (verallgemeinernd).

## 6. Die kognitive Schleife (Cognitive Loop)

Ein guter Prompt-Ablauf ist kein Einzelzug, sondern eine Schleife:

```
Lern-Prompt (Vorwissen, Konflikte)
        v
Ausfuehrungs-Prompt (gefuehrte Konstruktion, Selbst-Erklaerung)
        v
Verifikations-Prompt (Edge-Cases, Fehleranalyse, Review)
        v
Schema-Prompt (Einkapseln, Verallgemeinern, in Framework einordnen)
        |
        +--> zurueck zu Lern-Prompt (naechster Zyklus)
```

Schema-Konstruktion in vier Schritten: **Aktivierung** (Vorwissen aufrufen) →
**Operation** (kognitive Schritte ausfuehren) → **Einkapselung** (Ergebnis
zu Objekt verdichten) → **Automatisierung** (durch Uebung abrufbar machen).

## 7. APOS als Validierungs-Framework fuer Prompt-Qualitaet

Damit ist Prompt-Qualitaet messbar statt Geschmackssache — sechs Kriterien:

1. **Stufen-Zuordnung:** Auf welcher APOS-Ebene operiert der Prompt? Passt
   die Stufe zur Aufgabenart (deterministisch vs. kreativ)?
2. **Mechanismus:** Ist die Struktur kognitiver Operationen explizit
   (Schritte, Nummerierung, Reflexionsschleifen)?
3. **Inhalt:** Sitzen laernpsychologische Mechanismen (Vorwissen, Worked
   Example, Selbst-Erklaerung) an den richtigen Stellen?
4. **Transfer:** Trainiert der Prompt Schema-Bildung (loest die Loesung
   auch den *naechsten*, ungewohnten Fall)?
5. **Verifizierbarkeit:** Ist das Ergebnis objektiv pruefbar (Edge-Cases,
   A/B-Vergleich, Reverse-Ausfuehrung)?
6. **Determinismus:** Deterministische Prompts muessen bei jedem Lauf
   gleichwertige Ergebnisse liefern; kreative Prompts duerfen Vielfalt
   erzeugen — beides bewusst getrennt halten.

## 8. Integration in das A-TownChain-Ökosystem

- **Aurora AI (L2):** System- und Tool-Orchestrierung = Action-/Process-
  Stufe (deterministisch, reproduzierbar — konsistent mit der
  Differential-Testing-Pflicht aus AD-021/022). Agent-Loops mit
  Verifikations-Prompts (Edge-Cases vor finaler Antwort).
- **KAI (AI-Kernel):** Scheduler-/Agent-Aufgaben auf Process-Ebene mit
  Review-Pflicht; Lern-Schleifen (ATC-24 Agent Scheduling) als
  kognitive Schleife nach Abschnitt 6.
- **Genesis AI (L6, kreativ):** Schema-Stufe, Multi-Perspektive,
  Verallgemeinerung — kreative Kognition darf nicht auf Action-Stufe
  gezwungen werden.
- **Meta-Ebene:** ATC-STD-000 §36 Naming-Prinzip uebertragen — auch
  Prompt-Bibliotheken bekommen IDs (PF-NNN, kuenftig per Schema
  erweiterbar), damit Prompts versionierbar und differenztestbar werden.
- **Promotion-Pfad:** Nach Praxisvalidierung Foerderung zu
  **ATC-STD-701** (AI-Domaene 700-799) moeglich — dann normativ mit
  Review-Chain nach ATC-STD-000 §13.

## 9. Kernthesen

1. Ein guter Prompt ist eine praezise Beschreibung der Struktur kognitiver
   Operationen — Inhalt = Lernpsychologie, Mechanismus = Operationsstruktur,
   Checkliste = Schema-Training.
2. APOS ist nicht nur Lerntheorie, sondern ein Validierungs-Framework fuer
   Prompt-Qualitaet (sechs Kriterien, Abschnitt 7).
3. Vom Einzelzug zur kognitiven Schleife: Lernen → Ausfuehren →
   Verifizieren → Schema.
4. Schema-Konstruktion: Aktivierung → Operation → Einkapselung →
   Automatisierung.
5. Echtes Prompt Engineering ist eine Ingenieursdisziplin auf der Basis
   davon, wie KI denkt — nicht Formulierungskunst.

## References

- Dubinsky, E. & McDonald, M. (2001): APOS: A Constructivist Theory of
  Learning in Undergraduate Mathematics Education. (Theoretischer Ursprung)
- ACE-Lehrzyklus (Activities, Class discussion, Exercises) der APOS-Schule.
- Quellenmaterial des Owners (07.09.2026, chinesischsprachige Ausarbeitung
  zu APOS/ACE und Prompt Engineering) — Grundlage dieser Spezifikation.
