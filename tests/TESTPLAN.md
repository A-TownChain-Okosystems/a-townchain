# Testplan — atc-wallet

Bis zur Implementierung (Roadmap M6/M7, AD-027) dokumentiert dieser Plan die
geplanten Testkategorien (ATC-STD-203 Blockchain-Testkategorien):

1. **Unit-Tests:** Kernfunktionen je Modul (rust)
2. **Integration:** Zusammenspiel mit Nachbarschichten (L5)
3. **End-to-End:** Kriterium des zustaendigen Meilensteins (M6: Dienste;
   M7: Spiel/NFT auf Chain)
4. **Governance:** atc-repo-audit laeuft in CI (governance-ci.yml)

Tests entstehen MIT der Implementierung — nicht danach (AD-027-Prinzip).
