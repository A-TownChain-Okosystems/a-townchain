# Cluster-Architektur — historische Deployment-Notizen und aktuelle Prüfkriterien

> **Aktualisiert:** 2026-10-09  
> **Status:** Historische Beschreibung; kein Nachweis eines laufenden Testnets, produktiven Clusters oder Mainnet-Deployments.

## Geltungsbereich

Die vorherige Fassung (auto-generiert 2026-07-08) beschrieb konkrete Docker-Compose-Stacks, Portbelegungen, Service-Rollen, Healthchecks und angebliche produktionsnahe Eigenschaften. Diese Angaben müssen vor Wiederverwendung mit den aktuellen Compose-Dateien, Dockerfiles, CI-Workflows und Secrets-Konfigurationen abgeglichen werden. Sie werden hier nicht als aktueller Betriebszustand bestätigt.

**Kein Produktionsclaim:** Der Name eines Compose-Stacks, ein vorhandener Dockerfile oder ein konfigurierter Healthcheck beweist weder eine laufende Umgebung noch Produktionsreife.

## Verbindliche Prüfpunkte für einen aktuellen Cluster-Status

1. **Source SHA:** Compose-Dateien, Dockerfiles, Images und Deployment-Manifeste auf einem konkreten Commit-SHA erfassen.
2. **Runtime evidence:** Build, Image-Pinning, Start, Healthchecks und Integrationstests als aktuelle Workflow-Run-/Job-/Step-Evidence sichern.
3. **Secrets:** Keine Passwörter, Tokens oder privaten Schlüssel in Compose-Dateien, Logs oder Dokumentation ausgeben. Secret-Handling muss gegen aktuelle Dateien geprüft werden; historische Werte aus alten Dokumenten dürfen nicht weiterverwendet werden.
4. **Netzwerk:** Portbelegung, Subnetze, Service-Abhängigkeiten, Authentifizierung und TLS prüfen. Parallelität verschiedener Stacks nicht annehmen, ohne die aktuelle Konfiguration zu validieren.
5. **Konsens und Rollen:** Validator-/Proposer-/Fullnode-Rollen müssen der kanonischen Chain-/Algorithmus-Spezifikation entsprechen. Historische Beschreibungen von PoW/PoS/PoH sind keine Protokollentscheidung.
6. **Release readiness:** Produktions- oder Mainnet-Bereitschaft erst nach allen obligatorischen Security-, Reliability-, Recovery- und Release-Gates behaupten.

## Kanonische Grenzen

- Chain-/Protocol-/State-Transition: `a-townchain`.
- ATC-VM: `a-townchain/components/vm`.
- Algorithmus/Konsens: `a-townchain/components/algorithm`.
- Node Runtime: `atc-node`, soweit im aktuellen Architekturvertrag vorgesehen.
- Integration und systemweite Evidence: `a-townchain-ecosystem` und `a-townchain-os`.
- Normative Standards: `atc-standards`.

## Statusvokabular

- **CONFIGURED:** Konfiguration vorhanden.
- **BUILT:** Artefakt wurde für einen bestimmten SHA gebaut.
- **RUNNING:** Laufzeitnachweis für eine definierte Umgebung liegt vor.
- **VERIFIED:** geforderte Tests und Nachweise bestehen für den beanspruchten SHA und Scope.
- **PRODUCTION_READY:** alle verbindlichen Release-Gates sind erfüllt und dokumentiert.

Diese Zustände sind nicht austauschbar. Bis aktuelle Deployment-Evidence vorliegt, lautet der Betriebsstatus **NOT VERIFIED / UNKNOWN**.
