# Economy & Trade System

## Zweck

Die Gameplay-Schicht erhält eine deterministische Grundlage für Währung, Marktangebote und Handel. Sie verbindet Inventar, Crafting, Ressourcen und Siedlungen mit einer späteren Wirtschafts- und NPC-Simulation.

## Wallet

`Wallet` verwaltet Credits mit saturierender Addition und atomarer Auszahlung. Eine Auszahlung verändert den Kontostand nur bei ausreichendem Guthaben.

## Handelsangebote

`TradeOffer` beschreibt Item-ID, Menge, Preis pro Einheit und den berechneten Gesamtpreis. Ungültige Angebote werden nicht in den Markt aufgenommen.

## Markt

`Market` hält Angebote und sortiert sie deterministisch nach Item-ID, Preis und Menge. Dadurch ist die Reihenfolge reproduzierbar und kann später für UI, NPC-Handel und Netzwerkreplikation verwendet werden.

## Integration

Die Gameplay-API exportiert `Wallet`, `TradeOffer` und `Market`.

Geplante Kette:

`Ressourcen → Inventar → Crafting → Items → Markt → Wallet → Gebäude/NPC-Wirtschaft`

## Tests

Implementiert sind Tests für saturierende Wallet-Einzahlungen, sichere Auszahlungen, Preisberechnung, Angebotsvalidierung und deterministische Marktordnung.

## Produktionslücken

Noch erforderlich sind insbesondere Kauf-/Verkaufs-Transaktionen, atomarer Inventartransfer, Händler/NPC-Angebote, Angebot/Nachfrage, Steuern/Gebühren, Fraktionen, persistente Wirtschaftszustände, Multiplayer-Autorität, Konfliktauflösung, UI und Editor-Unterstützung. Blockchain-basierte Eigentums- oder Handelsvorgänge müssen ausdrücklich über die A-TownChain-Integrationsschicht erfolgen.

## Status

`FOUNDATION_IMPLEMENTED / PRODUCTION_NOT_ESTABLISHED`
