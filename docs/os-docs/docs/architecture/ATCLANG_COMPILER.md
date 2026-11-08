# ATCLang Compiler — aktuelle Source-of-Truth-Grenze

> **Aktualisiert:** 2026-10-09  
> **Status:** Architektur-Referenz; keine aktuelle Modul- oder Testinventur.

## Kanonische Implementierung

Das aktive [`atclang`](https://github.com/A-TownChain-Okosystems/atclang)-Repository beschreibt die Rust-only Implementierungslinie. Die frühere Fassung dieser Datei listete Python-Dateien, Zeilenzahlen, 92 Produktionsdateien, 60 grüne Tests und eine 100%-Parse-Rate aus einem Sprintbericht vom 2026-07-05. Diese Zahlen sind **historisch** und dürfen nicht als aktueller Bestand oder als aktuelle Verifikation verwendet werden.

Der Compiler, Verifier, die Artefaktvalidierung und die Security-Gates müssen anhand des aktuellen Rust-Quellbaums, der kanonischen Standards und der zugehörigen SHA-gebundenen CI-Evidence beschrieben werden. Python-Referenzmaterial, das aus dem aktiven Tree entfernt wurde, ist keine aktive Runtime-Komponente.

## Pipeline als konzeptioneller Vertrag

```text
ATCLang source (.atc)
       ↓
Lexer / Parser / AST / Type checking
       ↓
Compiler + artifact validation
       ↓
ATCB bytecode
       ↓
Canonical ATC-VM: a-townchain/components/vm
       ↓
A-TownChain chain runtime
```

Dieses Diagramm beschreibt die Verantwortungsgrenzen, nicht den Nachweis, dass jeder Schritt auf dem aktuellen Head vollständig implementiert oder verifiziert ist.

## Bytecode- und Runtime-Grenze

Die Bytecode-Formate und Laufzeitregeln müssen aus den aktuellen kanonischen Specs und der Implementierung abgeleitet werden. Diese Dokumentation darf keine widersprüchlichen Magic-Werte, Versionen, Limits oder Compilerfeatures als final deklarieren, solange die normative Quelle dies nicht festlegt.

## Verifikation

Statusangaben müssen den exakten Commit-SHA und relevante Workflow-Evidenz nennen. Alte Testzahlen, Modulzählungen oder Parse-Raten werden nicht auf den aktuellen Stand übertragen. Sie sind keine Aussage über Security-Audit, Conformance, Produktionsreife oder Mainnet-Readiness.

Normative SSOT: [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards).
