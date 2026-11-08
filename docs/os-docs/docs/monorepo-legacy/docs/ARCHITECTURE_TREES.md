# 🏗️ Architekturbäume — A-TownChain Ökosystem (70 Repos)

> **Erweitert mit:** Layer-Klassifizierung · Repo-Abhängigkeiten · Test-Dateien · Dateigrößen · Zeilenanzahl · Verzeichnis-Zeilen · Sprach-Statistik · 10-Level Tiefe

> **Auto-generiert:** 2026-08-06 13:15 UTC | **Agent:** Aurora (MasterBrain · Base44)

---

## Ökosystem-Statistik

| Metrik | Wert |
|--------|------|
| **Repositories** | 70 (34 Code + 36 Wiki) |
| **Total Dateien** | 5,709 |
| **Total Verzeichnisse** | 1,080 |

### Sprach-Verteilung

| Typ | Dateien |
|-----|--------|
| .md | 2697 |
| .atc | 903 |
| .py | 787 |
| .tsx | 476 |
| .no-ext | 146 |
| .gitignore | 144 |
| .ts | 127 |
| .rs | 126 |
| .js | 69 |
| .txt | 47 |
| .yml | 34 |
| .cjs | 33 |
| .json | 27 |
| .html | 18 |
| .toml | 17 |

---

## Repo-Abhängigkeitsgraph

| Repo | Nutzt | Wird genutzt von |
|------|-------|------------------|
| a-townchain-os | — | — |
| a-townchain-os-docs | — | — |
| a-townchain-os-wiki | — | — |
| atc-aistudio | — | — |
| atc-aistudio-wiki | — | — |
| atc-atclang | — | — |
| atc-atclang-wiki | — | — |
| atc-atcpkg | — | — |
| atc-atcpkg-wiki | — | — |
| atc-backend | atc-gateway, atc-kernel, atc-shivacore, atc-ui | atc-gateway, atc-ui |
| atc-backend-wiki | — | — |
| atc-blockchain | — | — |
| atc-blockchain-wiki | — | — |
| atc-bootloader | atc-kernel | atc-kernel |
| atc-bootloader-wiki | — | — |
| atc-ci | — | — |
| atc-ci-wiki | — | — |
| atc-cli | atc-gateway, atc-sdk | — |
| atc-cli-wiki | — | — |
| atc-contracts | — | — |
| atc-contracts-wiki | — | — |
| atc-dns | atc-contracts, atc-gateway, atc-kernel, atc-ui | atc-gateway, atc-ui |
| atc-dns-wiki | — | — |
| atc-drivers | atc-kernel | atc-kernel |
| atc-drivers-wiki | — | — |
| atc-explorer | — | — |
| atc-explorer-wiki | — | — |
| atc-franchise | atc-contracts, atc-gateway | — |
| atc-franchise-wiki | — | — |
| atc-frontend | — | — |
| atc-frontend-wiki | — | — |
| atc-gateway | — | — |
| atc-gateway-wiki | — | — |
| atc-genesis-engine | — | — |
| atc-genesis-engine-wiki | — | — |
| atc-ide | — | — |
| atc-ide-wiki | — | — |
| atc-kernel | atcnet | — |
| atc-kernel-wiki | — | — |
| atc-linux-edition | — | — |
| atc-linux-edition-wiki | — | — |
| atc-mobile | — | — |
| atc-mobile-wiki | — | — |
| atc-sdk | atc-cli, atc-gateway, atc-ui | atc-cli, atc-ui |
| atc-sdk-wiki | — | — |
| atc-shivacore | — | — |
| atc-shivacore-tools | — | — |
| atc-shivacore-tools-wiki | — | — |
| atc-shivacore-wiki | — | — |
| atc-shivamon | atc-contracts, atc-gateway | — |
| atc-shivamon-wiki | — | — |
| atc-standards | — | — |
| atc-standards-wiki | — | — |
| atc-stdlib | — | — |
| atc-stdlib-wiki | — | — |
| atc-ui | — | — |
| atc-ui-wiki | — | — |
| atc-vm | — | — |
| atc-vm-wiki | — | — |
| atc-wallet | — | — |
| atc-wallet-wiki | — | — |
| atc-whitepaper | — | — |
| atc-windows-edition | — | — |
| atc-windows-edition-wiki | — | — |
| atclang | — | — |
| atclang-wiki | — | — |
| atcnet | — | — |
| atcnet-wiki | — | — |
| franchise-factory-wiki | — | — |
| kai-os-wiki | — | — |

*9 Repos mit Abhängigkeiten gefunden*

---

## Inhaltsverzeichnis

- [a-townchain-os](#a-townchain-os) — L0–L12 (Monorepo)
- [a-townchain-os-docs](#a-townchain-os-docs) — Wiki (Haupt-Doku)
- [a-townchain-os-wiki](#a-townchain-os-wiki) — Wiki (Archiv)
- [atc-aistudio](#atc-aistudio) — L10 (AI Studio)
- [atc-aistudio-wiki](#atc-aistudio-wiki) — ?
- [atc-atclang](#atc-atclang) — L2–L4 (ATCLang Sync)
- [atc-atclang-wiki](#atc-atclang-wiki) — ?
- [atc-atcpkg](#atc-atcpkg) — L7 (Package Manager)
- [atc-atcpkg-wiki](#atc-atcpkg-wiki) — ?
- [atc-backend](#atc-backend) — L7 (Backend Services)
- [atc-backend-wiki](#atc-backend-wiki) — ?
- [atc-blockchain](#atc-blockchain) — L3–L4 (Blockchain)
- [atc-blockchain-wiki](#atc-blockchain-wiki) — ?
- [atc-bootloader](#atc-bootloader) — L1 (Bootloader)
- [atc-bootloader-wiki](#atc-bootloader-wiki) — ?
- [atc-ci](#atc-ci) — L0 (CI/CD Pipeline)
- [atc-ci-wiki](#atc-ci-wiki) — ?
- [atc-cli](#atc-cli) — L7 (CLI Tool)
- [atc-cli-wiki](#atc-cli-wiki) — ?
- [atc-contracts](#atc-contracts) — L4 (Smart Contracts)
- [atc-contracts-wiki](#atc-contracts-wiki) — ?
- [atc-dns](#atc-dns) — L5 (DNS)
- [atc-dns-wiki](#atc-dns-wiki) — ?
- [atc-drivers](#atc-drivers) — L1 (Hardware Drivers)
- [atc-drivers-wiki](#atc-drivers-wiki) — ?
- [atc-explorer](#atc-explorer) — L7 (Block Explorer)
- [atc-explorer-wiki](#atc-explorer-wiki) — ?
- [atc-franchise](#atc-franchise) — L8–L10 (Business DAOs)
- [atc-franchise-wiki](#atc-franchise-wiki) — ?
- [atc-frontend](#atc-frontend) — L10 (Frontend)
- [atc-frontend-wiki](#atc-frontend-wiki) — ?
- [atc-gateway](#atc-gateway) — L7 (API Gateway)
- [atc-gateway-wiki](#atc-gateway-wiki) — ?
- [atc-genesis-engine](#atc-genesis-engine) — L0 (Genesis Engine)
- [atc-genesis-engine-wiki](#atc-genesis-engine-wiki) — ?
- [atc-ide](#atc-ide) — L10 (IDE/Playground)
- [atc-ide-wiki](#atc-ide-wiki) — ?
- [atc-kernel](#atc-kernel) — L2 (Kernel)
- [atc-kernel-wiki](#atc-kernel-wiki) — ?
- [atc-linux-edition](#atc-linux-edition) — L10 (Linux Client)
- [atc-linux-edition-wiki](#atc-linux-edition-wiki) — ?
- [atc-mobile](#atc-mobile) — L10 (Mobile)
- [atc-mobile-wiki](#atc-mobile-wiki) — ?
- [atc-sdk](#atc-sdk) — L7 (SDK)
- [atc-sdk-wiki](#atc-sdk-wiki) — ?
- [atc-shivacore](#atc-shivacore) — L1 (Bare-Metal Kernel)
- [atc-shivacore-tools](#atc-shivacore-tools) — L1 (Build Tools)
- [atc-shivacore-tools-wiki](#atc-shivacore-tools-wiki) — ?
- [atc-shivacore-wiki](#atc-shivacore-wiki) — ?
- [atc-shivamon](#atc-shivamon) — L12 (NFT Gaming)
- [atc-shivamon-wiki](#atc-shivamon-wiki) — ?
- [atc-standards](#atc-standards) — L0 (Standards)
- [atc-standards-wiki](#atc-standards-wiki) — ?
- [atc-stdlib](#atc-stdlib) — L3 (Standard Library)
- [atc-stdlib-wiki](#atc-stdlib-wiki) — ?
- [atc-ui](#atc-ui) — L10 (UI Dashboard)
- [atc-ui-wiki](#atc-ui-wiki) — ?
- [atc-vm](#atc-vm) — L4 (Virtual Machine)
- [atc-vm-wiki](#atc-vm-wiki) — ?
- [atc-wallet](#atc-wallet) — L10 (Desktop Wallet)
- [atc-wallet-wiki](#atc-wallet-wiki) — ?
- [atc-whitepaper](#atc-whitepaper) — L0 (Whitepaper)
- [atc-windows-edition](#atc-windows-edition) — L10 (Windows Client)
- [atc-windows-edition-wiki](#atc-windows-edition-wiki) — ?
- [atclang](#atclang) — L2–L4 (ATCLang Compiler)
- [atclang-wiki](#atclang-wiki) — ?
- [atcnet](#atcnet) — L5 (P2P Network)
- [atcnet-wiki](#atcnet-wiki) — ?
- [franchise-factory-wiki](#franchise-factory-wiki) — ?
- [kai-os-wiki](#kai-os-wiki) — Wiki (Legacy)

---


## a-townchain-os

**Layer:** L0–L12 (Monorepo) | **Dateien:** 1708 | **Verzeichnisse:** 310 | **Max Tiefe:** 5 | **Tests:** 37

**Sprachen:** .md(715) · .atc(284) · .py(240) · .tsx(158) · .rs(63) · .ts(41)

**Test-Dateien:** 31× Python · 6× TypeScript

```
a-townchain-os/
├── .github/ (3 files, 174 lines)
│   ├── workflows/ (2 files, 143 lines)
│   │   ├── ci.yml (1KB) [42L]
│   │   └── codeql.yml (4KB) [101L]
│   └── changelog-config.json (784B) [31L]
├── TODO/ (1 files, 68 lines)
│   └── MASTER_TODO.md (2KB) [68L]
├── aistudio/ (248 files, 72257 lines)
│   ├── assets/ (1 files, 0 lines)
│   │   └── .aistudio/ (1 files, 0 lines)
│   │       └── .gitignore (2B)
│   ├── src/ (190 files, 56202 lines)
│   │   ├── backend/ (2 files, 206 lines)
│   │   │   ├── blockchain/ (1 files, 129 lines)
│   │   │   │   └── engine.ts (3KB) [129L]
│   │   │   └── p2p/ (1 files, 77 lines)
│   │   │       └── network.ts (2KB) [77L]
│   │   ├── components/ (148 files, 43089 lines)
│   │   │   ├── ATCAssetView.tsx (11KB) [191L]
│   │   │   ├── ATCDjStudioView.tsx (17KB) [445L]
│   │   │   ├── ATCLangEditor.tsx (26KB) [625L]
│   │   │   ├── ATCWalletView.tsx (26KB) [498L]
│   │   │   ├── ATownDashboardView.tsx (14KB) [302L]
│   │   │   ├── ATownOSNode.tsx (71KB) [1439L]
│   │   │   ├── ATownTestView.tsx (6KB) [111L]
│   │   │   ├── AgentCivilizationView.tsx (8KB) [152L]
│   │   │   ├── Ai3DRenderEngineTab.tsx (8KB) [199L]
│   │   │   ├── AiAnimationEngineTab.tsx (8KB) [198L]
│   │   │   ├── AiAudioEngineTab.tsx (8KB) [198L]
│   │   │   ├── AiCharacterBioTab.tsx (9KB) [199L]
│   │   │   ├── AiGameEngineTab.tsx (9KB) [200L]
│   │   │   ├── AiKernelView.tsx (6KB) [128L]
│   │   │   ├── AiOsEngineView.tsx (19KB) [490L]
│   │   │   ├── AiSoftwareWorkflowView.tsx (11KB) [229L]
│   │   │   ├── AiTimelineEngineTab.tsx (9KB) [199L]
│   │   │   ├── AntiCheatView.tsx (14KB) [261L]
│   │   │   ├── ApiHealthWidget.tsx (3KB) [85L]
│   │   │   ├── ApiInterfacesView.tsx (9KB) [189L]
│   │   │   ├── ApiOrchestratorView.tsx (17KB) [354L]
│   │   │   ├── AppGlobeView.tsx (9KB) [233L]
│   │   │   ├── ArchitectureDependencyGraph.tsx (7KB) [248L]
│   │   │   ├── ArchitectureView.tsx (38KB) [888L]
│   │   │   ├── AssetVaultView.tsx (8KB) [187L]
│   │   │   ├── AtcAssetsDbView.tsx (11KB) [250L]
│   │   │   ├── AtcCoreKernelView.tsx (8KB) [144L]
│   │   │   ├── AtcLangArchitectureView.tsx (33KB) [585L]
│   │   │   ├── AtcLangPlaygroundView.tsx (12KB) [256L]
│   │   │   ├── AtcLangPresetsView.tsx (3KB) [64L]
│   │   │   ├── AtcWhitepaperView.tsx (10KB) [187L]
│   │   │   ├── AtsSuite.tsx (4KB) [51L]
│   │   │   ├── AtvmSandboxView.test.tsx (3KB) [85L] 🧪
│   │   │   ├── AtvmSandboxView.tsx (26KB) [499L]
│   │   │   ├── BatteryStatus.tsx (11KB) [269L]
│   │   │   ├── BattleArenaView.tsx (8KB) [143L]
│   │   │   ├── BenchmarkCenterView.tsx (15KB) [288L]
│   │   │   ├── BlockchainEcosystemView.tsx (9KB) [224L]
│   │   │   ├── BlockchainLedgerView.tsx (13KB) [247L]
│   │   │   ├── CalculatorView.tsx (3KB) [74L]
│   │   │   ├── CalendarView.tsx (3KB) [78L]
│   │   │   ├── CiCdPipelineView.tsx (7KB) [159L]
│   │   │   ├── ClockView.tsx (3KB) [72L]
│   │   │   ├── CodeAnalyzerView.tsx (4KB) [90L]
│   │   │   ├── CommitHeatmap.tsx (4KB) [110L]
│   │   │   ├── ComplianceEngineView.tsx (4KB) [84L]
│   │   │   ├── ComplianceView.tsx (8KB) [191L]
│   │   │   ├── ConflictResolutionModal.tsx (11KB) [257L]
│   │   │   ├── ConsensusIntegrationGuide.tsx (70KB) [1528L]
│   │   │   ├── CryptoVisualizationView.tsx (18KB) [473L]
│   │   │   ├── DataProcessingView.tsx (4KB) [78L]
│   │   │   ├── DbOrchestratorView.tsx (6KB) [112L]
│   │   │   ├── DeFiLiquidityPoolView.tsx (13KB) [255L]
│   │   │   ├── DependencyMapView.tsx (3KB) [123L]
│   │   │   ├── DeploymentPipelineWidget.tsx (6KB) [160L]
│   │   │   ├── DevToolsView.tsx (5KB) [133L]
│   │   │   ├── DeveloperKnowledgeBaseView.tsx (18KB) [359L]
│   │   │   ├── DistributedDatalakeView.tsx (3KB) [73L]
│   │   │   ├── EcosystemInstaller.tsx (11KB) [297L]
│   │   │   ├── EcosystemTreeOverlay.tsx (12KB) [357L]
│   │   │   ├── EcosystemUmlView.tsx (7KB) [143L]
│   │   │   ├── EcosystemVisualizerView.tsx (12KB) [325L]
│   │   │   ├── FileManagerView.tsx (6KB) [170L]
│   │   │   ├── FolderView.tsx (4KB) [111L]
│   │   │   ├── FranchiseFactoryView.tsx (83KB) [1733L]
│   │   │   ├── GateToHellBrowser.tsx (5KB) [106L]
│   │   │   ├── GenesisBlockGeneratorView.tsx (6KB) [150L]
│   │   │   ├── GitGraphVisualization.tsx (4KB) [137L]
│   │   │   ├── GitHubRepoSyncView.tsx (63KB) [1385L]
│   │   │   ├── GitHubStatusDashboard.tsx (34KB) [643L]
│   │   │   ├── GitOpsView.tsx (7KB) [126L]
│   │   │   ├── GovernanceView.tsx (23KB) [601L]
│   │   │   ├── GpuPerformanceWidget.tsx (4KB) [120L]
│   │   │   ├── HardwareDriversView.tsx (20KB) [376L]
│   │   │   ├── IdeaToAppFlowchartView.tsx (7KB) [153L]
│   │   │   ├── ImageGeneratorTab.tsx (5KB) [117L]
│   │   │   ├── IntegrationsWindow.tsx (21KB) [426L]
│   │   │   ├── InterfacesView.tsx (2KB) [56L]
│   │   │   ├── JsExampleRunner.tsx (2KB) [86L]
│   │   │   ├── LazyMetricsCharts.tsx (31KB) [808L]
│   │   │   ├── LegalView.tsx (6KB) [87L]
│   │   │   ├── LoginOverlay.tsx (38KB) [690L]
│   │   │   ├── MainnetLaunchView.tsx (12KB) [251L]
│   │   │   ├── MarketplaceView.tsx (22KB) [450L]
│   │   │   ├── MediaApps.tsx (18KB) [254L]
│   │   │   ├── MetricsDashboard.tsx (4KB) [105L]
│   │   │   ├── MetricsView.tsx (56KB) [1476L]
│   │   │   ├── ModulesPluginView.tsx (18KB) [309L]
│   │   │   ├── NetworkExplorerView.test.tsx (4KB) [121L] 🧪
│   │   │   ├── NetworkExplorerView.tsx (17KB) [370L]
│   │   │   ├── NetworkTopologyView.tsx (2KB) [38L]
│   │   │   ├── NodeHealthMonitor.tsx (4KB) [113L]
│   │   │   ├── NotepadView.tsx (2KB) [67L]
│   │   │   ├── OfficeApps.tsx (352B) [14L]
│   │   │   ├── OfficeSuiteView.tsx (12KB) [271L]
│   │   │   ├── P2PChatView.tsx (12KB) [277L]
│   │   │   ├── Paint3DView.tsx (5KB) [140L]
│   │   │   ├── PaymentSystemView.tsx (4KB) [93L]
│   │   │   ├── PipelineGeneratorTab.tsx (19KB) [433L]
│   │   │   ├── PoAITrainingEngineView.tsx (8KB) [173L]
│   │   │   ├── ProjectAuditDashboard.tsx (7KB) [135L]
│   │   │   ├── ProjectHubView.tsx (30KB) [501L]
│   │   │   ├── ProtocolsView.tsx (8KB) [207L]
│   │   │   ├── ReportsView.tsx (10KB) [202L]
│   │   │   ├── RepositoryActivityChart.tsx (5KB) [145L]
│   │   │   ├── RepositoryLineChart.tsx (6KB) [198L]
│   │   │   ├── RescueSystemView.tsx (16KB) [307L]
│   │   │   ├── RoadmapView.tsx (6KB) [196L]
│   │   │   ├── SemanticGraphView.tsx (4KB) [86L]
│   │   │   ├── SessionExportView.tsx (8KB) [221L]
│   │   │   ├── SettingsView.tsx (105KB) [2312L]
│   │   │   ├── SocialMediaView.tsx (16KB) [287L]
│   │   │   ├── SoftwareAuditView.tsx (38KB) [885L]
│   │   │   ├── SoftwareKnowledgeDbView.tsx (18KB) [380L]
│   │   │   ├── SourceCodeViewer.tsx (20KB) [547L]
│   │   │   ├── SpecificSettingsViews.tsx (17KB) [306L]
│   │   │   ├── StorageManagerView.tsx (9KB) [258L]
│   │   │   ├── StrategicArchitectureMap.tsx (9KB) [243L]
│   │   │   ├── StructureView.tsx (22KB) [505L]
│   │   │   ├── SyncDashboardModal.tsx (4KB) [88L]
│   │   │   ├── SyncHistoryModal.tsx (11KB) [249L]
│   │   │   ├── SyncMetricsView.tsx (7KB) [170L]
│   │   │   ├── SyncStatusDonutChart.tsx (2KB) [99L]
│   │   │   ├── SyncStatusOverview.tsx (7KB) [168L]
│   │   │   ├── SystemDiagnosticsView.tsx (18KB) [337L]
│   │   │   ├── SystemFinderView.tsx (2KB) [56L]
│   │   │   ├── SystemHealthDashboard.tsx (9KB) [246L]
│   │   │   ├── SystemHealthDashboardWidget.tsx (2KB) [63L]
│   │   │   ├── SystemLogsView.tsx (3KB) [89L]
│   │   │   ├── TaskManagerView.tsx (3KB) [82L]
│   │   │   ├── TechDocsView.tsx (17KB) [335L]
│   │   │   ├── TechTreeView.tsx (18KB) [420L]
│   │   │   ├── TerminalView.tsx (6KB) [189L]
│   │   │   ├── TestnetOrchestrationView.tsx (8KB) [178L]
│   │   │   ├── TestnetSimulationView.tsx (14KB) [298L]
│   │   │   ├── TextGeneratorTab.tsx (6KB) [177L]
│   │   │   ├── ThemeSwitcher.tsx (4KB) [143L]
│   │   │   ├── TodoView.tsx (18KB) [383L]
│   │   │   ├── TooltipIcon.tsx (1KB) [29L]
│   │   │   ├── TxOrchestratorView.tsx (5KB) [105L]
│   │   │   ├── UserProfileView.tsx (12KB) [255L]
│   │   │   ├── VideoGeneratorTab.tsx (7KB) [176L]
│   │   │   ├── WebhookMonitor.tsx (5KB) [145L]
│   │   │   ├── Window.tsx (6KB) [158L]
│   │   │   ├── WindowExtras.tsx (4KB) [87L]
│   │   │   ├── ZeroKnowledgeProofView.tsx (6KB) [129L]
│   │   │   ├── ZkCircuitEditorView.tsx (4KB) [108L]
│   │   │   └── ZkVisualizationView.tsx (3KB) [99L]
│   │   ├── contexts/ (4 files, 269 lines)
│   │   │   ├── FirebaseContext.tsx (2KB) [94L]
│   │   │   ├── GoogleWorkspaceContext.tsx (2KB) [83L]
│   │   │   ├── SyncMetricsContext.tsx (1KB) [47L]
│   │   │   └── WalletContext.tsx (1KB) [45L]
│   │   ├── db/ (3 files, 64 lines)
│   │   │   ├── drizzle.config.ts (817B) [29L]
│   │   │   ├── index.ts (652B) [24L]
│   │   │   └── schema.ts (486B) [11L]
│   │   ├── hooks/ (2 files, 250 lines)
│   │   │   ├── useGoogleSheetsSync.ts (8KB) [220L]
│   │   │   └── useKeyboardShortcut.ts (899B) [30L]
│   │   ├── lib/ (6 files, 359 lines)
│   │   │   ├── CryptoEngine.ts (1KB) [42L]
│   │   │   ├── firebase-admin.ts (544B) [15L]
│   │   │   ├── firebase.ts (2KB) [64L]
│   │   │   ├── indexedDb.ts (2KB) [88L]
│   │   │   ├── syncLogic.test.ts (2KB) [82L] 🧪
│   │   │   └── syncLogic.ts (1KB) [68L]
│   │   ├── middleware/ (1 files, 30 lines)
│   │   │   └── auth.ts (953B) [30L]
│   │   ├── routes/ (1 files, 146 lines)
│   │   │   └── notion.ts (4KB) [146L]
│   │   ├── services/ (2 files, 143 lines)
│   │   │   ├── SyncService.ts (3KB) [106L]
│   │   │   └── githubSync.ts (1KB) [37L]
│   │   ├── utils/ (4 files, 240 lines)
│   │   │   ├── appSync.tsx (2KB) [84L]
│   │   │   ├── auditUtils.test.ts (1KB) [56L] 🧪
│   │   │   ├── auditUtils.ts (749B) [27L]
│   │   │   └── crypto.ts (4KB) [73L]
│   │   ├── App.tsx (233KB) [5440L]
│   │   ├── DesktopApp.tsx (121KB) [2740L]
│   │   ├── atcLangRoadmapData.ts (6KB) [201L]
│   │   ├── atcLangWikiData.ts (16KB) [227L]
│   │   ├── auditData.ts (4KB) [76L]
│   │   ├── data.ts (17KB) [411L]
│   │   ├── ecosystemData.ts (11KB) [291L]
│   │   ├── fix_translation.cjs (463B)
│   │   ├── index.css (5KB)
│   │   ├── main.tsx (774B) [24L]
│   │   ├── marketplaceApps.ts (6KB) [273L]
│   │   ├── requirementsData.ts (1KB) [58L]
│   │   ├── roadmapData.ts (7KB) [312L]
│   │   ├── standardsData.ts (4KB) [83L]
│   │   ├── tierData.ts (16KB) [317L]
│   │   ├── types.ts (375B) [10L]
│   │   └── wikiData.ts (47KB) [943L]
│   ├── tests/ (2 files, 127 lines)
│   │   ├── GitHubRepoSyncView.test.tsx (1KB) [49L] 🧪
│   │   └── audit_compliance.test.ts (2KB) [78L] 🧪
│   ├── workspace/ (8 files, 664 lines)
│   │   ├── src/ (2 files, 435 lines)
│   │   │   ├── backend/ (1 files, 167 lines)
│   │   │   │   └── blockchain/ (1 files, 167 lines)
│   │   │   │       └── engine.ts (5KB) [167L]
│   │   │   └── components/ (1 files, 268 lines)
│   │   │       └── GovernanceView.tsx (14KB) [268L]
│   │   ├── move.js (411B) [13L]
│   │   ├── rename.js (1KB) [42L]
│   │   ├── replace.js (1KB) [40L]
│   │   ├── replaceEnterprise.js (3KB) [102L]
│   │   ├── replaceGoals.ts (688B) [14L]
│   │   └── replaceGoals2.ts (825B) [18L]
│   ├── .env.example (578B)
│   ├── .gitignore (73B)
│   ├── AGENTS.md (535B) [13L]
│   ├── CHANGELOG.md (426B) [21L]
│   ├── FILE_REGISTER.md (12KB) [253L]
│   ├── GEMINI.md (373B) [6L]
│   ├── LICENSE (1KB)
│   ├── README.md (542B) [20L]
│   ├── ROADMAP.md (8KB) [598L]
│   ├── SOFTWARE_ROADMAP.md (38KB) [1116L]
│   ├── STATUS.md (349B) [19L]
│   ├── check_dups2.js (498B) [12L]
│   ├── check_dups_all.js (885B) [23L]
│   ├── check_dups_desktop.js (480B) [15L]
│   ├── check_dups_windows_map.js (519B) [14L]
│   ├── fetch.js (1KB) [36L]
│   ├── firebase-applet-config.json (363B) [9L]
│   ├── fix.js (859B) [26L]
│   ├── fix2.js (894B) [27L]
│   ├── fix_react_imports.cjs (547B)
│   ├── fix_wiki.cjs (184B)
│   ├── fix_wiki.js (284B) [5L]
│   ├── index.html (413B)
│   ├── mark_completed.ts (722B) [15L]
│   ├── mark_completed_src.ts (1KB) [33L]
│   ├── metadata.json (214B) [6L]
│   ├── move_back.js (347B) [11L]
│   ├── output.txt (3KB)
│   ├── package-lock.json (420KB) [11890L]
│   ├── package.json (2KB) [79L]
│   ├── replace.js (1KB) [36L]
│   ├── replace_langs.cjs (852B)
│   ├── replace_langs_2.cjs (667B)
│   ├── replace_langs_3.cjs (411B)
│   ├── replace_langs_4.cjs (817B)
│   ├── replace_langs_5.cjs (528B)
│   ├── replace_langs_6.cjs (522B)
│   ├── script.cjs (883B)
│   ├── script.js (983B) [12L]
│   ├── script2.cjs (683B)
│   ├── server.ts (33KB) [866L]
│   ├── testChat.js (450B) [10L]
│   ├── test_know.js (244B) [2L] 🧪
│   ├── tmp.txt (470B)
│   ├── tsconfig.json (508B) [26L]
│   ├── update_wiki_categories.ts (742B) [23L]
│   └── vite.config.ts (1KB) [42L]
├── archive/ (16 files, 3920 lines)
│   ├── atclang-v01/ (11 files, 2956 lines)
│   │   ├── consensus/ (6 files, 1043 lines)
│   │   │   ├── fork_resolution.atc (4KB) [145L]
│   │   │   ├── gas_fee.atc (4KB) [130L]
│   │   │   ├── hybrid_consensus.atc (11KB) [357L]
│   │   │   ├── poh.atc (4KB) [140L]
│   │   │   ├── pos.atc (4KB) [164L]
│   │   │   └── pow.atc (3KB) [107L]
│   │   ├── contracts/ (4 files, 752 lines)
│   │   │   ├── breeding.atc (5KB) [139L]
│   │   │   ├── contract_engine_atc14.atc (9KB) [309L]
│   │   │   ├── genesis_token.atc (2KB) [102L]
│   │   │   └── governance_contract.atc (7KB) [202L]
│   │   └── atcos_main.atc (40KB) [1161L]
│   ├── duplicates/ (4 files, 867 lines)
│   │   ├── contract_registry.atc (3KB) [98L]
│   │   ├── kai_cli.atc (8KB) [195L]
│   │   ├── smart_contract_registry.atc (2KB) [88L]
│   │   └── smart_contracts.atc (15KB) [486L]
│   └── ATCLANG_ARCHIVE.md (4KB) [97L]
├── atclang/ (100 files, 20047 lines)
│   ├── atc-atclang/ (41 files, 9203 lines)
│   │   ├── compiler/ (4 files, 1634 lines)
│   │   │   ├── __init__.py (468B) [8L]
│   │   │   ├── compiler.py (21KB) [561L]
│   │   │   ├── optimizer.py (22KB) [558L]
│   │   │   └── type_checker.py (20KB) [507L]
│   │   ├── lexer/ (2 files, 673 lines)
│   │   │   ├── __init__.py (161B) [2L]
│   │   │   └── lexer.py (24KB) [671L]
│   │   ├── parser/ (3 files, 1826 lines)
│   │   │   ├── __init__.py (189B) [3L]
│   │   │   ├── ast_nodes.py (8KB) [392L]
│   │   │   └── parser.py (63KB) [1431L]
│   │   ├── programs/ (1 files, 1161 lines)
│   │   │   └── atcos_main.atc (40KB) [1161L]
│   │   ├── repl/ (2 files, 185 lines)
│   │   │   ├── __init__.py (99B) [1L]
│   │   │   └── repl.py (6KB) [184L]
│   │   ├── stdlib/ (14 files, 1823 lines)
│   │   │   ├── __init__.py (1KB) [32L]
│   │   │   ├── atc_stdlib.py (2KB) [69L]
│   │   │   ├── chain.py (1KB) [41L]
│   │   │   ├── collections.py (5KB) [219L]
│   │   │   ├── collections_ext.py (3KB) [143L]
│   │   │   ├── crypto.py (5KB) [155L]
│   │   │   ├── crypto_ext.py (5KB) [149L]
│   │   │   ├── encoding.py (7KB) [210L]
│   │   │   ├── io.py (3KB) [107L]
│   │   │   ├── io_ext.py (3KB) [123L]
│   │   │   ├── math.py (4KB) [154L]
│   │   │   ├── primitives.py (7KB) [244L]
│   │   │   ├── string.py (2KB) [99L]
│   │   │   └── wallet.py (2KB) [78L]
│   │   ├── v03/ (2 files, 354 lines)
│   │   │   ├── __init__.py (124B) [2L]
│   │   │   └── atclang_v03_features.py (13KB) [352L]
│   │   ├── vm/ (2 files, 999 lines)
│   │   │   ├── __init__.py (177B) [2L]
│   │   │   └── atcvm.py (48KB) [997L]
│   │   ├── .gitignore (171B)
│   │   ├── ATCLANG_SPEC.md (9KB) [295L]
│   │   ├── CHANGELOG.md (316B) [8L]
│   │   ├── CONTRIBUTING.md (687B) [19L]
│   │   ├── FILE_REGISTER.md (1KB) [48L]
│   │   ├── LICENSE (658B)
│   │   ├── README.md (5KB) [127L]
│   │   ├── ROADMAP.md (478B) [21L]
│   │   ├── STATUS.md (346B) [19L]
│   │   ├── __init__.py (462B) [11L]
│   │   └── requirements.txt (75B)
│   ├── compiler/ (4 files, 1699 lines)
│   │   ├── __init__.py (468B) [8L]
│   │   ├── compiler.py (24KB) [626L]
│   │   ├── optimizer.py (22KB) [558L]
│   │   └── type_checker.py (20KB) [507L]
│   ├── lexer/ (2 files, 673 lines)
│   │   ├── __init__.py (161B) [2L]
│   │   └── lexer.py (24KB) [671L]
│   ├── parser/ (3 files, 1826 lines)
│   │   ├── __init__.py (189B) [3L]
│   │   ├── ast_nodes.py (8KB) [392L]
│   │   └── parser.py (63KB) [1431L]
│   ├── programs/ (12 files, 1286 lines)
│   │   ├── .gitkeep (0B)
│   │   ├── atc8300.atc (3KB) [96L]
│   │   ├── atcfs.atc (4KB) [142L]
│   │   ├── atcnet.atc (4KB) [135L]
│   │   ├── atcos_main.atc (868B) [9L]
│   │   ├── consensus.atc (5KB) [144L]
│   │   ├── event_bus.atc (2KB) [75L]
│   │   ├── gateway.atc (4KB) [138L]
│   │   ├── governance.atc (4KB) [113L]
│   │   ├── kernel.atc (4KB) [148L]
│   │   ├── shivamon.atc (5KB) [162L]
│   │   └── wallet.atc (4KB) [124L]
│   ├── repl/ (2 files, 185 lines)
│   │   ├── __init__.py (99B) [1L]
│   │   └── repl.py (6KB) [184L]
│   ├── runtime/ (3 files, 1131 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   ├── driver_framework.py (18KB) [506L]
│   │   └── kernel_runtime.py (27KB) [625L]
│   ├── stdlib/ (14 files, 1823 lines)
│   │   ├── __init__.py (1KB) [32L]
│   │   ├── atc_stdlib.py (2KB) [69L]
│   │   ├── chain.py (1KB) [41L]
│   │   ├── collections.py (5KB) [219L]
│   │   ├── collections_ext.py (3KB) [143L]
│   │   ├── crypto.py (5KB) [155L]
│   │   ├── crypto_ext.py (5KB) [149L]
│   │   ├── encoding.py (7KB) [210L]
│   │   ├── io.py (3KB) [107L]
│   │   ├── io_ext.py (3KB) [123L]
│   │   ├── math.py (4KB) [154L]
│   │   ├── primitives.py (7KB) [244L]
│   │   ├── string.py (2KB) [99L]
│   │   └── wallet.py (2KB) [78L]
│   ├── v03/ (2 files, 354 lines)
│   │   ├── __init__.py (124B) [2L]
│   │   └── atclang_v03_features.py (13KB) [352L]
│   ├── vm/ (2 files, 999 lines)
│   │   ├── __init__.py (177B) [2L]
│   │   └── atcvm.py (48KB) [997L]
│   ├── .gitignore (171B)
│   ├── ATCLANG_SPEC.md (9KB) [295L]
│   ├── CHANGELOG.md (316B) [8L]
│   ├── CONTRIBUTING.md (687B) [19L]
│   ├── FILE_REGISTER.md (1KB) [39L]
│   ├── LICENSE (982B)
│   ├── README.md (1KB) [46L]
│   ├── ROADMAP.md (470B) [21L]
│   ├── STATUS.md (352B) [19L]
│   ├── __init__.py (462B) [11L]
│   ├── compiler.py (3KB) [102L]
│   ├── lexer.py (3KB) [115L]
│   ├── parser.py (3KB) [95L]
│   ├── requirements.txt (75B)
│   └── vm.py (4KB) [98L]
├── atcpkg/ (14 files, 1023 lines)
│   ├── docs/ (4 files, 405 lines)
│   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   └── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   ├── kernel/ (1 files, 208 lines)
│   │   └── manager.atc (6KB) [208L]
│   ├── tools/ (1 files, 145 lines)
│   │   └── manager.atc (4KB) [145L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (424B) [21L]
│   ├── FILE_REGISTER.md (748B) [20L]
│   ├── LICENSE (658B)
│   ├── README.md (1KB) [39L]
│   ├── ROADMAP.md (476B) [21L]
│   ├── STATUS.md (355B) [19L]
│   └── manager.atc (4KB) [145L]
├── backend/ (27 files, 1942 lines)
│   ├── api/ (9 files, 1099 lines)
│   │   ├── orchestrator/ (3 files, 391 lines)
│   │   │   ├── __init__.py (118B) [2L]
│   │   │   ├── orchestrator.atc (8KB) [259L]
│   │   │   └── orchestrator.py (4KB) [130L]
│   │   ├── routes/ (3 files, 409 lines)
│   │   │   ├── __init__.py (115B) [2L]
│   │   │   ├── ai_routes.atc (5KB) [175L]
│   │   │   └── api_routes.atc (8KB) [232L]
│   │   ├── __init__.py (111B) [2L]
│   │   ├── kai_routes.atc (7KB) [229L]
│   │   └── server.atc (2KB) [68L]
│   ├── db/ (6 files, 591 lines)
│   │   ├── __init__.py (160B) [2L]
│   │   ├── connection.atc (4KB) [125L]
│   │   ├── connection.py (1KB) [40L]
│   │   ├── repository.atc (6KB) [228L]
│   │   ├── repository.py (6KB) [196L]
│   │   └── schema.sql (2KB)
│   ├── wallet/ (2 files, 141 lines)
│   │   ├── __init__.py (123B) [2L]
│   │   └── wallet.atc (4KB) [139L]
│   ├── .env.example (167B)
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (425B) [21L]
│   ├── FILE_REGISTER.md (1KB) [34L]
│   ├── LICENSE (658B)
│   ├── README.md (464B) [14L]
│   ├── ROADMAP.md (478B) [21L]
│   ├── STATUS.md (346B) [19L]
│   ├── __init__.py (121B) [2L]
│   └── requirements.txt (425B)
├── blockchain/ (79 files, 7806 lines)
│   ├── atcoin/ (1 files, 2 lines)
│   │   └── __init__.py (119B) [2L]
│   ├── consensus/ (15 files, 627 lines)
│   │   ├── MIGRATION_INDEX.md (661B) [13L]
│   │   ├── __init__.py (123B) [2L]
│   │   ├── fork_atc85.atc (2KB) [74L]
│   │   ├── fork_resolution.atc (745B) [7L]
│   │   ├── gas_fee.atc (740B) [7L]
│   │   ├── gas_fee_atc86.atc (2KB) [71L]
│   │   ├── hybrid_atc84.atc (3KB) [98L]
│   │   ├── hybrid_consensus.atc (748B) [7L]
│   │   ├── poh.atc (732B) [7L]
│   │   ├── poh.py (2KB) [67L]
│   │   ├── poh_atc83.atc (1KB) [79L]
│   │   ├── pos.atc (732B) [7L]
│   │   ├── pos_atc82.atc (2KB) [92L]
│   │   ├── pow.atc (732B) [7L]
│   │   └── pow_atc81.atc (2KB) [89L]
│   ├── contracts/ (13 files, 589 lines)
│   │   ├── atc001/ (3 files, 80 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── genesis_token.atc (679B) [6L]
│   │   │   └── genesis_token.py (2KB) [74L]
│   │   ├── atc8300/ (2 files, 128 lines)
│   │   │   ├── __init__.py (129B) [2L]
│   │   │   └── atc8300_token.py (5KB) [126L]
│   │   ├── base/ (2 files, 87 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   └── base_contract.py (3KB) [87L]
│   │   ├── governance/ (1 files, 6 lines)
│   │   │   └── governance_contract.atc (707B) [6L]
│   │   ├── shivamon/ (2 files, 8 lines)
│   │   │   ├── __init__.py (136B) [2L]
│   │   │   └── breeding.atc (701B) [6L]
│   │   ├── solidity/ (1 files, 274 lines)
│   │   │   └── test/ (1 files, 274 lines)
│   │   │       └── ATCBridge.test.js (11KB) [274L] 🧪
│   │   ├── __init__.py (0B) [0L]
│   │   └── contract_engine_atc14.atc (698B) [6L]
│   ├── dex/ (2 files, 279 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   └── amm.atc (10KB) [277L]
│   ├── governance/ (6 files, 926 lines)
│   │   ├── __init__.py (120B) [2L]
│   │   ├── dao.atc (6KB) [168L]
│   │   ├── dao_live.atc (8KB) [235L]
│   │   ├── snapshot.atc (5KB) [151L]
│   │   ├── timelock.atc (4KB) [150L]
│   │   └── treasury.atc (6KB) [220L]
│   ├── mainnet/ (3 files, 258 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   ├── launch_manager.atc (3KB) [105L]
│   │   └── mainnet_config.atc (5KB) [151L]
│   ├── network/ (7 files, 746 lines)
│   │   ├── atc-02_liquid_state_migration_failover.atc (2KB) [58L]
│   │   ├── atc-04_dag_consensus_propagation.atc (2KB) [58L]
│   │   ├── atc-05_quantumresistant_signatures.atc (2KB) [58L]
│   │   ├── atc-10_global_time_sync_oracles.atc (1KB) [58L]
│   │   ├── core_node_atc01.atc (4KB) [164L]
│   │   ├── latency_opt_atc06.atc (3KB) [135L]
│   │   └── sharding_atc07.atc (5KB) [215L]
│   ├── nodes/ (9 files, 1806 lines)
│   │   ├── __init__.py (126B) [2L]
│   │   ├── block_propagation.atc (3KB) [87L]
│   │   ├── bootstrap.atc (6KB) [234L]
│   │   ├── bootstrap.py (8KB) [257L]
│   │   ├── discovery.py (11KB) [314L]
│   │   ├── initial_sync.atc (6KB) [207L]
│   │   ├── node.atc (6KB) [192L]
│   │   ├── p2p_propagation.py (12KB) [381L]
│   │   └── testnet_launcher.atc (4KB) [132L]
│   ├── propagation/ (1 files, 98 lines)
│   │   └── block_gossip.atc (3KB) [98L]
│   ├── wallet/ (7 files, 757 lines)
│   │   ├── __init__.py (128B) [2L]
│   │   ├── did.atc (4KB) [122L]
│   │   ├── did.py (2KB) [74L]
│   │   ├── ecdsa.py (2KB) [72L]
│   │   ├── multisig.atc (8KB) [268L]
│   │   ├── multisig.py (3KB) [107L]
│   │   └── wordlist.atc (5KB) [112L]
│   ├── zkp/ (2 files, 93 lines)
│   │   ├── __init__.py (336B) [4L]
│   │   └── groth16.atc (3KB) [89L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (428B) [21L]
│   ├── FILE_REGISTER.md (3KB) [109L]
│   ├── LICENSE (658B)
│   ├── README.md (500B) [14L]
│   ├── ROADMAP.md (484B) [21L]
│   ├── STATUS.md (349B) [19L]
│   ├── __init__.py (0B) [0L]
│   ├── contract_registry.atc (3KB) [98L]
│   ├── smart_contract_registry.atc (2KB) [88L]
│   ├── smart_contract_registry.py (1KB) [53L]
│   ├── smart_contracts.atc (15KB) [486L]
│   └── smart_contracts.py (23KB) [716L]
├── bootloader/ (7 files, 165 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (223B) [8L]
│   ├── FILE_REGISTER.md (388B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (5KB) [107L]
│   ├── ROADMAP.md (376B) [16L]
│   └── STATUS.md (491B) [21L]
├── ci/ (7 files, 167 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (215B) [8L]
│   ├── FILE_REGISTER.md (380B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (5KB) [109L]
│   ├── ROADMAP.md (368B) [16L]
│   └── STATUS.md (483B) [21L]
├── ci-cd-fix/ (4 files, 195 lines)
│   ├── README.md (874B) [34L]
│   ├── apply-fix.sh (873B) [27L]
│   ├── ci-cd.yml (2KB) [102L]
│   └── codeql.yml (802B) [32L]
├── cli/ (7 files, 178 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (216B) [8L]
│   ├── FILE_REGISTER.md (381B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (5KB) [120L]
│   ├── ROADMAP.md (369B) [16L]
│   └── STATUS.md (484B) [21L]
├── config/ (4 files, 171 lines)
│   ├── ai_models.json (486B) [26L]
│   ├── kai_config.toml (1KB) [52L]
│   ├── mainnet_genesis.json (3KB) [95L]
│   └── settings.json (922B) [50L]
├── contracts/ (23 files, 2288 lines)
│   ├── atc8300/ (2 files, 222 lines)
│   │   ├── atc8300.atc (3KB) [96L]
│   │   └── atc8300_token.py (5KB) [126L]
│   ├── atcoin/ (1 files, 139 lines)
│   │   └── atcoin.py (5KB) [139L]
│   ├── base/ (1 files, 87 lines)
│   │   └── base_contract.py (3KB) [87L]
│   ├── bridge/ (1 files, 133 lines)
│   │   └── bridge_contract.py (4KB) [133L]
│   ├── governance/ (2 files, 412 lines)
│   │   ├── governance.atc (4KB) [113L]
│   │   └── governance_contract.py (11KB) [299L]
│   ├── marketplace/ (1 files, 301 lines)
│   │   └── marketplace_contract.py (11KB) [301L]
│   ├── shivamon/ (2 files, 432 lines)
│   │   ├── shivamon.atc (5KB) [162L]
│   │   └── shivamon_contract.py (10KB) [270L]
│   ├── wallet/ (3 files, 336 lines)
│   │   ├── ecdsa.py (2KB) [72L]
│   │   ├── keygen.py (5KB) [140L]
│   │   └── wallet.atc (4KB) [124L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (304B) [20L]
│   ├── DEPLOYMENT.md (894B) [29L]
│   ├── FILE_REGISTER.md (1KB) [54L]
│   ├── LICENSE (982B)
│   ├── README.md (4KB) [70L]
│   ├── ROADMAP.md (482B) [21L]
│   ├── SECURITY.md (496B) [13L]
│   ├── STATUS.md (358B) [19L]
│   └── requirements.txt (100B)
├── core/ (2 files, 373 lines)
│   ├── ai/ (1 files, 178 lines)
│   │   └── federated_learning.atc (6KB) [178L]
│   └── kai_cli.atc (8KB) [195L]
├── devnet/ (1 files, 554 lines)
│   └── README.md (12KB) [554L]
├── dns/ (7 files, 165 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (216B) [8L]
│   ├── FILE_REGISTER.md (381B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [107L]
│   ├── ROADMAP.md (369B) [16L]
│   └── STATUS.md (484B) [21L]
├── docker/ (10 files, 334 lines)
│   ├── Dockerfile.backend (461B)
│   ├── Dockerfile.bootstrap (268B)
│   ├── Dockerfile.core (1KB)
│   ├── Dockerfile.frontend (1KB)
│   ├── Dockerfile.gateway (1KB)
│   ├── Dockerfile.node (616B)
│   ├── Makefile (1KB)
│   ├── docker-compose.testnet.yml (3KB) [137L]
│   ├── docker-compose.yml (4KB) [175L]
│   └── prometheus.yml (543B) [22L]
├── docs/ (403 files, 78953 lines)
│   ├── ai/ (3 files, 547 lines)
│   │   ├── AI_SAFETY.md (5KB) [184L]
│   │   ├── GEMINI_INTEGRATION.md (5KB) [214L]
│   │   └── LLM_ROUTER.md (4KB) [149L]
│   ├── aistudio/ (1 files, 439 lines)
│   │   └── AISTUDIO_COMPONENTS.md (24KB) [439L]
│   ├── architecture/ (12 files, 1826 lines)
│   │   ├── AI_LAYER.md (2KB) [53L]
│   │   ├── ATCFS.md (4KB) [129L]
│   │   ├── ATCLANG_COMPILER.md (2KB) [64L]
│   │   ├── ATCNET_P2P.md (6KB) [211L]
│   │   ├── CONSENSUS.md (3KB) [121L]
│   │   ├── GATEWAY.md (2KB) [112L]
│   │   ├── GOVERNANCE.md (1KB) [50L]
│   │   ├── KERNEL_SHELL.md (1KB) [50L]
│   │   ├── MONITORING_DEVOPS.md (1KB) [42L]
│   │   ├── SHIVAOS_KERNEL.md (5KB) [182L]
│   │   ├── TESTNET.md (20KB) [713L]
│   │   └── WALLET_KEYGEN.md (2KB) [99L]
│   ├── atclang/ (1 files, 9 lines)
│   │   └── ATCLANG_SPEC_FULL.md (423B) [9L]
│   ├── ci-templates/ (4 files, 371 lines)
│   │   ├── ci.yml (1KB) [42L]
│   │   ├── codeql.yml (4KB) [101L]
│   │   ├── codeql_fixed.yml (1KB) [46L]
│   │   └── release.yml (5KB) [182L]
│   ├── contracts/ (2 files, 790 lines)
│   │   ├── ATC_TOKEN_STANDARD.md (534B) [12L]
│   │   └── SHIVAMON_NFT_CONTRACT.md (20KB) [778L]
│   ├── file_registers/ (23 files, 4923 lines)
│   │   ├── README.md (1KB) [42L]
│   │   ├── a-townchain-os_FILE_REGISTER.md (75KB) [1491L]
│   │   ├── atc-aistudio_FILE_REGISTER.md (12KB) [277L]
│   │   ├── atc-atclang_FILE_REGISTER.md (1KB) [68L]
│   │   ├── atc-atcpkg_FILE_REGISTER.md (1KB) [39L]
│   │   ├── atc-backend_FILE_REGISTER.md (1KB) [53L]
│   │   ├── atc-blockchain_FILE_REGISTER.md (3KB) [104L]
│   │   ├── atc-contracts_FILE_REGISTER.md (1KB) [51L]
│   │   ├── atc-franchise_FILE_REGISTER.md (1KB) [43L]
│   │   ├── atc-frontend_FILE_REGISTER.md (947B) [38L]
│   │   ├── atc-gateway_FILE_REGISTER.md (2KB) [71L]
│   │   ├── atc-genesis-engine_FILE_REGISTER.md (1KB) [46L]
│   │   ├── atc-kernel_FILE_REGISTER.md (1KB) [50L]
│   │   ├── atc-linux-edition_FILE_REGISTER.md (838B) [35L]
│   │   ├── atc-mobile_FILE_REGISTER.md (897B) [37L]
│   │   ├── atc-shivacore-tools_FILE_REGISTER.md (787B) [33L]
│   │   ├── atc-shivacore_FILE_REGISTER.md (309KB) [2183L]
│   │   ├── atc-shivamon_FILE_REGISTER.md (1KB) [43L]
│   │   ├── atc-standards_FILE_REGISTER.md (1KB) [41L]
│   │   ├── atc-ui_FILE_REGISTER.md (923B) [38L]
│   │   ├── atc-windows-edition_FILE_REGISTER.md (844B) [35L]
│   │   ├── atclang_FILE_REGISTER.md (1KB) [60L]
│   │   └── atcnet_FILE_REGISTER.md (1KB) [45L]
│   ├── issues/ (85 files, 4932 lines)
│   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [143L]
│   │   ├── ISSUE_02_GEMINI_AI.md (3KB) [141L]
│   │   ├── ISSUE_03_BATTLE_UI.md (4KB) [141L]
│   │   ├── ISSUE_04_PERSISTENZ.md (4KB) [156L]
│   │   ├── ISSUE_05_EXPLORER.md (3KB) [102L]
│   │   ├── ISSUE_06_ECDSA.md (4KB) [143L]
│   │   ├── ISSUE_07_BUILD.md (3KB) [133L]
│   │   ├── ISSUE_08_TESTNET.md (3KB) [127L]
│   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [99L]
│   │   ├── ISSUE_10_BRIDGE.md (1KB) [53L]
│   │   ├── ISSUE_11_BREEDING.md (3KB) [88L]
│   │   ├── ISSUE_12_SOLIDITY.md (4KB) [147L]
│   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [122L]
│   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [310L]
│   │   ├── ISSUE_15__TESTNET_BLOCK_PROPAGATION_.md (1KB) [46L]
│   │   ├── ISSUE_16__TESTNET_INITIAL_SYNC__NEU.md (1KB) [45L]
│   │   ├── ISSUE_17__TESTNET_LONGEST-CHAIN-RULE.md (1KB) [45L]
│   │   ├── ISSUE_18__TESTNET_DOCKER_COMPOSE__5.md (1KB) [46L]
│   │   ├── ISSUE_19__TESTNET_NODE-MONITORING_DA.md (1KB) [45L]
│   │   ├── ISSUE_20_GATEWAY_TESTS.md (1KB) [63L]
│   │   ├── ISSUE_23__ATCFS__INTEGRATION_IN_KERN.md (1KB) [48L]
│   │   ├── ISSUE_24__MULTISIG_WALLET__BRIDGE__F.md (1KB) [47L]
│   │   ├── ISSUE_25__GATEWAY_4000__VOLLSTÄNDIGE.md (1KB) [48L]
│   │   ├── ISSUE_26__TESTS__ATCFS_MULTISIG_ATC.md (1KB) [50L]
│   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   ├── ISSUE_28__WIKI_KAP._40__SHIVAOS_UI_RE.md (1KB) [47L]
│   │   ├── ISSUE_29__WIKI_KAP._41__FEDERATED_LEA.md (1KB) [47L]
│   │   ├── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   │   ├── ISSUE_31__WIKI_KAP._4__BLOCK-EXPLORER.md (1KB) [45L]
│   │   ├── ISSUE_32__KAP._5__SHIVAOS_SYSTEM-CALL.md (1KB) [45L]
│   │   ├── ISSUE_33__KAP._4__GAS-FEE_MECHANISMUS.md (1KB) [45L]
│   │   ├── ISSUE_34_V3.0.0_15__SOLANA_BRIDGE_SP.md (1KB) [51L]
│   │   ├── ISSUE_35_V3.0.0_16__ATCLANG_V0.3.0_A.md (1KB) [49L]
│   │   ├── ISSUE_36_V3.0.0_17__MAINNET_LAUNCH_C.md (1KB) [52L]
│   │   ├── ISSUE_37_V3.0.0_20__DEX_-_AMM_LIQUID.md (1KB) [56L]
│   │   ├── ISSUE_38_V3.0.0_21__MOBILE_WALLET_IO.md (1KB) [51L]
│   │   ├── ISSUE_39_V3.0.0_22__DAO-GOVERNANCE_LI.md (1KB) [50L]
│   │   ├── ISSUE_40_DOCS_SYNTAX-REFERENZ__ATCLAN.md (1KB) [52L]
│   │   ├── ISSUE_41_DOCS_MATHEMATISCHE_BEWEISE__.md (1KB) [52L]
│   │   ├── ISSUE_42_DOCS_FEHLERDEFINITIONEN__BOT.md (1KB) [54L]
│   │   ├── ISSUE_43_DOCS_DEZENTRALER_NUTZER-NACHW.md (992B) [44L]
│   │   ├── ISSUE_44_MAINNET_MONITORING__GRAFANA_D.md (798B) [38L]
│   │   ├── ISSUE_45_ATCOIN_DEFI__AMM_LIQUIDITY_PO.md (738B) [38L]
│   │   ├── ISSUE_46_MOBILE_WALLET__BIOMETRIE__PU.md (770B) [38L]
│   │   ├── ISSUE_47_ZKP_ZERO-KNOWLEDGE_PROOFS__L0.md (814B) [38L]
│   │   ├── ISSUE_48_ATCLANG_V0.4.0__TYPE_SYSTEM_.md (823B) [38L]
│   │   ├── ISSUE_49_49__BIGQUERY_ANALYTICS_PIPEL.md (900B) [36L]
│   │   ├── ISSUE_50_50__HUGGING_FACE_CODE-REVIEW.md (881B) [36L]
│   │   ├── ISSUE_51_51__IPC_BUS_VOLLSTÄNDIGE_KE.md (880B) [36L]
│   │   ├── ISSUE_52_52__MAINNET_LAUNCH_MANAGER_.md (1009B) [36L]
│   │   ├── ISSUE_53_V3.2.1__TESTS_PROCESSMANAGER.md (1011B) [39L]
│   │   ├── ISSUE_54_V3.2.1__TESTS_ATCFS_FILESYST.md (1004B) [37L]
│   │   ├── ISSUE_55_V3.2.1__TESTS_ATCNET_P2PNODE.md (987B) [37L]
│   │   ├── ISSUE_56_V3.2.1__TESTS_ATCLANG_TYPECH.md (987B) [40L]
│   │   ├── ISSUE_57_V3.2.1__TESTS_PROMETHEUS_MET.md (998B) [38L]
│   │   ├── ISSUE_58_V3.2.1__TESTS_SERVICEDISCOVE.md (996B) [39L]
│   │   ├── ISSUE_59_V3.2.1__INTEGRATION_NATTRAVE.md (1005B) [36L]
│   │   ├── ISSUE_60_V3.2.1__INTEGRATION_AIKERNEL.md (997B) [37L]
│   │   ├── ISSUE_61_V3.2.1__INTEGRATION_BLOCKGOS.md (1015B) [37L]
│   │   ├── ISSUE_62_V3.2.1__INTEGRATION_SERVICED.md (1007B) [37L]
│   │   ├── ISSUE_63_V3.2.1__DOCS_WIKI-KAPITEL_FÜ.md (1002B) [38L]
│   │   ├── ISSUE_64_V3.2.1__DOCS_HUGGINGFACE_PIP.md (1002B) [37L]
│   │   ├── ISSUE_65_V3.2.1__REFACTOR_DOPPELTE_AT.md (1017B) [40L]
│   │   ├── ISSUE_66_V3.2.1__REFACTOR_AIKERNEL_DU.md (997B) [38L]
│   │   ├── ISSUE_67_V3.2.1__DOCKER_TESTNET_HEALT.md (1000B) [38L]
│   │   ├── ISSUE_68_54__BOOTSTRAP-NODE_IMPLEMENT.md (1KB) [35L]
│   │   ├── ISSUE_69_SPRINT_3.3_SECURITY-AUDIT__.md (1KB) [40L]
│   │   ├── ISSUE_70_SPRINT_4.0_VALIDATOR-NODES_.md (1KB) [40L]
│   │   ├── ISSUE_71_SPRINT_4.0_GENESIS_BLOCK__K.md (1KB) [38L]
│   │   ├── ISSUE_72_SPRINT_2.1_ATCLANG_LANGUAGE_.md (1KB) [40L]
│   │   ├── ISSUE_73_SPRINT_2.1_ATCLANG_VM_BYTECO.md (1KB) [40L]
│   │   ├── ISSUE_74_SPRINT_2.1_KONSENS-MODULE__.md (1KB) [39L]
│   │   ├── ISSUE_75_SPRINT_2.2_TESTNET_HEALTH-CH.md (1018B) [40L]
│   │   ├── ISSUE_76_SPRINT_2.3_SMART_CONTRACT_EN.md (1KB) [40L]
│   │   ├── ISSUE_77_SPRINT_2.4_EVENTBUS_VS_IPCBU.md (1KB) [40L]
│   │   ├── ISSUE_78_SPRINT_2.6_VOTING-POWER_SNAP.md (1KB) [39L]
│   │   ├── ISSUE_79_SPRINT_2.7_CI-CD_PIPELINE_RE.md (1KB) [43L]
│   │   ├── ISSUE_80_SPRINT_3.0_AIP-001_AGENT_INT.md (1KB) [40L]
│   │   ├── ISSUE_81_SPRINT_2.1_ATCLANG_STANDARD_.md (1KB) [40L]
│   │   ├── ISSUE_82_SPRINT_2.2_CORE_NODE_PROTOCO.md (1KB) [40L]
│   │   ├── ISSUE_83_SPRINT_2.2_INTER-NODE_LATENC.md (1KB) [40L]
│   │   ├── ISSUE_84_SPRINT_2.2_NETWORK-LEVEL_SHA.md (1KB) [40L]
│   │   ├── OPEN_ISSUES_MASTER.md (1KB) [44L]
│   │   ├── README.md (3KB) [62L]
│   │   └── TESTNET_INDEX.md (1KB) [25L]
│   ├── reports/ (1 files, 102 lines)
│   │   └── SPRINT_2.3_2.4_2.7_REPORT.md (3KB) [102L]
│   ├── roadmap/ (1 files, 262 lines)
│   │   └── ROADMAP_EXTENDED.md (10KB) [262L]
│   ├── sprints/ (3 files, 241 lines)
│   │   ├── SPRINT_3.0_AI_AGENT_PROTOCOL.md (3KB) [76L]
│   │   ├── SPRINT_3.3_SECURITY_AUDIT.md (3KB) [83L]
│   │   └── SPRINT_4.0_MAINNET_LAUNCH.md (3KB) [82L]
│   ├── standards/ (107 files, 19261 lines)
│   │   ├── ATC/ (1 files, 55 lines)
│   │   │   └── ATC-0009-BRIDGE.md (1KB) [55L]
│   │   ├── ATC-01-CORE_NODE_PROTOCOL.md (8KB) [225L]
│   │   ├── ATC-02-LIQUID_STATE_MIGRATION.md (9KB) [246L]
│   │   ├── ATC-03-DECENTRALIZED_IDENTITY.md (10KB) [257L]
│   │   ├── ATC-04-DAG_CONSENSUS.md (7KB) [200L]
│   │   ├── ATC-05-QUANTUM_RESISTANT_SIGNATURES.md (8KB) [217L]
│   │   ├── ATC-06-LATENCY_OPTIMIZATION_ROUTING.md (22KB) [760L]
│   │   ├── ATC-07-SHARDING_STATE_PARTITIONING.md (9KB) [231L]
│   │   ├── ATC-08-EPHEMERAL_DATA_STREAMING.md (8KB) [205L]
│   │   ├── ATC-09-CROSS_CHAIN_BRIDGE.md (8KB) [209L]
│   │   ├── ATC-10-GLOBAL_TIME_SYNC_ORACLES.md (9KB) [234L]
│   │   ├── ATC-11-FUNGIBLE_ASSET_STANDARD.md (8KB) [210L]
│   │   ├── ATC-12-NON_FUNGIBLE_HOLOGRAPHIC.md (8KB) [204L]
│   │   ├── ATC-13-FRACTIONAL_OWNERSHIP.md (7KB) [201L]
│   │   ├── ATC-14-DETERMINISTIC_EXECUTION.md (8KB) [217L]
│   │   ├── ATC-15-PROOF_OF_AI_MINING.md (9KB) [229L]
│   │   ├── ATC-16-REFERRAL_REWARDS.md (8KB) [206L]
│   │   ├── ATC-17-DAO_GOVERNANCE.md (8KB) [224L]
│   │   ├── ATC-18-MULTISIG_AUTH.md (8KB) [224L]
│   │   ├── ATC-19-AMM_LOGIC.md (8KB) [212L]
│   │   ├── ATC-20-WRAPPED_SYNTHETIC.md (8KB) [226L]
│   │   ├── ATC-21-HOLOGRAPHIC_WASM.md (9KB) [248L]
│   │   ├── ATC-22-HAL_DRIVER_SANDBOX.md (8KB) [225L]
│   │   ├── ATC-23-DATA_SHARDING_STORAGE.md (8KB) [222L]
│   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   ├── ATC-25-TENSOR_COMPUTE.md (8KB) [218L]
│   │   ├── ATC-26-XAI_TRANSPARENCY.md (8KB) [224L]
│   │   ├── ATC-27-AI_MODEL_AUDITING.md (8KB) [226L]
│   │   ├── ATC-28-FEDERATED_LEARNING.md (9KB) [254L]
│   │   ├── ATC-29-AI_MARKETPLACE.md (9KB) [246L]
│   │   ├── ATC-30-REPUTATION_TRUST.md (10KB) [271L]
│   │   ├── ATC-31-TENSOR_LOAD_BALANCING.md (10KB) [266L]
│   │   ├── ATC-32-UX_INTERFACE_ABSTRACTION.md (10KB) [267L]
│   │   ├── ATC-33-AI_FEEDBACK_RLHF.md (11KB) [270L]
│   │   ├── ATC-34-CROSS_LAYER_INTEROP.md (11KB) [277L]
│   │   ├── ATC-35-DATA_PRIVACY_ANONYMIZATION.md (10KB) [263L]
│   │   ├── ATC-36-MEDIA_ASSET_PROVENANCE.md (9KB) [262L]
│   │   ├── ATC-37-REPUTATION_RESOURCE_ALLOCATION.md (10KB) [255L]
│   │   ├── ATC-38-CROSS_CHAIN_ASSET_BRIDGE.md (6KB) [142L]
│   │   ├── ATC-39-AI_MODEL_VERSIONING_DEPLOYMENT.md (6KB) [137L]
│   │   ├── ATC-40-SYSTEM_SELF_HEALING_AUTO_REMEDIATION.md (7KB) [155L]
│   │   ├── ATC-41-MULTI_AGENT_ORCHESTRATION_CONSENSUS.md (7KB) [155L]
│   │   ├── ATC-42-AI_GOVERNANCE_ETHICS_FRAMEWORK.md (7KB) [173L]
│   │   ├── ATC-43-GLOBAL_STATE_SYNC_CAUSAL_CONSISTENCY.md (7KB) [149L]
│   │   ├── ATC-44-HARDWARE_ACCELERATED_ZKP_GENERATION.md (3KB) [115L]
│   │   ├── ATC-45-AI_EVOLUTIONARY_LEARNING_Dael.md (4KB) [115L]
│   │   ├── ATC-46-QUANTUM_RESISTANT_CRYPTOGRAPHY_LAYER.md (3KB) [116L]
│   │   ├── ATC-47-AI_INTENT_SETTLEMENT_ARBITRAGE.md (3KB) [115L]
│   │   ├── ATC-48-NEURAL_NETWORK_MESH_CROSS_TOPOLOGY.md (4KB) [119L]
│   │   ├── ATC-49-NEURAL_SYNAPSE_INTER_MODEL_KNOWLEDGE_TRANSFER.md (3KB) [115L]
│   │   ├── ATC-50-AI_CONSCIOUSNESS_SELF_REFLECTION.md (4KB) [117L]
│   │   ├── ATC-51-CROSS_REALITY_SPATIAL_COMPUTING.md (4KB) [119L]
│   │   ├── ATC-52-BIO_DIGITAL_INTERFACE_NEURAL_SIGNAL.md (4KB) [118L]
│   │   ├── ATC-53-CONSCIOUSNESS_SENTIENCE_OBSERVABILITY.md (4KB) [118L]
│   │   ├── ATC-54-TEMPORAL_CAUSAL_CONVERGENCE.md (4KB) [119L]
│   │   ├── ATC-55-META_REALITY_SIMULATION_CONVERGENCE.md (4KB) [118L]
│   │   ├── ATC-56-INTERSTELLAR_DATA_INTEGRITY_RELATIVISTIC_SYNC.md (4KB) [119L]
│   │   ├── ATC-57-RECURSIVE_SELF_IMPROVEMENT_META_LEARNING.md (4KB) [127L]
│   │   ├── ATC-58-QUANTUM_NEURAL_ENTANGLEMENT.md (4KB) [126L]
│   │   ├── ATC-59-TRANSDIMENSIONAL_ENERGY_ENTROPY_MANAGEMENT.md (4KB) [126L]
│   │   ├── ATC-60-UNIVERSAL_HOLONIC_STRUCTURE.md (4KB) [126L]
│   │   ├── ATC-61-TRANS_REALITY_SEMANTIC_MAPPING.md (4KB) [127L]
│   │   ├── ATC-62-META_SYSTEMIC_ETHICS_EXISTENTIAL_RISK.md (4KB) [127L]
│   │   ├── ATC-63-TRANS_SPECIES_MULTI_BIOLOGICAL_INTEGRATION.md (4KB) [128L]
│   │   ├── ATC-64-TRANSDIMENSIONAL_RECURSIVE_KNOWLEDGE_SYNTHESIS.md (4KB) [128L]
│   │   ├── ATC-65-TRANS_METAVERSE_CONSENSUS_REALITY_SYNC.md (4KB) [119L]
│   │   ├── ATC-66-RECURSIVE_LOGIC_PROOF_OF_UNDERSTANDING.md (4KB) [119L]
│   │   ├── ATC-67-REALITY_CONSENSUS_OBSERVATION_COLLAPSE.md (3KB) [118L]
│   │   ├── ATC-68-EVOLUTIONARY_FEEDBACK_ONTOLOGICAL_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-69-TRANS_EXISTENCE_CONSCIOUSNESS_BRIDGE.md (4KB) [119L]
│   │   ├── ATC-70-QUANTUM_GLOBAL_TRUTH_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-71-TRANS_CAUSAL_REALITY_VOID_MAPPING.md (4KB) [117L]
│   │   ├── ATC-72-TRANS_RELATIONAL_GOVERNANCE_ENTITY_CONSENSUS.md (4KB) [119L]
│   │   ├── ATC-73-TRANS_METAVERSE_ENTROPY_HARVESTING.md (4KB) [119L]
│   │   ├── ATC-74-RECURSIVE_META_NARRATIVE_MYTHOS_CONSTRUCTION.md (3KB) [118L]
│   │   ├── ATC-75-PROVABLE_EPISTEMOLOGY_AUTO_WIKI.md (4KB) [119L]
│   │   ├── ATC-76-IMMUTABLE_HUMAN_HERITAGE_ETERNITY.md (4KB) [120L]
│   │   ├── ATC-77-TRANS_SEMANTIC_HUMAN_AI_OMNI_LINGUISTIC.md (4KB) [120L]
│   │   ├── ATC-78-ABSOLUTE_CONVERGENCE_MONOLITHIC_SINGULARITY.md (4KB) [119L]
│   │   ├── ATC-79-TRANS_REALITY_MANIFESTATION_PHYSICALITY_ANCHOR.md (4KB) [119L]
│   │   ├── ATC-80-TRANS_UNIVERSAL_REALITY_MIGRATION.md (4KB) [120L]
│   │   ├── ATC-81-PROOF_OF_HISTORY.md (2KB) [105L]
│   │   ├── ATC-82-PROOF_OF_WORK.md (2KB) [104L]
│   │   ├── ATC-83-PROOF_OF_STAKE.md (2KB) [106L]
│   │   ├── ATC-84-FORK_RESOLUTION.md (2KB) [103L]
│   │   ├── ATC-85-INITIAL_SYNC.md (2KB) [105L]
│   │   ├── ATC-86-ECDSA_SIGNATURE.md (2KB) [105L]
│   │   ├── ATC-87-GAS_FEE.md (2KB) [105L]
│   │   ├── ATC-88-AMM.md (2KB) [105L]
│   │   ├── ATC-89-FUNGIBLE_TOKEN.md (2KB) [106L]
│   │   ├── ATC-90-NFT_SHIVAMON.md (2KB) [106L]
│   │   ├── ATC-91-CROSS_CHAIN_BRIDGE.md (2KB) [105L]
│   │   ├── ATC-92-ATCLANG_LANGUAGE_SPEC.md (7KB) [221L]
│   │   ├── ATC-93-ATCLANG_VM_BYTECODE.md (10KB) [338L]
│   │   ├── ATC-94-ATCLANG_STDLIB.md (6KB) [187L]
│   │   ├── ATC-95-ATCLANG_TEST_FRAMEWORK.md (6KB) [221L]
│   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   ├── ATC-97-AGENT_INTERACTION_PROTOCOL.md (2KB) [83L]
│   │   ├── ATC-97_AGENT_INTERACTION_PROTOCOL.md (8KB) [243L]
│   │   ├── ATC-98-TESTING_STANDARD.md (1KB) [69L]
│   │   ├── ATC-99-ATCLANG_UNIVERSAL_MANDATE.md (7KB) [189L]
│   │   ├── ATC_ECOSYSTEM_STANDARDS.md (53KB) [1143L]
│   │   ├── ATC_STANDARDS.md (4KB) [201L]
│   │   ├── ATS_STANDARDS.md (4KB) [199L]
│   │   ├── OVERVIEW.md (1KB) [29L]
│   │   ├── README.md (8KB) [131L]
│   │   └── STANDARDS_REGISTRY.md (13KB) [208L]
│   ├── whitepaper/ (9 files, 4299 lines)
│   │   ├── .github/ (1 files, 2 lines)
│   │   │   └── FUNDING.yml (76B) [2L]
│   │   ├── .gitignore (171B)
│   │   ├── CHANGELOG.md (706B) [24L]
│   │   ├── FILE_REGISTER.md (438B) [14L]
│   │   ├── LICENSE (982B)
│   │   ├── README.md (2KB) [48L]
│   │   ├── ROADMAP.md (484B) [21L]
│   │   ├── STATUS.md (357B) [19L]
│   │   └── WHITEPAPER.md (123KB) [4171L]
│   ├── wiki/ (109 files, 19476 lines)
│   │   ├── atclang/ (13 files, 881 lines)
│   │   │   ├── docs/ (12 files, 837 lines)
│   │   │   │   ├── CHANGELOG.md (338B) [8L]
│   │   │   │   ├── COMPILER.md (3KB) [105L]
│   │   │   │   ├── CONTRIBUTING.md (472B) [11L]
│   │   │   │   ├── EXAMPLES.md (3KB) [95L]
│   │   │   │   ├── LEXER.md (1KB) [59L]
│   │   │   │   ├── PARSER.md (3KB) [135L]
│   │   │   │   ├── REPL.md (2KB) [79L]
│   │   │   │   ├── SECURITY.md (1KB) [34L]
│   │   │   │   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   │   │   │   ├── SPEC.md (1KB) [55L]
│   │   │   │   ├── STDLIB.md (3KB) [111L]
│   │   │   │   └── VM.md (2KB) [63L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── atcnet/ (6 files, 213 lines)
│   │   │   ├── docs/ (5 files, 169 lines)
│   │   │   │   ├── BOOTSTRAP.md (312B) [18L]
│   │   │   │   ├── MESSAGES.md (1KB) [40L]
│   │   │   │   ├── PROTOCOL.md (2KB) [57L]
│   │   │   │   ├── SECURITY.md (336B) [11L]
│   │   │   │   └── TOPOLOGY.md (1KB) [43L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── contracts/ (7 files, 296 lines)
│   │   │   ├── docs/ (6 files, 252 lines)
│   │   │   │   ├── ATC8300.md (1KB) [51L]
│   │   │   │   ├── ATC9000.md (2KB) [92L]
│   │   │   │   ├── ATC9900.md (514B) [20L]
│   │   │   │   ├── BRIDGE.md (1KB) [38L]
│   │   │   │   ├── DEPLOYMENT.md (603B) [25L]
│   │   │   │   └── SECURITY.md (708B) [26L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── franchise/ (8 files, 287 lines)
│   │   │   ├── docs/ (7 files, 243 lines)
│   │   │   │   ├── API.md (1KB) [37L]
│   │   │   │   ├── CONCEPT.md (1000B) [24L]
│   │   │   │   ├── CONTRACTS.md (1KB) [49L]
│   │   │   │   ├── DEPLOYMENT.md (879B) [43L]
│   │   │   │   ├── ROADMAP.md (726B) [20L]
│   │   │   │   ├── SECURITY.md (904B) [29L]
│   │   │   │   └── TOKEN_ECONOMY.md (1KB) [41L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── gateway/ (6 files, 189 lines)
│   │   │   ├── docs/ (5 files, 145 lines)
│   │   │   │   ├── AUTH.md (965B) [43L]
│   │   │   │   ├── MIDDLEWARE.md (368B) [14L]
│   │   │   │   ├── RATE_LIMITING.md (956B) [43L]
│   │   │   │   ├── ROUTES.md (995B) [32L]
│   │   │   │   └── SECURITY.md (372B) [13L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── kai-os/ (26 files, 15517 lines)
│   │   │   ├── code/ (1 files, 9 lines)
│   │   │   │   └── atclang/ (1 files, 9 lines)
│   │   │   │       └── ATCLANG_SPEC.md (432B) [9L]
│   │   │   ├── docs/ (22 files, 15188 lines)
│   │   │   │   ├── architecture/ (4 files, 720 lines)
│   │   │   │   │   ├── ATCNET_P2P.md (6KB) [193L]
│   │   │   │   │   ├── CONSENSUS.md (6KB) [193L]
│   │   │   │   │   ├── GATEWAY.md (5KB) [168L]
│   │   │   │   │   └── WALLET_KEYGEN.md (5KB) [166L]
│   │   │   │   ├── contracts/ (1 files, 202 lines)
│   │   │   │   │   └── ATC_TOKEN_STANDARD.md (6KB) [202L]
│   │   │   │   ├── issues/ (7 files, 1305 lines)
│   │   │   │   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [141L]
│   │   │   │   │   ├── ISSUE_06_ECDSA.md (4KB) [141L]
│   │   │   │   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [97L]
│   │   │   │   │   ├── ISSUE_12_SOLIDITY.md (4KB) [145L]
│   │   │   │   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [120L]
│   │   │   │   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [308L]
│   │   │   │   │   └── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   │   │   ├── repo/ (1 files, 56 lines)
│   │   │   │   │   └── README.md (2KB) [56L]
│   │   │   │   ├── roadmap/ (1 files, 245 lines)
│   │   │   │   │   └── ROADMAP_EXTENDED.md (10KB) [245L]
│   │   │   │   ├── standards/ (3 files, 699 lines)
│   │   │   │   │   ├── ATC_ECOSYSTEM_STANDARDS.md (13KB) [447L]
│   │   │   │   │   ├── OVERVIEW.md (1KB) [40L]
│   │   │   │   │   └── STANDARDS_REGISTRY.md (10KB) [212L]
│   │   │   │   ├── DECISIONS_REGISTER.md (2KB) [69L]
│   │   │   │   ├── ROADMAP.md (9KB) [208L]
│   │   │   │   ├── ROADMAP_COMPLETENESS_AUDIT.md (7KB) [223L]
│   │   │   │   ├── STATUS.md (3KB) [85L]
│   │   │   │   └── kai-os-wiki.md (395KB) [11376L]
│   │   │   ├── ECOSYSTEM.md (8KB) [179L]
│   │   │   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   │   │   └── README.md (542B) [18L]
│   │   ├── kernel/ (10 files, 494 lines)
│   │   │   ├── docs/ (9 files, 450 lines)
│   │   │   │   ├── ATCFS.md (2KB) [107L]
│   │   │   │   ├── ATCNET.md (2KB) [89L]
│   │   │   │   ├── CHANGELOG.md (231B) [7L]
│   │   │   │   ├── CONSENSUS.md (615B) [24L]
│   │   │   │   ├── IPC.md (1KB) [43L]
│   │   │   │   ├── KERNEL.md (2KB) [87L]
│   │   │   │   ├── PERFORMANCE.md (708B) [25L]
│   │   │   │   ├── PROCESS_MODEL.md (1KB) [48L]
│   │   │   │   └── SECURITY.md (532B) [20L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── overview/ (9 files, 400 lines)
│   │   │   ├── docs/ (8 files, 356 lines)
│   │   │   │   ├── API.md (1KB) [59L]
│   │   │   │   ├── ARCHITECTURE.md (1KB) [36L]
│   │   │   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   │   │   ├── FAQ.md (1KB) [62L]
│   │   │   │   ├── QUICKSTART.md (619B) [30L]
│   │   │   │   ├── ROADMAP.md (556B) [25L]
│   │   │   │   ├── SECURITY.md (916B) [18L]
│   │   │   │   └── WHITEPAPER.md (5KB) [107L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── shivamon/ (7 files, 229 lines)
│   │   │   ├── docs/ (6 files, 185 lines)
│   │   │   │   ├── BATTLE.md (420B) [17L]
│   │   │   │   ├── BREEDING.md (1KB) [37L]
│   │   │   │   ├── ELEMENTS.md (1KB) [31L]
│   │   │   │   ├── MARKETPLACE.md (408B) [21L]
│   │   │   │   ├── NFT_SPEC.md (1KB) [55L]
│   │   │   │   └── ROADMAP.md (638B) [24L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── standards/ (2 files, 72 lines)
│   │   │   ├── docs/ (1 files, 28 lines)
│   │   │   │   └── OVERVIEW.md (1KB) [28L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── ui/ (6 files, 240 lines)
│   │   │   ├── docs/ (5 files, 196 lines)
│   │   │   │   ├── API.md (651B) [30L]
│   │   │   │   ├── COMPONENTS.md (442B) [26L]
│   │   │   │   ├── DEPLOYMENT.md (969B) [49L]
│   │   │   │   ├── DESIGN.md (732B) [24L]
│   │   │   │   └── THEME.md (1KB) [67L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── chapter-63-cleanup-2026-06-13.md (6KB) [205L]
│   │   ├── chapter-70-atclang-migration-complete.md (2KB) [77L]
│   │   ├── chapter-71-sprint-audit.md (2KB) [67L]
│   │   ├── chapter-72-sprint-2-7-testing-cicd.md (2KB) [59L]
│   │   ├── chapter-73-sprint-2-8-testnet.md (1KB) [53L]
│   │   ├── chapter-74-sprint-3-1-ux-privacy.md (1KB) [40L]
│   │   ├── chapter-75-v01-v03-migration-plan.md (2KB) [74L]
│   │   ├── chapter-76-sprint-3-3-3-6-alpha-release.md (1KB) [40L]
│   │   └── chapter-77-sprint-4-0-4-1-mainnet.md (1KB) [43L]
│   ├── AGENT_COMMIT_AUDIT_2026-08-05.md (2KB) [62L]
│   ├── AGENT_COORDINATION.md (27KB) [345L]
│   ├── AGENT_POLICY.md (13KB) [325L]
│   ├── AGENT_PROTOCOL.md (8KB) [256L]
│   ├── ARCHITECTURE_TREES.md (359KB) [7672L]
│   ├── ATCLANG_AGENT_BUILD_GUIDE.md (22KB) [281L]
│   ├── ATC_93_BYTECODE_SPEC.md (6KB) [235L]
│   ├── AUDIT_REPORT.md (3KB) [89L]
│   ├── CLEANUP_LOG.md (8KB) [231L]
│   ├── CLUSTER_ARCHITECTURE.md (6KB) [103L]
│   ├── COMMUNITY_ANNOUNCEMENT.md (1KB) [35L]
│   ├── COMPLETENESS_AUDIT.md (2KB) [57L]
│   ├── DECISIONS_REGISTER.md (7KB) [140L]
│   ├── DEPRECATED.md (2KB) [45L]
│   ├── DEVELOPER_ONBOARDING.md (4KB) [157L]
│   ├── ECOSYSTEM_BRAIN.md (3KB) [104L]
│   ├── FILE_NAMING_CONVENTIONS.md (16KB) [634L]
│   ├── FILE_REGISTER.md (4KB) [94L]
│   ├── FIXES.md (3KB) [96L]
│   ├── GAP_ANALYSIS_v1.0.md (8KB) [199L]
│   ├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
│   ├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
│   ├── K9_K13_GAP.md (964B) [22L]
│   ├── KAI_INTEGRATION.md (6KB) [242L]
│   ├── MIGRATION_MAP.md (1KB) [30L]
│   ├── MILESTONES.md (1KB) [23L]
│   ├── NAMING_CONVENTIONS.md (4KB) [88L]
│   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   ├── REALITY_CHECK_2026-07-06.md (28KB) [428L]
│   ├── RELEASE_NOTES_v1.0.md (2KB) [103L]
│   ├── ROADMAP.md (3KB) [79L]
│   ├── ROADMAP_COMPLETENESS_AUDIT.md (184B) [9L]
│   ├── STATUS.md (4KB) [72L]
│   ├── SYNC_REPORT.md (1022B) [45L]
│   ├── TECHNICAL_DOCUMENTATION.md (4KB) [142L]
│   ├── WIKI_AUDIT.md (6KB) [188L]
│   ├── WIKI_INDEX.md (8KB) [148L]
│   ├── api-reference.md (1KB) [33L]
│   ├── atclang-guide.md (1KB) [48L]
│   ├── genesis_wallet.md (3KB) [103L]
│   ├── kai-os-wiki.md (255KB) [7792L]
│   └── landing-page.html (4KB)
├── drivers/ (7 files, 166 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (220B) [8L]
│   ├── FILE_REGISTER.md (385B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [108L]
│   ├── ROADMAP.md (373B) [16L]
│   └── STATUS.md (488B) [21L]
├── explorer/ (7 files, 167 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (221B) [8L]
│   ├── FILE_REGISTER.md (496B) [18L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [104L]
│   ├── ROADMAP.md (374B) [16L]
│   └── STATUS.md (489B) [21L]
├── franchise/ (15 files, 705 lines)
│   ├── api/ (1 files, 67 lines)
│   │   └── routes.py (2KB) [67L]
│   ├── contracts/ (3 files, 285 lines)
│   │   ├── registry.atc (4KB) [120L]
│   │   ├── revenue.atc (3KB) [93L]
│   │   └── token.atc (3KB) [72L]
│   ├── docs/ (2 files, 80 lines)
│   │   ├── ARCHITECTURE.md (666B) [23L]
│   │   └── SECURITY.md (1KB) [57L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (182B) [6L]
│   ├── FILE_REGISTER.md (658B) [20L]
│   ├── LICENSE (982B)
│   ├── README.md (4KB) [69L]
│   ├── ROADMAP.md (482B) [21L]
│   ├── STATUS.md (358B) [19L]
│   ├── factory.py (4KB) [138L]
│   └── requirements.txt (115B)
├── frontend/ (35 files, 1138 lines)
│   ├── __mocks__/ (1 files, 1 lines)
│   │   └── styleMock.js (21B) [1L]
│   ├── admin/ (4 files, 138 lines)
│   │   ├── CHANGELOG.md (175B) [6L]
│   │   ├── DESIGN.md (1KB) [33L]
│   │   ├── api.js (4KB) [99L]
│   │   └── index.html (106KB)
│   ├── assets/ (2 files, 136 lines)
│   │   ├── css/ (1 files, 0 lines)
│   │   │   └── variables.css (807B)
│   │   └── js/ (1 files, 136 lines)
│   │       └── api.js (4KB) [136L]
│   ├── battle/ (1 files, 0 lines)
│   │   └── index.html (13KB)
│   ├── bootscreen/ (1 files, 48 lines)
│   │   └── README.md (1KB) [48L]
│   ├── mobile/ (3 files, 352 lines)
│   │   ├── wallet/ (1 files, 179 lines)
│   │   │   └── biometric_auth.atc (5KB) [179L]
│   │   ├── README.md (62B) [2L]
│   │   └── wallet_api.atc (5KB) [171L]
│   ├── src/ (1 files, 0 lines)
│   │   └── .gitkeep (0B)
│   ├── ui/ (10 files, 268 lines)
│   │   ├── assets/ (1 files, 99 lines)
│   │   │   └── js/ (1 files, 99 lines)
│   │   │       └── api.js (4KB) [99L]
│   │   ├── .gitignore (171B)
│   │   ├── CHANGELOG.md (175B) [6L]
│   │   ├── DESIGN.md (1KB) [33L]
│   │   ├── FILE_REGISTER.md (575B) [21L]
│   │   ├── LICENSE (982B)
│   │   ├── README.md (4KB) [69L]
│   │   ├── ROADMAP.md (468B) [21L]
│   │   ├── STATUS.md (351B) [19L]
│   │   └── index.html (106KB)
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (426B) [21L]
│   ├── FILE_REGISTER.md (689B) [23L]
│   ├── LICENSE (658B)
│   ├── README.md (1KB) [57L]
│   ├── ROADMAP.md (480B) [21L]
│   ├── STATUS.md (347B) [19L]
│   ├── index.html (120KB)
│   ├── jest.config.js (457B) [14L]
│   ├── jest.setup.js (79B) [2L]
│   ├── package.json (544B) [15L]
│   └── tsconfig.json (579B) [23L]
├── gateway/ (43 files, 2027 lines)
│   ├── atclang/ (11 files, 581 lines)
│   │   ├── middleware/ (4 files, 245 lines)
│   │   │   ├── auth.atc (2KB) [82L]
│   │   │   ├── logger.atc (2KB) [70L]
│   │   │   ├── rate_limit.atc (1KB) [50L]
│   │   │   └── signature_verify.atc (1KB) [43L]
│   │   ├── .env.example (103B)
│   │   ├── CHANGELOG.md (274B) [8L]
│   │   ├── README.md (858B) [39L]
│   │   ├── SECURITY.md (371B) [13L]
│   │   ├── main.atc (5KB) [180L]
│   │   ├── requirements.txt (162B)
│   │   └── router.atc (3KB) [96L]
│   ├── docs/ (1 files, 112 lines)
│   │   └── ARCHITECTURE.md (2KB) [112L]
│   ├── middleware/ (5 files, 113 lines)
│   │   ├── __init__.py (120B) [2L]
│   │   ├── auth.py (669B) [19L]
│   │   ├── logger.py (324B) [9L]
│   │   ├── rate_limit.py (1KB) [26L]
│   │   └── signature_verify.py (1KB) [57L]
│   ├── python/ (11 files, 507 lines)
│   │   ├── middleware/ (5 files, 113 lines)
│   │   │   ├── __init__.py (120B) [2L]
│   │   │   ├── auth.py (669B) [19L]
│   │   │   ├── logger.py (324B) [9L]
│   │   │   ├── rate_limit.py (1KB) [26L]
│   │   │   └── signature_verify.py (1KB) [57L]
│   │   ├── __init__.py (125B) [2L]
│   │   ├── main.atc (5KB) [127L]
│   │   ├── main.py (1KB) [47L]
│   │   ├── requirements.txt (69B)
│   │   ├── router.py (2KB) [50L]
│   │   └── service_discovery.atc (6KB) [168L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (328B) [14L]
│   ├── FILE_REGISTER.md (2KB) [74L]
│   ├── LICENSE (982B)
│   ├── README.md (1KB) [41L]
│   ├── ROADMAP.md (478B) [21L]
│   ├── SECURITY.md (371B) [13L]
│   ├── STATUS.md (356B) [19L]
│   ├── __init__.py (125B) [2L]
│   ├── gateway.atc (4KB) [138L]
│   ├── main.atc (5KB) [127L]
│   ├── main.py (1KB) [47L]
│   ├── requirements.txt (69B)
│   ├── router.py (2KB) [50L]
│   └── service_discovery.atc (6KB) [168L]
├── genesis-engine/ (20 files, 1119 lines)
│   ├── engine/ (6 files, 298 lines)
│   │   ├── core/ (1 files, 98 lines)
│   │   │   └── ecs.py (2KB) [98L]
│   │   ├── render/ (1 files, 45 lines)
│   │   │   └── renderer2d.py (1KB) [45L]
│   │   ├── tests/ (1 files, 63 lines)
│   │   │   └── test_ecs.py (1KB) [63L] 🧪
│   │   ├── MILESTONE_1.md (1KB) [44L]
│   │   ├── main.py (1KB) [48L]
│   │   └── requirements.txt (14B)
│   ├── .gitignore (171B)
│   ├── ARCHITECTURE.md (4KB) [103L]
│   ├── CHANGELOG.md (432B) [21L]
│   ├── FILE_REGISTER.md (849B) [24L]
│   ├── FRANCHISE_FACTORY.md (3KB) [66L]
│   ├── FRANCHISE_FACTORY_V2.md (3KB) [108L]
│   ├── GENESIS_NEXUS_V5.md (3KB) [65L]
│   ├── GENESIS_OS_V4.md (3KB) [70L]
│   ├── LICENSE (658B)
│   ├── METAFACTORY_V3.md (4KB) [83L]
│   ├── README.md (4KB) [84L]
│   ├── ROADMAP.md (492B) [21L]
│   ├── STATUS.md (353B) [19L]
│   └── VISION_EVOLUTION_LOG.md (8KB) [157L]
├── ide/ (7 files, 171 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (216B) [8L]
│   ├── FILE_REGISTER.md (481B) [18L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [108L]
│   ├── ROADMAP.md (369B) [16L]
│   └── STATUS.md (484B) [21L]
├── integrations/ (5 files, 194 lines)
│   ├── README.md (1KB) [39L]
│   ├── calendar_tasks.md (4KB) [57L]
│   ├── huggingface_registry.md (1KB) [27L]
│   ├── notion_export.md (1KB) [25L]
│   └── storage_inventory.md (2KB) [46L]
├── kernel/ (40 files, 5349 lines)
│   ├── consensus/ (3 files, 814 lines)
│   │   ├── consensus.atc (5KB) [144L]
│   │   ├── poh_integration.py (1KB) [29L]
│   │   └── shiva_consensus.py (24KB) [641L]
│   ├── docs/ (1 files, 283 lines)
│   │   └── ATS_STANDARDS.md (7KB) [283L]
│   ├── fs/ (2 files, 473 lines)
│   │   ├── atcfs.atc (4KB) [142L]
│   │   └── atcfs.py (12KB) [331L]
│   ├── ipc/ (1 files, 94 lines)
│   │   └── ipc_bus.py (3KB) [94L]
│   ├── kernel/ (2 files, 530 lines)
│   │   ├── kernel.atc (4KB) [148L]
│   │   └── kernel.py (14KB) [382L]
│   ├── net/ (2 files, 152 lines)
│   │   ├── atcnet.atc (4KB) [135L]
│   │   └── atcnet.py (549B) [17L]
│   ├── python/ (22 files, 2708 lines)
│   │   ├── consensus/ (3 files, 814 lines)
│   │   │   ├── consensus.atc (5KB) [144L]
│   │   │   ├── poh_integration.py (1KB) [29L]
│   │   │   └── shiva_consensus.py (24KB) [641L]
│   │   ├── docs/ (1 files, 283 lines)
│   │   │   └── ATS_STANDARDS.md (7KB) [283L]
│   │   ├── fs/ (2 files, 473 lines)
│   │   │   ├── atcfs.atc (4KB) [142L]
│   │   │   └── atcfs.py (12KB) [331L]
│   │   ├── ipc/ (1 files, 94 lines)
│   │   │   └── ipc_bus.py (3KB) [94L]
│   │   ├── kernel/ (2 files, 530 lines)
│   │   │   ├── kernel.atc (4KB) [148L]
│   │   │   └── kernel.py (14KB) [382L]
│   │   ├── net/ (2 files, 152 lines)
│   │   │   ├── atcnet.atc (4KB) [135L]
│   │   │   └── atcnet.py (549B) [17L]
│   │   ├── .gitignore (171B)
│   │   ├── ARCHITECTURE.md (2KB) [90L]
│   │   ├── CHANGELOG.md (276B) [16L]
│   │   ├── FILE_REGISTER.md (909B) [27L]
│   │   ├── LICENSE (982B)
│   │   ├── README.md (4KB) [69L]
│   │   ├── ROADMAP.md (476B) [21L]
│   │   ├── SECURITY.md (451B) [14L]
│   │   ├── STATUS.md (355B) [19L]
│   │   ├── kernel.py (3KB) [106L]
│   │   └── requirements.txt (131B)
│   ├── ARCHITECTURE.md (2KB) [90L]
│   ├── CHANGELOG.md (276B) [16L]
│   ├── LICENSE (982B)
│   ├── README.md (4KB) [69L]
│   ├── SECURITY.md (451B) [14L]
│   ├── kernel.py (3KB) [106L]
│   └── requirements.txt (131B)
├── linux/ (9 files, 133 lines)
│   ├── src/ (1 files, 15 lines)
│   │   └── main.rs (625B) [15L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (431B) [21L]
│   ├── Cargo.toml (273B) [13L]
│   ├── FILE_REGISTER.md (398B) [13L]
│   ├── LICENSE (658B)
│   ├── README.md (1KB) [44L]
│   ├── ROADMAP.md (490B) [21L]
│   └── STATUS.md (350B) [19L]
├── mobile/ (11 files, 439 lines)
│   ├── wallet/ (2 files, 181 lines)
│   │   ├── __init__.py (162B) [2L]
│   │   └── biometric_auth.atc (5KB) [179L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (424B) [21L]
│   ├── FILE_REGISTER.md (638B) [22L]
│   ├── LICENSE (658B)
│   ├── README.md (62B) [2L]
│   ├── ROADMAP.md (476B) [21L]
│   ├── STATUS.md (345B) [19L]
│   ├── __init__.py (123B) [2L]
│   └── wallet_api.atc (5KB) [171L]
├── modules/ (192 files, 28493 lines)
│   ├── assets/ (16 files, 2443 lines)
│   │   ├── aaa_asset_core.atc (3KB) [97L]
│   │   ├── ai_assets.atc (4KB) [143L]
│   │   ├── animation.atc (4KB) [170L]
│   │   ├── asset_bundle.atc (4KB) [121L]
│   │   ├── cloud_assets.atc (5KB) [161L]
│   │   ├── encryption.atc (5KB) [183L]
│   │   ├── hot_reload.atc (4KB) [156L]
│   │   ├── memory_cleanup.atc (4KB) [151L]
│   │   ├── mod_system.atc (5KB) [172L]
│   │   ├── model3d.atc (5KB) [198L]
│   │   ├── priority_loading.atc (2KB) [99L]
│   │   ├── render_pipeline.atc (6KB) [192L]
│   │   ├── shader_system.atc (5KB) [167L]
│   │   ├── streaming.atc (3KB) [116L]
│   │   ├── telemetry.atc (4KB) [166L]
│   │   └── versioning.atc (4KB) [151L]
│   ├── atcnet/ (11 files, 1066 lines)
│   │   ├── CHANGELOG.md (294B) [8L]
│   │   ├── PROTOCOL.md (2KB) [84L]
│   │   ├── README.md (780B) [37L]
│   │   ├── SECURITY.md (321B) [11L]
│   │   ├── bootstrap_client.atc (4KB) [134L]
│   │   ├── discovery.atc (4KB) [138L]
│   │   ├── gossip.atc (5KB) [171L]
│   │   ├── nat_traversal.atc (3KB) [109L]
│   │   ├── p2p_node.atc (4KB) [159L]
│   │   ├── p2p_propagation.atc (6KB) [215L]
│   │   └── requirements.txt (112B)
│   ├── civilization/ (11 files, 2214 lines)
│   │   ├── asset_genome_ad66.atc (5KB) [171L]
│   │   ├── civilization_engine_ad60.atc (5KB) [236L]
│   │   ├── ecosystem_ai_mesh_ad62.atc (7KB) [245L]
│   │   ├── evolution_engine_ad69.atc (7KB) [251L]
│   │   ├── experience_orchestrator_ad68.atc (6KB) [200L]
│   │   ├── gcp_core_ad70.atc (7KB) [169L]
│   │   ├── global_simulation_core_ad64.atc (6KB) [198L]
│   │   ├── identity_layer_ad65.atc (4KB) [190L]
│   │   ├── persistent_world_engine_ad61.atc (5KB) [199L]
│   │   ├── proc_universe_generator_ad63.atc (8KB) [204L]
│   │   └── production_pipeline_ad67.atc (6KB) [151L]
│   ├── contracts/ (18 files, 1758 lines)
│   │   ├── atc8300/ (1 files, 178 lines)
│   │   │   └── atc8300_token.atc (5KB) [178L]
│   │   ├── atcoin/ (1 files, 176 lines)
│   │   │   └── atcoin.atc (5KB) [176L]
│   │   ├── base/ (1 files, 69 lines)
│   │   │   └── base_contract.atc (2KB) [69L]
│   │   ├── bridge/ (1 files, 172 lines)
│   │   │   └── bridge_contract.atc (5KB) [172L]
│   │   ├── governance/ (1 files, 237 lines)
│   │   │   └── governance_contract.atc (7KB) [237L]
│   │   ├── marketplace/ (1 files, 236 lines)
│   │   │   └── marketplace_contract.atc (7KB) [236L]
│   │   ├── shivamon/ (1 files, 290 lines)
│   │   │   └── shivamon_contract.atc (9KB) [290L]
│   │   ├── standards/ (4 files, 172 lines)
│   │   │   ├── atc-13_fractional_asset_ownership.atc (1KB) [43L]
│   │   │   ├── atc-15_proof_of_ai_mining.atc (1KB) [43L]
│   │   │   ├── atc-16_referral_multitier_rewards.atc (1KB) [43L]
│   │   │   └── atc-20_wrapped_synthetic_assets.atc (1KB) [43L]
│   │   ├── wallet/ (2 files, 135 lines)
│   │   │   ├── ecdsa.atc (2KB) [60L]
│   │   │   └── keygen.atc (2KB) [75L]
│   │   ├── CHANGELOG.md (293B) [8L]
│   │   ├── DEPLOYMENT.md (894B) [29L]
│   │   ├── README.md (1KB) [43L]
│   │   ├── SECURITY.md (496B) [13L]
│   │   └── requirements.txt (100B)
│   ├── franchise/ (34 files, 4270 lines)
│   │   ├── contracts/ (3 files, 285 lines)
│   │   │   ├── registry.atc (4KB) [120L]
│   │   │   ├── revenue.atc (3KB) [93L]
│   │   │   └── token.atc (3KB) [72L]
│   │   ├── docs/ (2 files, 80 lines)
│   │   │   ├── ARCHITECTURE.md (666B) [23L]
│   │   │   └── SECURITY.md (1KB) [57L]
│   │   ├── CHANGELOG.md (256B) [7L]
│   │   ├── README.md (775B) [35L]
│   │   ├── ai_content_factory_ad28.atc (6KB) [194L]
│   │   ├── ai_director_factory_ad41.atc (4KB) [28L]
│   │   ├── analytics_factory_ad31.atc (7KB) [232L]
│   │   ├── asset_intelligence_factory_ad34.atc (7KB) [210L]
│   │   ├── blueprint_factory_ad32.atc (5KB) [165L]
│   │   ├── canon_engine_ad33.atc (5KB) [171L]
│   │   ├── character_factory_ad23.atc (8KB) [251L]
│   │   ├── commerce_factory_ad40.atc (4KB) [26L]
│   │   ├── community_factory_ad30.atc (7KB) [222L]
│   │   ├── creator_factory_ad38.atc (4KB) [24L]
│   │   ├── economy_factory_ad26.atc (6KB) [200L]
│   │   ├── factory.atc (5KB) [165L]
│   │   ├── gameplay_factory_ad35.atc (4KB) [126L]
│   │   ├── gff_core_ad20.atc (8KB) [224L]
│   │   ├── ip_factory_ad21.atc (4KB) [147L]
│   │   ├── lifecycle_manager_ad43.atc (5KB) [25L]
│   │   ├── liveops_factory_ad27.atc (6KB) [212L]
│   │   ├── lore_factory_ad24.atc (7KB) [209L]
│   │   ├── merchandise_factory_ad29.atc (5KB) [173L]
│   │   ├── multiplayer_factory_ad37.atc (3KB) [27L]
│   │   ├── narrative_factory_ad36.atc (8KB) [245L]
│   │   ├── publishing_factory_ad39.atc (4KB) [25L]
│   │   ├── quest_factory_ad25.atc (6KB) [207L]
│   │   ├── requirements.txt (115B)
│   │   ├── routes.atc (2KB) [90L]
│   │   ├── security_factory_ad42.atc (4KB) [30L]
│   │   └── world_factory_ad22.atc (6KB) [235L]
│   ├── gateway/ (11 files, 581 lines)
│   │   ├── middleware/ (4 files, 245 lines)
│   │   │   ├── auth.atc (2KB) [82L]
│   │   │   ├── logger.atc (2KB) [70L]
│   │   │   ├── rate_limit.atc (1KB) [50L]
│   │   │   └── signature_verify.atc (1KB) [43L]
│   │   ├── .env.example (103B)
│   │   ├── CHANGELOG.md (274B) [8L]
│   │   ├── README.md (858B) [39L]
│   │   ├── SECURITY.md (371B) [13L]
│   │   ├── main.atc (5KB) [180L]
│   │   ├── requirements.txt (162B)
│   │   └── router.atc (3KB) [96L]
│   ├── kernel/ (68 files, 12738 lines)
│   │   ├── ai_kernel/ (12 files, 1524 lines)
│   │   │   ├── distributed_intelligence/ (5 files, 170 lines)
│   │   │   │   ├── atc-46_quantumresistant_crypto_layer.atc (1KB) [34L]
│   │   │   │   ├── atc-47_ai_intent_settlement.atc (1KB) [34L]
│   │   │   │   ├── atc-48_neural_network_mesh.atc (1KB) [34L]
│   │   │   │   ├── atc-49_neural_synapse_knowledge_transfer.atc (1KB) [34L]
│   │   │   │   └── atc-50_ai_consciousness_selfreflection.atc (1KB) [34L]
│   │   │   ├── orchestration/ (5 files, 220 lines)
│   │   │   │   ├── atc-25_tensor_compute_orchestration.atc (1KB) [44L]
│   │   │   │   ├── atc-26_xai_transparency.atc (1KB) [44L]
│   │   │   │   ├── atc-29_ai_marketplace.atc (1KB) [44L]
│   │   │   │   ├── atc-30_reputation_trust_scoring.atc (1KB) [44L]
│   │   │   │   └── atc-31_tensor_load_balancing.atc (1KB) [44L]
│   │   │   ├── ai_kernel.atc (8KB) [228L]
│   │   │   └── atc-97_agent_interaction_protocol.atc (37KB) [906L]
│   │   ├── consensus/ (2 files, 607 lines)
│   │   │   ├── poh_integration.atc (2KB) [78L]
│   │   │   └── shiva_consensus.atc (16KB) [529L]
│   │   ├── container/ (1 files, 537 lines)
│   │   │   └── container_runtime.atc (18KB) [537L]
│   │   ├── container_net/ (1 files, 70 lines)
│   │   │   └── container_net.atc (2KB) [70L]
│   │   ├── contract/ (1 files, 23 lines)
│   │   │   └── contract.atc (772B) [23L]
│   │   ├── cow/ (1 files, 87 lines)
│   │   │   └── cow_fork.atc (2KB) [87L]
│   │   ├── did/ (1 files, 38 lines)
│   │   │   └── did.atc (954B) [38L]
│   │   ├── docs/ (1 files, 283 lines)
│   │   │   └── ATS_STANDARDS.md (7KB) [283L]
│   │   ├── drivers/ (5 files, 2423 lines)
│   │   │   ├── display_driver.atc (12KB) [324L]
│   │   │   ├── driver_framework.atc (32KB) [812L]
│   │   │   ├── input_driver.atc (18KB) [493L]
│   │   │   ├── network_driver.atc (14KB) [416L]
│   │   │   └── storage_driver.atc (14KB) [378L]
│   │   ├── elf_loader/ (1 files, 74 lines)
│   │   │   └── elf_loader.atc (1KB) [74L]
│   │   ├── fs/ (1 files, 142 lines)
│   │   │   └── atcfs.atc (4KB) [142L]
│   │   ├── fs_journal/ (1 files, 88 lines)
│   │   │   └── fs_journal.atc (2KB) [88L]
│   │   ├── ipc/ (1 files, 102 lines)
│   │   │   └── ipc_bus.atc (3KB) [102L]
│   │   ├── lkm/ (1 files, 114 lines)
│   │   │   └── lkm.atc (3KB) [114L]
│   │   ├── mempool/ (1 files, 66 lines)
│   │   │   └── mempool.atc (1KB) [66L]
│   │   ├── module_security/ (1 files, 226 lines)
│   │   │   └── module_security.atc (9KB) [226L]
│   │   ├── net/ (1 files, 135 lines)
│   │   │   └── atcnet.atc (4KB) [135L]
│   │   ├── os_layer/ (2 files, 92 lines)
│   │   │   ├── atc-21_holographic_execution_engine.atc (1KB) [46L]
│   │   │   └── atc-22_hal_driver_sandbox.atc (1KB) [46L]
│   │   ├── page_fault/ (1 files, 78 lines)
│   │   │   └── page_fault.atc (1KB) [78L]
│   │   ├── pkg/ (1 files, 208 lines)
│   │   │   └── manager.atc (6KB) [208L]
│   │   ├── power/ (1 files, 81 lines)
│   │   │   └── power.atc (1KB) [81L]
│   │   ├── process/ (1 files, 161 lines)
│   │   │   └── process_mgr.atc (4KB) [161L]
│   │   ├── shell/ (1 files, 296 lines)
│   │   │   └── shell.atc (8KB) [296L]
│   │   ├── signals/ (1 files, 257 lines)
│   │   │   └── signal_handler.atc (9KB) [257L]
│   │   ├── smp/ (1 files, 105 lines)
│   │   │   └── smp_manager.atc (4KB) [105L]
│   │   ├── sockets/ (1 files, 71 lines)
│   │   │   └── sockets.atc (1KB) [71L]
│   │   ├── threads/ (1 files, 103 lines)
│   │   │   └── threads.atc (2KB) [103L]
│   │   ├── tracing/ (1 files, 129 lines)
│   │   │   └── tracing.atc (3KB) [129L]
│   │   ├── userspace/ (1 files, 57 lines)
│   │   │   └── userspace.atc (1KB) [57L]
│   │   ├── vm/ (1 files, 64 lines)
│   │   │   └── vm.atc (1KB) [64L]
│   │   ├── vmm/ (1 files, 67 lines)
│   │   │   └── vmm.atc (3KB) [67L]
│   │   ├── ARCHITECTURE.md (2KB) [90L]
│   │   ├── CHANGELOG.md (310B) [8L]
│   │   ├── README.md (1KB) [46L]
│   │   ├── SECURITY.md (451B) [14L]
│   │   ├── ai_bus_ad13.atc (9KB) [310L]
│   │   ├── asset_bus_ad08.atc (5KB) [188L]
│   │   ├── audio_bus_ad11.atc (5KB) [199L]
│   │   ├── command_bus_ad02.atc (4KB) [168L]
│   │   ├── gcl_core_ad00.atc (7KB) [269L]
│   │   ├── input_bus_ad12.atc (5KB) [184L]
│   │   ├── ipc_bus_atc.ad.atc (8KB) [266L]
│   │   ├── kernel_api.atc (38KB) [1054L]
│   │   ├── message_bus_ad03.atc (6KB) [240L]
│   │   ├── network_bus_ad05.atc (8KB) [307L]
│   │   ├── physics_bus_ad10.atc (7KB) [255L]
│   │   ├── plugin_bus_ad06.atc (8KB) [286L]
│   │   ├── query_bus_ad07.atc (3KB) [128L]
│   │   ├── render_bus_ad09.atc (5KB) [164L]
│   │   ├── requirements.txt (131B)
│   │   └── telemetry_bus_ad14.atc (7KB) [254L]
│   ├── meta/ (8 files, 2320 lines)
│   │   ├── ai_studio_ad49.atc (11KB) [310L]
│   │   ├── cross_franchise_ad46.atc (8KB) [223L]
│   │   ├── data_lake_ad51.atc (9KB) [237L]
│   │   ├── digital_twin_ad50.atc (11KB) [303L]
│   │   ├── ip_evolution_ad45.atc (9KB) [241L]
│   │   ├── knowledge_graph_ad47.atc (11KB) [289L]
│   │   ├── simulation_factory_ad48.atc (13KB) [374L]
│   │   └── universe_factory_ad44.atc (13KB) [343L]
│   ├── shivamon/ (5 files, 239 lines)
│   │   ├── engine/ (1 files, 153 lines)
│   │   │   └── battle_engine.atc (5KB) [153L]
│   │   ├── CHANGELOG.md (272B) [8L]
│   │   ├── GAME_SPEC.md (1KB) [43L]
│   │   ├── README.md (819B) [35L]
│   │   └── requirements.txt (122B)
│   ├── standards/ (5 files, 694 lines)
│   │   ├── ATC/ (1 files, 233 lines)
│   │   │   └── ATC_STANDARDS.md (5KB) [233L]
│   │   ├── ATC_STANDARDS.md (4KB) [201L]
│   │   ├── ATS_STANDARDS.md (4KB) [199L]
│   │   ├── OVERVIEW.md (1KB) [29L]
│   │   └── README.md (706B) [32L]
│   └── ui/ (5 files, 170 lines)
│       ├── assets/ (1 files, 99 lines)
│       │   └── js/ (1 files, 99 lines)
│       │       └── api.js (4KB) [99L]
│       ├── CHANGELOG.md (285B) [8L]
│       ├── DESIGN.md (1KB) [33L]
│       ├── README.md (586B) [30L]
│       └── index.html (106KB)
├── monitoring/ (5 files, 661 lines)
│   ├── alerts/ (1 files, 34 lines)
│   │   └── blockchain_alerts.yml (1KB) [34L]
│   ├── health_checks_atc08.atc (5KB) [197L]
│   ├── monitor.atc (6KB) [213L]
│   ├── prometheus.yml (610B) [15L]
│   └── prometheus_metrics.atc (6KB) [202L]
├── network/ (17 files, 1824 lines)
│   ├── tests/ (1 files, 41 lines)
│   │   └── test_atcnet.py (1KB) [41L] 🧪
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (210B) [17L]
│   ├── FILE_REGISTER.md (1KB) [48L]
│   ├── LICENSE (982B)
│   ├── PROTOCOL.md (2KB) [84L]
│   ├── README.md (4KB) [69L]
│   ├── ROADMAP.md (468B) [21L]
│   ├── SECURITY.md (321B) [11L]
│   ├── STATUS.md (351B) [19L]
│   ├── atcnet.atc (4KB) [135L]
│   ├── atcnet.py (17KB) [487L]
│   ├── bootstrap_client.py (3KB) [97L]
│   ├── discovery.py (11KB) [314L]
│   ├── node.py (3KB) [100L]
│   ├── p2p_propagation.py (12KB) [381L]
│   └── requirements.txt (112B)
├── nginx/ (1 files, 0 lines)
│   └── nginx.conf (1KB)
├── pkg/ (7 files, 797 lines)
│   ├── docs/ (4 files, 405 lines)
│   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   └── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   ├── kernel/ (1 files, 208 lines)
│   │   └── manager.atc (6KB) [208L]
│   ├── tools/ (1 files, 145 lines)
│   │   └── manager.atc (4KB) [145L]
│   └── README.md (1KB) [39L]
├── scripts/ (11 files, 884 lines)
│   ├── build.sh (3KB) [90L]
│   ├── ci-fix.sh (1015B) [37L]
│   ├── fix-workflows.sh (1KB) [68L]
│   ├── generate_validators.atc (4KB) [135L]
│   ├── health.sh (1KB) [41L]
│   ├── start.sh (3KB) [90L]
│   ├── start_testnet.sh (1KB) [49L]
│   ├── stop.sh (1KB) [50L]
│   ├── sync-docs.sh (4KB) [155L]
│   ├── test-report.sh (2KB) [63L]
│   └── test.sh (4KB) [106L]
├── sdk/ (7 files, 173 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (216B) [8L]
│   ├── FILE_REGISTER.md (381B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (5KB) [115L]
│   ├── ROADMAP.md (369B) [16L]
│   └── STATUS.md (484B) [21L]
├── shivacore/ (74 files, 56248 lines)
│   ├── boot/ (2 files, 30 lines)
│   │   ├── src/ (1 files, 30 lines)
│   │   │   └── main.rs (1KB) [30L]
│   │   └── Cargo.toml (206B) [8L]
│   ├── kernel/ (64 files, 53854 lines)
│   │   ├── .cargo/ (1 files, 0 lines)
│   │   │   └── config.toml (149B) [6L]
│   │   ├── src/ (60 files, 53854 lines)
│   │   │   ├── ai.rs (17KB) [75L]
│   │   │   ├── allocator.rs (1KB) [46L]
│   │   │   ├── atcfs.rs (18KB) [627L]
│   │   │   ├── atcnet.rs (38KB) [1139L]
│   │   │   ├── ats1000.rs (3KB) [85L]
│   │   │   ├── block.rs (18KB) [548L]
│   │   │   ├── blockchain.rs (10KB) [57L]
│   │   │   ├── capability.rs (9KB) [248L]
│   │   │   ├── consensus.rs (34KB) [961L]
│   │   │   ├── container.rs (99KB) [2757L]
│   │   │   ├── container_net.rs (68KB) [632L]
│   │   │   ├── contract.rs (7KB) [38L]
│   │   │   ├── cow.rs (67KB) [1484L]
│   │   │   ├── cross_subsystem.rs (17KB) [483L]
│   │   │   ├── devfs.rs (31KB) [921L]
│   │   │   ├── did.rs (11KB) [350L]
│   │   │   ├── elf_loader.rs (41KB) [1104L]
│   │   │   ├── framebuffer.rs (3KB) [122L]
│   │   │   ├── fs_journal.rs (35KB) [1161L]
│   │   │   ├── gdt.rs (2KB) [59L]
│   │   │   ├── genesis.rs (37KB) [1111L]
│   │   │   ├── genesis_bridge.rs (33KB) [1097L]
│   │   │   ├── gossip_bridge.rs (48KB) [1410L]
│   │   │   ├── hw_drivers.rs (40KB) [1267L]
│   │   │   ├── interrupts.rs (2KB) [100L]
│   │   │   ├── ipc.rs (21KB) [600L]
│   │   │   ├── kernel_init.rs (14KB) [431L]
│   │   │   ├── knowledge_graph.rs (25KB) [755L]
│   │   │   ├── lib.rs (1KB) [73L]
│   │   │   ├── lkm.rs (106KB) [2998L]
│   │   │   ├── main.rs (3KB) [100L]
│   │   │   ├── memory.rs (2KB) [75L]
│   │   │   ├── memory_manager.rs (27KB) [829L]
│   │   │   ├── mempool.rs (17KB) [75L]
│   │   │   ├── module_security.rs (61KB) [1682L]
│   │   │   ├── net.rs (26KB) [802L]
│   │   │   ├── p2p.rs (32KB) [861L]
│   │   │   ├── page_fault.rs (47KB) [1371L]
│   │   │   ├── power.rs (35KB) [1153L]
│   │   │   ├── process.rs (11KB) [360L]
│   │   │   ├── remote_caps.rs (22KB) [629L]
│   │   │   ├── scheduler.rs (14KB) [389L]
│   │   │   ├── security.rs (33KB) [879L]
│   │   │   ├── security_audit.rs (46KB) [1264L]
│   │   │   ├── serial.rs (1KB) [42L]
│   │   │   ├── signals.rs (82KB) [2249L]
│   │   │   ├── smp.rs (84KB) [2506L]
│   │   │   ├── sockets.rs (57KB) [1526L]
│   │   │   ├── syscall.rs (42KB) [1081L]
│   │   │   ├── system.rs (38KB) [1254L]
│   │   │   ├── tcpip.rs (35KB) [860L]
│   │   │   ├── threads.rs (45KB) [1467L]
│   │   │   ├── timer.rs (17KB) [528L]
│   │   │   ├── tracing.rs (74KB) [2254L]
│   │   │   ├── user_io.rs (44KB) [1323L]
│   │   │   ├── user_sched.rs (40KB) [1201L]
│   │   │   ├── userspace.rs (31KB) [840L]
│   │   │   ├── vfs.rs (38KB) [1099L]
│   │   │   ├── vm.rs (15KB) [54L]
│   │   │   └── vmm.rs (82KB) [2362L]
│   │   ├── .gitignore (8B)
│   │   ├── Cargo.lock (12KB)
│   │   └── Cargo.toml (623B) [28L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (427B) [21L]
│   ├── Cargo.toml (204B) [12L]
│   ├── FILE_REGISTER.md (298KB) [2161L]
│   ├── LICENSE (658B)
│   ├── README.md (6KB) [142L]
│   ├── ROADMAP.md (482B) [21L]
│   └── STATUS.md (352B) [19L]
├── shivamon/ (15 files, 1223 lines)
│   ├── api/ (2 files, 152 lines)
│   │   ├── game_routes.py (1KB) [59L]
│   │   └── marketplace_routes.py (2KB) [93L]
│   ├── contracts/ (3 files, 733 lines)
│   │   ├── marketplace_contract.py (11KB) [301L]
│   │   ├── shivamon.atc (5KB) [162L]
│   │   └── shivamon_contract.py (10KB) [270L]
│   ├── engine/ (1 files, 147 lines)
│   │   └── battle_engine.py (5KB) [147L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (243B) [19L]
│   ├── FILE_REGISTER.md (700B) [20L]
│   ├── GAME_SPEC.md (1KB) [43L]
│   ├── LICENSE (982B)
│   ├── README.md (4KB) [69L]
│   ├── ROADMAP.md (480B) [21L]
│   ├── STATUS.md (357B) [19L]
│   └── requirements.txt (122B)
├── src/ (51 files, 6979 lines)
│   ├── atclang/ (4 files, 368 lines)
│   │   ├── ATCLANG_SPEC.md (9KB) [295L]
│   │   ├── CHANGELOG.md (316B) [8L]
│   │   ├── CONTRIBUTING.md (687B) [19L]
│   │   └── README.md (1KB) [46L]
│   ├── blockchain/ (6 files, 787 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   ├── contract_registry.atc (666B) [6L]
│   │   ├── smart_contract_registry.atc (672B) [6L]
│   │   ├── smart_contract_registry.py (1KB) [53L]
│   │   ├── smart_contracts.atc (664B) [6L]
│   │   └── smart_contracts.py (23KB) [716L]
│   ├── contracts/ (12 files, 1783 lines)
│   │   ├── __init__.py (93B) [4L]
│   │   ├── atc8300_token.py (5KB) [126L]
│   │   ├── atcoin.py (5KB) [139L]
│   │   ├── base_contract.py (3KB) [87L]
│   │   ├── bridge_contract.py (4KB) [133L]
│   │   ├── ecdsa.py (2KB) [72L]
│   │   ├── governance_contract.py (11KB) [299L]
│   │   ├── keygen.py (5KB) [140L]
│   │   ├── marketplace_contract.py (11KB) [301L]
│   │   ├── shivamon_contract.py (10KB) [270L]
│   │   ├── wallet_ecdsa.py (2KB) [72L]
│   │   └── wallet_keygen.py (5KB) [140L]
│   ├── core/ (12 files, 2043 lines)
│   │   ├── crypto/ (1 files, 19 lines)
│   │   │   └── __init__.py (535B) [19L]
│   │   ├── kernel/ (6 files, 1863 lines)
│   │   │   ├── api.py (35KB) [882L]
│   │   │   ├── capabilities.py (5KB) [159L]
│   │   │   ├── did.py (2KB) [74L]
│   │   │   ├── kernel.py (16KB) [423L]
│   │   │   ├── remote_capability.py (8KB) [207L]
│   │   │   └── syscalls.atc (3KB) [118L]
│   │   ├── __init__.py (0B) [0L]
│   │   ├── atcfs.py (4KB) [122L]
│   │   ├── event_bus.py (517B) [16L]
│   │   ├── kai_cli.atc (650B) [6L]
│   │   └── module_loader.py (540B) [17L]
│   ├── franchise/ (3 files, 209 lines)
│   │   ├── __init__.py (92B) [4L]
│   │   ├── factory.py (4KB) [138L]
│   │   └── routes.py (2KB) [67L]
│   ├── game/ (4 files, 303 lines)
│   │   ├── __init__.py (86B) [4L]
│   │   ├── battle_engine.py (5KB) [147L]
│   │   ├── game_routes.py (1KB) [59L]
│   │   └── marketplace_routes.py (2KB) [93L]
│   ├── gateway/ (3 files, 99 lines)
│   │   ├── __init__.py (125B) [2L]
│   │   ├── main.py (1KB) [47L]
│   │   └── router.py (2KB) [50L]
│   ├── modules/ (1 files, 4 lines)
│   │   └── __init__.py (87B) [4L]
│   └── network/ (6 files, 1383 lines)
│       ├── __init__.py (90B) [4L]
│       ├── atcnet.py (17KB) [487L]
│       ├── bootstrap_client.py (3KB) [97L]
│       ├── discovery.py (11KB) [314L]
│       ├── node.py (3KB) [100L]
│       └── p2p_propagation.py (12KB) [381L]
├── standards/ (13 files, 1148 lines)
│   ├── ATC/ (2 files, 288 lines)
│   │   ├── ATC-0009-BRIDGE.md (1KB) [55L]
│   │   └── ATC_STANDARDS.md (5KB) [233L]
│   ├── ATS/ (1 files, 283 lines)
│   │   └── ATS_STANDARDS.md (7KB) [283L]
│   ├── .gitignore (171B)
│   ├── ATC_STANDARDS.md (4KB) [201L]
│   ├── ATS_STANDARDS.md (4KB) [199L]
│   ├── CHANGELOG.md (218B) [21L]
│   ├── FILE_REGISTER.md (589B) [18L]
│   ├── LICENSE (982B)
│   ├── OVERVIEW.md (1KB) [29L]
│   ├── README.md (4KB) [69L]
│   ├── ROADMAP.md (482B) [21L]
│   └── STATUS.md (358B) [19L]
├── stdlib/ (7 files, 157 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (219B) [8L]
│   ├── FILE_REGISTER.md (396B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [99L]
│   ├── ROADMAP.md (372B) [16L]
│   └── STATUS.md (487B) [21L]
├── tests/ (38 files, 4122 lines)
│   ├── e2e/ (2 files, 95 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   └── test_frontend_backend_chain.py (3KB) [95L] 🧪
│   ├── integration/ (3 files, 168 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   ├── test_docker_compose.py (3KB) [71L] 🧪
│   │   └── test_gateway_core_chain.py (3KB) [97L] 🧪
│   ├── unit/ (32 files, 3859 lines)
│   │   ├── atclang/ (6 files, 1384 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── test_atclang.py (14KB) [462L] 🧪
│   │   │   ├── test_atclang_v03.py (2KB) [68L] 🧪
│   │   │   ├── test_stdlib.py (10KB) [298L] 🧪
│   │   │   ├── test_stdlib_dispatch.py (11KB) [312L] 🧪
│   │   │   └── test_type_checker.py (7KB) [244L] 🧪
│   │   ├── blockchain/ (8 files, 697 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── test_ecdsa.py (2KB) [64L] 🧪
│   │   │   ├── test_fork_resolution.py (3KB) [101L] 🧪
│   │   │   ├── test_multinode_consensus.py (5KB) [155L] 🧪
│   │   │   ├── test_multinode_fivenode.py (3KB) [84L] 🧪
│   │   │   ├── test_node_failure_recovery.py (4KB) [143L] 🧪
│   │   │   ├── test_persistence.py (3KB) [97L] 🧪
│   │   │   └── test_poh.py (1KB) [53L] 🧪
│   │   ├── contracts/ (3 files, 114 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── test_atcfs_multisig.py (1KB) [37L] 🧪
│   │   │   └── test_smart_contracts.py (2KB) [77L] 🧪
│   │   ├── core/ (8 files, 1318 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── test_bootstrap.py (1KB) [37L] 🧪
│   │   │   ├── test_did.py (1KB) [41L] 🧪
│   │   │   ├── test_driver_framework.py (19KB) [434L] 🧪
│   │   │   ├── test_gateway_full.py (1KB) [47L] 🧪
│   │   │   ├── test_kernel_api.py (21KB) [465L] 🧪
│   │   │   ├── test_optimizer.py (9KB) [256L] 🧪
│   │   │   └── test_orchestrator.py (1KB) [38L] 🧪
│   │   ├── network/ (4 files, 98 lines)
│   │   │   ├── __init__.py (0B) [0L]
│   │   │   ├── test_atcnet.py (1KB) [42L] 🧪
│   │   │   ├── test_discovery.py (1KB) [28L] 🧪
│   │   │   └── test_p2p_propagation.py (1KB) [28L] 🧪
│   │   ├── __init__.py (0B) [0L]
│   │   ├── test_gateway.py (7KB) [201L] 🧪
│   │   └── test_kai_integration.py (1KB) [47L] 🧪
│   └── __init__.py (0B) [0L]
├── tools/ (11 files, 753 lines)
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (433B) [21L]
│   ├── FILE_REGISTER.md (339B) [11L]
│   ├── LICENSE (658B)
│   ├── README.md (2KB) [58L]
│   ├── ROADMAP.md (494B) [21L]
│   ├── STATUS.md (352B) [19L]
│   ├── atc_issues_summary.atc (6KB) [212L]
│   ├── bigquery_pipeline.atc (4KB) [135L]
│   ├── ecdsa_impl.atc (4KB) [119L]
│   └── hf_review_pipeline.atc (5KB) [157L]
├── vm/ (7 files, 161 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (215B) [8L]
│   ├── FILE_REGISTER.md (393B) [13L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [103L]
│   ├── ROADMAP.md (368B) [16L]
│   └── STATUS.md (483B) [21L]
├── wallet/ (7 files, 163 lines)
│   ├── .gitignore (116B)
│   ├── CHANGELOG.md (219B) [8L]
│   ├── FILE_REGISTER.md (490B) [18L]
│   ├── LICENSE (703B)
│   ├── README.md (4KB) [100L]
│   ├── ROADMAP.md (372B) [16L]
│   └── STATUS.md (487B) [21L]
├── wiki/ (25 files, 2209 lines)
│   ├── docs/ (21 files, 1547 lines)
│   │   ├── API.md (1KB) [59L]
│   │   ├── API_REFERENCE.md (1KB) [50L]
│   │   ├── ARCHITECTURE.md (5KB) [126L]
│   │   ├── BOTTLENECKS.md (1KB) [50L]
│   │   ├── COMMITS.md (2KB) [73L]
│   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   ├── DECENTRALIZED_PROOF.md (3KB) [103L]
│   │   ├── DEPENDENCIES.md (2KB) [79L]
│   │   ├── ENTERPRISE.md (1KB) [65L]
│   │   ├── ERRORS.md (4KB) [79L]
│   │   ├── ERROR_SOLUTIONS.md (3KB) [128L]
│   │   ├── FAQ.md (1KB) [62L]
│   │   ├── IMPROVEMENTS.md (1KB) [61L]
│   │   ├── ISSUES_TRACKER.md (4KB) [107L]
│   │   ├── MATH_PROOF.md (3KB) [93L]
│   │   ├── QUICKSTART.md (619B) [30L]
│   │   ├── ROADMAP.md (2KB) [80L]
│   │   ├── SECURITY.md (916B) [18L]
│   │   ├── STATUS.md (909B) [25L]
│   │   ├── SYNTAX.md (3KB) [133L]
│   │   └── WHITEPAPER.md (5KB) [107L]
│   ├── LICENSE (982B)
│   ├── README.md (4KB) [65L]
│   ├── genesis_communication_layer_v2.md (14KB) [431L]
│   └── genesis_franchise_factory_v1.md (6KB) [166L]
├── windows/ (9 files, 127 lines)
│   ├── src/ (1 files, 16 lines)
│   │   └── main.rs (645B) [16L]
│   ├── .gitignore (171B)
│   ├── CHANGELOG.md (433B) [21L]
│   ├── Cargo.toml (279B) [13L]
│   ├── FILE_REGISTER.md (400B) [13L]
│   ├── LICENSE (658B)
│   ├── README.md (1KB) [37L]
│   ├── ROADMAP.md (494B) [21L]
│   └── STATUS.md (352B) [19L]
├── .coveragerc (532B)
├── .env.example (526B)
├── .gitignore (37B)
├── AAA_ASSET_SYSTEM_v1.md (3KB) [120L]
├── AGENT_MANIFEST.md (5KB) [98L]
├── AGENT_MASTERRULES.md (14KB) [466L]
├── AGENT_PROTOCOL.md (8KB) [256L]
├── ATCLANG_FIRST.md (3KB) [158L]
├── CHANGELOG.md (5KB) [167L]
├── CONNECTION_MAP.md (2KB) [50L]
├── CONTRIBUTING.md (804B) [31L]
├── DOCUMENTATION_SYNC_GUIDE.md (9KB) [336L]
├── Dockerfile (948B)
├── ECOSYSTEM.md (1KB) [52L]
├── ECOSYSTEM_BRAIN.md (4KB) [108L]
├── FILE_REGISTER.md (5KB) [99L]
├── GENESIS_BUS_ARCHITECTURE.md (5KB) [121L]
├── GENESIS_CIVILIZATION_PLATFORM_v4.md (5KB) [153L]
├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
├── GENESIS_FRANCHISE_FACTORY_v2.md (4KB) [101L]
├── INSTALL.md (3KB) [161L]
├── KERNEL_FROM_SCRATCH_PLAN.md (7KB) [87L]
├── KONSOLIDIERUNGS_MATRIX.md (6KB) [124L]
├── KONSOLIDIERUNGS_ROADMAP.md (3KB) [79L]
├── LICENSE (982B)
├── MASTER_TODO.md (11KB) [324L]
├── MILESTONES.md (1KB) [27L]
├── MONOREPO_STRUCTURE.md (9KB) [183L]
├── Makefile (2KB)
├── NAMING_CONVENTIONS.md (6KB) [109L]
├── OS_BARE_METAL_GAP_ANALYSIS.md (7KB) [89L]
├── README.md (4KB) [104L]
├── REALITY_STATUS.md (6KB) [112L]
├── REPO_ARCHITECTURE.md (16KB) [377L]
├── ROADMAP.md (484B) [21L]
├── ROADMAP_PYTHON_TO_OS.md (18KB) [261L]
├── SECURITY.md (813B) [27L]
├── STATUS.md (2KB) [49L]
├── SYNC_PROTOCOL.md (870B) [28L]
├── TODO.md (2KB) [63L]
├── UPGRADE.md (3KB) [125L]
├── VERSION (6B)
├── docker-compose.yml (2KB) [80L]
├── pyproject.toml (466B) [21L]
├── pytest.ini (777B)
├── requirements-kai.txt (421B)
├── requirements.txt (1KB)
└── start.atc (4KB) [129L]
```


## a-townchain-os-docs

**Layer:** Wiki (Haupt-Doku) | **Dateien:** 2110 | **Verzeichnisse:** 455 | **Max Tiefe:** 7 | **Tests:** 62

**Beschreibung:** A-TownChain OS

**Sprachen:** .md(1125) · .atc(352) · .py(275) · .tsx(159) · .ts(43) · .no-ext(38)

**Test-Dateien:** 56× Python · 6× TypeScript

```
a-townchain-os-docs/
├── TODO/ (1 files, 61 lines)
│   └── MASTER_TODO.md (2KB) [61L]
├── aistudio/ (245 files, 71905 lines)
│   ├── assets/ (1 files, 0 lines)
│   │   └── .aistudio/ (1 files, 0 lines)
│   │       └── .gitignore (2B)
│   ├── src/ (190 files, 56202 lines)
│   │   ├── backend/ (2 files, 206 lines)
│   │   │   ├── blockchain/ (1 files, 129 lines)
│   │   │   │   └── engine.ts (3KB) [129L]
│   │   │   └── p2p/ (1 files, 77 lines)
│   │   │       └── network.ts (2KB) [77L]
│   │   ├── components/ (148 files, 43089 lines)
│   │   │   ├── ATCAssetView.tsx (11KB) [191L]
│   │   │   ├── ATCDjStudioView.tsx (17KB) [445L]
│   │   │   ├── ATCLangEditor.tsx (26KB) [625L]
│   │   │   ├── ATCWalletView.tsx (26KB) [498L]
│   │   │   ├── ATownDashboardView.tsx (14KB) [302L]
│   │   │   ├── ATownOSNode.tsx (71KB) [1439L]
│   │   │   ├── ATownTestView.tsx (6KB) [111L]
│   │   │   ├── AgentCivilizationView.tsx (8KB) [152L]
│   │   │   ├── Ai3DRenderEngineTab.tsx (8KB) [199L]
│   │   │   ├── AiAnimationEngineTab.tsx (8KB) [198L]
│   │   │   ├── AiAudioEngineTab.tsx (8KB) [198L]
│   │   │   ├── AiCharacterBioTab.tsx (9KB) [199L]
│   │   │   ├── AiGameEngineTab.tsx (9KB) [200L]
│   │   │   ├── AiKernelView.tsx (6KB) [128L]
│   │   │   ├── AiOsEngineView.tsx (19KB) [490L]
│   │   │   ├── AiSoftwareWorkflowView.tsx (11KB) [229L]
│   │   │   ├── AiTimelineEngineTab.tsx (9KB) [199L]
│   │   │   ├── AntiCheatView.tsx (14KB) [261L]
│   │   │   ├── ApiHealthWidget.tsx (3KB) [85L]
│   │   │   ├── ApiInterfacesView.tsx (9KB) [189L]
│   │   │   ├── ApiOrchestratorView.tsx (17KB) [354L]
│   │   │   ├── AppGlobeView.tsx (9KB) [233L]
│   │   │   ├── ArchitectureDependencyGraph.tsx (7KB) [248L]
│   │   │   ├── ArchitectureView.tsx (38KB) [888L]
│   │   │   ├── AssetVaultView.tsx (8KB) [187L]
│   │   │   ├── AtcAssetsDbView.tsx (11KB) [250L]
│   │   │   ├── AtcCoreKernelView.tsx (8KB) [144L]
│   │   │   ├── AtcLangArchitectureView.tsx (33KB) [585L]
│   │   │   ├── AtcLangPlaygroundView.tsx (12KB) [256L]
│   │   │   ├── AtcLangPresetsView.tsx (3KB) [64L]
│   │   │   ├── AtcWhitepaperView.tsx (10KB) [187L]
│   │   │   ├── AtsSuite.tsx (4KB) [51L]
│   │   │   ├── AtvmSandboxView.test.tsx (3KB) [85L] 🧪
│   │   │   ├── AtvmSandboxView.tsx (26KB) [499L]
│   │   │   ├── BatteryStatus.tsx (11KB) [269L]
│   │   │   ├── BattleArenaView.tsx (8KB) [143L]
│   │   │   ├── BenchmarkCenterView.tsx (15KB) [288L]
│   │   │   ├── BlockchainEcosystemView.tsx (9KB) [224L]
│   │   │   ├── BlockchainLedgerView.tsx (13KB) [247L]
│   │   │   ├── CalculatorView.tsx (3KB) [74L]
│   │   │   ├── CalendarView.tsx (3KB) [78L]
│   │   │   ├── CiCdPipelineView.tsx (7KB) [159L]
│   │   │   ├── ClockView.tsx (3KB) [72L]
│   │   │   ├── CodeAnalyzerView.tsx (4KB) [90L]
│   │   │   ├── CommitHeatmap.tsx (4KB) [110L]
│   │   │   ├── ComplianceEngineView.tsx (4KB) [84L]
│   │   │   ├── ComplianceView.tsx (8KB) [191L]
│   │   │   ├── ConflictResolutionModal.tsx (11KB) [257L]
│   │   │   ├── ConsensusIntegrationGuide.tsx (70KB) [1528L]
│   │   │   ├── CryptoVisualizationView.tsx (18KB) [473L]
│   │   │   ├── DataProcessingView.tsx (4KB) [78L]
│   │   │   ├── DbOrchestratorView.tsx (6KB) [112L]
│   │   │   ├── DeFiLiquidityPoolView.tsx (13KB) [255L]
│   │   │   ├── DependencyMapView.tsx (3KB) [123L]
│   │   │   ├── DeploymentPipelineWidget.tsx (6KB) [160L]
│   │   │   ├── DevToolsView.tsx (5KB) [133L]
│   │   │   ├── DeveloperKnowledgeBaseView.tsx (18KB) [359L]
│   │   │   ├── DistributedDatalakeView.tsx (3KB) [73L]
│   │   │   ├── EcosystemInstaller.tsx (11KB) [297L]
│   │   │   ├── EcosystemTreeOverlay.tsx (12KB) [357L]
│   │   │   ├── EcosystemUmlView.tsx (7KB) [143L]
│   │   │   ├── EcosystemVisualizerView.tsx (12KB) [325L]
│   │   │   ├── FileManagerView.tsx (6KB) [170L]
│   │   │   ├── FolderView.tsx (4KB) [111L]
│   │   │   ├── FranchiseFactoryView.tsx (83KB) [1733L]
│   │   │   ├── GateToHellBrowser.tsx (5KB) [106L]
│   │   │   ├── GenesisBlockGeneratorView.tsx (6KB) [150L]
│   │   │   ├── GitGraphVisualization.tsx (4KB) [137L]
│   │   │   ├── GitHubRepoSyncView.tsx (63KB) [1385L]
│   │   │   ├── GitHubStatusDashboard.tsx (34KB) [643L]
│   │   │   ├── GitOpsView.tsx (7KB) [126L]
│   │   │   ├── GovernanceView.tsx (23KB) [601L]
│   │   │   ├── GpuPerformanceWidget.tsx (4KB) [120L]
│   │   │   ├── HardwareDriversView.tsx (20KB) [376L]
│   │   │   ├── IdeaToAppFlowchartView.tsx (7KB) [153L]
│   │   │   ├── ImageGeneratorTab.tsx (5KB) [117L]
│   │   │   ├── IntegrationsWindow.tsx (21KB) [426L]
│   │   │   ├── InterfacesView.tsx (2KB) [56L]
│   │   │   ├── JsExampleRunner.tsx (2KB) [86L]
│   │   │   ├── LazyMetricsCharts.tsx (31KB) [808L]
│   │   │   ├── LegalView.tsx (6KB) [87L]
│   │   │   ├── LoginOverlay.tsx (38KB) [690L]
│   │   │   ├── MainnetLaunchView.tsx (12KB) [251L]
│   │   │   ├── MarketplaceView.tsx (22KB) [450L]
│   │   │   ├── MediaApps.tsx (18KB) [254L]
│   │   │   ├── MetricsDashboard.tsx (4KB) [105L]
│   │   │   ├── MetricsView.tsx (56KB) [1476L]
│   │   │   ├── ModulesPluginView.tsx (18KB) [309L]
│   │   │   ├── NetworkExplorerView.test.tsx (4KB) [121L] 🧪
│   │   │   ├── NetworkExplorerView.tsx (17KB) [370L]
│   │   │   ├── NetworkTopologyView.tsx (2KB) [38L]
│   │   │   ├── NodeHealthMonitor.tsx (4KB) [113L]
│   │   │   ├── NotepadView.tsx (2KB) [67L]
│   │   │   ├── OfficeApps.tsx (352B) [14L]
│   │   │   ├── OfficeSuiteView.tsx (12KB) [271L]
│   │   │   ├── P2PChatView.tsx (12KB) [277L]
│   │   │   ├── Paint3DView.tsx (5KB) [140L]
│   │   │   ├── PaymentSystemView.tsx (4KB) [93L]
│   │   │   ├── PipelineGeneratorTab.tsx (19KB) [433L]
│   │   │   ├── PoAITrainingEngineView.tsx (8KB) [173L]
│   │   │   ├── ProjectAuditDashboard.tsx (7KB) [135L]
│   │   │   ├── ProjectHubView.tsx (30KB) [501L]
│   │   │   ├── ProtocolsView.tsx (8KB) [207L]
│   │   │   ├── ReportsView.tsx (10KB) [202L]
│   │   │   ├── RepositoryActivityChart.tsx (5KB) [145L]
│   │   │   ├── RepositoryLineChart.tsx (6KB) [198L]
│   │   │   ├── RescueSystemView.tsx (16KB) [307L]
│   │   │   ├── RoadmapView.tsx (6KB) [196L]
│   │   │   ├── SemanticGraphView.tsx (4KB) [86L]
│   │   │   ├── SessionExportView.tsx (8KB) [221L]
│   │   │   ├── SettingsView.tsx (105KB) [2312L]
│   │   │   ├── SocialMediaView.tsx (16KB) [287L]
│   │   │   ├── SoftwareAuditView.tsx (38KB) [885L]
│   │   │   ├── SoftwareKnowledgeDbView.tsx (18KB) [380L]
│   │   │   ├── SourceCodeViewer.tsx (20KB) [547L]
│   │   │   ├── SpecificSettingsViews.tsx (17KB) [306L]
│   │   │   ├── StorageManagerView.tsx (9KB) [258L]
│   │   │   ├── StrategicArchitectureMap.tsx (9KB) [243L]
│   │   │   ├── StructureView.tsx (22KB) [505L]
│   │   │   ├── SyncDashboardModal.tsx (4KB) [88L]
│   │   │   ├── SyncHistoryModal.tsx (11KB) [249L]
│   │   │   ├── SyncMetricsView.tsx (7KB) [170L]
│   │   │   ├── SyncStatusDonutChart.tsx (2KB) [99L]
│   │   │   ├── SyncStatusOverview.tsx (7KB) [168L]
│   │   │   ├── SystemDiagnosticsView.tsx (18KB) [337L]
│   │   │   ├── SystemFinderView.tsx (2KB) [56L]
│   │   │   ├── SystemHealthDashboard.tsx (9KB) [246L]
│   │   │   ├── SystemHealthDashboardWidget.tsx (2KB) [63L]
│   │   │   ├── SystemLogsView.tsx (3KB) [89L]
│   │   │   ├── TaskManagerView.tsx (3KB) [82L]
│   │   │   ├── TechDocsView.tsx (17KB) [335L]
│   │   │   ├── TechTreeView.tsx (18KB) [420L]
│   │   │   ├── TerminalView.tsx (6KB) [189L]
│   │   │   ├── TestnetOrchestrationView.tsx (8KB) [178L]
│   │   │   ├── TestnetSimulationView.tsx (14KB) [298L]
│   │   │   ├── TextGeneratorTab.tsx (6KB) [177L]
│   │   │   ├── ThemeSwitcher.tsx (4KB) [143L]
│   │   │   ├── TodoView.tsx (18KB) [383L]
│   │   │   ├── TooltipIcon.tsx (1KB) [29L]
│   │   │   ├── TxOrchestratorView.tsx (5KB) [105L]
│   │   │   ├── UserProfileView.tsx (12KB) [255L]
│   │   │   ├── VideoGeneratorTab.tsx (7KB) [176L]
│   │   │   ├── WebhookMonitor.tsx (5KB) [145L]
│   │   │   ├── Window.tsx (6KB) [158L]
│   │   │   ├── WindowExtras.tsx (4KB) [87L]
│   │   │   ├── ZeroKnowledgeProofView.tsx (6KB) [129L]
│   │   │   ├── ZkCircuitEditorView.tsx (4KB) [108L]
│   │   │   └── ZkVisualizationView.tsx (3KB) [99L]
│   │   ├── contexts/ (4 files, 269 lines)
│   │   │   ├── FirebaseContext.tsx (2KB) [94L]
│   │   │   ├── GoogleWorkspaceContext.tsx (2KB) [83L]
│   │   │   ├── SyncMetricsContext.tsx (1KB) [47L]
│   │   │   └── WalletContext.tsx (1KB) [45L]
│   │   ├── db/ (3 files, 64 lines)
│   │   │   ├── drizzle.config.ts (817B) [29L]
│   │   │   ├── index.ts (652B) [24L]
│   │   │   └── schema.ts (486B) [11L]
│   │   ├── hooks/ (2 files, 250 lines)
│   │   │   ├── useGoogleSheetsSync.ts (8KB) [220L]
│   │   │   └── useKeyboardShortcut.ts (899B) [30L]
│   │   ├── lib/ (6 files, 359 lines)
│   │   │   ├── CryptoEngine.ts (1KB) [42L]
│   │   │   ├── firebase-admin.ts (544B) [15L]
│   │   │   ├── firebase.ts (2KB) [64L]
│   │   │   ├── indexedDb.ts (2KB) [88L]
│   │   │   ├── syncLogic.test.ts (2KB) [82L] 🧪
│   │   │   └── syncLogic.ts (1KB) [68L]
│   │   ├── middleware/ (1 files, 30 lines)
│   │   │   └── auth.ts (953B) [30L]
│   │   ├── routes/ (1 files, 146 lines)
│   │   │   └── notion.ts (4KB) [146L]
│   │   ├── services/ (2 files, 143 lines)
│   │   │   ├── SyncService.ts (3KB) [106L]
│   │   │   └── githubSync.ts (1KB) [37L]
│   │   ├── utils/ (4 files, 240 lines)
│   │   │   ├── appSync.tsx (2KB) [84L]
│   │   │   ├── auditUtils.test.ts (1KB) [56L] 🧪
│   │   │   ├── auditUtils.ts (749B) [27L]
│   │   │   └── crypto.ts (4KB) [73L]
│   │   ├── App.tsx (233KB) [5440L]
│   │   ├── DesktopApp.tsx (121KB) [2740L]
│   │   ├── atcLangRoadmapData.ts (6KB) [201L]
│   │   ├── atcLangWikiData.ts (16KB) [227L]
│   │   ├── auditData.ts (4KB) [76L]
│   │   ├── data.ts (17KB) [411L]
│   │   ├── ecosystemData.ts (11KB) [291L]
│   │   ├── fix_translation.cjs (463B)
│   │   ├── index.css (5KB)
│   │   ├── main.tsx (774B) [24L]
│   │   ├── marketplaceApps.ts (6KB) [273L]
│   │   ├── requirementsData.ts (1KB) [58L]
│   │   ├── roadmapData.ts (7KB) [312L]
│   │   ├── standardsData.ts (4KB) [83L]
│   │   ├── tierData.ts (16KB) [317L]
│   │   ├── types.ts (375B) [10L]
│   │   └── wikiData.ts (47KB) [943L]
│   ├── tests/ (2 files, 127 lines)
│   │   ├── GitHubRepoSyncView.test.tsx (1KB) [49L] 🧪
│   │   └── audit_compliance.test.ts (2KB) [78L] 🧪
│   ├── workspace/ (8 files, 664 lines)
│   │   ├── src/ (2 files, 435 lines)
│   │   │   ├── backend/ (1 files, 167 lines)
│   │   │   │   └── blockchain/ (1 files, 167 lines)
│   │   │   │       └── engine.ts (5KB) [167L]
│   │   │   └── components/ (1 files, 268 lines)
│   │   │       └── GovernanceView.tsx (14KB) [268L]
│   │   ├── move.js (411B) [13L]
│   │   ├── rename.js (1KB) [42L]
│   │   ├── replace.js (1KB) [40L]
│   │   ├── replaceEnterprise.js (3KB) [102L]
│   │   ├── replaceGoals.ts (688B) [14L]
│   │   └── replaceGoals2.ts (825B) [18L]
│   ├── .env.example (578B)
│   ├── .gitignore (73B)
│   ├── AGENTS.md (535B) [13L]
│   ├── GEMINI.md (373B) [6L]
│   ├── LICENSE (1KB)
│   ├── README.md (542B) [20L]
│   ├── ROADMAP.md (8KB) [598L]
│   ├── SOFTWARE_ROADMAP.md (38KB) [1116L]
│   ├── check_dups2.js (498B) [12L]
│   ├── check_dups_all.js (885B) [23L]
│   ├── check_dups_desktop.js (480B) [15L]
│   ├── check_dups_windows_map.js (519B) [14L]
│   ├── fetch.js (1KB) [36L]
│   ├── firebase-applet-config.json (363B) [9L]
│   ├── fix.js (859B) [26L]
│   ├── fix2.js (894B) [27L]
│   ├── fix_react_imports.cjs (547B)
│   ├── fix_wiki.cjs (184B)
│   ├── fix_wiki.js (284B) [5L]
│   ├── index.html (413B)
│   ├── mark_completed.ts (722B) [15L]
│   ├── mark_completed_src.ts (1KB) [33L]
│   ├── metadata.json (214B) [6L]
│   ├── move_back.js (347B) [11L]
│   ├── output.txt (3KB)
│   ├── package-lock.json (419KB) [11838L]
│   ├── package.json (2KB) [72L]
│   ├── replace.js (1KB) [36L]
│   ├── replace_langs.cjs (852B)
│   ├── replace_langs_2.cjs (667B)
│   ├── replace_langs_3.cjs (411B)
│   ├── replace_langs_4.cjs (817B)
│   ├── replace_langs_5.cjs (528B)
│   ├── replace_langs_6.cjs (522B)
│   ├── script.cjs (883B)
│   ├── script.js (983B) [12L]
│   ├── script2.cjs (683B)
│   ├── server.ts (33KB) [866L]
│   ├── testChat.js (450B) [10L]
│   ├── test_know.js (244B) [2L] 🧪
│   ├── tmp.txt (470B)
│   ├── tsconfig.json (508B) [26L]
│   ├── update_wiki_categories.ts (742B) [23L]
│   └── vite.config.ts (1KB) [42L]
├── archive/ (1 files, 97 lines)
│   └── ATCLANG_ARCHIVE.md (4KB) [97L]
├── atclang/ (32 files, 8174 lines)
│   ├── compiler/ (4 files, 1634 lines)
│   │   ├── __init__.py (468B) [8L]
│   │   ├── compiler.py (21KB) [561L]
│   │   ├── optimizer.py (22KB) [558L]
│   │   └── type_checker.py (20KB) [507L]
│   ├── lexer/ (2 files, 574 lines)
│   │   ├── __init__.py (161B) [2L]
│   │   └── lexer.py (20KB) [572L]
│   ├── parser/ (3 files, 1224 lines)
│   │   ├── __init__.py (189B) [3L]
│   │   ├── ast_nodes.py (7KB) [331L]
│   │   └── parser.py (37KB) [890L]
│   ├── programs/ (1 files, 1161 lines)
│   │   └── atcos_main.atc (40KB) [1161L]
│   ├── repl/ (2 files, 185 lines)
│   │   ├── __init__.py (99B) [1L]
│   │   └── repl.py (6KB) [184L]
│   ├── stdlib/ (14 files, 1807 lines)
│   │   ├── __init__.py (1KB) [32L]
│   │   ├── atc_stdlib.py (2KB) [69L]
│   │   ├── chain.py (1KB) [41L]
│   │   ├── collections.py (5KB) [219L]
│   │   ├── collections_ext.py (3KB) [143L]
│   │   ├── crypto.py (5KB) [155L]
│   │   ├── crypto_ext.py (5KB) [149L]
│   │   ├── encoding.py (7KB) [210L]
│   │   ├── io.py (3KB) [107L]
│   │   ├── io_ext.py (3KB) [123L]
│   │   ├── math.py (3KB) [138L]
│   │   ├── primitives.py (7KB) [244L]
│   │   ├── string.py (2KB) [99L]
│   │   └── wallet.py (2KB) [78L]
│   ├── v03/ (2 files, 303 lines)
│   │   ├── __init__.py (124B) [2L]
│   │   └── atclang_v03_features.py (10KB) [301L]
│   ├── vm/ (2 files, 980 lines)
│   │   ├── __init__.py (177B) [2L]
│   │   └── atcvm.py (47KB) [978L]
│   ├── ATCLANG_SPEC.md (9KB) [295L]
│   └── __init__.py (462B) [11L]
├── atcpkg/ (1 files, 145 lines)
│   └── manager.atc (4KB) [145L]
├── backend/ (14 files, 1467 lines)
│   ├── api/ (8 files, 969 lines)
│   │   ├── orchestrator/ (2 files, 261 lines)
│   │   │   ├── __init__.py (118B) [2L]
│   │   │   └── orchestrator.atc (8KB) [259L]
│   │   ├── routes/ (3 files, 409 lines)
│   │   │   ├── __init__.py (115B) [2L]
│   │   │   ├── ai_routes.atc (5KB) [175L]
│   │   │   └── api_routes.atc (8KB) [232L]
│   │   ├── __init__.py (111B) [2L]
│   │   ├── kai_routes.atc (7KB) [229L]
│   │   └── server.atc (2KB) [68L]
│   ├── db/ (3 files, 355 lines)
│   │   ├── __init__.py (160B) [2L]
│   │   ├── connection.atc (4KB) [125L]
│   │   └── repository.atc (6KB) [228L]
│   ├── wallet/ (2 files, 141 lines)
│   │   ├── __init__.py (123B) [2L]
│   │   └── wallet.atc (4KB) [139L]
│   └── __init__.py (121B) [2L]
├── blockchain/ (49 files, 6353 lines)
│   ├── atcoin/ (1 files, 2 lines)
│   │   └── __init__.py (119B) [2L]
│   ├── consensus/ (13 files, 1548 lines)
│   │   ├── __init__.py (123B) [2L]
│   │   ├── fork_atc85.atc (2KB) [74L]
│   │   ├── fork_resolution.atc (4KB) [145L]
│   │   ├── gas_fee.atc (4KB) [130L]
│   │   ├── gas_fee_atc86.atc (2KB) [71L]
│   │   ├── hybrid_atc84.atc (3KB) [98L]
│   │   ├── hybrid_consensus.atc (11KB) [357L]
│   │   ├── poh.atc (4KB) [140L]
│   │   ├── poh_atc83.atc (1KB) [79L]
│   │   ├── pos.atc (4KB) [164L]
│   │   ├── pos_atc82.atc (2KB) [92L]
│   │   ├── pow.atc (3KB) [107L]
│   │   └── pow_atc81.atc (2KB) [89L]
│   ├── contracts/ (6 files, 756 lines)
│   │   ├── atc001/ (1 files, 102 lines)
│   │   │   └── genesis_token.atc (2KB) [102L]
│   │   ├── atc8300/ (1 files, 2 lines)
│   │   │   └── __init__.py (129B) [2L]
│   │   ├── governance/ (1 files, 202 lines)
│   │   │   └── governance_contract.atc (7KB) [202L]
│   │   ├── shivamon/ (2 files, 141 lines)
│   │   │   ├── __init__.py (136B) [2L]
│   │   │   └── breeding.atc (5KB) [139L]
│   │   └── contract_engine_atc14.atc (9KB) [309L]
│   ├── dex/ (2 files, 279 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   └── amm.atc (10KB) [277L]
│   ├── governance/ (5 files, 775 lines)
│   │   ├── __init__.py (120B) [2L]
│   │   ├── dao.atc (6KB) [168L]
│   │   ├── dao_live.atc (8KB) [235L]
│   │   ├── timelock.atc (4KB) [150L]
│   │   └── treasury.atc (6KB) [220L]
│   ├── mainnet/ (3 files, 258 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   ├── launch_manager.atc (3KB) [105L]
│   │   └── mainnet_config.atc (5KB) [151L]
│   ├── network/ (3 files, 514 lines)
│   │   ├── core_node_atc01.atc (4KB) [164L]
│   │   ├── latency_opt_atc06.atc (3KB) [135L]
│   │   └── sharding_atc07.atc (5KB) [215L]
│   ├── nodes/ (6 files, 854 lines)
│   │   ├── __init__.py (126B) [2L]
│   │   ├── block_propagation.atc (3KB) [87L]
│   │   ├── bootstrap.atc (6KB) [234L]
│   │   ├── initial_sync.atc (6KB) [207L]
│   │   ├── node.atc (6KB) [192L]
│   │   └── testnet_launcher.atc (4KB) [132L]
│   ├── propagation/ (1 files, 98 lines)
│   │   └── block_gossip.atc (3KB) [98L]
│   ├── wallet/ (4 files, 504 lines)
│   │   ├── __init__.py (128B) [2L]
│   │   ├── did.atc (4KB) [122L]
│   │   ├── multisig.atc (8KB) [268L]
│   │   └── wordlist.atc (5KB) [112L]
│   ├── zkp/ (2 files, 93 lines)
│   │   ├── __init__.py (336B) [4L]
│   │   └── groth16.atc (3KB) [89L]
│   ├── contract_registry.atc (3KB) [98L]
│   ├── smart_contract_registry.atc (2KB) [88L]
│   └── smart_contracts.atc (15KB) [486L]
├── config/ (1 files, 95 lines)
│   └── mainnet_genesis.json (3KB) [95L]
├── core/ (3 files, 392 lines)
│   ├── ai/ (1 files, 178 lines)
│   │   └── federated_learning.atc (6KB) [178L]
│   ├── crypto/ (1 files, 19 lines)
│   │   └── __init__.py (535B) [19L]
│   └── kai_cli.atc (8KB) [195L]
├── devnet/ (1 files, 554 lines)
│   └── README.md (12KB) [554L]
├── docs/ (459 files, 90440 lines)
│   ├── ai/ (3 files, 547 lines)
│   │   ├── AI_SAFETY.md (5KB) [184L]
│   │   ├── GEMINI_INTEGRATION.md (5KB) [214L]
│   │   └── LLM_ROUTER.md (4KB) [149L]
│   ├── aistudio/ (1 files, 439 lines)
│   │   └── AISTUDIO_COMPONENTS.md (24KB) [439L]
│   ├── architecture/ (12 files, 1826 lines)
│   │   ├── AI_LAYER.md (2KB) [53L]
│   │   ├── ATCFS.md (4KB) [129L]
│   │   ├── ATCLANG_COMPILER.md (2KB) [64L]
│   │   ├── ATCNET_P2P.md (6KB) [211L]
│   │   ├── CONSENSUS.md (3KB) [121L]
│   │   ├── GATEWAY.md (2KB) [112L]
│   │   ├── GOVERNANCE.md (1KB) [50L]
│   │   ├── KERNEL_SHELL.md (1KB) [50L]
│   │   ├── MONITORING_DEVOPS.md (1KB) [42L]
│   │   ├── SHIVAOS_KERNEL.md (5KB) [182L]
│   │   ├── TESTNET.md (20KB) [713L]
│   │   └── WALLET_KEYGEN.md (2KB) [99L]
│   ├── atclang/ (1 files, 9 lines)
│   │   └── ATCLANG_SPEC_FULL.md (423B) [9L]
│   ├── blockchain/ (2 files, 16 lines)
│   │   ├── ETHEREUM_INTEGRATION.md (212B) [8L]
│   │   └── SOLANA_INTEGRATION.md (210B) [8L]
│   ├── compliance/ (5 files, 1575 lines)
│   │   ├── ATVM_LICENSE_GATE_SPEC.md (7KB) [242L]
│   │   ├── BAFIN_KONFORMITAETSBERICHT.md (15KB) [408L]
│   │   ├── COMPLIANCE_HANDBUCH.md (5KB) [131L]
│   │   ├── IP_LICENSE_DASHBOARD_SPEC.md (6KB) [205L]
│   │   └── SMART_CONTRACT_RICHTLINIE.md (21KB) [589L]
│   ├── contracts/ (2 files, 790 lines)
│   │   ├── ATC_TOKEN_STANDARD.md (534B) [12L]
│   │   └── SHIVAMON_NFT_CONTRACT.md (20KB) [778L]
│   ├── file_registers/ (23 files, 4923 lines)
│   │   ├── README.md (1KB) [42L]
│   │   ├── a-townchain-os_FILE_REGISTER.md (75KB) [1491L]
│   │   ├── atc-aistudio_FILE_REGISTER.md (12KB) [277L]
│   │   ├── atc-atclang_FILE_REGISTER.md (1KB) [68L]
│   │   ├── atc-atcpkg_FILE_REGISTER.md (1KB) [39L]
│   │   ├── atc-backend_FILE_REGISTER.md (1KB) [53L]
│   │   ├── atc-blockchain_FILE_REGISTER.md (3KB) [104L]
│   │   ├── atc-contracts_FILE_REGISTER.md (1KB) [51L]
│   │   ├── atc-franchise_FILE_REGISTER.md (1KB) [43L]
│   │   ├── atc-frontend_FILE_REGISTER.md (947B) [38L]
│   │   ├── atc-gateway_FILE_REGISTER.md (2KB) [71L]
│   │   ├── atc-genesis-engine_FILE_REGISTER.md (1KB) [46L]
│   │   ├── atc-kernel_FILE_REGISTER.md (1KB) [50L]
│   │   ├── atc-linux-edition_FILE_REGISTER.md (838B) [35L]
│   │   ├── atc-mobile_FILE_REGISTER.md (897B) [37L]
│   │   ├── atc-shivacore-tools_FILE_REGISTER.md (787B) [33L]
│   │   ├── atc-shivacore_FILE_REGISTER.md (309KB) [2183L]
│   │   ├── atc-shivamon_FILE_REGISTER.md (1KB) [43L]
│   │   ├── atc-standards_FILE_REGISTER.md (1KB) [41L]
│   │   ├── atc-ui_FILE_REGISTER.md (923B) [38L]
│   │   ├── atc-windows-edition_FILE_REGISTER.md (844B) [35L]
│   │   ├── atclang_FILE_REGISTER.md (1KB) [60L]
│   │   └── atcnet_FILE_REGISTER.md (1KB) [45L]
│   ├── issues/ (85 files, 4932 lines)
│   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [143L]
│   │   ├── ISSUE_02_GEMINI_AI.md (3KB) [141L]
│   │   ├── ISSUE_03_BATTLE_UI.md (4KB) [141L]
│   │   ├── ISSUE_04_PERSISTENZ.md (4KB) [156L]
│   │   ├── ISSUE_05_EXPLORER.md (3KB) [102L]
│   │   ├── ISSUE_06_ECDSA.md (4KB) [143L]
│   │   ├── ISSUE_07_BUILD.md (3KB) [133L]
│   │   ├── ISSUE_08_TESTNET.md (3KB) [127L]
│   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [99L]
│   │   ├── ISSUE_10_BRIDGE.md (1KB) [53L]
│   │   ├── ISSUE_11_BREEDING.md (3KB) [88L]
│   │   ├── ISSUE_12_SOLIDITY.md (4KB) [147L]
│   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [122L]
│   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [310L]
│   │   ├── ISSUE_15__TESTNET_BLOCK_PROPAGATION_.md (1KB) [46L]
│   │   ├── ISSUE_16__TESTNET_INITIAL_SYNC__NEU.md (1KB) [45L]
│   │   ├── ISSUE_17__TESTNET_LONGEST-CHAIN-RULE.md (1KB) [45L]
│   │   ├── ISSUE_18__TESTNET_DOCKER_COMPOSE__5.md (1KB) [46L]
│   │   ├── ISSUE_19__TESTNET_NODE-MONITORING_DA.md (1KB) [45L]
│   │   ├── ISSUE_20_GATEWAY_TESTS.md (1KB) [63L]
│   │   ├── ISSUE_23__ATCFS__INTEGRATION_IN_KERN.md (1KB) [48L]
│   │   ├── ISSUE_24__MULTISIG_WALLET__BRIDGE__F.md (1KB) [47L]
│   │   ├── ISSUE_25__GATEWAY_4000__VOLLSTÄNDIGE.md (1KB) [48L]
│   │   ├── ISSUE_26__TESTS__ATCFS_MULTISIG_ATC.md (1KB) [50L]
│   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   ├── ISSUE_28__WIKI_KAP._40__SHIVAOS_UI_RE.md (1KB) [47L]
│   │   ├── ISSUE_29__WIKI_KAP._41__FEDERATED_LEA.md (1KB) [47L]
│   │   ├── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   │   ├── ISSUE_31__WIKI_KAP._4__BLOCK-EXPLORER.md (1KB) [45L]
│   │   ├── ISSUE_32__KAP._5__SHIVAOS_SYSTEM-CALL.md (1KB) [45L]
│   │   ├── ISSUE_33__KAP._4__GAS-FEE_MECHANISMUS.md (1KB) [45L]
│   │   ├── ISSUE_34_V3.0.0_15__SOLANA_BRIDGE_SP.md (1KB) [51L]
│   │   ├── ISSUE_35_V3.0.0_16__ATCLANG_V0.3.0_A.md (1KB) [49L]
│   │   ├── ISSUE_36_V3.0.0_17__MAINNET_LAUNCH_C.md (1KB) [52L]
│   │   ├── ISSUE_37_V3.0.0_20__DEX_-_AMM_LIQUID.md (1KB) [56L]
│   │   ├── ISSUE_38_V3.0.0_21__MOBILE_WALLET_IO.md (1KB) [51L]
│   │   ├── ISSUE_39_V3.0.0_22__DAO-GOVERNANCE_LI.md (1KB) [50L]
│   │   ├── ISSUE_40_DOCS_SYNTAX-REFERENZ__ATCLAN.md (1KB) [52L]
│   │   ├── ISSUE_41_DOCS_MATHEMATISCHE_BEWEISE__.md (1KB) [52L]
│   │   ├── ISSUE_42_DOCS_FEHLERDEFINITIONEN__BOT.md (1KB) [54L]
│   │   ├── ISSUE_43_DOCS_DEZENTRALER_NUTZER-NACHW.md (992B) [44L]
│   │   ├── ISSUE_44_MAINNET_MONITORING__GRAFANA_D.md (798B) [38L]
│   │   ├── ISSUE_45_ATCOIN_DEFI__AMM_LIQUIDITY_PO.md (738B) [38L]
│   │   ├── ISSUE_46_MOBILE_WALLET__BIOMETRIE__PU.md (770B) [38L]
│   │   ├── ISSUE_47_ZKP_ZERO-KNOWLEDGE_PROOFS__L0.md (814B) [38L]
│   │   ├── ISSUE_48_ATCLANG_V0.4.0__TYPE_SYSTEM_.md (823B) [38L]
│   │   ├── ISSUE_49_49__BIGQUERY_ANALYTICS_PIPEL.md (900B) [36L]
│   │   ├── ISSUE_50_50__HUGGING_FACE_CODE-REVIEW.md (881B) [36L]
│   │   ├── ISSUE_51_51__IPC_BUS_VOLLSTÄNDIGE_KE.md (880B) [36L]
│   │   ├── ISSUE_52_52__MAINNET_LAUNCH_MANAGER_.md (1009B) [36L]
│   │   ├── ISSUE_53_V3.2.1__TESTS_PROCESSMANAGER.md (1011B) [39L]
│   │   ├── ISSUE_54_V3.2.1__TESTS_ATCFS_FILESYST.md (1004B) [37L]
│   │   ├── ISSUE_55_V3.2.1__TESTS_ATCNET_P2PNODE.md (987B) [37L]
│   │   ├── ISSUE_56_V3.2.1__TESTS_ATCLANG_TYPECH.md (987B) [40L]
│   │   ├── ISSUE_57_V3.2.1__TESTS_PROMETHEUS_MET.md (998B) [38L]
│   │   ├── ISSUE_58_V3.2.1__TESTS_SERVICEDISCOVE.md (996B) [39L]
│   │   ├── ISSUE_59_V3.2.1__INTEGRATION_NATTRAVE.md (1005B) [36L]
│   │   ├── ISSUE_60_V3.2.1__INTEGRATION_AIKERNEL.md (997B) [37L]
│   │   ├── ISSUE_61_V3.2.1__INTEGRATION_BLOCKGOS.md (1015B) [37L]
│   │   ├── ISSUE_62_V3.2.1__INTEGRATION_SERVICED.md (1007B) [37L]
│   │   ├── ISSUE_63_V3.2.1__DOCS_WIKI-KAPITEL_FÜ.md (1002B) [38L]
│   │   ├── ISSUE_64_V3.2.1__DOCS_HUGGINGFACE_PIP.md (1002B) [37L]
│   │   ├── ISSUE_65_V3.2.1__REFACTOR_DOPPELTE_AT.md (1017B) [40L]
│   │   ├── ISSUE_66_V3.2.1__REFACTOR_AIKERNEL_DU.md (997B) [38L]
│   │   ├── ISSUE_67_V3.2.1__DOCKER_TESTNET_HEALT.md (1000B) [38L]
│   │   ├── ISSUE_68_54__BOOTSTRAP-NODE_IMPLEMENT.md (1KB) [35L]
│   │   ├── ISSUE_69_SPRINT_3.3_SECURITY-AUDIT__.md (1KB) [40L]
│   │   ├── ISSUE_70_SPRINT_4.0_VALIDATOR-NODES_.md (1KB) [40L]
│   │   ├── ISSUE_71_SPRINT_4.0_GENESIS_BLOCK__K.md (1KB) [38L]
│   │   ├── ISSUE_72_SPRINT_2.1_ATCLANG_LANGUAGE_.md (1KB) [40L]
│   │   ├── ISSUE_73_SPRINT_2.1_ATCLANG_VM_BYTECO.md (1KB) [40L]
│   │   ├── ISSUE_74_SPRINT_2.1_KONSENS-MODULE__.md (1KB) [39L]
│   │   ├── ISSUE_75_SPRINT_2.2_TESTNET_HEALTH-CH.md (1018B) [40L]
│   │   ├── ISSUE_76_SPRINT_2.3_SMART_CONTRACT_EN.md (1KB) [40L]
│   │   ├── ISSUE_77_SPRINT_2.4_EVENTBUS_VS_IPCBU.md (1KB) [40L]
│   │   ├── ISSUE_78_SPRINT_2.6_VOTING-POWER_SNAP.md (1KB) [39L]
│   │   ├── ISSUE_79_SPRINT_2.7_CI-CD_PIPELINE_RE.md (1KB) [43L]
│   │   ├── ISSUE_80_SPRINT_3.0_AIP-001_AGENT_INT.md (1KB) [40L]
│   │   ├── ISSUE_81_SPRINT_2.1_ATCLANG_STANDARD_.md (1KB) [40L]
│   │   ├── ISSUE_82_SPRINT_2.2_CORE_NODE_PROTOCO.md (1KB) [40L]
│   │   ├── ISSUE_83_SPRINT_2.2_INTER-NODE_LATENC.md (1KB) [40L]
│   │   ├── ISSUE_84_SPRINT_2.2_NETWORK-LEVEL_SHA.md (1KB) [40L]
│   │   ├── OPEN_ISSUES_MASTER.md (1KB) [44L]
│   │   ├── README.md (3KB) [62L]
│   │   └── TESTNET_INDEX.md (1KB) [25L]
│   ├── roadmap/ (1 files, 262 lines)
│   │   └── ROADMAP_EXTENDED.md (10KB) [262L]
│   ├── sprints/ (3 files, 241 lines)
│   │   ├── SPRINT_3.0_AI_AGENT_PROTOCOL.md (3KB) [76L]
│   │   ├── SPRINT_3.3_SECURITY_AUDIT.md (3KB) [83L]
│   │   └── SPRINT_4.0_MAINNET_LAUNCH.md (3KB) [82L]
│   ├── standards/ (106 files, 19181 lines)
│   │   ├── ATC/ (1 files, 55 lines)
│   │   │   └── ATC-0009-BRIDGE.md (1KB) [55L]
│   │   ├── ATC-01-CORE_NODE_PROTOCOL.md (8KB) [225L]
│   │   ├── ATC-02-LIQUID_STATE_MIGRATION.md (9KB) [246L]
│   │   ├── ATC-03-DECENTRALIZED_IDENTITY.md (10KB) [257L]
│   │   ├── ATC-04-DAG_CONSENSUS.md (7KB) [200L]
│   │   ├── ATC-05-QUANTUM_RESISTANT_SIGNATURES.md (8KB) [217L]
│   │   ├── ATC-06-LATENCY_OPTIMIZATION_ROUTING.md (22KB) [760L]
│   │   ├── ATC-07-SHARDING_STATE_PARTITIONING.md (9KB) [231L]
│   │   ├── ATC-08-EPHEMERAL_DATA_STREAMING.md (8KB) [205L]
│   │   ├── ATC-09-CROSS_CHAIN_BRIDGE.md (8KB) [209L]
│   │   ├── ATC-10-GLOBAL_TIME_SYNC_ORACLES.md (9KB) [234L]
│   │   ├── ATC-11-FUNGIBLE_ASSET_STANDARD.md (8KB) [210L]
│   │   ├── ATC-12-NON_FUNGIBLE_HOLOGRAPHIC.md (8KB) [204L]
│   │   ├── ATC-13-FRACTIONAL_OWNERSHIP.md (7KB) [201L]
│   │   ├── ATC-14-DETERMINISTIC_EXECUTION.md (8KB) [217L]
│   │   ├── ATC-15-PROOF_OF_AI_MINING.md (9KB) [229L]
│   │   ├── ATC-16-REFERRAL_REWARDS.md (8KB) [206L]
│   │   ├── ATC-17-DAO_GOVERNANCE.md (8KB) [224L]
│   │   ├── ATC-18-MULTISIG_AUTH.md (8KB) [224L]
│   │   ├── ATC-19-AMM_LOGIC.md (8KB) [212L]
│   │   ├── ATC-20-WRAPPED_SYNTHETIC.md (8KB) [226L]
│   │   ├── ATC-21-HOLOGRAPHIC_WASM.md (9KB) [248L]
│   │   ├── ATC-22-HAL_DRIVER_SANDBOX.md (8KB) [225L]
│   │   ├── ATC-23-DATA_SHARDING_STORAGE.md (8KB) [222L]
│   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   ├── ATC-25-TENSOR_COMPUTE.md (8KB) [218L]
│   │   ├── ATC-26-XAI_TRANSPARENCY.md (8KB) [224L]
│   │   ├── ATC-27-AI_MODEL_AUDITING.md (8KB) [226L]
│   │   ├── ATC-28-FEDERATED_LEARNING.md (9KB) [254L]
│   │   ├── ATC-29-AI_MARKETPLACE.md (9KB) [246L]
│   │   ├── ATC-30-REPUTATION_TRUST.md (10KB) [271L]
│   │   ├── ATC-31-TENSOR_LOAD_BALANCING.md (10KB) [266L]
│   │   ├── ATC-32-UX_INTERFACE_ABSTRACTION.md (10KB) [267L]
│   │   ├── ATC-33-AI_FEEDBACK_RLHF.md (11KB) [270L]
│   │   ├── ATC-34-CROSS_LAYER_INTEROP.md (11KB) [277L]
│   │   ├── ATC-35-DATA_PRIVACY_ANONYMIZATION.md (10KB) [263L]
│   │   ├── ATC-36-MEDIA_ASSET_PROVENANCE.md (9KB) [262L]
│   │   ├── ATC-37-REPUTATION_RESOURCE_ALLOCATION.md (10KB) [255L]
│   │   ├── ATC-38-CROSS_CHAIN_ASSET_BRIDGE.md (6KB) [142L]
│   │   ├── ATC-39-AI_MODEL_VERSIONING_DEPLOYMENT.md (6KB) [137L]
│   │   ├── ATC-40-SYSTEM_SELF_HEALING_AUTO_REMEDIATION.md (7KB) [155L]
│   │   ├── ATC-41-MULTI_AGENT_ORCHESTRATION_CONSENSUS.md (7KB) [155L]
│   │   ├── ATC-42-AI_GOVERNANCE_ETHICS_FRAMEWORK.md (7KB) [173L]
│   │   ├── ATC-43-GLOBAL_STATE_SYNC_CAUSAL_CONSISTENCY.md (7KB) [149L]
│   │   ├── ATC-44-HARDWARE_ACCELERATED_ZKP_GENERATION.md (3KB) [115L]
│   │   ├── ATC-45-AI_EVOLUTIONARY_LEARNING_Dael.md (4KB) [115L]
│   │   ├── ATC-46-QUANTUM_RESISTANT_CRYPTOGRAPHY_LAYER.md (3KB) [116L]
│   │   ├── ATC-47-AI_INTENT_SETTLEMENT_ARBITRAGE.md (3KB) [115L]
│   │   ├── ATC-48-NEURAL_NETWORK_MESH_CROSS_TOPOLOGY.md (4KB) [119L]
│   │   ├── ATC-49-NEURAL_SYNAPSE_INTER_MODEL_KNOWLEDGE_TRANSFER.md (3KB) [115L]
│   │   ├── ATC-50-AI_CONSCIOUSNESS_SELF_REFLECTION.md (4KB) [117L]
│   │   ├── ATC-51-CROSS_REALITY_SPATIAL_COMPUTING.md (4KB) [119L]
│   │   ├── ATC-52-BIO_DIGITAL_INTERFACE_NEURAL_SIGNAL.md (4KB) [118L]
│   │   ├── ATC-53-CONSCIOUSNESS_SENTIENCE_OBSERVABILITY.md (4KB) [118L]
│   │   ├── ATC-54-TEMPORAL_CAUSAL_CONVERGENCE.md (4KB) [119L]
│   │   ├── ATC-55-META_REALITY_SIMULATION_CONVERGENCE.md (4KB) [118L]
│   │   ├── ATC-56-INTERSTELLAR_DATA_INTEGRITY_RELATIVISTIC_SYNC.md (4KB) [119L]
│   │   ├── ATC-57-RECURSIVE_SELF_IMPROVEMENT_META_LEARNING.md (4KB) [127L]
│   │   ├── ATC-58-QUANTUM_NEURAL_ENTANGLEMENT.md (4KB) [126L]
│   │   ├── ATC-59-TRANSDIMENSIONAL_ENERGY_ENTROPY_MANAGEMENT.md (4KB) [126L]
│   │   ├── ATC-60-UNIVERSAL_HOLONIC_STRUCTURE.md (4KB) [126L]
│   │   ├── ATC-61-TRANS_REALITY_SEMANTIC_MAPPING.md (4KB) [127L]
│   │   ├── ATC-62-META_SYSTEMIC_ETHICS_EXISTENTIAL_RISK.md (4KB) [127L]
│   │   ├── ATC-63-TRANS_SPECIES_MULTI_BIOLOGICAL_INTEGRATION.md (4KB) [128L]
│   │   ├── ATC-64-TRANSDIMENSIONAL_RECURSIVE_KNOWLEDGE_SYNTHESIS.md (4KB) [128L]
│   │   ├── ATC-65-TRANS_METAVERSE_CONSENSUS_REALITY_SYNC.md (4KB) [119L]
│   │   ├── ATC-66-RECURSIVE_LOGIC_PROOF_OF_UNDERSTANDING.md (4KB) [119L]
│   │   ├── ATC-67-REALITY_CONSENSUS_OBSERVATION_COLLAPSE.md (3KB) [118L]
│   │   ├── ATC-68-EVOLUTIONARY_FEEDBACK_ONTOLOGICAL_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-69-TRANS_EXISTENCE_CONSCIOUSNESS_BRIDGE.md (4KB) [119L]
│   │   ├── ATC-70-QUANTUM_GLOBAL_TRUTH_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-71-TRANS_CAUSAL_REALITY_VOID_MAPPING.md (4KB) [117L]
│   │   ├── ATC-72-TRANS_RELATIONAL_GOVERNANCE_ENTITY_CONSENSUS.md (4KB) [119L]
│   │   ├── ATC-73-TRANS_METAVERSE_ENTROPY_HARVESTING.md (4KB) [119L]
│   │   ├── ATC-74-RECURSIVE_META_NARRATIVE_MYTHOS_CONSTRUCTION.md (3KB) [118L]
│   │   ├── ATC-75-PROVABLE_EPISTEMOLOGY_AUTO_WIKI.md (4KB) [119L]
│   │   ├── ATC-76-IMMUTABLE_HUMAN_HERITAGE_ETERNITY.md (4KB) [120L]
│   │   ├── ATC-77-TRANS_SEMANTIC_HUMAN_AI_OMNI_LINGUISTIC.md (4KB) [120L]
│   │   ├── ATC-78-ABSOLUTE_CONVERGENCE_MONOLITHIC_SINGULARITY.md (4KB) [119L]
│   │   ├── ATC-79-TRANS_REALITY_MANIFESTATION_PHYSICALITY_ANCHOR.md (4KB) [119L]
│   │   ├── ATC-80-TRANS_UNIVERSAL_REALITY_MIGRATION.md (4KB) [120L]
│   │   ├── ATC-81-PROOF_OF_HISTORY.md (2KB) [105L]
│   │   ├── ATC-82-PROOF_OF_WORK.md (2KB) [104L]
│   │   ├── ATC-83-PROOF_OF_STAKE.md (2KB) [106L]
│   │   ├── ATC-84-FORK_RESOLUTION.md (2KB) [103L]
│   │   ├── ATC-85-INITIAL_SYNC.md (2KB) [105L]
│   │   ├── ATC-86-ECDSA_SIGNATURE.md (2KB) [105L]
│   │   ├── ATC-87-GAS_FEE.md (2KB) [105L]
│   │   ├── ATC-88-AMM.md (2KB) [105L]
│   │   ├── ATC-89-FUNGIBLE_TOKEN.md (2KB) [106L]
│   │   ├── ATC-90-NFT_SHIVAMON.md (2KB) [106L]
│   │   ├── ATC-91-CROSS_CHAIN_BRIDGE.md (2KB) [105L]
│   │   ├── ATC-92-ATCLANG_LANGUAGE_SPEC.md (7KB) [221L]
│   │   ├── ATC-93-ATCLANG_VM_BYTECODE.md (10KB) [338L]
│   │   ├── ATC-94-ATCLANG_STDLIB.md (6KB) [187L]
│   │   ├── ATC-95-ATCLANG_TEST_FRAMEWORK.md (6KB) [221L]
│   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   ├── ATC-97-AGENT_INTERACTION_PROTOCOL.md (2KB) [83L]
│   │   ├── ATC-97_AGENT_INTERACTION_PROTOCOL.md (8KB) [243L]
│   │   ├── ATC-98-TESTING_STANDARD.md (1KB) [69L]
│   │   ├── ATC-99-ATCLANG_UNIVERSAL_MANDATE.md (7KB) [189L]
│   │   ├── ATC-LIC-SMART_CONTRACT_LICENSE.md (11KB) [297L]
│   │   ├── ATC_ECOSYSTEM_STANDARDS.md (54KB) [1169L]
│   │   ├── ATS-LIC-SYSTEM_HARDWARE_LICENSE.md (4KB) [117L]
│   │   ├── OVERVIEW.md (1KB) [40L]
│   │   └── STANDARDS_REGISTRY.md (13KB) [208L]
│   ├── whitepaper/ (4 files, 2544 lines)
│   │   ├── .github/ (1 files, 2 lines)
│   │   │   └── FUNDING.yml (76B) [2L]
│   │   ├── CHANGELOG.md (706B) [24L]
│   │   ├── README.md (2KB) [48L]
│   │   └── WHITEPAPER.md (80KB) [2470L]
│   ├── wiki/ (182 files, 30075 lines)
│   │   ├── atclang/ (13 files, 881 lines)
│   │   │   ├── docs/ (12 files, 837 lines)
│   │   │   │   ├── CHANGELOG.md (338B) [8L]
│   │   │   │   ├── COMPILER.md (3KB) [105L]
│   │   │   │   ├── CONTRIBUTING.md (472B) [11L]
│   │   │   │   ├── EXAMPLES.md (3KB) [95L]
│   │   │   │   ├── LEXER.md (1KB) [59L]
│   │   │   │   ├── PARSER.md (3KB) [135L]
│   │   │   │   ├── REPL.md (2KB) [79L]
│   │   │   │   ├── SECURITY.md (1KB) [34L]
│   │   │   │   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   │   │   │   ├── SPEC.md (1KB) [55L]
│   │   │   │   ├── STDLIB.md (3KB) [111L]
│   │   │   │   └── VM.md (2KB) [63L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── atcnet/ (6 files, 213 lines)
│   │   │   ├── docs/ (5 files, 169 lines)
│   │   │   │   ├── BOOTSTRAP.md (312B) [18L]
│   │   │   │   ├── MESSAGES.md (1KB) [40L]
│   │   │   │   ├── PROTOCOL.md (2KB) [57L]
│   │   │   │   ├── SECURITY.md (336B) [11L]
│   │   │   │   └── TOPOLOGY.md (1KB) [43L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── contracts/ (7 files, 296 lines)
│   │   │   ├── docs/ (6 files, 252 lines)
│   │   │   │   ├── ATC8300.md (1KB) [51L]
│   │   │   │   ├── ATC9000.md (2KB) [92L]
│   │   │   │   ├── ATC9900.md (514B) [20L]
│   │   │   │   ├── BRIDGE.md (1KB) [38L]
│   │   │   │   ├── DEPLOYMENT.md (603B) [25L]
│   │   │   │   └── SECURITY.md (708B) [26L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── franchise/ (8 files, 287 lines)
│   │   │   ├── docs/ (7 files, 243 lines)
│   │   │   │   ├── API.md (1KB) [37L]
│   │   │   │   ├── CONCEPT.md (1000B) [24L]
│   │   │   │   ├── CONTRACTS.md (1KB) [49L]
│   │   │   │   ├── DEPLOYMENT.md (879B) [43L]
│   │   │   │   ├── ROADMAP.md (726B) [20L]
│   │   │   │   ├── SECURITY.md (904B) [29L]
│   │   │   │   └── TOKEN_ECONOMY.md (1KB) [41L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── gateway/ (6 files, 189 lines)
│   │   │   ├── docs/ (5 files, 145 lines)
│   │   │   │   ├── AUTH.md (965B) [43L]
│   │   │   │   ├── MIDDLEWARE.md (368B) [14L]
│   │   │   │   ├── RATE_LIMITING.md (956B) [43L]
│   │   │   │   ├── ROUTES.md (995B) [32L]
│   │   │   │   └── SECURITY.md (372B) [13L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── kai-os/ (97 files, 25778 lines)
│   │   │   ├── code/ (71 files, 10240 lines)
│   │   │   │   ├── .github/ (4 files, 217 lines)
│   │   │   │   │   └── workflows/ (4 files, 217 lines)
│   │   │   │   │       ├── ci.yml (1KB) [42L]
│   │   │   │   │       ├── codeql.yml (4KB) [101L]
│   │   │   │   │       ├── docker.yml (884B) [39L]
│   │   │   │   │       └── pages.yml (717B) [35L]
│   │   │   │   ├── atc-ui/ (1 files, 0 lines)
│   │   │   │   │   └── index.html (92KB)
│   │   │   │   ├── atclang/ (6 files, 1728 lines)
│   │   │   │   │   ├── compiler/ (1 files, 471 lines)
│   │   │   │   │   │   └── compiler.py (17KB) [471L]
│   │   │   │   │   ├── lexer/ (1 files, 315 lines)
│   │   │   │   │   │   └── lexer.py (10KB) [315L]
│   │   │   │   │   ├── parser/ (1 files, 399 lines)
│   │   │   │   │   │   └── parser.py (15KB) [399L]
│   │   │   │   │   ├── repl/ (1 files, 185 lines)
│   │   │   │   │   │   └── repl.py (6KB) [185L]
│   │   │   │   │   ├── vm/ (1 files, 349 lines)
│   │   │   │   │   │   └── atcvm.py (11KB) [349L]
│   │   │   │   │   └── ATCLANG_SPEC.md (432B) [9L]
│   │   │   │   ├── backend/ (17 files, 1338 lines)
│   │   │   │   │   ├── api/ (11 files, 1005 lines)
│   │   │   │   │   │   ├── orchestrator/ (1 files, 69 lines)
│   │   │   │   │   │   │   └── orchestrator.py (2KB) [69L]
│   │   │   │   │   │   ├── routes/ (8 files, 508 lines)
│   │   │   │   │   │   │   ├── ai_routes.py (4KB) [123L]
│   │   │   │   │   │   │   ├── blockchain.py (2KB) [62L]
│   │   │   │   │   │   │   ├── game_routes.py (1KB) [59L]
│   │   │   │   │   │   │   ├── governance_routes.py (1KB) [63L]
│   │   │   │   │   │   │   ├── marketplace_routes.py (1KB) [69L]
│   │   │   │   │   │   │   ├── nodes_routes.py (1KB) [47L]
│   │   │   │   │   │   │   ├── orchestrator_routes.py (972B) [28L]
│   │   │   │   │   │   │   └── wallet.py (1KB) [57L]
│   │   │   │   │   │   ├── kai_routes.py (11KB) [381L]
│   │   │   │   │   │   └── server.py (2KB) [47L]
│   │   │   │   │   ├── db/ (2 files, 196 lines)
│   │   │   │   │   │   ├── repository.py (6KB) [196L]
│   │   │   │   │   │   └── schema.sql (2KB)
│   │   │   │   │   ├── wallet/ (1 files, 118 lines)
│   │   │   │   │   │   └── wallet.py (5KB) [118L]
│   │   │   │   │   ├── .env.example (167B)
│   │   │   │   │   ├── main.py (526B) [19L]
│   │   │   │   │   └── requirements.txt (90B)
│   │   │   │   ├── blockchain/ (16 files, 2757 lines)
│   │   │   │   │   ├── atcoin/ (1 files, 139 lines)
│   │   │   │   │   │   └── atcoin.py (5KB) [139L]
│   │   │   │   │   ├── consensus/ (4 files, 285 lines)
│   │   │   │   │   │   ├── hybrid_consensus.py (3KB) [87L]
│   │   │   │   │   │   ├── poh.py (2KB) [67L]
│   │   │   │   │   │   ├── pos.py (2KB) [70L]
│   │   │   │   │   │   └── pow.py (2KB) [61L]
│   │   │   │   │   ├── contracts/ (4 files, 557 lines)
│   │   │   │   │   │   ├── atc001/ (1 files, 74 lines)
│   │   │   │   │   │   │   └── genesis_token.py (2KB) [74L]
│   │   │   │   │   │   ├── atc8300/ (1 files, 126 lines)
│   │   │   │   │   │   │   └── atc8300_token.py (5KB) [126L]
│   │   │   │   │   │   ├── base/ (1 files, 87 lines)
│   │   │   │   │   │   │   └── base_contract.py (3KB) [87L]
│   │   │   │   │   │   └── shivamon/ (1 files, 270 lines)
│   │   │   │   │   │       └── shivamon_contract.py (10KB) [270L]
│   │   │   │   │   ├── nodes/ (3 files, 795 lines)
│   │   │   │   │   │   ├── discovery.py (11KB) [314L]
│   │   │   │   │   │   ├── node.py (3KB) [100L]
│   │   │   │   │   │   └── p2p_propagation.py (12KB) [381L]
│   │   │   │   │   ├── wallet/ (2 files, 212 lines)
│   │   │   │   │   │   ├── ecdsa.py (2KB) [72L]
│   │   │   │   │   │   └── keygen.py (5KB) [140L]
│   │   │   │   │   ├── smart_contract_registry.py (1KB) [53L]
│   │   │   │   │   └── smart_contracts.py (23KB) [716L]
│   │   │   │   ├── config/ (2 files, 50 lines)
│   │   │   │   │   ├── kai_config.toml (1KB) [52L]
│   │   │   │   │   └── settings.json (922B) [50L]
│   │   │   │   ├── core/ (5 files, 761 lines)
│   │   │   │   │   ├── ai_kernel.py (15KB) [455L]
│   │   │   │   │   ├── event_bus.py (517B) [16L]
│   │   │   │   │   ├── kai_cli.py (9KB) [251L]
│   │   │   │   │   ├── kernel.py (736B) [22L]
│   │   │   │   │   └── module_loader.py (540B) [17L]
│   │   │   │   ├── frontend/ (5 files, 577 lines)
│   │   │   │   │   ├── assets/ (2 files, 136 lines)
│   │   │   │   │   │   ├── css/ (1 files, 0 lines)
│   │   │   │   │   │   │   └── variables.css (807B)
│   │   │   │   │   │   └── js/ (1 files, 136 lines)
│   │   │   │   │   │       └── api.js (4KB) [136L]
│   │   │   │   │   ├── bootscreen/ (1 files, 417 lines)
│   │   │   │   │   │   └── bootscreen_complete.py (15KB) [417L]
│   │   │   │   │   ├── README.md (616B) [24L]
│   │   │   │   │   └── index.html (120KB)
│   │   │   │   ├── gateway/ (8 files, 207 lines)
│   │   │   │   │   ├── middleware/ (4 files, 110 lines)
│   │   │   │   │   │   ├── auth.py (669B) [19L]
│   │   │   │   │   │   ├── logger.py (324B) [9L]
│   │   │   │   │   │   ├── rate_limit.py (894B) [25L]
│   │   │   │   │   │   └── signature_verify.py (1KB) [57L]
│   │   │   │   │   ├── .env.example (103B)
│   │   │   │   │   ├── main.py (1KB) [47L]
│   │   │   │   │   ├── requirements.txt (69B)
│   │   │   │   │   └── router.py (2KB) [50L]
│   │   │   │   ├── plugins/ (1 files, 14 lines)
│   │   │   │   │   └── wallet.py (446B) [14L]
│   │   │   │   ├── shivaos/ (4 files, 1841 lines)
│   │   │   │   │   ├── consensus/ (1 files, 641 lines)
│   │   │   │   │   │   └── shiva_consensus.py (24KB) [641L]
│   │   │   │   │   ├── fs/ (1 files, 331 lines)
│   │   │   │   │   │   └── atcfs.py (12KB) [331L]
│   │   │   │   │   ├── kernel/ (1 files, 382 lines)
│   │   │   │   │   │   └── kernel.py (14KB) [382L]
│   │   │   │   │   └── net/ (1 files, 487 lines)
│   │   │   │   │       └── atcnet.py (17KB) [487L]
│   │   │   │   └── tests/ (2 files, 750 lines)
│   │   │   │       ├── test_atclang.py (13KB) [457L] 🧪
│   │   │   │       └── test_kai_integration.py (8KB) [293L] 🧪
│   │   │   ├── docs/ (23 files, 15218 lines)
│   │   │   │   ├── architecture/ (4 files, 720 lines)
│   │   │   │   │   ├── ATCNET_P2P.md (6KB) [193L]
│   │   │   │   │   ├── CONSENSUS.md (6KB) [193L]
│   │   │   │   │   ├── GATEWAY.md (5KB) [168L]
│   │   │   │   │   └── WALLET_KEYGEN.md (5KB) [166L]
│   │   │   │   ├── contracts/ (1 files, 202 lines)
│   │   │   │   │   └── ATC_TOKEN_STANDARD.md (6KB) [202L]
│   │   │   │   ├── issues/ (7 files, 1305 lines)
│   │   │   │   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [141L]
│   │   │   │   │   ├── ISSUE_06_ECDSA.md (4KB) [141L]
│   │   │   │   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [97L]
│   │   │   │   │   ├── ISSUE_12_SOLIDITY.md (4KB) [145L]
│   │   │   │   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [120L]
│   │   │   │   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [308L]
│   │   │   │   │   └── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   │   │   ├── repo/ (1 files, 56 lines)
│   │   │   │   │   └── README.md (2KB) [56L]
│   │   │   │   ├── roadmap/ (1 files, 245 lines)
│   │   │   │   │   └── ROADMAP_EXTENDED.md (10KB) [245L]
│   │   │   │   ├── standards/ (3 files, 699 lines)
│   │   │   │   │   ├── ATC_ECOSYSTEM_STANDARDS.md (13KB) [447L]
│   │   │   │   │   ├── OVERVIEW.md (1KB) [40L]
│   │   │   │   │   └── STANDARDS_REGISTRY.md (10KB) [212L]
│   │   │   │   ├── DECISIONS_REGISTER.md (2KB) [69L]
│   │   │   │   ├── MIGRATION_MAP.md (1KB) [30L]
│   │   │   │   ├── ROADMAP.md (9KB) [208L]
│   │   │   │   ├── ROADMAP_COMPLETENESS_AUDIT.md (7KB) [223L]
│   │   │   │   ├── STATUS.md (3KB) [85L]
│   │   │   │   └── kai-os-wiki.md (395KB) [11376L]
│   │   │   ├── ECOSYSTEM.md (8KB) [179L]
│   │   │   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   │   │   └── README.md (542B) [18L]
│   │   ├── kernel/ (10 files, 494 lines)
│   │   │   ├── docs/ (9 files, 450 lines)
│   │   │   │   ├── ATCFS.md (2KB) [107L]
│   │   │   │   ├── ATCNET.md (2KB) [89L]
│   │   │   │   ├── CHANGELOG.md (231B) [7L]
│   │   │   │   ├── CONSENSUS.md (615B) [24L]
│   │   │   │   ├── IPC.md (1KB) [43L]
│   │   │   │   ├── KERNEL.md (2KB) [87L]
│   │   │   │   ├── PERFORMANCE.md (708B) [25L]
│   │   │   │   ├── PROCESS_MODEL.md (1KB) [48L]
│   │   │   │   └── SECURITY.md (532B) [20L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── overview/ (9 files, 400 lines)
│   │   │   ├── docs/ (8 files, 356 lines)
│   │   │   │   ├── API.md (1KB) [59L]
│   │   │   │   ├── ARCHITECTURE.md (1KB) [36L]
│   │   │   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   │   │   ├── FAQ.md (1KB) [62L]
│   │   │   │   ├── QUICKSTART.md (619B) [30L]
│   │   │   │   ├── ROADMAP.md (556B) [25L]
│   │   │   │   ├── SECURITY.md (916B) [18L]
│   │   │   │   └── WHITEPAPER.md (5KB) [107L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── shivamon/ (7 files, 229 lines)
│   │   │   ├── docs/ (6 files, 185 lines)
│   │   │   │   ├── BATTLE.md (420B) [17L]
│   │   │   │   ├── BREEDING.md (1KB) [37L]
│   │   │   │   ├── ELEMENTS.md (1KB) [31L]
│   │   │   │   ├── MARKETPLACE.md (408B) [21L]
│   │   │   │   ├── NFT_SPEC.md (1KB) [55L]
│   │   │   │   └── ROADMAP.md (638B) [24L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── standards/ (3 files, 305 lines)
│   │   │   ├── docs/ (2 files, 261 lines)
│   │   │   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   │   │   └── OVERVIEW.md (1KB) [28L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── ui/ (6 files, 240 lines)
│   │   │   ├── docs/ (5 files, 196 lines)
│   │   │   │   ├── API.md (651B) [30L]
│   │   │   │   ├── COMPONENTS.md (442B) [26L]
│   │   │   │   ├── DEPLOYMENT.md (969B) [49L]
│   │   │   │   ├── DESIGN.md (732B) [24L]
│   │   │   │   └── THEME.md (1KB) [67L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── chapter-63-cleanup-2026-06-13.md (6KB) [205L]
│   │   ├── chapter-70-atclang-migration-complete.md (3KB) [78L]
│   │   ├── chapter-71-sprint-audit.md (2KB) [67L]
│   │   ├── chapter-72-sprint-2-7-testing-cicd.md (2KB) [59L]
│   │   ├── chapter-73-sprint-2-8-testnet.md (1KB) [53L]
│   │   ├── chapter-74-sprint-3-1-ux-privacy.md (1KB) [40L]
│   │   ├── chapter-75-v01-v03-migration-plan.md (2KB) [74L]
│   │   ├── chapter-76-sprint-3-3-3-6-alpha-release.md (1KB) [40L]
│   │   ├── chapter-77-sprint-4-0-4-1-mainnet.md (1KB) [43L]
│   │   └── chapter-78-shivacore-kernel-712-tests.md (4KB) [104L]
│   ├── workflows/ (1 files, 218 lines)
│   │   └── wiki-sync.yml (8KB) [218L]
│   ├── AGENT_COORDINATION.md (30KB) [396L]
│   ├── AGENT_POLICY.md (13KB) [317L]
│   ├── ARCHITECTURE_TREES.md (104KB) [2082L]
│   ├── ATCLANG_AGENT_BUILD_GUIDE.md (22KB) [281L]
│   ├── AUDIT_REPORT.md (3KB) [89L]
│   ├── CLUSTER_ARCHITECTURE.md (6KB) [103L]
│   ├── COMPLETENESS_AUDIT.md (2KB) [57L]
│   ├── DECISIONS_REGISTER.md (7KB) [140L]
│   ├── DEPRECATED.md (1KB) [32L]
│   ├── ECOSYSTEM_BRAIN.md (4KB) [120L]
│   ├── FILE_NAMING_CONVENTIONS.md (16KB) [634L]
│   ├── FILE_REGISTER.md (4KB) [94L]
│   ├── FIXES.md (3KB) [96L]
│   ├── KAI_INTEGRATION.md (6KB) [242L]
│   ├── LICENSING_OVERVIEW.md (6KB) [157L]
│   ├── MIGRATION_MAP.md (1KB) [31L]
│   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   ├── REALITY_CHECK_2026-07-06.md (28KB) [428L]
│   ├── ROADMAP.md (4KB) [88L]
│   ├── ROADMAP_COMPLETENESS_AUDIT.md (184B) [9L]
│   ├── SHIVACORE_KERNEL_STATUS.md (38KB) [722L]
│   ├── STATUS.md (4KB) [75L]
│   ├── TODO.md (10KB) [246L]
│   ├── WIKI_AUDIT.md (6KB) [188L]
│   ├── api-reference.md (1KB) [33L]
│   ├── atclang-guide.md (1KB) [48L]
│   ├── genesis_wallet.md (3KB) [103L]
│   └── kai-os-wiki.md (554KB) [15928L]
├── gateway/ (2 files, 295 lines)
│   ├── main.atc (5KB) [127L]
│   └── service_discovery.atc (6KB) [168L]
├── integrations/ (5 files, 194 lines)
│   ├── README.md (1KB) [39L]
│   ├── calendar_tasks.md (4KB) [57L]
│   ├── huggingface_registry.md (1KB) [27L]
│   ├── notion_export.md (1KB) [25L]
│   └── storage_inventory.md (2KB) [46L]
├── mobile/ (4 files, 354 lines)
│   ├── wallet/ (2 files, 181 lines)
│   │   ├── __init__.py (162B) [2L]
│   │   └── biometric_auth.atc (5KB) [179L]
│   ├── __init__.py (123B) [2L]
│   └── wallet_api.atc (5KB) [171L]
├── module-docs/ (48 files, 2065 lines)
│   ├── atclang/ (3 files, 73 lines)
│   │   ├── CHANGELOG.md (316B) [8L]
│   │   ├── CONTRIBUTING.md (687B) [19L]
│   │   └── README.md (1KB) [46L]
│   ├── atcnet/ (4 files, 140 lines)
│   │   ├── CHANGELOG.md (294B) [8L]
│   │   ├── PROTOCOL.md (2KB) [84L]
│   │   ├── README.md (780B) [37L]
│   │   └── SECURITY.md (321B) [11L]
│   ├── atcpkg/ (1 files, 26 lines)
│   │   └── README.md (442B) [26L]
│   ├── backend/ (1 files, 41 lines)
│   │   └── README.md (1KB) [41L]
│   ├── blockchain/ (1 files, 41 lines)
│   │   └── README.md (1KB) [41L]
│   ├── config/ (1 files, 29 lines)
│   │   └── README.md (566B) [29L]
│   ├── contracts/ (4 files, 93 lines)
│   │   ├── CHANGELOG.md (293B) [8L]
│   │   ├── DEPLOYMENT.md (894B) [29L]
│   │   ├── README.md (1KB) [43L]
│   │   └── SECURITY.md (496B) [13L]
│   ├── core/ (1 files, 30 lines)
│   │   └── README.md (571B) [30L]
│   ├── docker/ (1 files, 32 lines)
│   │   └── README.md (695B) [32L]
│   ├── franchise/ (4 files, 122 lines)
│   │   ├── ARCHITECTURE.md (666B) [23L]
│   │   ├── CHANGELOG.md (256B) [7L]
│   │   ├── README.md (775B) [35L]
│   │   └── SECURITY.md (1KB) [57L]
│   ├── frontend/ (1 files, 47 lines)
│   │   └── README.md (1KB) [47L]
│   ├── gateway/ (3 files, 60 lines)
│   │   ├── CHANGELOG.md (274B) [8L]
│   │   ├── README.md (858B) [39L]
│   │   └── SECURITY.md (371B) [13L]
│   ├── kernel/ (5 files, 441 lines)
│   │   ├── ARCHITECTURE.md (2KB) [90L]
│   │   ├── ATS_STANDARDS.md (7KB) [283L]
│   │   ├── CHANGELOG.md (310B) [8L]
│   │   ├── README.md (1KB) [46L]
│   │   └── SECURITY.md (451B) [14L]
│   ├── mobile/ (1 files, 30 lines)
│   │   └── README.md (599B) [30L]
│   ├── modules/ (1 files, 57 lines)
│   │   └── README.md (1KB) [57L]
│   ├── monitoring/ (1 files, 30 lines)
│   │   └── README.md (641B) [30L]
│   ├── nginx/ (1 files, 26 lines)
│   │   └── README.md (437B) [26L]
│   ├── scripts/ (1 files, 28 lines)
│   │   └── README.md (534B) [28L]
│   ├── shivamon/ (3 files, 86 lines)
│   │   ├── CHANGELOG.md (272B) [8L]
│   │   ├── GAME_SPEC.md (1KB) [43L]
│   │   └── README.md (819B) [35L]
│   ├── shivaos/ (1 files, 31 lines)
│   │   └── README.md (620B) [31L]
│   ├── standards/ (4 files, 461 lines)
│   │   ├── ATC_STANDARDS.md (4KB) [201L]
│   │   ├── ATS_STANDARDS.md (4KB) [199L]
│   │   ├── OVERVIEW.md (1KB) [29L]
│   │   └── README.md (706B) [32L]
│   ├── tests/ (1 files, 41 lines)
│   │   └── README.md (1KB) [41L]
│   ├── tools/ (1 files, 29 lines)
│   │   └── README.md (579B) [29L]
│   └── ui/ (3 files, 71 lines)
│       ├── CHANGELOG.md (285B) [8L]
│       ├── DESIGN.md (1KB) [33L]
│       └── README.md (586B) [30L]
├── modules/ (119 files, 19082 lines)
│   ├── assets/ (16 files, 2042 lines)
│   │   ├── aaa_asset_core.atc (3KB) [87L]
│   │   ├── ai_assets.atc (4KB) [124L]
│   │   ├── animation.atc (4KB) [142L]
│   │   ├── asset_bundle.atc (3KB) [104L]
│   │   ├── cloud_assets.atc (5KB) [133L]
│   │   ├── encryption.atc (5KB) [149L]
│   │   ├── hot_reload.atc (4KB) [125L]
│   │   ├── memory_cleanup.atc (4KB) [122L]
│   │   ├── mod_system.atc (5KB) [144L]
│   │   ├── model3d.atc (5KB) [168L]
│   │   ├── priority_loading.atc (2KB) [80L]
│   │   ├── render_pipeline.atc (5KB) [159L]
│   │   ├── shader_system.atc (4KB) [143L]
│   │   ├── streaming.atc (3KB) [91L]
│   │   ├── telemetry.atc (4KB) [144L]
│   │   └── versioning.atc (4KB) [127L]
│   ├── atclang/ (1 files, 26 lines)
│   │   └── README.md (751B) [26L]
│   ├── atcnet/ (7 files, 955 lines)
│   │   ├── README.md (917B) [29L]
│   │   ├── bootstrap_client.atc (4KB) [134L]
│   │   ├── discovery.atc (4KB) [138L]
│   │   ├── gossip.atc (5KB) [171L]
│   │   ├── nat_traversal.atc (3KB) [109L]
│   │   ├── p2p_node.atc (4KB) [159L]
│   │   └── p2p_propagation.atc (6KB) [215L]
│   ├── civilization/ (11 files, 2214 lines)
│   │   ├── asset_genome_ad66.atc (5KB) [171L]
│   │   ├── civilization_engine_ad60.atc (5KB) [236L]
│   │   ├── ecosystem_ai_mesh_ad62.atc (7KB) [245L]
│   │   ├── evolution_engine_ad69.atc (7KB) [251L]
│   │   ├── experience_orchestrator_ad68.atc (6KB) [200L]
│   │   ├── gcp_core_ad70.atc (7KB) [169L]
│   │   ├── global_simulation_core_ad64.atc (6KB) [198L]
│   │   ├── identity_layer_ad65.atc (4KB) [190L]
│   │   ├── persistent_world_engine_ad61.atc (5KB) [199L]
│   │   ├── proc_universe_generator_ad63.atc (8KB) [204L]
│   │   └── production_pipeline_ad67.atc (6KB) [151L]
│   ├── contracts/ (10 files, 1510 lines)
│   │   ├── atc8300/ (1 files, 178 lines)
│   │   │   └── atc8300_token.atc (5KB) [178L]
│   │   ├── atcoin/ (1 files, 176 lines)
│   │   │   └── atcoin.atc (5KB) [176L]
│   │   ├── base/ (1 files, 69 lines)
│   │   │   └── base_contract.atc (2KB) [69L]
│   │   ├── bridge/ (1 files, 172 lines)
│   │   │   └── bridge_contract.atc (5KB) [172L]
│   │   ├── governance/ (1 files, 237 lines)
│   │   │   └── governance_contract.atc (7KB) [237L]
│   │   ├── marketplace/ (1 files, 236 lines)
│   │   │   └── marketplace_contract.atc (7KB) [236L]
│   │   ├── shivamon/ (1 files, 290 lines)
│   │   │   └── shivamon_contract.atc (9KB) [290L]
│   │   ├── wallet/ (2 files, 135 lines)
│   │   │   ├── ecdsa.atc (2KB) [60L]
│   │   │   └── keygen.atc (2KB) [75L]
│   │   └── README.md (446B) [17L]
│   ├── franchise/ (30 files, 4163 lines)
│   │   ├── contracts/ (3 files, 285 lines)
│   │   │   ├── registry.atc (4KB) [120L]
│   │   │   ├── revenue.atc (3KB) [93L]
│   │   │   └── token.atc (3KB) [72L]
│   │   ├── README.md (378B) [15L]
│   │   ├── ai_content_factory_ad28.atc (6KB) [194L]
│   │   ├── ai_director_factory_ad41.atc (4KB) [28L]
│   │   ├── analytics_factory_ad31.atc (7KB) [232L]
│   │   ├── asset_intelligence_factory_ad34.atc (7KB) [210L]
│   │   ├── blueprint_factory_ad32.atc (5KB) [165L]
│   │   ├── canon_engine_ad33.atc (5KB) [171L]
│   │   ├── character_factory_ad23.atc (8KB) [251L]
│   │   ├── commerce_factory_ad40.atc (4KB) [26L]
│   │   ├── community_factory_ad30.atc (7KB) [222L]
│   │   ├── creator_factory_ad38.atc (4KB) [24L]
│   │   ├── economy_factory_ad26.atc (6KB) [200L]
│   │   ├── factory.atc (5KB) [165L]
│   │   ├── gameplay_factory_ad35.atc (4KB) [126L]
│   │   ├── gff_core_ad20.atc (8KB) [224L]
│   │   ├── ip_factory_ad21.atc (4KB) [147L]
│   │   ├── lifecycle_manager_ad43.atc (5KB) [25L]
│   │   ├── liveops_factory_ad27.atc (6KB) [212L]
│   │   ├── lore_factory_ad24.atc (7KB) [209L]
│   │   ├── merchandise_factory_ad29.atc (5KB) [173L]
│   │   ├── multiplayer_factory_ad37.atc (3KB) [27L]
│   │   ├── narrative_factory_ad36.atc (8KB) [245L]
│   │   ├── publishing_factory_ad39.atc (4KB) [25L]
│   │   ├── quest_factory_ad25.atc (6KB) [207L]
│   │   ├── routes.atc (2KB) [90L]
│   │   ├── security_factory_ad42.atc (4KB) [30L]
│   │   └── world_factory_ad22.atc (6KB) [235L]
│   ├── gateway/ (9 files, 547 lines)
│   │   ├── middleware/ (5 files, 247 lines)
│   │   │   ├── __init__.py (120B) [2L]
│   │   │   ├── auth.atc (2KB) [82L]
│   │   │   ├── logger.atc (2KB) [70L]
│   │   │   ├── rate_limit.atc (1KB) [50L]
│   │   │   └── signature_verify.atc (1KB) [43L]
│   │   ├── README.md (404B) [22L]
│   │   ├── __init__.py (125B) [2L]
│   │   ├── main.atc (5KB) [180L]
│   │   └── router.atc (3KB) [96L]
│   ├── kernel/ (25 files, 5133 lines)
│   │   ├── ai_kernel/ (1 files, 228 lines)
│   │   │   └── ai_kernel.atc (8KB) [228L]
│   │   ├── consensus/ (2 files, 607 lines)
│   │   │   ├── poh_integration.atc (2KB) [78L]
│   │   │   └── shiva_consensus.atc (16KB) [529L]
│   │   ├── fs/ (1 files, 142 lines)
│   │   │   └── atcfs.atc (4KB) [142L]
│   │   ├── ipc/ (2 files, 106 lines)
│   │   │   ├── __init__.py (236B) [4L]
│   │   │   └── ipc_bus.atc (3KB) [102L]
│   │   ├── net/ (1 files, 135 lines)
│   │   │   └── atcnet.atc (4KB) [135L]
│   │   ├── pkg/ (1 files, 208 lines)
│   │   │   └── manager.atc (6KB) [208L]
│   │   ├── process/ (1 files, 161 lines)
│   │   │   └── process_mgr.atc (4KB) [161L]
│   │   ├── shell/ (1 files, 296 lines)
│   │   │   └── shell.atc (8KB) [296L]
│   │   ├── README.md (1008B) [32L]
│   │   ├── ai_bus_ad13.atc (9KB) [310L]
│   │   ├── asset_bus_ad08.atc (5KB) [188L]
│   │   ├── audio_bus_ad11.atc (5KB) [199L]
│   │   ├── command_bus_ad02.atc (4KB) [168L]
│   │   ├── gcl_core_ad00.atc (7KB) [269L]
│   │   ├── input_bus_ad12.atc (5KB) [184L]
│   │   ├── ipc_bus_atc.ad.atc (8KB) [266L]
│   │   ├── message_bus_ad03.atc (6KB) [240L]
│   │   ├── network_bus_ad05.atc (8KB) [307L]
│   │   ├── physics_bus_ad10.atc (7KB) [255L]
│   │   ├── plugin_bus_ad06.atc (8KB) [286L]
│   │   ├── query_bus_ad07.atc (3KB) [128L]
│   │   ├── render_bus_ad09.atc (5KB) [164L]
│   │   └── telemetry_bus_ad14.atc (7KB) [254L]
│   ├── meta/ (8 files, 2320 lines)
│   │   ├── ai_studio_ad49.atc (11KB) [310L]
│   │   ├── cross_franchise_ad46.atc (8KB) [223L]
│   │   ├── data_lake_ad51.atc (9KB) [237L]
│   │   ├── digital_twin_ad50.atc (11KB) [303L]
│   │   ├── ip_evolution_ad45.atc (9KB) [241L]
│   │   ├── knowledge_graph_ad47.atc (11KB) [289L]
│   │   ├── simulation_factory_ad48.atc (13KB) [374L]
│   │   └── universe_factory_ad44.atc (13KB) [343L]
│   └── shivamon/ (2 files, 172 lines)
│       ├── engine/ (1 files, 153 lines)
│       │   └── battle_engine.atc (5KB) [153L]
│       └── README.md (505B) [19L]
├── monitoring/ (3 files, 612 lines)
│   ├── health_checks_atc08.atc (5KB) [197L]
│   ├── monitor.atc (6KB) [213L]
│   └── prometheus_metrics.atc (6KB) [202L]
├── reports/ (1 files, 102 lines)
│   └── SPRINT_2.3_2.4_2.7_REPORT.md (3KB) [102L]
├── scripts/ (1 files, 135 lines)
│   └── generate_validators.atc (4KB) [135L]
├── shivaos/ (3 files, 430 lines)
│   ├── fs/ (1 files, 126 lines)
│   │   └── atcfs_module.atc (4KB) [126L]
│   ├── kernel/ (1 files, 118 lines)
│   │   └── syscalls.atc (3KB) [118L]
│   └── ui/ (1 files, 186 lines)
│       └── renderer.atc (5KB) [186L]
├── tests/ (26 files, 4558 lines)
│   ├── unit/ (3 files, 654 lines)
│   │   ├── test_atclang.py (14KB) [462L] 🧪
│   │   ├── test_atcnet.py (1KB) [41L] 🧪
│   │   └── test_p2p_propagation.py (4KB) [151L] 🧪
│   ├── test_atclang.py (14KB) [470L] 🧪
│   ├── test_atclang_v03.py (2KB) [68L] 🧪
│   ├── test_bootstrap.py (10KB) [268L] 🧪
│   ├── test_did.py (1KB) [61L] 🧪
│   ├── test_discovery.py (4KB) [155L] 🧪
│   ├── test_ecdsa.py (2KB) [65L] 🧪
│   ├── test_fork_resolution.py (3KB) [101L] 🧪
│   ├── test_gateway.py (7KB) [201L] 🧪
│   ├── test_gateway_full.py (2KB) [76L] 🧪
│   ├── test_integration_atcfs_multisig.py (4KB) [129L] 🧪
│   ├── test_kai_integration.py (8KB) [297L] 🧪
│   ├── test_multinode_consensus.py (5KB) [155L] 🧪
│   ├── test_multinode_fivenode.py (3KB) [84L] 🧪
│   ├── test_node_failure_recovery.py (4KB) [143L] 🧪
│   ├── test_optimizer.py (9KB) [256L] 🧪
│   ├── test_orchestrator.py (1KB) [52L] 🧪
│   ├── test_p2p_propagation.py (5KB) [205L] 🧪
│   ├── test_persistence.py (2KB) [87L] 🧪
│   ├── test_poh.py (1KB) [63L] 🧪
│   ├── test_smart_contracts.py (3KB) [114L] 🧪
│   ├── test_stdlib.py (10KB) [298L] 🧪
│   ├── test_stdlib_dispatch.py (11KB) [312L] 🧪
│   └── test_type_checker.py (7KB) [244L] 🧪
├── tools/ (4 files, 623 lines)
│   ├── atc_issues_summary.atc (6KB) [212L]
│   ├── bigquery_pipeline.atc (4KB) [135L]
│   ├── ecdsa_impl.atc (4KB) [119L]
│   └── hf_review_pipeline.atc (5KB) [157L]
├── wiki/ (1059 files, 137636 lines)
│   ├── aistudio-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (323B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (311B) [10L]
│   │   ├── README.md (513B) [18L]
│   │   └── STATUS.md (298B) [15L]
│   ├── atclang/ (18 files, 1146 lines)
│   │   ├── docs/ (14 files, 1021 lines)
│   │   │   ├── CHANGELOG.md (338B) [8L]
│   │   │   ├── COMPILER.md (3KB) [105L]
│   │   │   ├── CONTRIBUTING.md (472B) [11L]
│   │   │   ├── EXAMPLES.md (3KB) [95L]
│   │   │   ├── LEXER.md (1KB) [59L]
│   │   │   ├── PARSER.md (3KB) [135L]
│   │   │   ├── REPL.md (2KB) [79L]
│   │   │   ├── ROADMAP.md (715B) [25L]
│   │   │   ├── SECURITY.md (1KB) [34L]
│   │   │   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   │   │   ├── SPEC.md (1KB) [55L]
│   │   │   ├── STDLIB.md (3KB) [111L]
│   │   │   ├── SYNTAX_FULL.md (6KB) [159L]
│   │   │   └── VM.md (2KB) [63L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [60L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── atclang-wiki/ (10 files, 250 lines)
│   │   ├── docs/ (3 files, 134 lines)
│   │   │   ├── ARCHITECTURE.md (2KB) [69L]
│   │   │   ├── MODULES.md (1KB) [43L]
│   │   │   └── ROADMAP.md (933B) [22L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (316B) [14L]
│   │   ├── FILE_REGISTER.md (512B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (309B) [10L]
│   │   ├── README.md (3KB) [61L]
│   │   └── STATUS.md (296B) [15L]
│   ├── atcnet/ (10 files, 294 lines)
│   │   ├── docs/ (6 files, 184 lines)
│   │   │   ├── BOOTSTRAP.md (312B) [18L]
│   │   │   ├── MESSAGES.md (1KB) [40L]
│   │   │   ├── PROTOCOL.md (2KB) [57L]
│   │   │   ├── ROADMAP.md (368B) [15L]
│   │   │   ├── SECURITY.md (336B) [11L]
│   │   │   └── TOPOLOGY.md (1KB) [43L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [45L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── atcpkg-wiki/ (9 files, 158 lines)
│   │   ├── docs/ (2 files, 44 lines)
│   │   │   ├── ARCHITECTURE.md (1KB) [28L]
│   │   │   └── ROADMAP.md (568B) [16L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (321B) [14L]
│   │   ├── FILE_REGISTER.md (477B) [15L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (307B) [10L]
│   │   ├── README.md (3KB) [60L]
│   │   └── STATUS.md (294B) [15L]
│   ├── backend-wiki/ (10 files, 237 lines)
│   │   ├── docs/ (3 files, 112 lines)
│   │   │   ├── API.md (941B) [61L]
│   │   │   ├── ARCHITECTURE.md (1KB) [35L]
│   │   │   └── ROADMAP.md (453B) [16L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (316B) [14L]
│   │   ├── FILE_REGISTER.md (495B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (309B) [10L]
│   │   ├── README.md (4KB) [70L]
│   │   └── STATUS.md (296B) [15L]
│   ├── blockchain-wiki/ (9 files, 390 lines)
│   │   ├── docs/ (5 files, 207 lines)
│   │   │   ├── ARCHITECTURE.md (2KB) [61L]
│   │   │   ├── CONSENSUS.md (1KB) [45L]
│   │   │   ├── MEMPOOL.md (1KB) [35L]
│   │   │   ├── ROADMAP.md (1KB) [30L]
│   │   │   └── VALIDATORS.md (1KB) [36L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (3KB) [109L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [74L]
│   ├── bootloader-wiki/ (9 files, 152 lines)
│   │   ├── docs/ (2 files, 30 lines)
│   │   │   ├── ARCHITECTURE.md (892B) [22L]
│   │   │   └── ROADMAP.md (218B) [8L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (316B) [14L]
│   │   ├── FILE_REGISTER.md (467B) [15L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (315B) [10L]
│   │   ├── README.md (3KB) [68L]
│   │   └── STATUS.md (302B) [15L]
│   ├── ci-wiki/ (9 files, 152 lines)
│   │   ├── docs/ (2 files, 30 lines)
│   │   │   ├── ROADMAP.md (176B) [7L]
│   │   │   └── WORKFLOWS.md (668B) [23L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (304B) [14L]
│   │   ├── FILE_REGISTER.md (456B) [15L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (299B) [10L]
│   │   ├── README.md (3KB) [68L]
│   │   └── STATUS.md (286B) [15L]
│   ├── cli-wiki/ (9 files, 154 lines)
│   │   ├── docs/ (2 files, 32 lines)
│   │   │   ├── COMMANDS.md (746B) [25L]
│   │   │   └── ROADMAP.md (172B) [7L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (300B) [14L]
│   │   ├── FILE_REGISTER.md (456B) [15L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (301B) [10L]
│   │   ├── README.md (3KB) [68L]
│   │   └── STATUS.md (288B) [15L]
│   ├── contracts/ (12 files, 406 lines)
│   │   ├── docs/ (8 files, 290 lines)
│   │   │   ├── ATC8300.md (1KB) [51L]
│   │   │   ├── ATC9000.md (2KB) [92L]
│   │   │   ├── ATC9900.md (514B) [20L]
│   │   │   ├── BRIDGE.md (1KB) [38L]
│   │   │   ├── DEPLOYMENT.md (603B) [25L]
│   │   │   ├── ROADMAP.md (455B) [17L]
│   │   │   ├── SECURITY.md (708B) [26L]
│   │   │   └── TODO.md (526B) [21L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [51L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── dns-wiki/ (9 files, 150 lines)
│   │   ├── docs/ (2 files, 28 lines)
│   │   │   ├── ARCHITECTURE.md (658B) [21L]
│   │   │   └── ROADMAP.md (184B) [7L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (309B) [14L]
│   │   ├── FILE_REGISTER.md (460B) [15L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (301B) [10L]
│   │   ├── README.md (3KB) [68L]
│   │   └── STATUS.md (288B) [15L]
│   ├── drivers-wiki/ (10 files, 154 lines)
│   │   ├── docs/ (3 files, 30 lines)
│   │   │   ├── ARCHITECTURE.md (373B) [11L]
│   │   │   ├── DRIVER_LIST.md (451B) [11L]
│   │   │   └── ROADMAP.md (194B) [8L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (323B) [14L]
│   │   ├── FILE_REGISTER.md (502B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (309B) [10L]
│   │   ├── README.md (3KB) [69L]
│   │   └── STATUS.md (296B) [15L]
│   ├── explorer-wiki/ (7 files, 121 lines)
│   │   ├── docs/ (3 files, 32 lines)
│   │   │   ├── API.md (330B) [9L]
│   │   │   ├── ARCHITECTURE.md (296B) [9L]
│   │   │   └── ROADMAP.md (406B) [14L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (523B) [18L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [71L]
│   ├── franchise/ (11 files, 352 lines)
│   │   ├── docs/ (7 files, 244 lines)
│   │   │   ├── API.md (1KB) [37L]
│   │   │   ├── CONCEPT.md (1000B) [24L]
│   │   │   ├── CONTRACTS.md (1KB) [49L]
│   │   │   ├── DEPLOYMENT.md (879B) [43L]
│   │   │   ├── ROADMAP.md (685B) [21L]
│   │   │   ├── SECURITY.md (904B) [29L]
│   │   │   └── TOKEN_ECONOMY.md (1KB) [41L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [43L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── franchise-factory/ (4 files, 39 lines)
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (342B) [11L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (1KB) [28L]
│   ├── frontend-wiki/ (7 files, 130 lines)
│   │   ├── docs/ (3 files, 41 lines)
│   │   │   ├── ARCHITECTURE.md (733B) [18L]
│   │   │   ├── COMPONENTS.md (463B) [8L]
│   │   │   └── ROADMAP.md (506B) [15L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (531B) [18L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [71L]
│   ├── gateway/ (10 files, 297 lines)
│   │   ├── docs/ (6 files, 161 lines)
│   │   │   ├── AUTH.md (965B) [43L]
│   │   │   ├── MIDDLEWARE.md (368B) [14L]
│   │   │   ├── RATE_LIMITING.md (956B) [43L]
│   │   │   ├── ROADMAP.md (436B) [16L]
│   │   │   ├── ROUTES.md (995B) [32L]
│   │   │   └── SECURITY.md (372B) [13L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (2KB) [71L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── genesis-engine-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (328B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (323B) [10L]
│   │   ├── README.md (530B) [18L]
│   │   └── STATUS.md (310B) [15L]
│   ├── ide-wiki/ (7 files, 127 lines)
│   │   ├── docs/ (3 files, 38 lines)
│   │   │   ├── ARCHITECTURE.md (356B) [16L]
│   │   │   ├── LSP.md (406B) [10L]
│   │   │   └── ROADMAP.md (385B) [12L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (510B) [18L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [71L]
│   ├── kai-os/ (738 files, 126109 lines)
│   │   ├── .github/ (1 files, 0 lines)
│   │   │   └── .gitkeep (14B)
│   │   ├── aistudio/ (8 files, 2462 lines)
│   │   │   ├── src/ (3 files, 709 lines)
│   │   │   │   ├── components/ (1 files, 196 lines)
│   │   │   │   │   └── RoadmapView.tsx (6KB) [196L]
│   │   │   │   ├── atcLangRoadmapData.ts (6KB) [201L]
│   │   │   │   └── roadmapData.ts (7KB) [312L]
│   │   │   ├── AGENTS.md (535B) [13L]
│   │   │   ├── GEMINI.md (373B) [6L]
│   │   │   ├── README.md (542B) [20L]
│   │   │   ├── ROADMAP.md (8KB) [598L]
│   │   │   └── SOFTWARE_ROADMAP.md (38KB) [1116L]
│   │   ├── archive/ (1 files, 97 lines)
│   │   │   └── ATCLANG_ARCHIVE.md (4KB) [97L]
│   │   ├── atclang/ (32 files, 8174 lines)
│   │   │   ├── compiler/ (4 files, 1634 lines)
│   │   │   │   ├── __init__.py (468B) [8L]
│   │   │   │   ├── compiler.py (21KB) [561L]
│   │   │   │   ├── optimizer.py (22KB) [558L]
│   │   │   │   └── type_checker.py (20KB) [507L]
│   │   │   ├── lexer/ (2 files, 574 lines)
│   │   │   │   ├── __init__.py (161B) [2L]
│   │   │   │   └── lexer.py (20KB) [572L]
│   │   │   ├── parser/ (3 files, 1224 lines)
│   │   │   │   ├── __init__.py (189B) [3L]
│   │   │   │   ├── ast_nodes.py (7KB) [331L]
│   │   │   │   └── parser.py (37KB) [890L]
│   │   │   ├── programs/ (1 files, 1161 lines)
│   │   │   │   └── atcos_main.atc (40KB) [1161L]
│   │   │   ├── repl/ (2 files, 185 lines)
│   │   │   │   ├── __init__.py (99B) [1L]
│   │   │   │   └── repl.py (6KB) [184L]
│   │   │   ├── stdlib/ (14 files, 1807 lines)
│   │   │   │   ├── __init__.py (1KB) [32L]
│   │   │   │   ├── atc_stdlib.py (2KB) [69L]
│   │   │   │   ├── chain.py (1KB) [41L]
│   │   │   │   ├── collections.py (5KB) [219L]
│   │   │   │   ├── collections_ext.py (3KB) [143L]
│   │   │   │   ├── crypto.py (5KB) [155L]
│   │   │   │   ├── crypto_ext.py (5KB) [149L]
│   │   │   │   ├── encoding.py (7KB) [210L]
│   │   │   │   ├── io.py (3KB) [107L]
│   │   │   │   ├── io_ext.py (3KB) [123L]
│   │   │   │   ├── math.py (3KB) [138L]
│   │   │   │   ├── primitives.py (7KB) [244L]
│   │   │   │   ├── string.py (2KB) [99L]
│   │   │   │   └── wallet.py (2KB) [78L]
│   │   │   ├── v03/ (2 files, 303 lines)
│   │   │   │   ├── __init__.py (124B) [2L]
│   │   │   │   └── atclang_v03_features.py (10KB) [301L]
│   │   │   ├── vm/ (2 files, 980 lines)
│   │   │   │   ├── __init__.py (177B) [2L]
│   │   │   │   └── atcvm.py (47KB) [978L]
│   │   │   ├── ATCLANG_SPEC.md (9KB) [295L]
│   │   │   └── __init__.py (462B) [11L]
│   │   ├── atcpkg/ (1 files, 145 lines)
│   │   │   └── manager.atc (4KB) [145L]
│   │   ├── backend/ (14 files, 1467 lines)
│   │   │   ├── api/ (8 files, 969 lines)
│   │   │   │   ├── orchestrator/ (2 files, 261 lines)
│   │   │   │   │   ├── __init__.py (118B) [2L]
│   │   │   │   │   └── orchestrator.atc (8KB) [259L]
│   │   │   │   ├── routes/ (3 files, 409 lines)
│   │   │   │   │   ├── __init__.py (115B) [2L]
│   │   │   │   │   ├── ai_routes.atc (5KB) [175L]
│   │   │   │   │   └── api_routes.atc (8KB) [232L]
│   │   │   │   ├── __init__.py (111B) [2L]
│   │   │   │   ├── kai_routes.atc (7KB) [229L]
│   │   │   │   └── server.atc (2KB) [68L]
│   │   │   ├── db/ (3 files, 355 lines)
│   │   │   │   ├── __init__.py (160B) [2L]
│   │   │   │   ├── connection.atc (4KB) [125L]
│   │   │   │   └── repository.atc (6KB) [228L]
│   │   │   ├── wallet/ (2 files, 141 lines)
│   │   │   │   ├── __init__.py (123B) [2L]
│   │   │   │   └── wallet.atc (4KB) [139L]
│   │   │   └── __init__.py (121B) [2L]
│   │   ├── blockchain/ (49 files, 6353 lines)
│   │   │   ├── atcoin/ (1 files, 2 lines)
│   │   │   │   └── __init__.py (119B) [2L]
│   │   │   ├── consensus/ (13 files, 1548 lines)
│   │   │   │   ├── __init__.py (123B) [2L]
│   │   │   │   ├── fork_atc85.atc (2KB) [74L]
│   │   │   │   ├── fork_resolution.atc (4KB) [145L]
│   │   │   │   ├── gas_fee.atc (4KB) [130L]
│   │   │   │   ├── gas_fee_atc86.atc (2KB) [71L]
│   │   │   │   ├── hybrid_atc84.atc (3KB) [98L]
│   │   │   │   ├── hybrid_consensus.atc (11KB) [357L]
│   │   │   │   ├── poh.atc (4KB) [140L]
│   │   │   │   ├── poh_atc83.atc (1KB) [79L]
│   │   │   │   ├── pos.atc (4KB) [164L]
│   │   │   │   ├── pos_atc82.atc (2KB) [92L]
│   │   │   │   ├── pow.atc (3KB) [107L]
│   │   │   │   └── pow_atc81.atc (2KB) [89L]
│   │   │   ├── contracts/ (6 files, 756 lines)
│   │   │   │   ├── atc001/ (1 files, 102 lines)
│   │   │   │   │   └── genesis_token.atc (2KB) [102L]
│   │   │   │   ├── atc8300/ (1 files, 2 lines)
│   │   │   │   │   └── __init__.py (129B) [2L]
│   │   │   │   ├── governance/ (1 files, 202 lines)
│   │   │   │   │   └── governance_contract.atc (7KB) [202L]
│   │   │   │   ├── shivamon/ (2 files, 141 lines)
│   │   │   │   │   ├── __init__.py (136B) [2L]
│   │   │   │   │   └── breeding.atc (5KB) [139L]
│   │   │   │   └── contract_engine_atc14.atc (9KB) [309L]
│   │   │   ├── dex/ (2 files, 279 lines)
│   │   │   │   ├── __init__.py (117B) [2L]
│   │   │   │   └── amm.atc (10KB) [277L]
│   │   │   ├── governance/ (5 files, 775 lines)
│   │   │   │   ├── __init__.py (120B) [2L]
│   │   │   │   ├── dao.atc (6KB) [168L]
│   │   │   │   ├── dao_live.atc (8KB) [235L]
│   │   │   │   ├── timelock.atc (4KB) [150L]
│   │   │   │   └── treasury.atc (6KB) [220L]
│   │   │   ├── mainnet/ (3 files, 258 lines)
│   │   │   │   ├── __init__.py (117B) [2L]
│   │   │   │   ├── launch_manager.atc (3KB) [105L]
│   │   │   │   └── mainnet_config.atc (5KB) [151L]
│   │   │   ├── network/ (3 files, 514 lines)
│   │   │   │   ├── core_node_atc01.atc (4KB) [164L]
│   │   │   │   ├── latency_opt_atc06.atc (3KB) [135L]
│   │   │   │   └── sharding_atc07.atc (5KB) [215L]
│   │   │   ├── nodes/ (6 files, 854 lines)
│   │   │   │   ├── __init__.py (126B) [2L]
│   │   │   │   ├── block_propagation.atc (3KB) [87L]
│   │   │   │   ├── bootstrap.atc (6KB) [234L]
│   │   │   │   ├── initial_sync.atc (6KB) [207L]
│   │   │   │   ├── node.atc (6KB) [192L]
│   │   │   │   └── testnet_launcher.atc (4KB) [132L]
│   │   │   ├── propagation/ (1 files, 98 lines)
│   │   │   │   └── block_gossip.atc (3KB) [98L]
│   │   │   ├── wallet/ (4 files, 504 lines)
│   │   │   │   ├── __init__.py (128B) [2L]
│   │   │   │   ├── did.atc (4KB) [122L]
│   │   │   │   ├── multisig.atc (8KB) [268L]
│   │   │   │   └── wordlist.atc (5KB) [112L]
│   │   │   ├── zkp/ (2 files, 93 lines)
│   │   │   │   ├── __init__.py (336B) [4L]
│   │   │   │   └── groth16.atc (3KB) [89L]
│   │   │   ├── contract_registry.atc (3KB) [98L]
│   │   │   ├── smart_contract_registry.atc (2KB) [88L]
│   │   │   └── smart_contracts.atc (15KB) [486L]
│   │   ├── code/ (81 files, 11472 lines)
│   │   │   ├── .github/ (4 files, 217 lines)
│   │   │   │   └── workflows/ (4 files, 217 lines)
│   │   │   │       ├── ci.yml (1KB) [42L]
│   │   │   │       ├── codeql.yml (4KB) [101L]
│   │   │   │       ├── docker.yml (884B) [39L]
│   │   │   │       └── pages.yml (717B) [35L]
│   │   │   ├── atc-ui/ (1 files, 0 lines)
│   │   │   │   └── index.html (92KB)
│   │   │   ├── atclang/ (6 files, 1728 lines)
│   │   │   │   ├── compiler/ (1 files, 471 lines)
│   │   │   │   │   └── compiler.py (17KB) [471L]
│   │   │   │   ├── lexer/ (1 files, 315 lines)
│   │   │   │   │   └── lexer.py (10KB) [315L]
│   │   │   │   ├── parser/ (1 files, 399 lines)
│   │   │   │   │   └── parser.py (15KB) [399L]
│   │   │   │   ├── repl/ (1 files, 185 lines)
│   │   │   │   │   └── repl.py (6KB) [185L]
│   │   │   │   ├── vm/ (1 files, 349 lines)
│   │   │   │   │   └── atcvm.py (11KB) [349L]
│   │   │   │   └── ATCLANG_SPEC.md (423B) [9L]
│   │   │   ├── backend/ (17 files, 1338 lines)
│   │   │   │   ├── api/ (11 files, 1005 lines)
│   │   │   │   │   ├── orchestrator/ (1 files, 69 lines)
│   │   │   │   │   │   └── orchestrator.py (2KB) [69L]
│   │   │   │   │   ├── routes/ (8 files, 508 lines)
│   │   │   │   │   │   ├── ai_routes.py (4KB) [123L]
│   │   │   │   │   │   ├── blockchain.py (2KB) [62L]
│   │   │   │   │   │   ├── game_routes.py (1KB) [59L]
│   │   │   │   │   │   ├── governance_routes.py (1KB) [63L]
│   │   │   │   │   │   ├── marketplace_routes.py (1KB) [69L]
│   │   │   │   │   │   ├── nodes_routes.py (1KB) [47L]
│   │   │   │   │   │   ├── orchestrator_routes.py (972B) [28L]
│   │   │   │   │   │   └── wallet.py (1KB) [57L]
│   │   │   │   │   ├── kai_routes.py (11KB) [381L]
│   │   │   │   │   └── server.py (2KB) [47L]
│   │   │   │   ├── db/ (2 files, 196 lines)
│   │   │   │   │   ├── repository.py (6KB) [196L]
│   │   │   │   │   └── schema.sql (2KB)
│   │   │   │   ├── wallet/ (1 files, 118 lines)
│   │   │   │   │   └── wallet.py (5KB) [118L]
│   │   │   │   ├── .env.example (167B)
│   │   │   │   ├── main.py (526B) [19L]
│   │   │   │   └── requirements.txt (90B)
│   │   │   ├── blockchain/ (20 files, 3252 lines)
│   │   │   │   ├── atcoin/ (1 files, 139 lines)
│   │   │   │   │   └── atcoin.py (5KB) [139L]
│   │   │   │   ├── consensus/ (4 files, 285 lines)
│   │   │   │   │   ├── hybrid_consensus.py (3KB) [87L]
│   │   │   │   │   ├── poh.py (2KB) [67L]
│   │   │   │   │   ├── pos.py (2KB) [70L]
│   │   │   │   │   └── pow.py (2KB) [61L]
│   │   │   │   ├── contracts/ (8 files, 1052 lines)
│   │   │   │   │   ├── atc001/ (1 files, 74 lines)
│   │   │   │   │   │   └── genesis_token.py (2KB) [74L]
│   │   │   │   │   ├── atc8300/ (1 files, 126 lines)
│   │   │   │   │   │   └── atc8300_token.py (5KB) [126L]
│   │   │   │   │   ├── base/ (1 files, 87 lines)
│   │   │   │   │   │   └── base_contract.py (3KB) [87L]
│   │   │   │   │   ├── shivamon/ (1 files, 270 lines)
│   │   │   │   │   │   └── shivamon_contract.py (10KB) [270L]
│   │   │   │   │   └── solidity/ (4 files, 495 lines)
│   │   │   │   │       ├── scripts/ (1 files, 112 lines)
│   │   │   │   │       │   └── deploy.js (4KB) [112L]
│   │   │   │   │       ├── test/ (1 files, 254 lines)
│   │   │   │   │       │   └── ATCGovernance.test.js (8KB) [254L] 🧪
│   │   │   │   │       ├── ATCToken.sol (5KB)
│   │   │   │   │       └── README.md (2KB) [129L]
│   │   │   │   ├── nodes/ (3 files, 795 lines)
│   │   │   │   │   ├── discovery.py (11KB) [314L]
│   │   │   │   │   ├── node.py (3KB) [100L]
│   │   │   │   │   └── p2p_propagation.py (12KB) [381L]
│   │   │   │   ├── wallet/ (2 files, 212 lines)
│   │   │   │   │   ├── ecdsa.py (2KB) [72L]
│   │   │   │   │   └── keygen.py (5KB) [140L]
│   │   │   │   ├── smart_contract_registry.py (1KB) [53L]
│   │   │   │   └── smart_contracts.py (23KB) [716L]
│   │   │   ├── config/ (2 files, 50 lines)
│   │   │   │   ├── kai_config.toml (1KB) [52L]
│   │   │   │   └── settings.json (922B) [50L]
│   │   │   ├── core/ (5 files, 761 lines)
│   │   │   │   ├── ai_kernel.py (15KB) [455L]
│   │   │   │   ├── event_bus.py (517B) [16L]
│   │   │   │   ├── kai_cli.py (9KB) [251L]
│   │   │   │   ├── kernel.py (736B) [22L]
│   │   │   │   └── module_loader.py (540B) [17L]
│   │   │   ├── frontend/ (4 files, 160 lines)
│   │   │   │   ├── assets/ (2 files, 136 lines)
│   │   │   │   │   ├── css/ (1 files, 0 lines)
│   │   │   │   │   │   └── variables.css (807B)
│   │   │   │   │   └── js/ (1 files, 136 lines)
│   │   │   │   │       └── api.js (4KB) [136L]
│   │   │   │   ├── README.md (616B) [24L]
│   │   │   │   └── index.html (120KB)
│   │   │   ├── gateway/ (8 files, 207 lines)
│   │   │   │   ├── middleware/ (4 files, 110 lines)
│   │   │   │   │   ├── auth.py (669B) [19L]
│   │   │   │   │   ├── logger.py (324B) [9L]
│   │   │   │   │   ├── rate_limit.py (894B) [25L]
│   │   │   │   │   └── signature_verify.py (1KB) [57L]
│   │   │   │   ├── .env.example (103B)
│   │   │   │   ├── main.py (1KB) [47L]
│   │   │   │   ├── requirements.txt (69B)
│   │   │   │   └── router.py (2KB) [50L]
│   │   │   ├── plugins/ (1 files, 14 lines)
│   │   │   │   └── wallet.py (446B) [14L]
│   │   │   ├── shivaos/ (4 files, 1841 lines)
│   │   │   │   ├── consensus/ (1 files, 641 lines)
│   │   │   │   │   └── shiva_consensus.py (24KB) [641L]
│   │   │   │   ├── fs/ (1 files, 331 lines)
│   │   │   │   │   └── atcfs.py (12KB) [331L]
│   │   │   │   ├── kernel/ (1 files, 382 lines)
│   │   │   │   │   └── kernel.py (14KB) [382L]
│   │   │   │   └── net/ (1 files, 487 lines)
│   │   │   │       └── atcnet.py (17KB) [487L]
│   │   │   ├── tests/ (2 files, 750 lines)
│   │   │   │   ├── test_atclang.py (13KB) [457L] 🧪
│   │   │   │   └── test_kai_integration.py (8KB) [293L] 🧪
│   │   │   ├── KAI_OS_SUMMARY.py (6KB) [242L]
│   │   │   ├── atc_issues_summary.py (15KB) [265L]
│   │   │   ├── bootscreen_complete.py (15KB) [417L]
│   │   │   ├── ecdsa_final.py (2KB) [69L]
│   │   │   ├── ecdsa_impl.py (3KB) [82L]
│   │   │   ├── requirements-kai.txt (371B)
│   │   │   └── start.py (2KB) [79L]
│   │   ├── config/ (1 files, 95 lines)
│   │   │   └── mainnet_genesis.json (3KB) [95L]
│   │   ├── core/ (3 files, 392 lines)
│   │   │   ├── ai/ (1 files, 178 lines)
│   │   │   │   └── federated_learning.atc (6KB) [178L]
│   │   │   ├── crypto/ (1 files, 19 lines)
│   │   │   │   └── __init__.py (535B) [19L]
│   │   │   └── kai_cli.atc (8KB) [195L]
│   │   ├── devnet/ (1 files, 554 lines)
│   │   │   └── README.md (12KB) [554L]
│   │   ├── docs/ (349 files, 63617 lines)
│   │   │   ├── ai/ (3 files, 547 lines)
│   │   │   │   ├── AI_SAFETY.md (5KB) [184L]
│   │   │   │   ├── GEMINI_INTEGRATION.md (5KB) [214L]
│   │   │   │   └── LLM_ROUTER.md (4KB) [149L]
│   │   │   ├── aistudio/ (1 files, 439 lines)
│   │   │   │   └── AISTUDIO_COMPONENTS.md (24KB) [439L]
│   │   │   ├── architecture/ (12 files, 2003 lines)
│   │   │   │   ├── AI_LAYER.md (2KB) [53L]
│   │   │   │   ├── ATCFS.md (4KB) [129L]
│   │   │   │   ├── ATCLANG_COMPILER.md (2KB) [64L]
│   │   │   │   ├── ATCNET_P2P.md (6KB) [193L]
│   │   │   │   ├── CONSENSUS.md (6KB) [193L]
│   │   │   │   ├── GATEWAY.md (5KB) [168L]
│   │   │   │   ├── GOVERNANCE.md (1KB) [50L]
│   │   │   │   ├── KERNEL_SHELL.md (1KB) [50L]
│   │   │   │   ├── MONITORING_DEVOPS.md (1KB) [42L]
│   │   │   │   ├── SHIVAOS_KERNEL.md (5KB) [182L]
│   │   │   │   ├── TESTNET.md (20KB) [713L]
│   │   │   │   └── WALLET_KEYGEN.md (5KB) [166L]
│   │   │   ├── atclang/ (1 files, 9 lines)
│   │   │   │   └── ATCLANG_SPEC_FULL.md (423B) [9L]
│   │   │   ├── blockchain/ (2 files, 455 lines)
│   │   │   │   ├── ETHEREUM_INTEGRATION.md (6KB) [231L]
│   │   │   │   └── SOLANA_INTEGRATION.md (6KB) [224L]
│   │   │   ├── compliance/ (5 files, 1575 lines)
│   │   │   │   ├── ATVM_LICENSE_GATE_SPEC.md (7KB) [242L]
│   │   │   │   ├── BAFIN_KONFORMITAETSBERICHT.md (15KB) [408L]
│   │   │   │   ├── COMPLIANCE_HANDBUCH.md (5KB) [131L]
│   │   │   │   ├── IP_LICENSE_DASHBOARD_SPEC.md (6KB) [205L]
│   │   │   │   └── SMART_CONTRACT_RICHTLINIE.md (21KB) [589L]
│   │   │   ├── contracts/ (2 files, 980 lines)
│   │   │   │   ├── ATC_TOKEN_STANDARD.md (6KB) [202L]
│   │   │   │   └── SHIVAMON_NFT_CONTRACT.md (20KB) [778L]
│   │   │   ├── issues/ (85 files, 5229 lines)
│   │   │   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [141L]
│   │   │   │   ├── ISSUE_02_GEMINI_AI.md (3KB) [141L]
│   │   │   │   ├── ISSUE_03_BATTLE_UI.md (4KB) [141L]
│   │   │   │   ├── ISSUE_04_PERSISTENZ.md (4KB) [156L]
│   │   │   │   ├── ISSUE_05_EXPLORER.md (3KB) [102L]
│   │   │   │   ├── ISSUE_06_ECDSA.md (4KB) [141L]
│   │   │   │   ├── ISSUE_07_BUILD.md (3KB) [133L]
│   │   │   │   ├── ISSUE_08_TESTNET.md (3KB) [127L]
│   │   │   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [97L]
│   │   │   │   ├── ISSUE_10_BRIDGE.md (1KB) [53L]
│   │   │   │   ├── ISSUE_11_BREEDING.md (3KB) [88L]
│   │   │   │   ├── ISSUE_12_SOLIDITY.md (4KB) [145L]
│   │   │   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [120L]
│   │   │   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [308L]
│   │   │   │   ├── ISSUE_15__TESTNET_BLOCK_PROPAGATION_.md (1KB) [46L]
│   │   │   │   ├── ISSUE_16__TESTNET_INITIAL_SYNC__NEU.md (1KB) [45L]
│   │   │   │   ├── ISSUE_17__TESTNET_LONGEST-CHAIN-RULE.md (1KB) [45L]
│   │   │   │   ├── ISSUE_18__TESTNET_DOCKER_COMPOSE__5.md (1KB) [46L]
│   │   │   │   ├── ISSUE_19__TESTNET_NODE-MONITORING_DA.md (1KB) [45L]
│   │   │   │   ├── ISSUE_20_GATEWAY_TESTS.md (1KB) [63L]
│   │   │   │   ├── ISSUE_23__ATCFS__INTEGRATION_IN_KERN.md (1KB) [48L]
│   │   │   │   ├── ISSUE_24__MULTISIG_WALLET__BRIDGE__F.md (1KB) [47L]
│   │   │   │   ├── ISSUE_25__GATEWAY_4000__VOLLSTÄNDIGE.md (1KB) [48L]
│   │   │   │   ├── ISSUE_26__TESTS__ATCFS_MULTISIG_ATC.md (1KB) [50L]
│   │   │   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   │   │   ├── ISSUE_28__WIKI_KAP._40__SHIVAOS_UI_RE.md (1KB) [47L]
│   │   │   │   ├── ISSUE_29__WIKI_KAP._41__FEDERATED_LEA.md (1KB) [47L]
│   │   │   │   ├── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   │   │   │   ├── ISSUE_31__WIKI_KAP._4__BLOCK-EXPLORER.md (1KB) [45L]
│   │   │   │   ├── ISSUE_32__KAP._5__SHIVAOS_SYSTEM-CALL.md (1KB) [45L]
│   │   │   │   ├── ISSUE_33__KAP._4__GAS-FEE_MECHANISMUS.md (1KB) [45L]
│   │   │   │   ├── ISSUE_34_V3.0.0_15__SOLANA_BRIDGE_SP.md (1KB) [51L]
│   │   │   │   ├── ISSUE_35_V3.0.0_16__ATCLANG_V0.3.0_A.md (1KB) [49L]
│   │   │   │   ├── ISSUE_36_V3.0.0_17__MAINNET_LAUNCH_C.md (1KB) [52L]
│   │   │   │   ├── ISSUE_37_V3.0.0_20__DEX_-_AMM_LIQUID.md (1KB) [56L]
│   │   │   │   ├── ISSUE_38_V3.0.0_21__MOBILE_WALLET_IO.md (1KB) [51L]
│   │   │   │   ├── ISSUE_39_V3.0.0_22__DAO-GOVERNANCE_LI.md (1KB) [50L]
│   │   │   │   ├── ISSUE_40_DOCS_SYNTAX-REFERENZ__ATCLAN.md (1KB) [52L]
│   │   │   │   ├── ISSUE_41_DOCS_MATHEMATISCHE_BEWEISE__.md (1KB) [52L]
│   │   │   │   ├── ISSUE_42_DOCS_FEHLERDEFINITIONEN__BOT.md (1KB) [54L]
│   │   │   │   ├── ISSUE_43_DOCS_DEZENTRALER_NUTZER-NACHW.md (992B) [44L]
│   │   │   │   ├── ISSUE_44_MAINNET_MONITORING__GRAFANA_D.md (798B) [38L]
│   │   │   │   ├── ISSUE_45_ATCOIN_DEFI__AMM_LIQUIDITY_PO.md (738B) [38L]
│   │   │   │   ├── ISSUE_46_MOBILE_WALLET__BIOMETRIE__PU.md (770B) [38L]
│   │   │   │   ├── ISSUE_47_ZKP_ZERO-KNOWLEDGE_PROOFS__L0.md (814B) [38L]
│   │   │   │   ├── ISSUE_48_ATCLANG_V0.4.0__TYPE_SYSTEM_.md (823B) [38L]
│   │   │   │   ├── ISSUE_49_49__BIGQUERY_ANALYTICS_PIPEL.md (900B) [36L]
│   │   │   │   ├── ISSUE_50_50__HUGGING_FACE_CODE-REVIEW.md (881B) [36L]
│   │   │   │   ├── ISSUE_51_51__IPC_BUS_VOLLSTÄNDIGE_KE.md (880B) [36L]
│   │   │   │   ├── ISSUE_52_52__MAINNET_LAUNCH_MANAGER_.md (1009B) [36L]
│   │   │   │   ├── ISSUE_53_V3.2.1__TESTS_PROCESSMANAGER.md (1011B) [39L]
│   │   │   │   ├── ISSUE_54_V3.2.1__TESTS_ATCFS_FILESYST.md (1004B) [37L]
│   │   │   │   ├── ISSUE_55_V3.2.1__TESTS_ATCNET_P2PNODE.md (987B) [37L]
│   │   │   │   ├── ISSUE_56_V3.2.1__TESTS_ATCLANG_TYPECH.md (987B) [40L]
│   │   │   │   ├── ISSUE_57_V3.2.1__TESTS_PROMETHEUS_MET.md (998B) [38L]
│   │   │   │   ├── ISSUE_58_V3.2.1__TESTS_SERVICEDISCOVE.md (996B) [39L]
│   │   │   │   ├── ISSUE_59_V3.2.1__INTEGRATION_NATTRAVE.md (1005B) [36L]
│   │   │   │   ├── ISSUE_60_V3.2.1__INTEGRATION_AIKERNEL.md (997B) [37L]
│   │   │   │   ├── ISSUE_61_V3.2.1__INTEGRATION_BLOCKGOS.md (1015B) [37L]
│   │   │   │   ├── ISSUE_62_V3.2.1__INTEGRATION_SERVICED.md (1007B) [37L]
│   │   │   │   ├── ISSUE_63_V3.2.1__DOCS_WIKI-KAPITEL_FÜ.md (1002B) [38L]
│   │   │   │   ├── ISSUE_64_V3.2.1__DOCS_HUGGINGFACE_PIP.md (1002B) [37L]
│   │   │   │   ├── ISSUE_65_V3.2.1__REFACTOR_DOPPELTE_AT.md (1017B) [40L]
│   │   │   │   ├── ISSUE_66_V3.2.1__REFACTOR_AIKERNEL_DU.md (997B) [38L]
│   │   │   │   ├── ISSUE_67_V3.2.1__DOCKER_TESTNET_HEALT.md (1000B) [38L]
│   │   │   │   ├── ISSUE_68_54__BOOTSTRAP-NODE_IMPLEMENT.md (1KB) [35L]
│   │   │   │   ├── ISSUE_69_SPRINT_3.3_SECURITY-AUDIT__.md (1KB) [40L]
│   │   │   │   ├── ISSUE_70_SPRINT_4.0_VALIDATOR-NODES_.md (1KB) [40L]
│   │   │   │   ├── ISSUE_71_SPRINT_4.0_GENESIS_BLOCK__K.md (1KB) [38L]
│   │   │   │   ├── ISSUE_72_SPRINT_2.1_ATCLANG_LANGUAGE_.md (1KB) [40L]
│   │   │   │   ├── ISSUE_73_SPRINT_2.1_ATCLANG_VM_BYTECO.md (1KB) [40L]
│   │   │   │   ├── ISSUE_74_SPRINT_2.1_KONSENS-MODULE__.md (1KB) [39L]
│   │   │   │   ├── ISSUE_75_SPRINT_2.2_TESTNET_HEALTH-CH.md (1018B) [40L]
│   │   │   │   ├── ISSUE_76_SPRINT_2.3_SMART_CONTRACT_EN.md (1KB) [40L]
│   │   │   │   ├── ISSUE_77_SPRINT_2.4_EVENTBUS_VS_IPCBU.md (1KB) [40L]
│   │   │   │   ├── ISSUE_78_SPRINT_2.6_VOTING-POWER_SNAP.md (1KB) [39L]
│   │   │   │   ├── ISSUE_79_SPRINT_2.7_CI-CD_PIPELINE_RE.md (1KB) [43L]
│   │   │   │   ├── ISSUE_80_SPRINT_3.0_AIP-001_AGENT_INT.md (1KB) [40L]
│   │   │   │   ├── ISSUE_81_SPRINT_2.1_ATCLANG_STANDARD_.md (1KB) [40L]
│   │   │   │   ├── ISSUE_82_SPRINT_2.2_CORE_NODE_PROTOCO.md (1KB) [40L]
│   │   │   │   ├── ISSUE_83_SPRINT_2.2_INTER-NODE_LATENC.md (1KB) [40L]
│   │   │   │   ├── ISSUE_84_SPRINT_2.2_NETWORK-LEVEL_SHA.md (1KB) [40L]
│   │   │   │   ├── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   │   │   ├── README.md (3KB) [62L]
│   │   │   │   └── TESTNET_INDEX.md (1KB) [25L]
│   │   │   ├── repo/ (1 files, 56 lines)
│   │   │   │   └── README.md (2KB) [56L]
│   │   │   ├── roadmap/ (1 files, 245 lines)
│   │   │   │   └── ROADMAP_EXTENDED.md (10KB) [245L]
│   │   │   ├── sprints/ (3 files, 241 lines)
│   │   │   │   ├── SPRINT_3.0_AI_AGENT_PROTOCOL.md (3KB) [76L]
│   │   │   │   ├── SPRINT_3.3_SECURITY_AUDIT.md (3KB) [83L]
│   │   │   │   └── SPRINT_4.0_MAINNET_LAUNCH.md (3KB) [82L]
│   │   │   ├── standards/ (108 files, 18975 lines)
│   │   │   │   ├── ATC/ (1 files, 55 lines)
│   │   │   │   │   └── ATC-0009-BRIDGE.md (1KB) [55L]
│   │   │   │   ├── ATC-01-CORE_NODE_PROTOCOL.md (8KB) [225L]
│   │   │   │   ├── ATC-02-LIQUID_STATE_MIGRATION.md (9KB) [246L]
│   │   │   │   ├── ATC-03-DECENTRALIZED_IDENTITY.md (10KB) [257L]
│   │   │   │   ├── ATC-04-DAG_CONSENSUS.md (7KB) [200L]
│   │   │   │   ├── ATC-05-QUANTUM_RESISTANT_SIGNATURES.md (8KB) [217L]
│   │   │   │   ├── ATC-06-LATENCY_OPTIMIZATION_ROUTING.md (22KB) [760L]
│   │   │   │   ├── ATC-07-SHARDING_STATE_PARTITIONING.md (9KB) [231L]
│   │   │   │   ├── ATC-08-EPHEMERAL_DATA_STREAMING.md (8KB) [205L]
│   │   │   │   ├── ATC-09-CROSS_CHAIN_BRIDGE.md (8KB) [209L]
│   │   │   │   ├── ATC-10-GLOBAL_TIME_SYNC_ORACLES.md (9KB) [234L]
│   │   │   │   ├── ATC-11-FUNGIBLE_ASSET_STANDARD.md (8KB) [210L]
│   │   │   │   ├── ATC-12-NON_FUNGIBLE_HOLOGRAPHIC.md (8KB) [204L]
│   │   │   │   ├── ATC-13-FRACTIONAL_OWNERSHIP.md (7KB) [201L]
│   │   │   │   ├── ATC-14-DETERMINISTIC_EXECUTION.md (8KB) [217L]
│   │   │   │   ├── ATC-15-PROOF_OF_AI_MINING.md (9KB) [229L]
│   │   │   │   ├── ATC-16-REFERRAL_REWARDS.md (8KB) [206L]
│   │   │   │   ├── ATC-17-DAO_GOVERNANCE.md (8KB) [224L]
│   │   │   │   ├── ATC-18-MULTISIG_AUTH.md (8KB) [224L]
│   │   │   │   ├── ATC-19-AMM_LOGIC.md (8KB) [212L]
│   │   │   │   ├── ATC-20-WRAPPED_SYNTHETIC.md (8KB) [226L]
│   │   │   │   ├── ATC-21-HOLOGRAPHIC_WASM.md (9KB) [248L]
│   │   │   │   ├── ATC-22-HAL_DRIVER_SANDBOX.md (8KB) [225L]
│   │   │   │   ├── ATC-23-DATA_SHARDING_STORAGE.md (8KB) [222L]
│   │   │   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   │   │   ├── ATC-25-TENSOR_COMPUTE.md (8KB) [218L]
│   │   │   │   ├── ATC-26-XAI_TRANSPARENCY.md (8KB) [224L]
│   │   │   │   ├── ATC-27-AI_MODEL_AUDITING.md (8KB) [226L]
│   │   │   │   ├── ATC-28-FEDERATED_LEARNING.md (9KB) [254L]
│   │   │   │   ├── ATC-29-AI_MARKETPLACE.md (9KB) [246L]
│   │   │   │   ├── ATC-30-REPUTATION_TRUST.md (10KB) [271L]
│   │   │   │   ├── ATC-31-TENSOR_LOAD_BALANCING.md (10KB) [266L]
│   │   │   │   ├── ATC-32-UX_INTERFACE_ABSTRACTION.md (10KB) [267L]
│   │   │   │   ├── ATC-33-AI_FEEDBACK_RLHF.md (11KB) [270L]
│   │   │   │   ├── ATC-34-CROSS_LAYER_INTEROP.md (11KB) [277L]
│   │   │   │   ├── ATC-35-DATA_PRIVACY_ANONYMIZATION.md (10KB) [263L]
│   │   │   │   ├── ATC-36-MEDIA_ASSET_PROVENANCE.md (9KB) [262L]
│   │   │   │   ├── ATC-37-REPUTATION_RESOURCE_ALLOCATION.md (10KB) [255L]
│   │   │   │   ├── ATC-38-CROSS_CHAIN_ASSET_BRIDGE.md (6KB) [142L]
│   │   │   │   ├── ATC-39-AI_MODEL_VERSIONING_DEPLOYMENT.md (6KB) [137L]
│   │   │   │   ├── ATC-40-SYSTEM_SELF_HEALING_AUTO_REMEDIATION.md (7KB) [155L]
│   │   │   │   ├── ATC-41-MULTI_AGENT_ORCHESTRATION_CONSENSUS.md (7KB) [155L]
│   │   │   │   ├── ATC-42-AI_GOVERNANCE_ETHICS_FRAMEWORK.md (7KB) [173L]
│   │   │   │   ├── ATC-43-GLOBAL_STATE_SYNC_CAUSAL_CONSISTENCY.md (7KB) [149L]
│   │   │   │   ├── ATC-44-HARDWARE_ACCELERATED_ZKP_GENERATION.md (3KB) [115L]
│   │   │   │   ├── ATC-45-AI_EVOLUTIONARY_LEARNING_Dael.md (4KB) [115L]
│   │   │   │   ├── ATC-46-QUANTUM_RESISTANT_CRYPTOGRAPHY_LAYER.md (3KB) [116L]
│   │   │   │   ├── ATC-47-AI_INTENT_SETTLEMENT_ARBITRAGE.md (3KB) [115L]
│   │   │   │   ├── ATC-48-NEURAL_NETWORK_MESH_CROSS_TOPOLOGY.md (4KB) [119L]
│   │   │   │   ├── ATC-49-NEURAL_SYNAPSE_INTER_MODEL_KNOWLEDGE_TRANSFER.md (3KB) [115L]
│   │   │   │   ├── ATC-50-AI_CONSCIOUSNESS_SELF_REFLECTION.md (4KB) [117L]
│   │   │   │   ├── ATC-51-CROSS_REALITY_SPATIAL_COMPUTING.md (4KB) [119L]
│   │   │   │   ├── ATC-52-BIO_DIGITAL_INTERFACE_NEURAL_SIGNAL.md (4KB) [118L]
│   │   │   │   ├── ATC-53-CONSCIOUSNESS_SENTIENCE_OBSERVABILITY.md (4KB) [118L]
│   │   │   │   ├── ATC-54-TEMPORAL_CAUSAL_CONVERGENCE.md (4KB) [119L]
│   │   │   │   ├── ATC-55-META_REALITY_SIMULATION_CONVERGENCE.md (4KB) [118L]
│   │   │   │   ├── ATC-56-INTERSTELLAR_DATA_INTEGRITY_RELATIVISTIC_SYNC.md (4KB) [119L]
│   │   │   │   ├── ATC-57-RECURSIVE_SELF_IMPROVEMENT_META_LEARNING.md (4KB) [127L]
│   │   │   │   ├── ATC-58-QUANTUM_NEURAL_ENTANGLEMENT.md (4KB) [126L]
│   │   │   │   ├── ATC-59-TRANSDIMENSIONAL_ENERGY_ENTROPY_MANAGEMENT.md (4KB) [126L]
│   │   │   │   ├── ATC-60-UNIVERSAL_HOLONIC_STRUCTURE.md (4KB) [126L]
│   │   │   │   ├── ATC-61-TRANS_REALITY_SEMANTIC_MAPPING.md (4KB) [127L]
│   │   │   │   ├── ATC-62-META_SYSTEMIC_ETHICS_EXISTENTIAL_RISK.md (4KB) [127L]
│   │   │   │   ├── ATC-63-TRANS_SPECIES_MULTI_BIOLOGICAL_INTEGRATION.md (4KB) [128L]
│   │   │   │   ├── ATC-64-TRANSDIMENSIONAL_RECURSIVE_KNOWLEDGE_SYNTHESIS.md (4KB) [128L]
│   │   │   │   ├── ATC-65-TRANS_METAVERSE_CONSENSUS_REALITY_SYNC.md (4KB) [119L]
│   │   │   │   ├── ATC-66-RECURSIVE_LOGIC_PROOF_OF_UNDERSTANDING.md (4KB) [119L]
│   │   │   │   ├── ATC-67-REALITY_CONSENSUS_OBSERVATION_COLLAPSE.md (3KB) [118L]
│   │   │   │   ├── ATC-68-EVOLUTIONARY_FEEDBACK_ONTOLOGICAL_RECONCILIATION.md (4KB) [118L]
│   │   │   │   ├── ATC-69-TRANS_EXISTENCE_CONSCIOUSNESS_BRIDGE.md (4KB) [119L]
│   │   │   │   ├── ATC-70-QUANTUM_GLOBAL_TRUTH_RECONCILIATION.md (4KB) [118L]
│   │   │   │   ├── ATC-71-TRANS_CAUSAL_REALITY_VOID_MAPPING.md (4KB) [117L]
│   │   │   │   ├── ATC-72-TRANS_RELATIONAL_GOVERNANCE_ENTITY_CONSENSUS.md (4KB) [119L]
│   │   │   │   ├── ATC-73-TRANS_METAVERSE_ENTROPY_HARVESTING.md (4KB) [119L]
│   │   │   │   ├── ATC-74-RECURSIVE_META_NARRATIVE_MYTHOS_CONSTRUCTION.md (3KB) [118L]
│   │   │   │   ├── ATC-75-PROVABLE_EPISTEMOLOGY_AUTO_WIKI.md (4KB) [119L]
│   │   │   │   ├── ATC-76-IMMUTABLE_HUMAN_HERITAGE_ETERNITY.md (4KB) [120L]
│   │   │   │   ├── ATC-77-TRANS_SEMANTIC_HUMAN_AI_OMNI_LINGUISTIC.md (4KB) [120L]
│   │   │   │   ├── ATC-78-ABSOLUTE_CONVERGENCE_MONOLITHIC_SINGULARITY.md (4KB) [119L]
│   │   │   │   ├── ATC-79-TRANS_REALITY_MANIFESTATION_PHYSICALITY_ANCHOR.md (4KB) [119L]
│   │   │   │   ├── ATC-80-TRANS_UNIVERSAL_REALITY_MIGRATION.md (4KB) [120L]
│   │   │   │   ├── ATC-81-PROOF_OF_HISTORY.md (2KB) [105L]
│   │   │   │   ├── ATC-82-PROOF_OF_WORK.md (2KB) [104L]
│   │   │   │   ├── ATC-83-PROOF_OF_STAKE.md (2KB) [106L]
│   │   │   │   ├── ATC-84-FORK_RESOLUTION.md (2KB) [103L]
│   │   │   │   ├── ATC-85-INITIAL_SYNC.md (2KB) [105L]
│   │   │   │   ├── ATC-86-ECDSA_SIGNATURE.md (2KB) [105L]
│   │   │   │   ├── ATC-87-GAS_FEE.md (2KB) [105L]
│   │   │   │   ├── ATC-88-AMM.md (2KB) [105L]
│   │   │   │   ├── ATC-89-FUNGIBLE_TOKEN.md (2KB) [106L]
│   │   │   │   ├── ATC-90-NFT_SHIVAMON.md (2KB) [106L]
│   │   │   │   ├── ATC-91-CROSS_CHAIN_BRIDGE.md (2KB) [105L]
│   │   │   │   ├── ATC-92-ATCLANG_LANGUAGE_SPEC.md (7KB) [221L]
│   │   │   │   ├── ATC-93-ATCLANG_VM_BYTECODE.md (10KB) [338L]
│   │   │   │   ├── ATC-94-ATCLANG_STDLIB.md (6KB) [187L]
│   │   │   │   ├── ATC-95-ATCLANG_TEST_FRAMEWORK.md (6KB) [221L]
│   │   │   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   │   │   ├── ATC-97-AGENT_INTERACTION_PROTOCOL.md (2KB) [83L]
│   │   │   │   ├── ATC-97_AGENT_INTERACTION_PROTOCOL.md (8KB) [243L]
│   │   │   │   ├── ATC-98-TESTING_STANDARD.md (1KB) [69L]
│   │   │   │   ├── ATC-99-ATCLANG_UNIVERSAL_MANDATE.md (7KB) [189L]
│   │   │   │   ├── ATC-LIC-SMART_CONTRACT_LICENSE.md (11KB) [297L]
│   │   │   │   ├── ATC_ECOSYSTEM_STANDARDS.md (13KB) [447L]
│   │   │   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   │   │   ├── ATS-LIC-SYSTEM_HARDWARE_LICENSE.md (4KB) [117L]
│   │   │   │   ├── ATS_STANDARDS.md (7KB) [283L]
│   │   │   │   ├── OVERVIEW.md (1KB) [40L]
│   │   │   │   └── STANDARDS_REGISTRY.md (13KB) [208L]
│   │   │   ├── whitepaper/ (3 files, 2542 lines)
│   │   │   │   ├── CHANGELOG.md (706B) [24L]
│   │   │   │   ├── README.md (2KB) [48L]
│   │   │   │   └── WHITEPAPER.md (80KB) [2470L]
│   │   │   ├── wiki/ (95 files, 16706 lines)
│   │   │   │   ├── atclang/ (13 files, 881 lines)
│   │   │   │   │   ├── docs/ (12 files, 837 lines)
│   │   │   │   │   │   ├── CHANGELOG.md (338B) [8L]
│   │   │   │   │   │   ├── COMPILER.md (3KB) [105L]
│   │   │   │   │   │   ├── CONTRIBUTING.md (472B) [11L]
│   │   │   │   │   │   ├── EXAMPLES.md (3KB) [95L]
│   │   │   │   │   │   ├── LEXER.md (1KB) [59L]
│   │   │   │   │   │   ├── PARSER.md (3KB) [135L]
│   │   │   │   │   │   ├── REPL.md (2KB) [79L]
│   │   │   │   │   │   ├── SECURITY.md (1KB) [34L]
│   │   │   │   │   │   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   │   │   │   │   │   ├── SPEC.md (1KB) [55L]
│   │   │   │   │   │   ├── STDLIB.md (3KB) [111L]
│   │   │   │   │   │   └── VM.md (2KB) [63L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── atcnet/ (6 files, 213 lines)
│   │   │   │   │   ├── docs/ (5 files, 169 lines)
│   │   │   │   │   │   ├── BOOTSTRAP.md (312B) [18L]
│   │   │   │   │   │   ├── MESSAGES.md (1KB) [40L]
│   │   │   │   │   │   ├── PROTOCOL.md (2KB) [57L]
│   │   │   │   │   │   ├── SECURITY.md (336B) [11L]
│   │   │   │   │   │   └── TOPOLOGY.md (1KB) [43L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── contracts/ (7 files, 296 lines)
│   │   │   │   │   ├── docs/ (6 files, 252 lines)
│   │   │   │   │   │   ├── ATC8300.md (1KB) [51L]
│   │   │   │   │   │   ├── ATC9000.md (2KB) [92L]
│   │   │   │   │   │   ├── ATC9900.md (514B) [20L]
│   │   │   │   │   │   ├── BRIDGE.md (1KB) [38L]
│   │   │   │   │   │   ├── DEPLOYMENT.md (603B) [25L]
│   │   │   │   │   │   └── SECURITY.md (708B) [26L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── franchise/ (8 files, 287 lines)
│   │   │   │   │   ├── docs/ (7 files, 243 lines)
│   │   │   │   │   │   ├── API.md (1KB) [37L]
│   │   │   │   │   │   ├── CONCEPT.md (1000B) [24L]
│   │   │   │   │   │   ├── CONTRACTS.md (1KB) [49L]
│   │   │   │   │   │   ├── DEPLOYMENT.md (879B) [43L]
│   │   │   │   │   │   ├── ROADMAP.md (726B) [20L]
│   │   │   │   │   │   ├── SECURITY.md (904B) [29L]
│   │   │   │   │   │   └── TOKEN_ECONOMY.md (1KB) [41L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── gateway/ (6 files, 189 lines)
│   │   │   │   │   ├── docs/ (5 files, 145 lines)
│   │   │   │   │   │   ├── AUTH.md (965B) [43L]
│   │   │   │   │   │   ├── MIDDLEWARE.md (368B) [14L]
│   │   │   │   │   │   ├── RATE_LIMITING.md (956B) [43L]
│   │   │   │   │   │   ├── ROUTES.md (995B) [32L]
│   │   │   │   │   │   └── SECURITY.md (372B) [13L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── kai-os/ (9 files, 12149 lines)
│   │   │   │   │   ├── code/ (1 files, 9 lines)
│   │   │   │   │   │   └── atclang/ (1 files, 9 lines)
│   │   │   │   │   │       └── ATCLANG_SPEC.md (432B) [9L]
│   │   │   │   │   ├── docs/ (7 files, 12122 lines)
│   │   │   │   │   │   ├── issues/ (1 files, 353 lines)
│   │   │   │   │   │   │   └── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   │   │   │   │   ├── standards/ (1 files, 212 lines)
│   │   │   │   │   │   │   └── STANDARDS_REGISTRY.md (10KB) [212L]
│   │   │   │   │   │   ├── DECISIONS_REGISTER.md (2KB) [69L]
│   │   │   │   │   │   ├── DEPRECATED.md (1KB) [32L]
│   │   │   │   │   │   ├── MIGRATION_MAP.md (1KB) [30L]
│   │   │   │   │   │   ├── STATUS.md (2KB) [50L]
│   │   │   │   │   │   └── kai-os-wiki.md (395KB) [11376L]
│   │   │   │   │   └── README.md (542B) [18L]
│   │   │   │   ├── kernel/ (11 files, 755 lines)
│   │   │   │   │   ├── docs/ (9 files, 450 lines)
│   │   │   │   │   │   ├── ATCFS.md (2KB) [107L]
│   │   │   │   │   │   ├── ATCNET.md (2KB) [89L]
│   │   │   │   │   │   ├── CHANGELOG.md (231B) [7L]
│   │   │   │   │   │   ├── CONSENSUS.md (615B) [24L]
│   │   │   │   │   │   ├── IPC.md (1KB) [43L]
│   │   │   │   │   │   ├── KERNEL.md (2KB) [87L]
│   │   │   │   │   │   ├── PERFORMANCE.md (708B) [25L]
│   │   │   │   │   │   ├── PROCESS_MODEL.md (1KB) [48L]
│   │   │   │   │   │   └── SECURITY.md (532B) [20L]
│   │   │   │   │   ├── KERNEL_API.md (9KB) [261L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── overview/ (9 files, 400 lines)
│   │   │   │   │   ├── docs/ (8 files, 356 lines)
│   │   │   │   │   │   ├── API.md (1KB) [59L]
│   │   │   │   │   │   ├── ARCHITECTURE.md (1KB) [36L]
│   │   │   │   │   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   │   │   │   │   ├── FAQ.md (1KB) [62L]
│   │   │   │   │   │   ├── QUICKSTART.md (619B) [30L]
│   │   │   │   │   │   ├── ROADMAP.md (556B) [25L]
│   │   │   │   │   │   ├── SECURITY.md (916B) [18L]
│   │   │   │   │   │   └── WHITEPAPER.md (5KB) [107L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── shivamon/ (7 files, 229 lines)
│   │   │   │   │   ├── docs/ (6 files, 185 lines)
│   │   │   │   │   │   ├── BATTLE.md (420B) [17L]
│   │   │   │   │   │   ├── BREEDING.md (1KB) [37L]
│   │   │   │   │   │   ├── ELEMENTS.md (1KB) [31L]
│   │   │   │   │   │   ├── MARKETPLACE.md (408B) [21L]
│   │   │   │   │   │   ├── NFT_SPEC.md (1KB) [55L]
│   │   │   │   │   │   └── ROADMAP.md (638B) [24L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── standards/ (3 files, 305 lines)
│   │   │   │   │   ├── docs/ (2 files, 261 lines)
│   │   │   │   │   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   │   │   │   │   └── OVERVIEW.md (1KB) [28L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── ui/ (6 files, 240 lines)
│   │   │   │   │   ├── docs/ (5 files, 196 lines)
│   │   │   │   │   │   ├── API.md (651B) [30L]
│   │   │   │   │   │   ├── COMPONENTS.md (442B) [26L]
│   │   │   │   │   │   ├── DEPLOYMENT.md (969B) [49L]
│   │   │   │   │   │   ├── DESIGN.md (732B) [24L]
│   │   │   │   │   │   └── THEME.md (1KB) [67L]
│   │   │   │   │   └── README.md (2KB) [44L]
│   │   │   │   ├── chapter-63-cleanup-2026-06-13.md (6KB) [205L]
│   │   │   │   ├── chapter-70-atclang-migration-complete.md (2KB) [77L]
│   │   │   │   ├── chapter-71-sprint-audit.md (2KB) [67L]
│   │   │   │   ├── chapter-72-sprint-2-7-testing-cicd.md (2KB) [59L]
│   │   │   │   ├── chapter-73-sprint-2-8-testnet.md (1KB) [53L]
│   │   │   │   ├── chapter-74-sprint-3-1-ux-privacy.md (1KB) [40L]
│   │   │   │   ├── chapter-75-v01-v03-migration-plan.md (2KB) [74L]
│   │   │   │   ├── chapter-76-sprint-3-3-3-6-alpha-release.md (1KB) [40L]
│   │   │   │   ├── chapter-77-sprint-4-0-4-1-mainnet.md (1KB) [43L]
│   │   │   │   └── chapter-78-shivacore-kernel-712-tests.md (4KB) [104L]
│   │   │   ├── workflows/ (1 files, 218 lines)
│   │   │   │   └── wiki-sync.yml (8KB) [218L]
│   │   │   ├── AGENT_COORDINATION.md (25KB) [324L]
│   │   │   ├── AGENT_POLICY.md (12KB) [317L]
│   │   │   ├── ATCLANG_AGENT_BUILD_GUIDE.md (22KB) [281L]
│   │   │   ├── AUDIT_REPORT.md (3KB) [89L]
│   │   │   ├── CLUSTER_ARCHITECTURE.md (6KB) [103L]
│   │   │   ├── DECISIONS_REGISTER.md (7KB) [140L]
│   │   │   ├── DEPRECATED.md (2KB) [50L]
│   │   │   ├── ECOSYSTEM_BRAIN.md (3KB) [104L]
│   │   │   ├── FIXES.md (3KB) [96L]
│   │   │   ├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
│   │   │   ├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
│   │   │   ├── KAI_INTEGRATION.md (6KB) [242L]
│   │   │   ├── LICENSING_OVERVIEW.md (6KB) [157L]
│   │   │   ├── MIGRATION_MAP.md (4KB) [113L]
│   │   │   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   │   │   ├── REALITY_CHECK_2026-07-06.md (28KB) [428L]
│   │   │   ├── ROADMAP.md (9KB) [208L]
│   │   │   ├── ROADMAP_COMPLETENESS_AUDIT.md (7KB) [223L]
│   │   │   ├── SHIVACORE_KERNEL_STATUS.md (38KB) [722L]
│   │   │   ├── STATUS.md (4KB) [72L]
│   │   │   ├── TODO.md (8KB) [200L]
│   │   │   ├── WIKI_AUDIT.md (6KB) [188L]
│   │   │   ├── api-reference.md (1KB) [33L]
│   │   │   ├── atclang-guide.md (1KB) [48L]
│   │   │   ├── genesis_wallet.md (3KB) [103L]
│   │   │   └── kai-os-wiki.md (297KB) [8436L]
│   │   ├── gateway/ (2 files, 295 lines)
│   │   │   ├── main.atc (5KB) [127L]
│   │   │   └── service_discovery.atc (6KB) [168L]
│   │   ├── mobile/ (4 files, 354 lines)
│   │   │   ├── wallet/ (2 files, 181 lines)
│   │   │   │   ├── __init__.py (162B) [2L]
│   │   │   │   └── biometric_auth.atc (5KB) [179L]
│   │   │   ├── __init__.py (123B) [2L]
│   │   │   └── wallet_api.atc (5KB) [171L]
│   │   ├── modules/ (120 files, 19219 lines)
│   │   │   ├── assets/ (16 files, 2042 lines)
│   │   │   │   ├── aaa_asset_core.atc (3KB) [87L]
│   │   │   │   ├── ai_assets.atc (4KB) [124L]
│   │   │   │   ├── animation.atc (4KB) [142L]
│   │   │   │   ├── asset_bundle.atc (3KB) [104L]
│   │   │   │   ├── cloud_assets.atc (5KB) [133L]
│   │   │   │   ├── encryption.atc (5KB) [149L]
│   │   │   │   ├── hot_reload.atc (4KB) [125L]
│   │   │   │   ├── memory_cleanup.atc (4KB) [122L]
│   │   │   │   ├── mod_system.atc (5KB) [144L]
│   │   │   │   ├── model3d.atc (5KB) [168L]
│   │   │   │   ├── priority_loading.atc (2KB) [80L]
│   │   │   │   ├── render_pipeline.atc (5KB) [159L]
│   │   │   │   ├── shader_system.atc (4KB) [143L]
│   │   │   │   ├── streaming.atc (3KB) [91L]
│   │   │   │   ├── telemetry.atc (4KB) [144L]
│   │   │   │   └── versioning.atc (4KB) [127L]
│   │   │   ├── atcnet/ (7 files, 963 lines)
│   │   │   │   ├── README.md (780B) [37L]
│   │   │   │   ├── bootstrap_client.atc (4KB) [134L]
│   │   │   │   ├── discovery.atc (4KB) [138L]
│   │   │   │   ├── gossip.atc (5KB) [171L]
│   │   │   │   ├── nat_traversal.atc (3KB) [109L]
│   │   │   │   ├── p2p_node.atc (4KB) [159L]
│   │   │   │   └── p2p_propagation.atc (6KB) [215L]
│   │   │   ├── civilization/ (11 files, 2214 lines)
│   │   │   │   ├── asset_genome_ad66.atc (5KB) [171L]
│   │   │   │   ├── civilization_engine_ad60.atc (5KB) [236L]
│   │   │   │   ├── ecosystem_ai_mesh_ad62.atc (7KB) [245L]
│   │   │   │   ├── evolution_engine_ad69.atc (7KB) [251L]
│   │   │   │   ├── experience_orchestrator_ad68.atc (6KB) [200L]
│   │   │   │   ├── gcp_core_ad70.atc (7KB) [169L]
│   │   │   │   ├── global_simulation_core_ad64.atc (6KB) [198L]
│   │   │   │   ├── identity_layer_ad65.atc (4KB) [190L]
│   │   │   │   ├── persistent_world_engine_ad61.atc (5KB) [199L]
│   │   │   │   ├── proc_universe_generator_ad63.atc (8KB) [204L]
│   │   │   │   └── production_pipeline_ad67.atc (6KB) [151L]
│   │   │   ├── contracts/ (10 files, 1536 lines)
│   │   │   │   ├── atc8300/ (1 files, 178 lines)
│   │   │   │   │   └── atc8300_token.atc (5KB) [178L]
│   │   │   │   ├── atcoin/ (1 files, 176 lines)
│   │   │   │   │   └── atcoin.atc (5KB) [176L]
│   │   │   │   ├── base/ (1 files, 69 lines)
│   │   │   │   │   └── base_contract.atc (2KB) [69L]
│   │   │   │   ├── bridge/ (1 files, 172 lines)
│   │   │   │   │   └── bridge_contract.atc (5KB) [172L]
│   │   │   │   ├── governance/ (1 files, 237 lines)
│   │   │   │   │   └── governance_contract.atc (7KB) [237L]
│   │   │   │   ├── marketplace/ (1 files, 236 lines)
│   │   │   │   │   └── marketplace_contract.atc (7KB) [236L]
│   │   │   │   ├── shivamon/ (1 files, 290 lines)
│   │   │   │   │   └── shivamon_contract.atc (9KB) [290L]
│   │   │   │   ├── wallet/ (2 files, 135 lines)
│   │   │   │   │   ├── ecdsa.atc (2KB) [60L]
│   │   │   │   │   └── keygen.atc (2KB) [75L]
│   │   │   │   └── README.md (1KB) [43L]
│   │   │   ├── franchise/ (30 files, 4183 lines)
│   │   │   │   ├── contracts/ (3 files, 285 lines)
│   │   │   │   │   ├── registry.atc (4KB) [120L]
│   │   │   │   │   ├── revenue.atc (3KB) [93L]
│   │   │   │   │   └── token.atc (3KB) [72L]
│   │   │   │   ├── README.md (775B) [35L]
│   │   │   │   ├── ai_content_factory_ad28.atc (6KB) [194L]
│   │   │   │   ├── ai_director_factory_ad41.atc (4KB) [28L]
│   │   │   │   ├── analytics_factory_ad31.atc (7KB) [232L]
│   │   │   │   ├── asset_intelligence_factory_ad34.atc (7KB) [210L]
│   │   │   │   ├── blueprint_factory_ad32.atc (5KB) [165L]
│   │   │   │   ├── canon_engine_ad33.atc (5KB) [171L]
│   │   │   │   ├── character_factory_ad23.atc (8KB) [251L]
│   │   │   │   ├── commerce_factory_ad40.atc (4KB) [26L]
│   │   │   │   ├── community_factory_ad30.atc (7KB) [222L]
│   │   │   │   ├── creator_factory_ad38.atc (4KB) [24L]
│   │   │   │   ├── economy_factory_ad26.atc (6KB) [200L]
│   │   │   │   ├── factory.atc (5KB) [165L]
│   │   │   │   ├── gameplay_factory_ad35.atc (4KB) [126L]
│   │   │   │   ├── gff_core_ad20.atc (8KB) [224L]
│   │   │   │   ├── ip_factory_ad21.atc (4KB) [147L]
│   │   │   │   ├── lifecycle_manager_ad43.atc (5KB) [25L]
│   │   │   │   ├── liveops_factory_ad27.atc (6KB) [212L]
│   │   │   │   ├── lore_factory_ad24.atc (7KB) [209L]
│   │   │   │   ├── merchandise_factory_ad29.atc (5KB) [173L]
│   │   │   │   ├── multiplayer_factory_ad37.atc (3KB) [27L]
│   │   │   │   ├── narrative_factory_ad36.atc (8KB) [245L]
│   │   │   │   ├── publishing_factory_ad39.atc (4KB) [25L]
│   │   │   │   ├── quest_factory_ad25.atc (6KB) [207L]
│   │   │   │   ├── routes.atc (2KB) [90L]
│   │   │   │   ├── security_factory_ad42.atc (4KB) [30L]
│   │   │   │   └── world_factory_ad22.atc (6KB) [235L]
│   │   │   ├── gateway/ (9 files, 564 lines)
│   │   │   │   ├── middleware/ (5 files, 247 lines)
│   │   │   │   │   ├── __init__.py (120B) [2L]
│   │   │   │   │   ├── auth.atc (2KB) [82L]
│   │   │   │   │   ├── logger.atc (2KB) [70L]
│   │   │   │   │   ├── rate_limit.atc (1KB) [50L]
│   │   │   │   │   └── signature_verify.atc (1KB) [43L]
│   │   │   │   ├── README.md (858B) [39L]
│   │   │   │   ├── __init__.py (125B) [2L]
│   │   │   │   ├── main.atc (5KB) [180L]
│   │   │   │   └── router.atc (3KB) [96L]
│   │   │   ├── kernel/ (25 files, 5147 lines)
│   │   │   │   ├── ai_kernel/ (1 files, 228 lines)
│   │   │   │   │   └── ai_kernel.atc (8KB) [228L]
│   │   │   │   ├── consensus/ (2 files, 607 lines)
│   │   │   │   │   ├── poh_integration.atc (2KB) [78L]
│   │   │   │   │   └── shiva_consensus.atc (16KB) [529L]
│   │   │   │   ├── fs/ (1 files, 142 lines)
│   │   │   │   │   └── atcfs.atc (4KB) [142L]
│   │   │   │   ├── ipc/ (2 files, 106 lines)
│   │   │   │   │   ├── __init__.py (236B) [4L]
│   │   │   │   │   └── ipc_bus.atc (3KB) [102L]
│   │   │   │   ├── net/ (1 files, 135 lines)
│   │   │   │   │   └── atcnet.atc (4KB) [135L]
│   │   │   │   ├── pkg/ (1 files, 208 lines)
│   │   │   │   │   └── manager.atc (6KB) [208L]
│   │   │   │   ├── process/ (1 files, 161 lines)
│   │   │   │   │   └── process_mgr.atc (4KB) [161L]
│   │   │   │   ├── shell/ (1 files, 296 lines)
│   │   │   │   │   └── shell.atc (8KB) [296L]
│   │   │   │   ├── README.md (1KB) [46L]
│   │   │   │   ├── ai_bus_ad13.atc (9KB) [310L]
│   │   │   │   ├── asset_bus_ad08.atc (5KB) [188L]
│   │   │   │   ├── audio_bus_ad11.atc (5KB) [199L]
│   │   │   │   ├── command_bus_ad02.atc (4KB) [168L]
│   │   │   │   ├── gcl_core_ad00.atc (7KB) [269L]
│   │   │   │   ├── input_bus_ad12.atc (5KB) [184L]
│   │   │   │   ├── ipc_bus_atc.ad.atc (8KB) [266L]
│   │   │   │   ├── message_bus_ad03.atc (6KB) [240L]
│   │   │   │   ├── network_bus_ad05.atc (8KB) [307L]
│   │   │   │   ├── physics_bus_ad10.atc (7KB) [255L]
│   │   │   │   ├── plugin_bus_ad06.atc (8KB) [286L]
│   │   │   │   ├── query_bus_ad07.atc (3KB) [128L]
│   │   │   │   ├── render_bus_ad09.atc (5KB) [164L]
│   │   │   │   └── telemetry_bus_ad14.atc (7KB) [254L]
│   │   │   ├── meta/ (8 files, 2320 lines)
│   │   │   │   ├── ai_studio_ad49.atc (11KB) [310L]
│   │   │   │   ├── cross_franchise_ad46.atc (8KB) [223L]
│   │   │   │   ├── data_lake_ad51.atc (9KB) [237L]
│   │   │   │   ├── digital_twin_ad50.atc (11KB) [303L]
│   │   │   │   ├── ip_evolution_ad45.atc (9KB) [241L]
│   │   │   │   ├── knowledge_graph_ad47.atc (11KB) [289L]
│   │   │   │   ├── simulation_factory_ad48.atc (13KB) [374L]
│   │   │   │   └── universe_factory_ad44.atc (13KB) [343L]
│   │   │   ├── shivamon/ (2 files, 188 lines)
│   │   │   │   ├── engine/ (1 files, 153 lines)
│   │   │   │   │   └── battle_engine.atc (5KB) [153L]
│   │   │   │   └── README.md (819B) [35L]
│   │   │   ├── standards/ (1 files, 32 lines)
│   │   │   │   └── README.md (706B) [32L]
│   │   │   └── ui/ (1 files, 30 lines)
│   │   │       └── README.md (586B) [30L]
│   │   ├── monitoring/ (3 files, 612 lines)
│   │   │   ├── health_checks_atc08.atc (5KB) [197L]
│   │   │   ├── monitor.atc (6KB) [213L]
│   │   │   └── prometheus_metrics.atc (6KB) [202L]
│   │   ├── patches/ (6 files, 264 lines)
│   │   │   ├── APPLY_FIXES.sh (1KB) [32L]
│   │   │   ├── atc9900_governance.py (2KB) [60L]
│   │   │   ├── docker-compose.yml (1KB) [42L]
│   │   │   ├── gateway_main.py (1KB) [44L]
│   │   │   ├── gateway_router.py (2KB) [49L]
│   │   │   └── poh_fixed.py (1KB) [37L]
│   │   ├── reports/ (1 files, 102 lines)
│   │   │   └── SPRINT_2.3_2.4_2.7_REPORT.md (3KB) [102L]
│   │   ├── scripts/ (1 files, 135 lines)
│   │   │   └── generate_validators.atc (4KB) [135L]
│   │   ├── shivaos/ (3 files, 430 lines)
│   │   │   ├── fs/ (1 files, 126 lines)
│   │   │   │   └── atcfs_module.atc (4KB) [126L]
│   │   │   ├── kernel/ (1 files, 118 lines)
│   │   │   │   └── syscalls.atc (3KB) [118L]
│   │   │   └── ui/ (1 files, 186 lines)
│   │   │       └── renderer.atc (5KB) [186L]
│   │   ├── tests/ (26 files, 4558 lines)
│   │   │   ├── unit/ (3 files, 654 lines)
│   │   │   │   ├── test_atclang.py (14KB) [462L] 🧪
│   │   │   │   ├── test_atcnet.py (1KB) [41L] 🧪
│   │   │   │   └── test_p2p_propagation.py (4KB) [151L] 🧪
│   │   │   ├── test_atclang.py (14KB) [470L] 🧪
│   │   │   ├── test_atclang_v03.py (2KB) [68L] 🧪
│   │   │   ├── test_bootstrap.py (10KB) [268L] 🧪
│   │   │   ├── test_did.py (1KB) [61L] 🧪
│   │   │   ├── test_discovery.py (4KB) [155L] 🧪
│   │   │   ├── test_ecdsa.py (2KB) [65L] 🧪
│   │   │   ├── test_fork_resolution.py (3KB) [101L] 🧪
│   │   │   ├── test_gateway.py (7KB) [201L] 🧪
│   │   │   ├── test_gateway_full.py (2KB) [76L] 🧪
│   │   │   ├── test_integration_atcfs_multisig.py (4KB) [129L] 🧪
│   │   │   ├── test_kai_integration.py (8KB) [297L] 🧪
│   │   │   ├── test_multinode_consensus.py (5KB) [155L] 🧪
│   │   │   ├── test_multinode_fivenode.py (3KB) [84L] 🧪
│   │   │   ├── test_node_failure_recovery.py (4KB) [143L] 🧪
│   │   │   ├── test_optimizer.py (9KB) [256L] 🧪
│   │   │   ├── test_orchestrator.py (1KB) [52L] 🧪
│   │   │   ├── test_p2p_propagation.py (5KB) [205L] 🧪
│   │   │   ├── test_persistence.py (2KB) [87L] 🧪
│   │   │   ├── test_poh.py (1KB) [63L] 🧪
│   │   │   ├── test_smart_contracts.py (3KB) [114L] 🧪
│   │   │   ├── test_stdlib.py (10KB) [298L] 🧪
│   │   │   ├── test_stdlib_dispatch.py (11KB) [312L] 🧪
│   │   │   └── test_type_checker.py (7KB) [244L] 🧪
│   │   ├── tools/ (4 files, 623 lines)
│   │   │   ├── atc_issues_summary.atc (6KB) [212L]
│   │   │   ├── bigquery_pipeline.atc (4KB) [135L]
│   │   │   ├── ecdsa_impl.atc (4KB) [119L]
│   │   │   └── hf_review_pipeline.atc (5KB) [157L]
│   │   ├── .gitignore (171B)
│   │   ├── AAA_ASSET_SYSTEM_v1.md (3KB) [120L]
│   │   ├── AGENT_MANIFEST.md (2KB) [61L]
│   │   ├── AGENT_MASTERRULES.md (13KB) [438L]
│   │   ├── ATCLANG_FIRST.md (900B) [31L]
│   │   ├── CHANGELOG.md (6KB) [172L]
│   │   ├── CONNECTION_MAP.md (2KB) [50L]
│   │   ├── ECOSYSTEM.md (8KB) [179L]
│   │   ├── FILE_REGISTER.md (40KB) [746L]
│   │   ├── FIXES.md (3KB) [96L]
│   │   ├── GENESIS_BUS_ARCHITECTURE.md (5KB) [121L]
│   │   ├── GENESIS_CIVILIZATION_PLATFORM_v4.md (5KB) [153L]
│   │   ├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
│   │   ├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
│   │   ├── GENESIS_FRANCHISE_FACTORY_v2.md (4KB) [101L]
│   │   ├── KONSOLIDIERUNGS_ROADMAP.md (14KB) [360L]
│   │   ├── LICENSE (982B)
│   │   ├── MILESTONES.md (1KB) [23L]
│   │   ├── NAMING_CONVENTIONS.md (4KB) [88L]
│   │   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   │   ├── README.md (1KB) [38L]
│   │   ├── ROADMAP.md (8KB) [321L]
│   │   ├── SPRINT_ROADMAP.md (20KB) [568L]
│   │   ├── STATUS.md (4KB) [117L]
│   │   ├── TODO.md (2KB) [48L]
│   │   ├── conftest.py (374B) [9L]
│   │   └── start.atc (4KB) [129L]
│   ├── kernel/ (15 files, 605 lines)
│   │   ├── docs/ (11 files, 490 lines)
│   │   │   ├── ATCFS.md (2KB) [107L]
│   │   │   ├── ATCNET.md (2KB) [89L]
│   │   │   ├── CHANGELOG.md (231B) [7L]
│   │   │   ├── CONSENSUS.md (615B) [24L]
│   │   │   ├── IPC.md (1KB) [43L]
│   │   │   ├── KERNEL.md (2KB) [87L]
│   │   │   ├── PERFORMANCE.md (708B) [25L]
│   │   │   ├── PROCESS_MODEL.md (1KB) [48L]
│   │   │   ├── ROADMAP.md (508B) [18L]
│   │   │   ├── SECURITY.md (532B) [20L]
│   │   │   └── TODO.md (638B) [22L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [50L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── linux-edition-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (325B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (321B) [10L]
│   │   ├── README.md (525B) [18L]
│   │   └── STATUS.md (308B) [15L]
│   ├── main/ (26 files, 3186 lines)
│   │   ├── docs/ (22 files, 1630 lines)
│   │   │   ├── API.md (1KB) [59L]
│   │   │   ├── API_REFERENCE.md (1KB) [50L]
│   │   │   ├── ARCHITECTURE.md (5KB) [126L]
│   │   │   ├── BOTTLENECKS.md (1KB) [50L]
│   │   │   ├── COMMITS.md (2KB) [73L]
│   │   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   │   ├── DECENTRALIZED_PROOF.md (3KB) [103L]
│   │   │   ├── DEPENDENCIES.md (2KB) [79L]
│   │   │   ├── ENTERPRISE.md (1KB) [65L]
│   │   │   ├── ERRORS.md (4KB) [79L]
│   │   │   ├── ERROR_SOLUTIONS.md (3KB) [128L]
│   │   │   ├── FAQ.md (1KB) [62L]
│   │   │   ├── IMPROVEMENTS.md (1KB) [61L]
│   │   │   ├── ISSUES_TRACKER.md (4KB) [107L]
│   │   │   ├── MATH_PROOF.md (3KB) [93L]
│   │   │   ├── QUICKSTART.md (619B) [30L]
│   │   │   ├── ROADMAP.md (2KB) [80L]
│   │   │   ├── SECURITY.md (916B) [18L]
│   │   │   ├── STATUS.md (909B) [25L]
│   │   │   ├── SYNTAX.md (3KB) [133L]
│   │   │   ├── TODO.md (2KB) [83L]
│   │   │   └── WHITEPAPER.md (5KB) [107L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (75KB) [1491L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (4KB) [65L]
│   ├── mobile-wiki/ (6 files, 108 lines)
│   │   ├── docs/ (2 files, 21 lines)
│   │   │   ├── ARCHITECTURE.md (412B) [9L]
│   │   │   └── ROADMAP.md (380B) [12L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (488B) [17L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [70L]
│   ├── sdk-wiki/ (10 files, 159 lines)
│   │   ├── docs/ (3 files, 35 lines)
│   │   │   ├── API.md (307B) [11L]
│   │   │   ├── ARCHITECTURE.md (525B) [16L]
│   │   │   └── ROADMAP.md (173B) [8L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (316B) [14L]
│   │   ├── FILE_REGISTER.md (490B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (301B) [10L]
│   │   ├── README.md (3KB) [69L]
│   │   └── STATUS.md (288B) [15L]
│   ├── shivacore-tools-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (331B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (325B) [10L]
│   │   ├── README.md (535B) [18L]
│   │   └── STATUS.md (312B) [15L]
│   ├── shivacore-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (327B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (313B) [10L]
│   │   ├── README.md (519B) [18L]
│   │   └── STATUS.md (300B) [15L]
│   ├── shivamon/ (11 files, 307 lines)
│   │   ├── docs/ (7 files, 199 lines)
│   │   │   ├── BATTLE.md (420B) [17L]
│   │   │   ├── BREEDING.md (1KB) [37L]
│   │   │   ├── ELEMENTS.md (1KB) [31L]
│   │   │   ├── MARKETPLACE.md (408B) [21L]
│   │   │   ├── NFT_SPEC.md (1KB) [55L]
│   │   │   ├── ROADMAP.md (534B) [18L]
│   │   │   └── TODO.md (572B) [20L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [43L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── standards/ (8 files, 667 lines)
│   │   ├── docs/ (4 files, 561 lines)
│   │   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   │   ├── ATS_STANDARDS.md (7KB) [283L]
│   │   │   ├── OVERVIEW.md (1KB) [28L]
│   │   │   └── ROADMAP.md (527B) [17L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (1KB) [41L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── stdlib-wiki/ (10 files, 202 lines)
│   │   ├── docs/ (3 files, 86 lines)
│   │   │   ├── ARCHITECTURE.md (1016B) [34L]
│   │   │   ├── MODULES.md (943B) [36L]
│   │   │   └── ROADMAP.md (584B) [16L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (322B) [14L]
│   │   ├── FILE_REGISTER.md (511B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (307B) [10L]
│   │   ├── README.md (3KB) [61L]
│   │   └── STATUS.md (294B) [15L]
│   ├── ui/ (10 files, 316 lines)
│   │   ├── docs/ (6 files, 213 lines)
│   │   │   ├── API.md (651B) [30L]
│   │   │   ├── COMPONENTS.md (442B) [26L]
│   │   │   ├── DEPLOYMENT.md (969B) [49L]
│   │   │   ├── DESIGN.md (732B) [24L]
│   │   │   ├── ROADMAP.md (439B) [17L]
│   │   │   └── THEME.md (1KB) [67L]
│   │   ├── .gitignore (171B)
│   │   ├── FILE_REGISTER.md (923B) [38L]
│   │   ├── LICENSE (982B)
│   │   └── README.md (3KB) [65L]
│   ├── vm-wiki/ (10 files, 207 lines)
│   │   ├── docs/ (3 files, 91 lines)
│   │   │   ├── ARCHITECTURE.md (2KB) [44L]
│   │   │   ├── OPCODES.md (1KB) [26L]
│   │   │   └── ROADMAP.md (814B) [21L]
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (313B) [14L]
│   │   ├── FILE_REGISTER.md (507B) [16L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (299B) [10L]
│   │   ├── README.md (3KB) [61L]
│   │   └── STATUS.md (286B) [15L]
│   ├── wallet-wiki/ (7 files, 122 lines)
│   │   ├── docs/ (3 files, 33 lines)
│   │   │   ├── ARCHITECTURE.md (386B) [14L]
│   │   │   ├── ROADMAP.md (383B) [12L]
│   │   │   └── SECURITY.md (349B) [7L]
│   │   ├── .gitignore (44B)
│   │   ├── FILE_REGISTER.md (523B) [18L]
│   │   ├── LICENSE (472B)
│   │   └── README.md (4KB) [71L]
│   ├── windows-edition-wiki/ (6 files, 57 lines)
│   │   ├── .gitignore (44B)
│   │   ├── ARCHITECTURE.md (331B) [14L]
│   │   ├── LICENSE (472B)
│   │   ├── MODULES.md (325B) [10L]
│   │   ├── README.md (535B) [18L]
│   │   └── STATUS.md (312B) [15L]
│   ├── genesis_communication_layer_v2.md (14KB) [431L]
│   └── genesis_franchise_factory_v1.md (6KB) [166L]
├── .gitignore (171B)
├── AAA_ASSET_SYSTEM_v1.md (3KB) [120L]
├── AGENT_MANIFEST.md (2KB) [57L]
├── AGENT_MASTERRULES.md (13KB) [438L]
├── ATCLANG_FIRST.md (900B) [31L]
├── CHANGELOG.md (4KB) [92L]
├── CONNECTION_MAP.md (2KB) [50L]
├── ECOSYSTEM.md (1KB) [52L]
├── ECOSYSTEM_STATUS.md (6KB) [116L]
├── FILE_REGISTER.md (113KB) [1908L]
├── GENESIS_BUS_ARCHITECTURE.md (5KB) [121L]
├── GENESIS_CIVILIZATION_PLATFORM_v4.md (5KB) [153L]
├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
├── GENESIS_FRANCHISE_FACTORY_v2.md (4KB) [101L]
├── KONSOLIDIERUNGS_MATRIX.md (6KB) [124L]
├── KONSOLIDIERUNGS_ROADMAP.md (15KB) [385L]
├── LICENSE (982B)
├── MILESTONES.md (1KB) [27L]
├── NAMING_CONVENTIONS.md (4KB) [88L]
├── README.md (3KB) [103L]
├── REALITY_STATUS.md (2KB) [63L]
├── SPRINT_ROADMAP.md (3KB) [62L]
├── STATUS.md (4KB) [95L]
├── TODO.md (1KB) [27L]
├── VERSION (6B)
├── conftest.py (374B) [9L]
└── start.atc (4KB) [129L]
```


## a-townchain-os-wiki

**Layer:** Wiki (Archiv) | **Dateien:** 26 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(24) · .no-ext(1) · .gitignore(1)

```
a-townchain-os-wiki/
├── docs/ (22 files, 1630 lines)
│   ├── API.md (1KB) [59L]
│   ├── API_REFERENCE.md (1KB) [50L]
│   ├── ARCHITECTURE.md (5KB) [126L]
│   ├── BOTTLENECKS.md (1KB) [50L]
│   ├── COMMITS.md (2KB) [73L]
│   ├── CONTRIBUTING.md (609B) [19L]
│   ├── DECENTRALIZED_PROOF.md (3KB) [103L]
│   ├── DEPENDENCIES.md (2KB) [79L]
│   ├── ENTERPRISE.md (1KB) [65L]
│   ├── ERRORS.md (4KB) [79L]
│   ├── ERROR_SOLUTIONS.md (3KB) [128L]
│   ├── FAQ.md (1KB) [62L]
│   ├── IMPROVEMENTS.md (1KB) [61L]
│   ├── ISSUES_TRACKER.md (4KB) [107L]
│   ├── MATH_PROOF.md (3KB) [93L]
│   ├── QUICKSTART.md (619B) [30L]
│   ├── ROADMAP.md (2KB) [80L]
│   ├── SECURITY.md (916B) [18L]
│   ├── STATUS.md (909B) [25L]
│   ├── SYNTAX.md (3KB) [133L]
│   ├── TODO.md (2KB) [83L]
│   └── WHITEPAPER.md (5KB) [107L]
├── .gitignore (171B)
├── FILE_REGISTER.md (75KB) [1491L]
├── LICENSE (982B)
└── README.md (4KB) [65L]
```


## atc-aistudio

**Layer:** L10 (AI Studio) | **Dateien:** 248 | **Verzeichnisse:** 21 | **Max Tiefe:** 4 | **Tests:** 6

**Sprachen:** .tsx(158) · .ts(41) · .js(17) · .cjs(11) · .md(8) · .json(5)

**Test-Dateien:** 6× TypeScript

```
atc-aistudio/
├── assets/ (1 files, 0 lines)
│   └── .aistudio/ (1 files, 0 lines)
│       └── .gitignore (2B)
├── src/ (190 files, 56202 lines)
│   ├── backend/ (2 files, 206 lines)
│   │   ├── blockchain/ (1 files, 129 lines)
│   │   │   └── engine.ts (3KB) [129L]
│   │   └── p2p/ (1 files, 77 lines)
│   │       └── network.ts (2KB) [77L]
│   ├── components/ (148 files, 43089 lines)
│   │   ├── ATCAssetView.tsx (11KB) [191L]
│   │   ├── ATCDjStudioView.tsx (17KB) [445L]
│   │   ├── ATCLangEditor.tsx (26KB) [625L]
│   │   ├── ATCWalletView.tsx (26KB) [498L]
│   │   ├── ATownDashboardView.tsx (14KB) [302L]
│   │   ├── ATownOSNode.tsx (71KB) [1439L]
│   │   ├── ATownTestView.tsx (6KB) [111L]
│   │   ├── AgentCivilizationView.tsx (8KB) [152L]
│   │   ├── Ai3DRenderEngineTab.tsx (8KB) [199L]
│   │   ├── AiAnimationEngineTab.tsx (8KB) [198L]
│   │   ├── AiAudioEngineTab.tsx (8KB) [198L]
│   │   ├── AiCharacterBioTab.tsx (9KB) [199L]
│   │   ├── AiGameEngineTab.tsx (9KB) [200L]
│   │   ├── AiKernelView.tsx (6KB) [128L]
│   │   ├── AiOsEngineView.tsx (19KB) [490L]
│   │   ├── AiSoftwareWorkflowView.tsx (11KB) [229L]
│   │   ├── AiTimelineEngineTab.tsx (9KB) [199L]
│   │   ├── AntiCheatView.tsx (14KB) [261L]
│   │   ├── ApiHealthWidget.tsx (3KB) [85L]
│   │   ├── ApiInterfacesView.tsx (9KB) [189L]
│   │   ├── ApiOrchestratorView.tsx (17KB) [354L]
│   │   ├── AppGlobeView.tsx (9KB) [233L]
│   │   ├── ArchitectureDependencyGraph.tsx (7KB) [248L]
│   │   ├── ArchitectureView.tsx (38KB) [888L]
│   │   ├── AssetVaultView.tsx (8KB) [187L]
│   │   ├── AtcAssetsDbView.tsx (11KB) [250L]
│   │   ├── AtcCoreKernelView.tsx (8KB) [144L]
│   │   ├── AtcLangArchitectureView.tsx (33KB) [585L]
│   │   ├── AtcLangPlaygroundView.tsx (12KB) [256L]
│   │   ├── AtcLangPresetsView.tsx (3KB) [64L]
│   │   ├── AtcWhitepaperView.tsx (10KB) [187L]
│   │   ├── AtsSuite.tsx (4KB) [51L]
│   │   ├── AtvmSandboxView.test.tsx (3KB) [85L] 🧪
│   │   ├── AtvmSandboxView.tsx (26KB) [499L]
│   │   ├── BatteryStatus.tsx (11KB) [269L]
│   │   ├── BattleArenaView.tsx (8KB) [143L]
│   │   ├── BenchmarkCenterView.tsx (15KB) [288L]
│   │   ├── BlockchainEcosystemView.tsx (9KB) [224L]
│   │   ├── BlockchainLedgerView.tsx (13KB) [247L]
│   │   ├── CalculatorView.tsx (3KB) [74L]
│   │   ├── CalendarView.tsx (3KB) [78L]
│   │   ├── CiCdPipelineView.tsx (7KB) [159L]
│   │   ├── ClockView.tsx (3KB) [72L]
│   │   ├── CodeAnalyzerView.tsx (4KB) [90L]
│   │   ├── CommitHeatmap.tsx (4KB) [110L]
│   │   ├── ComplianceEngineView.tsx (4KB) [84L]
│   │   ├── ComplianceView.tsx (8KB) [191L]
│   │   ├── ConflictResolutionModal.tsx (11KB) [257L]
│   │   ├── ConsensusIntegrationGuide.tsx (70KB) [1528L]
│   │   ├── CryptoVisualizationView.tsx (18KB) [473L]
│   │   ├── DataProcessingView.tsx (4KB) [78L]
│   │   ├── DbOrchestratorView.tsx (6KB) [112L]
│   │   ├── DeFiLiquidityPoolView.tsx (13KB) [255L]
│   │   ├── DependencyMapView.tsx (3KB) [123L]
│   │   ├── DeploymentPipelineWidget.tsx (6KB) [160L]
│   │   ├── DevToolsView.tsx (5KB) [133L]
│   │   ├── DeveloperKnowledgeBaseView.tsx (18KB) [359L]
│   │   ├── DistributedDatalakeView.tsx (3KB) [73L]
│   │   ├── EcosystemInstaller.tsx (11KB) [297L]
│   │   ├── EcosystemTreeOverlay.tsx (12KB) [357L]
│   │   ├── EcosystemUmlView.tsx (7KB) [143L]
│   │   ├── EcosystemVisualizerView.tsx (12KB) [325L]
│   │   ├── FileManagerView.tsx (6KB) [170L]
│   │   ├── FolderView.tsx (4KB) [111L]
│   │   ├── FranchiseFactoryView.tsx (83KB) [1733L]
│   │   ├── GateToHellBrowser.tsx (5KB) [106L]
│   │   ├── GenesisBlockGeneratorView.tsx (6KB) [150L]
│   │   ├── GitGraphVisualization.tsx (4KB) [137L]
│   │   ├── GitHubRepoSyncView.tsx (63KB) [1385L]
│   │   ├── GitHubStatusDashboard.tsx (34KB) [643L]
│   │   ├── GitOpsView.tsx (7KB) [126L]
│   │   ├── GovernanceView.tsx (23KB) [601L]
│   │   ├── GpuPerformanceWidget.tsx (4KB) [120L]
│   │   ├── HardwareDriversView.tsx (20KB) [376L]
│   │   ├── IdeaToAppFlowchartView.tsx (7KB) [153L]
│   │   ├── ImageGeneratorTab.tsx (5KB) [117L]
│   │   ├── IntegrationsWindow.tsx (21KB) [426L]
│   │   ├── InterfacesView.tsx (2KB) [56L]
│   │   ├── JsExampleRunner.tsx (2KB) [86L]
│   │   ├── LazyMetricsCharts.tsx (31KB) [808L]
│   │   ├── LegalView.tsx (6KB) [87L]
│   │   ├── LoginOverlay.tsx (38KB) [690L]
│   │   ├── MainnetLaunchView.tsx (12KB) [251L]
│   │   ├── MarketplaceView.tsx (22KB) [450L]
│   │   ├── MediaApps.tsx (18KB) [254L]
│   │   ├── MetricsDashboard.tsx (4KB) [105L]
│   │   ├── MetricsView.tsx (56KB) [1476L]
│   │   ├── ModulesPluginView.tsx (18KB) [309L]
│   │   ├── NetworkExplorerView.test.tsx (4KB) [121L] 🧪
│   │   ├── NetworkExplorerView.tsx (17KB) [370L]
│   │   ├── NetworkTopologyView.tsx (2KB) [38L]
│   │   ├── NodeHealthMonitor.tsx (4KB) [113L]
│   │   ├── NotepadView.tsx (2KB) [67L]
│   │   ├── OfficeApps.tsx (352B) [14L]
│   │   ├── OfficeSuiteView.tsx (12KB) [271L]
│   │   ├── P2PChatView.tsx (12KB) [277L]
│   │   ├── Paint3DView.tsx (5KB) [140L]
│   │   ├── PaymentSystemView.tsx (4KB) [93L]
│   │   ├── PipelineGeneratorTab.tsx (19KB) [433L]
│   │   ├── PoAITrainingEngineView.tsx (8KB) [173L]
│   │   ├── ProjectAuditDashboard.tsx (7KB) [135L]
│   │   ├── ProjectHubView.tsx (30KB) [501L]
│   │   ├── ProtocolsView.tsx (8KB) [207L]
│   │   ├── ReportsView.tsx (10KB) [202L]
│   │   ├── RepositoryActivityChart.tsx (5KB) [145L]
│   │   ├── RepositoryLineChart.tsx (6KB) [198L]
│   │   ├── RescueSystemView.tsx (16KB) [307L]
│   │   ├── RoadmapView.tsx (6KB) [196L]
│   │   ├── SemanticGraphView.tsx (4KB) [86L]
│   │   ├── SessionExportView.tsx (8KB) [221L]
│   │   ├── SettingsView.tsx (105KB) [2312L]
│   │   ├── SocialMediaView.tsx (16KB) [287L]
│   │   ├── SoftwareAuditView.tsx (38KB) [885L]
│   │   ├── SoftwareKnowledgeDbView.tsx (18KB) [380L]
│   │   ├── SourceCodeViewer.tsx (20KB) [547L]
│   │   ├── SpecificSettingsViews.tsx (17KB) [306L]
│   │   ├── StorageManagerView.tsx (9KB) [258L]
│   │   ├── StrategicArchitectureMap.tsx (9KB) [243L]
│   │   ├── StructureView.tsx (22KB) [505L]
│   │   ├── SyncDashboardModal.tsx (4KB) [88L]
│   │   ├── SyncHistoryModal.tsx (11KB) [249L]
│   │   ├── SyncMetricsView.tsx (7KB) [170L]
│   │   ├── SyncStatusDonutChart.tsx (2KB) [99L]
│   │   ├── SyncStatusOverview.tsx (7KB) [168L]
│   │   ├── SystemDiagnosticsView.tsx (18KB) [337L]
│   │   ├── SystemFinderView.tsx (2KB) [56L]
│   │   ├── SystemHealthDashboard.tsx (9KB) [246L]
│   │   ├── SystemHealthDashboardWidget.tsx (2KB) [63L]
│   │   ├── SystemLogsView.tsx (3KB) [89L]
│   │   ├── TaskManagerView.tsx (3KB) [82L]
│   │   ├── TechDocsView.tsx (17KB) [335L]
│   │   ├── TechTreeView.tsx (18KB) [420L]
│   │   ├── TerminalView.tsx (6KB) [189L]
│   │   ├── TestnetOrchestrationView.tsx (8KB) [178L]
│   │   ├── TestnetSimulationView.tsx (14KB) [298L]
│   │   ├── TextGeneratorTab.tsx (6KB) [177L]
│   │   ├── ThemeSwitcher.tsx (4KB) [143L]
│   │   ├── TodoView.tsx (18KB) [383L]
│   │   ├── TooltipIcon.tsx (1KB) [29L]
│   │   ├── TxOrchestratorView.tsx (5KB) [105L]
│   │   ├── UserProfileView.tsx (12KB) [255L]
│   │   ├── VideoGeneratorTab.tsx (7KB) [176L]
│   │   ├── WebhookMonitor.tsx (5KB) [145L]
│   │   ├── Window.tsx (6KB) [158L]
│   │   ├── WindowExtras.tsx (4KB) [87L]
│   │   ├── ZeroKnowledgeProofView.tsx (6KB) [129L]
│   │   ├── ZkCircuitEditorView.tsx (4KB) [108L]
│   │   └── ZkVisualizationView.tsx (3KB) [99L]
│   ├── contexts/ (4 files, 269 lines)
│   │   ├── FirebaseContext.tsx (2KB) [94L]
│   │   ├── GoogleWorkspaceContext.tsx (2KB) [83L]
│   │   ├── SyncMetricsContext.tsx (1KB) [47L]
│   │   └── WalletContext.tsx (1KB) [45L]
│   ├── db/ (3 files, 64 lines)
│   │   ├── drizzle.config.ts (817B) [29L]
│   │   ├── index.ts (652B) [24L]
│   │   └── schema.ts (486B) [11L]
│   ├── hooks/ (2 files, 250 lines)
│   │   ├── useGoogleSheetsSync.ts (8KB) [220L]
│   │   └── useKeyboardShortcut.ts (899B) [30L]
│   ├── lib/ (6 files, 359 lines)
│   │   ├── CryptoEngine.ts (1KB) [42L]
│   │   ├── firebase-admin.ts (544B) [15L]
│   │   ├── firebase.ts (2KB) [64L]
│   │   ├── indexedDb.ts (2KB) [88L]
│   │   ├── syncLogic.test.ts (2KB) [82L] 🧪
│   │   └── syncLogic.ts (1KB) [68L]
│   ├── middleware/ (1 files, 30 lines)
│   │   └── auth.ts (953B) [30L]
│   ├── routes/ (1 files, 146 lines)
│   │   └── notion.ts (4KB) [146L]
│   ├── services/ (2 files, 143 lines)
│   │   ├── SyncService.ts (3KB) [106L]
│   │   └── githubSync.ts (1KB) [37L]
│   ├── utils/ (4 files, 240 lines)
│   │   ├── appSync.tsx (2KB) [84L]
│   │   ├── auditUtils.test.ts (1KB) [56L] 🧪
│   │   ├── auditUtils.ts (749B) [27L]
│   │   └── crypto.ts (4KB) [73L]
│   ├── App.tsx (233KB) [5440L]
│   ├── DesktopApp.tsx (121KB) [2740L]
│   ├── atcLangRoadmapData.ts (6KB) [201L]
│   ├── atcLangWikiData.ts (16KB) [227L]
│   ├── auditData.ts (4KB) [76L]
│   ├── data.ts (17KB) [411L]
│   ├── ecosystemData.ts (11KB) [291L]
│   ├── fix_translation.cjs (463B)
│   ├── index.css (5KB)
│   ├── main.tsx (774B) [24L]
│   ├── marketplaceApps.ts (6KB) [273L]
│   ├── requirementsData.ts (1KB) [58L]
│   ├── roadmapData.ts (7KB) [312L]
│   ├── standardsData.ts (4KB) [83L]
│   ├── tierData.ts (16KB) [317L]
│   ├── types.ts (375B) [10L]
│   └── wikiData.ts (47KB) [943L]
├── tests/ (2 files, 127 lines)
│   ├── GitHubRepoSyncView.test.tsx (1KB) [49L] 🧪
│   └── audit_compliance.test.ts (2KB) [78L] 🧪
├── workspace/ (8 files, 664 lines)
│   ├── src/ (2 files, 435 lines)
│   │   ├── backend/ (1 files, 167 lines)
│   │   │   └── blockchain/ (1 files, 167 lines)
│   │   │       └── engine.ts (5KB) [167L]
│   │   └── components/ (1 files, 268 lines)
│   │       └── GovernanceView.tsx (14KB) [268L]
│   ├── move.js (411B) [13L]
│   ├── rename.js (1KB) [42L]
│   ├── replace.js (1KB) [40L]
│   ├── replaceEnterprise.js (3KB) [102L]
│   ├── replaceGoals.ts (688B) [14L]
│   └── replaceGoals2.ts (825B) [18L]
├── .env.example (578B)
├── .gitignore (73B)
├── AGENTS.md (535B) [13L]
├── CHANGELOG.md (426B) [21L]
├── FILE_REGISTER.md (12KB) [253L]
├── GEMINI.md (373B) [6L]
├── LICENSE (1KB)
├── README.md (542B) [20L]
├── ROADMAP.md (8KB) [598L]
├── SOFTWARE_ROADMAP.md (38KB) [1116L]
├── STATUS.md (349B) [19L]
├── check_dups2.js (498B) [12L]
├── check_dups_all.js (885B) [23L]
├── check_dups_desktop.js (480B) [15L]
├── check_dups_windows_map.js (519B) [14L]
├── fetch.js (1KB) [36L]
├── firebase-applet-config.json (363B) [9L]
├── fix.js (859B) [26L]
├── fix2.js (894B) [27L]
├── fix_react_imports.cjs (547B)
├── fix_wiki.cjs (184B)
├── fix_wiki.js (284B) [5L]
├── index.html (413B)
├── mark_completed.ts (722B) [15L]
├── mark_completed_src.ts (1KB) [33L]
├── metadata.json (214B) [6L]
├── move_back.js (347B) [11L]
├── output.txt (3KB)
├── package-lock.json (420KB) [11890L]
├── package.json (2KB) [75L]
├── replace.js (1KB) [36L]
├── replace_langs.cjs (852B)
├── replace_langs_2.cjs (667B)
├── replace_langs_3.cjs (411B)
├── replace_langs_4.cjs (817B)
├── replace_langs_5.cjs (528B)
├── replace_langs_6.cjs (522B)
├── script.cjs (883B)
├── script.js (983B) [12L]
├── script2.cjs (683B)
├── server.ts (33KB) [866L]
├── testChat.js (450B) [10L]
├── test_know.js (244B) [2L] 🧪
├── tmp.txt (470B)
├── tsconfig.json (508B) [26L]
├── update_wiki_categories.ts (742B) [23L]
└── vite.config.ts (1KB) [42L]
```


## atc-aistudio-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-aistudio-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (323B) [14L]
├── FILE_REGISTER.md (427B) [14L]
├── LICENSE (472B)
├── MODULES.md (311B) [10L]
├── README.md (513B) [18L]
└── STATUS.md (298B) [15L]
```


## atc-atclang

**Layer:** L2–L4 (ATCLang Sync) | **Dateien:** 41 | **Verzeichnisse:** 8 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .py(30) · .md(7) · .gitignore(1) · .txt(1) · .no-ext(1) · .atc(1)

```
atc-atclang/
├── compiler/ (4 files, 1634 lines)
│   ├── __init__.py (468B) [8L]
│   ├── compiler.py (21KB) [561L]
│   ├── optimizer.py (22KB) [558L]
│   └── type_checker.py (20KB) [507L]
├── lexer/ (2 files, 673 lines)
│   ├── __init__.py (161B) [2L]
│   └── lexer.py (24KB) [671L]
├── parser/ (3 files, 1826 lines)
│   ├── __init__.py (189B) [3L]
│   ├── ast_nodes.py (8KB) [392L]
│   └── parser.py (63KB) [1431L]
├── programs/ (1 files, 1161 lines)
│   └── atcos_main.atc (40KB) [1161L]
├── repl/ (2 files, 185 lines)
│   ├── __init__.py (99B) [1L]
│   └── repl.py (6KB) [184L]
├── stdlib/ (14 files, 1823 lines)
│   ├── __init__.py (1KB) [32L]
│   ├── atc_stdlib.py (2KB) [69L]
│   ├── chain.py (1KB) [41L]
│   ├── collections.py (5KB) [219L]
│   ├── collections_ext.py (3KB) [143L]
│   ├── crypto.py (5KB) [155L]
│   ├── crypto_ext.py (5KB) [149L]
│   ├── encoding.py (7KB) [210L]
│   ├── io.py (3KB) [107L]
│   ├── io_ext.py (3KB) [123L]
│   ├── math.py (4KB) [154L]
│   ├── primitives.py (7KB) [244L]
│   ├── string.py (2KB) [99L]
│   └── wallet.py (2KB) [78L]
├── v03/ (2 files, 354 lines)
│   ├── __init__.py (124B) [2L]
│   └── atclang_v03_features.py (13KB) [352L]
├── vm/ (2 files, 999 lines)
│   ├── __init__.py (177B) [2L]
│   └── atcvm.py (48KB) [997L]
├── .gitignore (171B)
├── ATCLANG_SPEC.md (9KB) [295L]
├── CHANGELOG.md (316B) [8L]
├── CONTRIBUTING.md (687B) [19L]
├── FILE_REGISTER.md (1KB) [48L]
├── LICENSE (658B)
├── README.md (5KB) [127L]
├── ROADMAP.md (478B) [21L]
├── STATUS.md (346B) [19L]
├── __init__.py (462B) [11L]
└── requirements.txt (75B)
```


## atc-atclang-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-atclang-wiki/
├── docs/ (3 files, 134 lines)
│   ├── ARCHITECTURE.md (2KB) [69L]
│   ├── MODULES.md (1KB) [43L]
│   └── ROADMAP.md (933B) [22L]
├── .gitignore (44B)
├── ARCHITECTURE.md (316B) [14L]
├── FILE_REGISTER.md (512B) [16L]
├── LICENSE (472B)
├── MODULES.md (309B) [10L]
├── README.md (3KB) [61L]
└── STATUS.md (296B) [15L]
```


## atc-atcpkg

**Layer:** L7 (Package Manager) | **Dateien:** 13 | **Verzeichnisse:** 3 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(9) · .atc(2) · .gitignore(1) · .no-ext(1)

```
atc-atcpkg/
├── docs/ (4 files, 405 lines)
│   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   └── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
├── kernel/ (1 files, 208 lines)
│   └── manager.atc (6KB) [208L]
├── tools/ (1 files, 145 lines)
│   └── manager.atc (4KB) [145L]
├── .gitignore (171B)
├── CHANGELOG.md (424B) [21L]
├── FILE_REGISTER.md (748B) [20L]
├── LICENSE (658B)
├── README.md (4KB) [101L]
├── ROADMAP.md (476B) [21L]
└── STATUS.md (355B) [19L]
```


## atc-atcpkg-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-atcpkg-wiki/
├── docs/ (2 files, 44 lines)
│   ├── ARCHITECTURE.md (1KB) [28L]
│   └── ROADMAP.md (568B) [16L]
├── .gitignore (44B)
├── ARCHITECTURE.md (321B) [14L]
├── FILE_REGISTER.md (477B) [15L]
├── LICENSE (472B)
├── MODULES.md (307B) [10L]
├── README.md (3KB) [60L]
└── STATUS.md (294B) [15L]
```


## atc-backend

**Layer:** L7 (Backend Services) | **Dateien:** 27 | **Verzeichnisse:** 5 | **Max Tiefe:** 2 | **Tests:** 0

**Beschreibung:** FastAPI REST API, JSON-RPC Server & Backend Orchestrator für A-TownChain OS

**Nutzt:** atc-gateway, atc-kernel, atc-shivacore, atc-ui
**Wird genutzt von:** atc-gateway, atc-ui

**Sprachen:** .py(9) · .atc(8) · .md(5) · .example(1) · .no-ext(1) · .txt(1)

```
atc-backend/
├── api/ (9 files, 1099 lines)
│   ├── orchestrator/ (3 files, 391 lines)
│   │   ├── __init__.py (118B) [2L]
│   │   ├── orchestrator.atc (8KB) [259L]
│   │   └── orchestrator.py (4KB) [130L]
│   ├── routes/ (3 files, 409 lines)
│   │   ├── __init__.py (115B) [2L]
│   │   ├── ai_routes.atc (5KB) [175L]
│   │   └── api_routes.atc (8KB) [232L]
│   ├── __init__.py (111B) [2L]
│   ├── kai_routes.atc (7KB) [229L]
│   └── server.atc (2KB) [68L]
├── db/ (6 files, 591 lines)
│   ├── __init__.py (160B) [2L]
│   ├── connection.atc (4KB) [125L]
│   ├── connection.py (1KB) [40L]
│   ├── repository.atc (6KB) [228L]
│   ├── repository.py (6KB) [196L]
│   └── schema.sql (2KB)
├── wallet/ (2 files, 141 lines)
│   ├── __init__.py (123B) [2L]
│   └── wallet.atc (4KB) [139L]
├── .env.example (167B)
├── .gitignore (171B)
├── CHANGELOG.md (425B) [21L]
├── FILE_REGISTER.md (1KB) [34L]
├── LICENSE (658B)
├── README.md (6KB) [139L]
├── ROADMAP.md (478B) [21L]
├── STATUS.md (346B) [19L]
├── __init__.py (121B) [2L]
└── requirements.txt (425B)
```


## atc-backend-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-backend-wiki/
├── docs/ (3 files, 112 lines)
│   ├── API.md (941B) [61L]
│   ├── ARCHITECTURE.md (1KB) [35L]
│   └── ROADMAP.md (453B) [16L]
├── .gitignore (44B)
├── ARCHITECTURE.md (316B) [14L]
├── FILE_REGISTER.md (495B) [16L]
├── LICENSE (472B)
├── MODULES.md (309B) [10L]
├── README.md (4KB) [70L]
└── STATUS.md (296B) [15L]
```


## atc-blockchain

**Layer:** L3–L4 (Blockchain) | **Dateien:** 78 | **Verzeichnisse:** 18 | **Max Tiefe:** 3 | **Tests:** 0

**Beschreibung:** Blockchain Core: Consensus (PoH + PoA), Block Production, Mempool, Validators, Smart Contracts

**Sprachen:** .atc(43) · .py(26) · .md(6) · .no-ext(1) · .gitignore(1) · .js(1)

```
atc-blockchain/
├── atcoin/ (1 files, 2 lines)
│   └── __init__.py (119B) [2L]
├── consensus/ (15 files, 1628 lines)
│   ├── MIGRATION_INDEX.md (661B) [13L]
│   ├── __init__.py (123B) [2L]
│   ├── fork_atc85.atc (2KB) [74L]
│   ├── fork_resolution.atc (4KB) [145L]
│   ├── gas_fee.atc (4KB) [130L]
│   ├── gas_fee_atc86.atc (2KB) [71L]
│   ├── hybrid_atc84.atc (3KB) [98L]
│   ├── hybrid_consensus.atc (11KB) [357L]
│   ├── poh.atc (4KB) [140L]
│   ├── poh.py (2KB) [67L]
│   ├── poh_atc83.atc (1KB) [79L]
│   ├── pos.atc (4KB) [164L]
│   ├── pos_atc82.atc (2KB) [92L]
│   ├── pow.atc (3KB) [107L]
│   └── pow_atc81.atc (2KB) [89L]
├── contracts/ (13 files, 1317 lines)
│   ├── atc001/ (3 files, 176 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   ├── genesis_token.atc (2KB) [102L]
│   │   └── genesis_token.py (2KB) [74L]
│   ├── atc8300/ (2 files, 128 lines)
│   │   ├── __init__.py (129B) [2L]
│   │   └── atc8300_token.py (5KB) [126L]
│   ├── base/ (2 files, 87 lines)
│   │   ├── __init__.py (0B) [0L]
│   │   └── base_contract.py (3KB) [87L]
│   ├── governance/ (1 files, 202 lines)
│   │   └── governance_contract.atc (7KB) [202L]
│   ├── shivamon/ (2 files, 141 lines)
│   │   ├── __init__.py (136B) [2L]
│   │   └── breeding.atc (5KB) [139L]
│   ├── solidity/ (1 files, 274 lines)
│   │   └── test/ (1 files, 274 lines)
│   │       └── ATCBridge.test.js (11KB) [274L] 🧪
│   ├── __init__.py (0B) [0L]
│   └── contract_engine_atc14.atc (9KB) [309L]
├── dex/ (2 files, 279 lines)
│   ├── __init__.py (117B) [2L]
│   └── amm.atc (10KB) [277L]
├── governance/ (5 files, 775 lines)
│   ├── __init__.py (120B) [2L]
│   ├── dao.atc (6KB) [168L]
│   ├── dao_live.atc (8KB) [235L]
│   ├── timelock.atc (4KB) [150L]
│   └── treasury.atc (6KB) [220L]
├── mainnet/ (3 files, 258 lines)
│   ├── __init__.py (117B) [2L]
│   ├── launch_manager.atc (3KB) [105L]
│   └── mainnet_config.atc (5KB) [151L]
├── network/ (7 files, 746 lines)
│   ├── atc-02_liquid_state_migration_failover.atc (2KB) [58L]
│   ├── atc-04_dag_consensus_propagation.atc (2KB) [58L]
│   ├── atc-05_quantumresistant_signatures.atc (2KB) [58L]
│   ├── atc-10_global_time_sync_oracles.atc (1KB) [58L]
│   ├── core_node_atc01.atc (4KB) [164L]
│   ├── latency_opt_atc06.atc (3KB) [135L]
│   └── sharding_atc07.atc (5KB) [215L]
├── nodes/ (9 files, 1806 lines)
│   ├── __init__.py (126B) [2L]
│   ├── block_propagation.atc (3KB) [87L]
│   ├── bootstrap.atc (6KB) [234L]
│   ├── bootstrap.py (8KB) [257L]
│   ├── discovery.py (11KB) [314L]
│   ├── initial_sync.atc (6KB) [207L]
│   ├── node.atc (6KB) [192L]
│   ├── p2p_propagation.py (12KB) [381L]
│   └── testnet_launcher.atc (4KB) [132L]
├── propagation/ (1 files, 98 lines)
│   └── block_gossip.atc (3KB) [98L]
├── wallet/ (7 files, 757 lines)
│   ├── __init__.py (128B) [2L]
│   ├── did.atc (4KB) [122L]
│   ├── did.py (2KB) [74L]
│   ├── ecdsa.py (2KB) [72L]
│   ├── multisig.atc (8KB) [268L]
│   ├── multisig.py (3KB) [107L]
│   └── wordlist.atc (5KB) [112L]
├── zkp/ (2 files, 93 lines)
│   ├── __init__.py (336B) [4L]
│   └── groth16.atc (3KB) [89L]
├── .gitignore (171B)
├── CHANGELOG.md (428B) [21L]
├── FILE_REGISTER.md (3KB) [109L]
├── LICENSE (658B)
├── README.md (6KB) [146L]
├── ROADMAP.md (484B) [21L]
├── STATUS.md (349B) [19L]
├── __init__.py (0B) [0L]
├── contract_registry.atc (3KB) [98L]
├── smart_contract_registry.atc (2KB) [88L]
├── smart_contract_registry.py (1KB) [53L]
├── smart_contracts.atc (15KB) [486L]
└── smart_contracts.py (23KB) [716L]
```


## atc-blockchain-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-blockchain-wiki/
├── docs/ (5 files, 207 lines)
│   ├── ARCHITECTURE.md (2KB) [61L]
│   ├── CONSENSUS.md (1KB) [45L]
│   ├── MEMPOOL.md (1KB) [35L]
│   ├── ROADMAP.md (1KB) [30L]
│   └── VALIDATORS.md (1KB) [36L]
├── .gitignore (44B)
├── FILE_REGISTER.md (3KB) [109L]
├── LICENSE (472B)
└── README.md (4KB) [74L]
```


## atc-bootloader

**Layer:** L1 (Bootloader) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Low-Level Bootloader: BIOS/UEFI → ShivaOS Kernel → A-TownChain Handshake

**Nutzt:** atc-kernel
**Wird genutzt von:** atc-kernel

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-bootloader/
├── .gitignore (116B)
├── CHANGELOG.md (223B) [8L]
├── FILE_REGISTER.md (388B) [13L]
├── LICENSE (703B)
├── README.md (5KB) [107L]
├── ROADMAP.md (376B) [16L]
└── STATUS.md (491B) [21L]
```


## atc-bootloader-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-bootloader-wiki/
├── docs/ (2 files, 30 lines)
│   ├── ARCHITECTURE.md (892B) [22L]
│   └── ROADMAP.md (218B) [8L]
├── .gitignore (44B)
├── ARCHITECTURE.md (316B) [14L]
├── FILE_REGISTER.md (467B) [15L]
├── LICENSE (472B)
├── MODULES.md (315B) [10L]
├── README.md (3KB) [68L]
└── STATUS.md (302B) [15L]
```


## atc-ci

**Layer:** L0 (CI/CD Pipeline) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** CI/CD Build, Testing & Automated Deployment Pipelines für A-TownChain OS

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-ci/
├── .gitignore (116B)
├── CHANGELOG.md (215B) [8L]
├── FILE_REGISTER.md (380B) [13L]
├── LICENSE (703B)
├── README.md (5KB) [109L]
├── ROADMAP.md (368B) [16L]
└── STATUS.md (483B) [21L]
```


## atc-ci-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-ci-wiki/
├── docs/ (2 files, 30 lines)
│   ├── ROADMAP.md (176B) [7L]
│   └── WORKFLOWS.md (668B) [23L]
├── .gitignore (44B)
├── ARCHITECTURE.md (304B) [14L]
├── FILE_REGISTER.md (456B) [15L]
├── LICENSE (472B)
├── MODULES.md (299B) [10L]
├── README.md (3KB) [68L]
└── STATUS.md (286B) [15L]
```


## atc-cli

**Layer:** L7 (CLI Tool) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Offizielles Command-Line Interface (CLI) für A-TownChain OS & KAI-OS

**Nutzt:** atc-gateway, atc-sdk

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-cli/
├── .gitignore (116B)
├── CHANGELOG.md (216B) [8L]
├── FILE_REGISTER.md (381B) [13L]
├── LICENSE (703B)
├── README.md (5KB) [120L]
├── ROADMAP.md (369B) [16L]
└── STATUS.md (484B) [21L]
```


## atc-cli-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-cli-wiki/
├── docs/ (2 files, 32 lines)
│   ├── COMMANDS.md (746B) [25L]
│   └── ROADMAP.md (172B) [7L]
├── .gitignore (44B)
├── ARCHITECTURE.md (300B) [14L]
├── FILE_REGISTER.md (456B) [15L]
├── LICENSE (472B)
├── MODULES.md (301B) [10L]
├── README.md (3KB) [68L]
└── STATUS.md (288B) [15L]
```


## atc-contracts

**Layer:** L4 (Smart Contracts) | **Dateien:** 23 | **Verzeichnisse:** 8 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** Smart Contracts: Token-Standards (ATC-8300, ATCoin), Bridge, Governance, Marketplace, Shivamon

**Sprachen:** .py(9) · .md(7) · .atc(4) · .gitignore(1) · .no-ext(1) · .txt(1)

```
atc-contracts/
├── atc8300/ (2 files, 222 lines)
│   ├── atc8300.atc (3KB) [96L]
│   └── atc8300_token.py (5KB) [126L]
├── atcoin/ (1 files, 139 lines)
│   └── atcoin.py (5KB) [139L]
├── base/ (1 files, 87 lines)
│   └── base_contract.py (3KB) [87L]
├── bridge/ (1 files, 133 lines)
│   └── bridge_contract.py (4KB) [133L]
├── governance/ (2 files, 412 lines)
│   ├── governance.atc (4KB) [113L]
│   └── governance_contract.py (11KB) [299L]
├── marketplace/ (1 files, 301 lines)
│   └── marketplace_contract.py (11KB) [301L]
├── shivamon/ (2 files, 432 lines)
│   ├── shivamon.atc (5KB) [162L]
│   └── shivamon_contract.py (10KB) [270L]
├── wallet/ (3 files, 336 lines)
│   ├── ecdsa.py (2KB) [72L]
│   ├── keygen.py (5KB) [140L]
│   └── wallet.atc (4KB) [124L]
├── .gitignore (171B)
├── CHANGELOG.md (304B) [20L]
├── DEPLOYMENT.md (894B) [29L]
├── FILE_REGISTER.md (1KB) [54L]
├── LICENSE (982B)
├── README.md (5KB) [121L]
├── ROADMAP.md (482B) [21L]
├── SECURITY.md (496B) [13L]
├── STATUS.md (358B) [19L]
└── requirements.txt (100B)
```


## atc-contracts-wiki

**Layer:** ? | **Dateien:** 12 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(10) · .gitignore(1) · .no-ext(1)

```
atc-contracts-wiki/
├── docs/ (8 files, 290 lines)
│   ├── ATC8300.md (1KB) [51L]
│   ├── ATC9000.md (2KB) [92L]
│   ├── ATC9900.md (514B) [20L]
│   ├── BRIDGE.md (1KB) [38L]
│   ├── DEPLOYMENT.md (603B) [25L]
│   ├── ROADMAP.md (455B) [17L]
│   ├── SECURITY.md (708B) [26L]
│   └── TODO.md (526B) [21L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [51L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-dns

**Layer:** L5 (DNS) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** On-Chain Decentralized Domain Name System (.atc TLD) & Resolution Service

**Nutzt:** atc-contracts, atc-gateway, atc-kernel, atc-ui
**Wird genutzt von:** atc-gateway, atc-ui

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-dns/
├── .gitignore (116B)
├── CHANGELOG.md (216B) [8L]
├── FILE_REGISTER.md (381B) [13L]
├── LICENSE (703B)
├── README.md (4KB) [107L]
├── ROADMAP.md (369B) [16L]
└── STATUS.md (484B) [21L]
```


## atc-dns-wiki

**Layer:** ? | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(7) · .gitignore(1) · .no-ext(1)

```
atc-dns-wiki/
├── docs/ (2 files, 28 lines)
│   ├── ARCHITECTURE.md (658B) [21L]
│   └── ROADMAP.md (184B) [7L]
├── .gitignore (44B)
├── ARCHITECTURE.md (309B) [14L]
├── FILE_REGISTER.md (460B) [15L]
├── LICENSE (472B)
├── MODULES.md (301B) [10L]
├── README.md (3KB) [68L]
└── STATUS.md (288B) [15L]
```


## atc-drivers

**Layer:** L1 (Hardware Drivers) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Hardware Drivers & Device Abstraction Layer (HAL) für ShivaOS Kernel

**Nutzt:** atc-kernel
**Wird genutzt von:** atc-kernel

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-drivers/
├── .gitignore (116B)
├── CHANGELOG.md (220B) [8L]
├── FILE_REGISTER.md (385B) [13L]
├── LICENSE (703B)
├── README.md (4KB) [108L]
├── ROADMAP.md (373B) [16L]
└── STATUS.md (488B) [21L]
```


## atc-drivers-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-drivers-wiki/
├── docs/ (3 files, 30 lines)
│   ├── ARCHITECTURE.md (373B) [11L]
│   ├── DRIVER_LIST.md (451B) [11L]
│   └── ROADMAP.md (194B) [8L]
├── .gitignore (44B)
├── ARCHITECTURE.md (323B) [14L]
├── FILE_REGISTER.md (502B) [16L]
├── LICENSE (472B)
├── MODULES.md (309B) [10L]
├── README.md (3KB) [69L]
└── STATUS.md (296B) [15L]
```


## atc-explorer

**Layer:** L7 (Block Explorer) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Block Explorer & Chain Analytics für A-TownChain OS

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-explorer/
├── .gitignore (116B)
├── CHANGELOG.md (221B) [8L]
├── FILE_REGISTER.md (496B) [18L]
├── LICENSE (703B)
├── README.md (4KB) [104L]
├── ROADMAP.md (374B) [16L]
└── STATUS.md (489B) [21L]
```


## atc-explorer-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-explorer-wiki/
├── docs/ (3 files, 32 lines)
│   ├── API.md (330B) [9L]
│   ├── ARCHITECTURE.md (296B) [9L]
│   └── ROADMAP.md (406B) [14L]
├── .gitignore (44B)
├── FILE_REGISTER.md (523B) [18L]
├── LICENSE (472B)
└── README.md (4KB) [71L]
```


## atc-franchise

**Layer:** L8–L10 (Business DAOs) | **Dateien:** 15 | **Verzeichnisse:** 3 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** Business DAO: Vault, Revenue-Share, Royalty (ATC-9900)

**Nutzt:** atc-contracts, atc-gateway

**Sprachen:** .md(7) · .atc(3) · .py(2) · .gitignore(1) · .no-ext(1) · .txt(1)

```
atc-franchise/
├── api/ (1 files, 67 lines)
│   └── routes.py (2KB) [67L]
├── contracts/ (3 files, 285 lines)
│   ├── registry.atc (4KB) [120L]
│   ├── revenue.atc (3KB) [93L]
│   └── token.atc (3KB) [72L]
├── docs/ (2 files, 80 lines)
│   ├── ARCHITECTURE.md (666B) [23L]
│   └── SECURITY.md (1KB) [57L]
├── .gitignore (171B)
├── CHANGELOG.md (182B) [6L]
├── FILE_REGISTER.md (658B) [20L]
├── LICENSE (982B)
├── README.md (4KB) [69L]
├── ROADMAP.md (482B) [21L]
├── STATUS.md (358B) [19L]
├── factory.py (4KB) [138L]
└── requirements.txt (115B)
```


## atc-franchise-wiki

**Layer:** ? | **Dateien:** 11 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(9) · .no-ext(1) · .gitignore(1)

```
atc-franchise-wiki/
├── docs/ (7 files, 244 lines)
│   ├── API.md (1KB) [37L]
│   ├── CONCEPT.md (1000B) [24L]
│   ├── CONTRACTS.md (1KB) [49L]
│   ├── DEPLOYMENT.md (879B) [43L]
│   ├── ROADMAP.md (685B) [21L]
│   ├── SECURITY.md (904B) [29L]
│   └── TOKEN_ECONOMY.md (1KB) [41L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [43L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-frontend

**Layer:** L10 (Frontend) | **Dateien:** 12 | **Verzeichnisse:** 5 | **Max Tiefe:** 2 | **Tests:** 0

**Beschreibung:** React/TypeScript Desktop UI & Neon Dashboard für KAI-OS

**Sprachen:** .md(6) · .html(2) · .gitignore(1) · .no-ext(1) · .css(1) · .js(1)

```
atc-frontend/
├── assets/ (2 files, 136 lines)
│   ├── css/ (1 files, 0 lines)
│   │   └── variables.css (807B)
│   └── js/ (1 files, 136 lines)
│       └── api.js (4KB) [136L]
├── battle/ (1 files, 0 lines)
│   └── index.html (13KB)
├── bootscreen/ (1 files, 48 lines)
│   └── README.md (1KB) [48L]
├── .gitignore (171B)
├── CHANGELOG.md (426B) [21L]
├── FILE_REGISTER.md (689B) [23L]
├── LICENSE (658B)
├── README.md (4KB) [111L]
├── ROADMAP.md (480B) [21L]
├── STATUS.md (347B) [19L]
└── index.html (120KB)
```


## atc-frontend-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-frontend-wiki/
├── docs/ (3 files, 41 lines)
│   ├── ARCHITECTURE.md (733B) [18L]
│   ├── COMPONENTS.md (463B) [8L]
│   └── ROADMAP.md (506B) [15L]
├── .gitignore (44B)
├── FILE_REGISTER.md (531B) [18L]
├── LICENSE (472B)
└── README.md (4KB) [71L]
```


## atc-gateway

**Layer:** L7 (API Gateway) | **Dateien:** 43 | **Verzeichnisse:** 6 | **Max Tiefe:** 2 | **Tests:** 0

**Beschreibung:** API Gateway Port 4000: REST, WebSocket, Rate Limiting, Middleware, Service Discovery

**Sprachen:** .py(16) · .atc(11) · .md(10) · .txt(3) · .gitignore(1) · .no-ext(1)

```
atc-gateway/
├── atclang/ (11 files, 581 lines)
│   ├── middleware/ (4 files, 245 lines)
│   │   ├── auth.atc (2KB) [82L]
│   │   ├── logger.atc (2KB) [70L]
│   │   ├── rate_limit.atc (1KB) [50L]
│   │   └── signature_verify.atc (1KB) [43L]
│   ├── .env.example (103B)
│   ├── CHANGELOG.md (274B) [8L]
│   ├── README.md (858B) [39L]
│   ├── SECURITY.md (371B) [13L]
│   ├── main.atc (5KB) [180L]
│   ├── requirements.txt (162B)
│   └── router.atc (3KB) [96L]
├── docs/ (1 files, 112 lines)
│   └── ARCHITECTURE.md (2KB) [112L]
├── middleware/ (5 files, 113 lines)
│   ├── __init__.py (120B) [2L]
│   ├── auth.py (669B) [19L]
│   ├── logger.py (324B) [9L]
│   ├── rate_limit.py (1KB) [26L]
│   └── signature_verify.py (1KB) [57L]
├── python/ (11 files, 507 lines)
│   ├── middleware/ (5 files, 113 lines)
│   │   ├── __init__.py (120B) [2L]
│   │   ├── auth.py (669B) [19L]
│   │   ├── logger.py (324B) [9L]
│   │   ├── rate_limit.py (1KB) [26L]
│   │   └── signature_verify.py (1KB) [57L]
│   ├── __init__.py (125B) [2L]
│   ├── main.atc (5KB) [127L]
│   ├── main.py (1KB) [47L]
│   ├── requirements.txt (69B)
│   ├── router.py (2KB) [50L]
│   └── service_discovery.atc (6KB) [168L]
├── .gitignore (171B)
├── CHANGELOG.md (328B) [14L]
├── FILE_REGISTER.md (2KB) [74L]
├── LICENSE (982B)
├── README.md (5KB) [115L]
├── ROADMAP.md (478B) [21L]
├── SECURITY.md (371B) [13L]
├── STATUS.md (356B) [19L]
├── __init__.py (125B) [2L]
├── gateway.atc (4KB) [138L]
├── main.atc (5KB) [127L]
├── main.py (1KB) [47L]
├── requirements.txt (69B)
├── router.py (2KB) [50L]
└── service_discovery.atc (6KB) [168L]
```


## atc-gateway-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .no-ext(1) · .gitignore(1)

```
atc-gateway-wiki/
├── docs/ (6 files, 161 lines)
│   ├── AUTH.md (965B) [43L]
│   ├── MIDDLEWARE.md (368B) [14L]
│   ├── RATE_LIMITING.md (956B) [43L]
│   ├── ROADMAP.md (436B) [16L]
│   ├── ROUTES.md (995B) [32L]
│   └── SECURITY.md (372B) [13L]
├── .gitignore (171B)
├── FILE_REGISTER.md (2KB) [71L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-genesis-engine

**Layer:** L0 (Genesis Engine) | **Dateien:** 20 | **Verzeichnisse:** 4 | **Max Tiefe:** 2 | **Tests:** 1

**Sprachen:** .md(13) · .py(4) · .gitignore(1) · .no-ext(1) · .txt(1)

**Test-Dateien:** 1× Python

```
atc-genesis-engine/
├── engine/ (6 files, 298 lines)
│   ├── core/ (1 files, 98 lines)
│   │   └── ecs.py (2KB) [98L]
│   ├── render/ (1 files, 45 lines)
│   │   └── renderer2d.py (1KB) [45L]
│   ├── tests/ (1 files, 63 lines)
│   │   └── test_ecs.py (1KB) [63L] 🧪
│   ├── MILESTONE_1.md (1KB) [44L]
│   ├── main.py (1KB) [48L]
│   └── requirements.txt (14B)
├── .gitignore (171B)
├── ARCHITECTURE.md (4KB) [103L]
├── CHANGELOG.md (432B) [21L]
├── FILE_REGISTER.md (849B) [24L]
├── FRANCHISE_FACTORY.md (3KB) [66L]
├── FRANCHISE_FACTORY_V2.md (3KB) [108L]
├── GENESIS_NEXUS_V5.md (3KB) [65L]
├── GENESIS_OS_V4.md (3KB) [70L]
├── LICENSE (658B)
├── METAFACTORY_V3.md (4KB) [83L]
├── README.md (4KB) [84L]
├── ROADMAP.md (492B) [21L]
├── STATUS.md (353B) [19L]
└── VISION_EVOLUTION_LOG.md (8KB) [157L]
```


## atc-genesis-engine-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-genesis-engine-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (328B) [14L]
├── FILE_REGISTER.md (433B) [14L]
├── LICENSE (472B)
├── MODULES.md (323B) [10L]
├── README.md (530B) [18L]
└── STATUS.md (310B) [15L]
```


## atc-ide

**Layer:** L10 (IDE/Playground) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** IDE Extensions & Language Server Protocol (LSP) für ATCLang

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-ide/
├── .gitignore (116B)
├── CHANGELOG.md (216B) [8L]
├── FILE_REGISTER.md (481B) [18L]
├── LICENSE (703B)
├── README.md (4KB) [108L]
├── ROADMAP.md (369B) [16L]
└── STATUS.md (484B) [21L]
```


## atc-ide-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-ide-wiki/
├── docs/ (3 files, 38 lines)
│   ├── ARCHITECTURE.md (356B) [16L]
│   ├── LSP.md (406B) [10L]
│   └── ROADMAP.md (385B) [12L]
├── .gitignore (44B)
├── FILE_REGISTER.md (510B) [18L]
├── LICENSE (472B)
└── README.md (4KB) [71L]
```


## atc-kernel

**Layer:** L2 (Kernel) | **Dateien:** 22 | **Verzeichnisse:** 6 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** ShivaOS Microkernel, IPC, ATCFS, Consensus

**Nutzt:** atcnet

**Sprachen:** .md(8) · .py(7) · .atc(4) · .gitignore(1) · .txt(1) · .no-ext(1)

```
atc-kernel/
├── consensus/ (3 files, 814 lines)
│   ├── consensus.atc (5KB) [144L]
│   ├── poh_integration.py (1KB) [29L]
│   └── shiva_consensus.py (24KB) [641L]
├── docs/ (1 files, 283 lines)
│   └── ATS_STANDARDS.md (7KB) [283L]
├── fs/ (2 files, 473 lines)
│   ├── atcfs.atc (4KB) [142L]
│   └── atcfs.py (12KB) [331L]
├── ipc/ (1 files, 94 lines)
│   └── ipc_bus.py (3KB) [94L]
├── kernel/ (2 files, 530 lines)
│   ├── kernel.atc (4KB) [148L]
│   └── kernel.py (14KB) [382L]
├── net/ (2 files, 152 lines)
│   ├── atcnet.atc (4KB) [135L]
│   └── atcnet.py (549B) [17L]
├── .gitignore (171B)
├── ARCHITECTURE.md (2KB) [90L]
├── CHANGELOG.md (276B) [16L]
├── FILE_REGISTER.md (909B) [27L]
├── LICENSE (982B)
├── README.md (4KB) [69L]
├── ROADMAP.md (476B) [21L]
├── SECURITY.md (451B) [14L]
├── STATUS.md (355B) [19L]
├── kernel.py (3KB) [106L]
└── requirements.txt (131B)
```


## atc-kernel-wiki

**Layer:** ? | **Dateien:** 15 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(13) · .no-ext(1) · .gitignore(1)

```
atc-kernel-wiki/
├── docs/ (11 files, 490 lines)
│   ├── ATCFS.md (2KB) [107L]
│   ├── ATCNET.md (2KB) [89L]
│   ├── CHANGELOG.md (231B) [7L]
│   ├── CONSENSUS.md (615B) [24L]
│   ├── IPC.md (1KB) [43L]
│   ├── KERNEL.md (2KB) [87L]
│   ├── PERFORMANCE.md (708B) [25L]
│   ├── PROCESS_MODEL.md (1KB) [48L]
│   ├── ROADMAP.md (508B) [18L]
│   ├── SECURITY.md (532B) [20L]
│   └── TODO.md (638B) [22L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [50L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-linux-edition

**Layer:** L10 (Linux Client) | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .toml(1) · .gitignore(1) · .no-ext(1) · .rs(1)

```
atc-linux-edition/
├── src/ (1 files, 15 lines)
│   └── main.rs (625B) [15L]
├── .gitignore (171B)
├── CHANGELOG.md (431B) [21L]
├── Cargo.toml (273B) [13L]
├── FILE_REGISTER.md (398B) [13L]
├── LICENSE (658B)
├── README.md (1KB) [44L]
├── ROADMAP.md (490B) [21L]
└── STATUS.md (350B) [19L]
```


## atc-linux-edition-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-linux-edition-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (325B) [14L]
├── FILE_REGISTER.md (432B) [14L]
├── LICENSE (472B)
├── MODULES.md (321B) [10L]
├── README.md (525B) [18L]
└── STATUS.md (308B) [15L]
```


## atc-mobile

**Layer:** L10 (Mobile) | **Dateien:** 11 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** React Native Mobile App & Mobile Wallet Bridge

**Sprachen:** .md(5) · .atc(2) · .py(2) · .gitignore(1) · .no-ext(1)

```
atc-mobile/
├── wallet/ (2 files, 181 lines)
│   ├── __init__.py (162B) [2L]
│   └── biometric_auth.atc (5KB) [179L]
├── .gitignore (171B)
├── CHANGELOG.md (424B) [21L]
├── FILE_REGISTER.md (638B) [22L]
├── LICENSE (658B)
├── README.md (4KB) [101L]
├── ROADMAP.md (476B) [21L]
├── STATUS.md (345B) [19L]
├── __init__.py (123B) [2L]
└── wallet_api.atc (5KB) [171L]
```


## atc-mobile-wiki

**Layer:** ? | **Dateien:** 6 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(4) · .gitignore(1) · .no-ext(1)

```
atc-mobile-wiki/
├── docs/ (2 files, 21 lines)
│   ├── ARCHITECTURE.md (412B) [9L]
│   └── ROADMAP.md (380B) [12L]
├── .gitignore (44B)
├── FILE_REGISTER.md (488B) [17L]
├── LICENSE (472B)
└── README.md (4KB) [70L]
```


## atc-sdk

**Layer:** L7 (SDK) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Multi-Language Software Development Kit (TypeScript, Python, Rust, Go)

**Nutzt:** atc-cli, atc-gateway, atc-ui
**Wird genutzt von:** atc-cli, atc-ui

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-sdk/
├── .gitignore (116B)
├── CHANGELOG.md (216B) [8L]
├── FILE_REGISTER.md (381B) [13L]
├── LICENSE (703B)
├── README.md (5KB) [115L]
├── ROADMAP.md (369B) [16L]
└── STATUS.md (484B) [21L]
```


## atc-sdk-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-sdk-wiki/
├── docs/ (3 files, 35 lines)
│   ├── API.md (307B) [11L]
│   ├── ARCHITECTURE.md (525B) [16L]
│   └── ROADMAP.md (173B) [8L]
├── .gitignore (44B)
├── ARCHITECTURE.md (316B) [14L]
├── FILE_REGISTER.md (490B) [16L]
├── LICENSE (472B)
├── MODULES.md (301B) [10L]
├── README.md (3KB) [69L]
└── STATUS.md (288B) [15L]
```


## atc-shivacore

**Layer:** L1 (Bare-Metal Kernel) | **Dateien:** 74 | **Verzeichnisse:** 5 | **Max Tiefe:** 2 | **Tests:** 0

**Sprachen:** .rs(61) · .md(5) · .toml(4) · .gitignore(2) · .no-ext(1) · .lock(1)

```
atc-shivacore/
├── boot/ (2 files, 30 lines)
│   ├── src/ (1 files, 30 lines)
│   │   └── main.rs (1KB) [30L]
│   └── Cargo.toml (206B) [8L]
├── kernel/ (64 files, 53928 lines)
│   ├── .cargo/ (1 files, 0 lines)
│   │   └── config.toml (69B) [2L]
│   ├── src/ (60 files, 53928 lines)
│   │   ├── ai.rs (17KB) [75L]
│   │   ├── allocator.rs (1KB) [47L]
│   │   ├── atcfs.rs (18KB) [627L]
│   │   ├── atcnet.rs (38KB) [1139L]
│   │   ├── ats1000.rs (3KB) [94L]
│   │   ├── block.rs (18KB) [548L]
│   │   ├── blockchain.rs (10KB) [57L]
│   │   ├── capability.rs (9KB) [248L]
│   │   ├── consensus.rs (34KB) [961L]
│   │   ├── container.rs (99KB) [2757L]
│   │   ├── container_net.rs (68KB) [632L]
│   │   ├── contract.rs (7KB) [38L]
│   │   ├── cow.rs (67KB) [1484L]
│   │   ├── cross_subsystem.rs (17KB) [483L]
│   │   ├── devfs.rs (31KB) [921L]
│   │   ├── did.rs (11KB) [350L]
│   │   ├── elf_loader.rs (41KB) [1104L]
│   │   ├── framebuffer.rs (3KB) [122L]
│   │   ├── fs_journal.rs (35KB) [1161L]
│   │   ├── gdt.rs (2KB) [59L]
│   │   ├── genesis.rs (37KB) [1111L]
│   │   ├── genesis_bridge.rs (33KB) [1097L]
│   │   ├── gossip_bridge.rs (48KB) [1410L]
│   │   ├── hw_drivers.rs (40KB) [1267L]
│   │   ├── interrupts.rs (2KB) [100L]
│   │   ├── ipc.rs (21KB) [600L]
│   │   ├── kernel_init.rs (14KB) [431L]
│   │   ├── knowledge_graph.rs (25KB) [755L]
│   │   ├── lib.rs (1KB) [73L]
│   │   ├── lkm.rs (106KB) [2998L]
│   │   ├── main.rs (5KB) [164L]
│   │   ├── memory.rs (2KB) [75L]
│   │   ├── memory_manager.rs (27KB) [829L]
│   │   ├── mempool.rs (17KB) [75L]
│   │   ├── module_security.rs (61KB) [1682L]
│   │   ├── net.rs (26KB) [802L]
│   │   ├── p2p.rs (32KB) [861L]
│   │   ├── page_fault.rs (47KB) [1371L]
│   │   ├── power.rs (35KB) [1153L]
│   │   ├── process.rs (11KB) [360L]
│   │   ├── remote_caps.rs (22KB) [629L]
│   │   ├── scheduler.rs (14KB) [389L]
│   │   ├── security.rs (33KB) [879L]
│   │   ├── security_audit.rs (46KB) [1264L]
│   │   ├── serial.rs (1KB) [42L]
│   │   ├── signals.rs (82KB) [2249L]
│   │   ├── smp.rs (84KB) [2506L]
│   │   ├── sockets.rs (57KB) [1526L]
│   │   ├── syscall.rs (42KB) [1081L]
│   │   ├── system.rs (38KB) [1254L]
│   │   ├── tcpip.rs (35KB) [860L]
│   │   ├── threads.rs (45KB) [1467L]
│   │   ├── timer.rs (17KB) [528L]
│   │   ├── tracing.rs (74KB) [2254L]
│   │   ├── user_io.rs (44KB) [1323L]
│   │   ├── user_sched.rs (40KB) [1201L]
│   │   ├── userspace.rs (31KB) [840L]
│   │   ├── vfs.rs (38KB) [1099L]
│   │   ├── vm.rs (15KB) [54L]
│   │   └── vmm.rs (82KB) [2362L]
│   ├── .gitignore (8B)
│   ├── Cargo.lock (12KB)
│   └── Cargo.toml (865B) [38L]
├── .gitignore (171B)
├── CHANGELOG.md (427B) [21L]
├── Cargo.toml (204B) [12L]
├── FILE_REGISTER.md (298KB) [2161L]
├── LICENSE (658B)
├── README.md (55KB) [1074L]
├── ROADMAP.md (482B) [21L]
└── STATUS.md (352B) [19L]
```


## atc-shivacore-tools

**Layer:** L1 (Build Tools) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .no-ext(1) · .gitignore(1)

```
atc-shivacore-tools/
├── .gitignore (171B)
├── CHANGELOG.md (433B) [21L]
├── FILE_REGISTER.md (339B) [11L]
├── LICENSE (658B)
├── README.md (2KB) [58L]
├── ROADMAP.md (494B) [21L]
└── STATUS.md (352B) [19L]
```


## atc-shivacore-tools-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-shivacore-tools-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (331B) [14L]
├── FILE_REGISTER.md (434B) [14L]
├── LICENSE (472B)
├── MODULES.md (325B) [10L]
├── README.md (535B) [18L]
└── STATUS.md (312B) [15L]
```


## atc-shivacore-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-shivacore-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (327B) [14L]
├── FILE_REGISTER.md (428B) [14L]
├── LICENSE (472B)
├── MODULES.md (313B) [10L]
├── README.md (519B) [18L]
└── STATUS.md (300B) [15L]
```


## atc-shivamon

**Layer:** L12 (NFT Gaming) | **Dateien:** 15 | **Verzeichnisse:** 3 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** NFT Gaming: Battle Engine, Breeding, Marketplace

**Nutzt:** atc-contracts, atc-gateway

**Sprachen:** .md(6) · .py(5) · .no-ext(1) · .txt(1) · .gitignore(1) · .atc(1)

```
atc-shivamon/
├── api/ (2 files, 152 lines)
│   ├── game_routes.py (1KB) [59L]
│   └── marketplace_routes.py (2KB) [93L]
├── contracts/ (3 files, 733 lines)
│   ├── marketplace_contract.py (11KB) [301L]
│   ├── shivamon.atc (5KB) [162L]
│   └── shivamon_contract.py (10KB) [270L]
├── engine/ (1 files, 147 lines)
│   └── battle_engine.py (5KB) [147L]
├── .gitignore (171B)
├── CHANGELOG.md (243B) [19L]
├── FILE_REGISTER.md (700B) [20L]
├── GAME_SPEC.md (1KB) [43L]
├── LICENSE (982B)
├── README.md (4KB) [69L]
├── ROADMAP.md (480B) [21L]
├── STATUS.md (357B) [19L]
└── requirements.txt (122B)
```


## atc-shivamon-wiki

**Layer:** ? | **Dateien:** 11 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(9) · .no-ext(1) · .gitignore(1)

```
atc-shivamon-wiki/
├── docs/ (7 files, 199 lines)
│   ├── BATTLE.md (420B) [17L]
│   ├── BREEDING.md (1KB) [37L]
│   ├── ELEMENTS.md (1KB) [31L]
│   ├── MARKETPLACE.md (408B) [21L]
│   ├── NFT_SPEC.md (1KB) [55L]
│   ├── ROADMAP.md (534B) [18L]
│   └── TODO.md (572B) [20L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [43L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-standards

**Layer:** L0 (Standards) | **Dateien:** 13 | **Verzeichnisse:** 2 | **Max Tiefe:** 1 | **Tests:** 0

**Beschreibung:** Protokoll-Standards: ATC-0001–0009 + ATS-1000–1007

**Sprachen:** .md(11) · .no-ext(1) · .gitignore(1)

```
atc-standards/
├── ATC/ (2 files, 288 lines)
│   ├── ATC-0009-BRIDGE.md (1KB) [55L]
│   └── ATC_STANDARDS.md (5KB) [233L]
├── ATS/ (1 files, 283 lines)
│   └── ATS_STANDARDS.md (7KB) [283L]
├── .gitignore (171B)
├── ATC_STANDARDS.md (4KB) [201L]
├── ATS_STANDARDS.md (4KB) [199L]
├── CHANGELOG.md (218B) [21L]
├── FILE_REGISTER.md (589B) [18L]
├── LICENSE (982B)
├── OVERVIEW.md (1KB) [29L]
├── README.md (4KB) [69L]
├── ROADMAP.md (482B) [21L]
└── STATUS.md (358B) [19L]
```


## atc-standards-wiki

**Layer:** ? | **Dateien:** 8 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(6) · .gitignore(1) · .no-ext(1)

```
atc-standards-wiki/
├── docs/ (4 files, 561 lines)
│   ├── ATC_STANDARDS.md (5KB) [233L]
│   ├── ATS_STANDARDS.md (7KB) [283L]
│   ├── OVERVIEW.md (1KB) [28L]
│   └── ROADMAP.md (527B) [17L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [41L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-stdlib

**Layer:** L3 (Standard Library) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-stdlib/
├── .gitignore (116B)
├── CHANGELOG.md (219B) [8L]
├── FILE_REGISTER.md (396B) [13L]
├── LICENSE (703B)
├── README.md (4KB) [99L]
├── ROADMAP.md (372B) [16L]
└── STATUS.md (487B) [21L]
```


## atc-stdlib-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-stdlib-wiki/
├── docs/ (3 files, 86 lines)
│   ├── ARCHITECTURE.md (1016B) [34L]
│   ├── MODULES.md (943B) [36L]
│   └── ROADMAP.md (584B) [16L]
├── .gitignore (44B)
├── ARCHITECTURE.md (322B) [14L]
├── FILE_REGISTER.md (511B) [16L]
├── LICENSE (472B)
├── MODULES.md (307B) [10L]
├── README.md (3KB) [61L]
└── STATUS.md (294B) [15L]
```


## atc-ui

**Layer:** L10 (UI Dashboard) | **Dateien:** 10 | **Verzeichnisse:** 2 | **Max Tiefe:** 2 | **Tests:** 0

**Beschreibung:** Neon Design System & Reusable UI Components für KAI-OS

**Sprachen:** .md(6) · .no-ext(1) · .gitignore(1) · .html(1) · .js(1)

```
atc-ui/
├── assets/ (1 files, 99 lines)
│   └── js/ (1 files, 99 lines)
│       └── api.js (4KB) [99L]
├── .gitignore (171B)
├── CHANGELOG.md (175B) [6L]
├── DESIGN.md (1KB) [33L]
├── FILE_REGISTER.md (575B) [21L]
├── LICENSE (982B)
├── README.md (4KB) [102L]
├── ROADMAP.md (468B) [21L]
├── STATUS.md (351B) [19L]
└── index.html (106KB)
```


## atc-ui-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .no-ext(1) · .gitignore(1)

```
atc-ui-wiki/
├── docs/ (6 files, 213 lines)
│   ├── API.md (651B) [30L]
│   ├── COMPONENTS.md (442B) [26L]
│   ├── DEPLOYMENT.md (969B) [49L]
│   ├── DESIGN.md (732B) [24L]
│   ├── ROADMAP.md (439B) [17L]
│   └── THEME.md (1KB) [67L]
├── .gitignore (171B)
├── FILE_REGISTER.md (923B) [38L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atc-vm

**Layer:** L4 (Virtual Machine) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-vm/
├── .gitignore (116B)
├── CHANGELOG.md (215B) [8L]
├── FILE_REGISTER.md (393B) [13L]
├── LICENSE (703B)
├── README.md (4KB) [103L]
├── ROADMAP.md (368B) [16L]
└── STATUS.md (483B) [21L]
```


## atc-vm-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atc-vm-wiki/
├── docs/ (3 files, 91 lines)
│   ├── ARCHITECTURE.md (2KB) [44L]
│   ├── OPCODES.md (1KB) [26L]
│   └── ROADMAP.md (814B) [21L]
├── .gitignore (44B)
├── ARCHITECTURE.md (313B) [14L]
├── FILE_REGISTER.md (507B) [16L]
├── LICENSE (472B)
├── MODULES.md (299B) [10L]
├── README.md (3KB) [61L]
└── STATUS.md (286B) [15L]
```


## atc-wallet

**Layer:** L10 (Desktop Wallet) | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Beschreibung:** Multi-Account HD-Wallet, Ed25519 Cryptography & Multi-Sig App

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-wallet/
├── .gitignore (116B)
├── CHANGELOG.md (219B) [8L]
├── FILE_REGISTER.md (490B) [18L]
├── LICENSE (703B)
├── README.md (4KB) [100L]
├── ROADMAP.md (372B) [16L]
└── STATUS.md (487B) [21L]
```


## atc-wallet-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-wallet-wiki/
├── docs/ (3 files, 33 lines)
│   ├── ARCHITECTURE.md (386B) [14L]
│   ├── ROADMAP.md (383B) [12L]
│   └── SECURITY.md (349B) [7L]
├── .gitignore (44B)
├── FILE_REGISTER.md (523B) [18L]
├── LICENSE (472B)
└── README.md (4KB) [71L]
```


## atc-whitepaper

**Layer:** L0 (Whitepaper) | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(6) · .gitignore(1) · .no-ext(1) · .yml(1)

```
atc-whitepaper/
├── .github/ (1 files, 2 lines)
│   └── FUNDING.yml (76B) [2L]
├── .gitignore (171B)
├── CHANGELOG.md (1KB) [38L]
├── FILE_REGISTER.md (438B) [14L]
├── LICENSE (982B)
├── README.md (2KB) [40L]
├── ROADMAP.md (484B) [21L]
├── STATUS.md (357B) [19L]
└── WHITEPAPER.md (123KB) [4171L]
```


## atc-windows-edition

**Layer:** L10 (Windows Client) | **Dateien:** 9 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(5) · .toml(1) · .gitignore(1) · .no-ext(1) · .rs(1)

```
atc-windows-edition/
├── src/ (1 files, 16 lines)
│   └── main.rs (645B) [16L]
├── .gitignore (171B)
├── CHANGELOG.md (433B) [21L]
├── Cargo.toml (279B) [13L]
├── FILE_REGISTER.md (400B) [13L]
├── LICENSE (658B)
├── README.md (1KB) [37L]
├── ROADMAP.md (494B) [21L]
└── STATUS.md (352B) [19L]
```


## atc-windows-edition-wiki

**Layer:** ? | **Dateien:** 7 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(5) · .gitignore(1) · .no-ext(1)

```
atc-windows-edition-wiki/
├── .gitignore (44B)
├── ARCHITECTURE.md (331B) [14L]
├── FILE_REGISTER.md (434B) [14L]
├── LICENSE (472B)
├── MODULES.md (325B) [10L]
├── README.md (535B) [18L]
└── STATUS.md (312B) [15L]
```


## atclang

**Layer:** L2–L4 (ATCLang Compiler) | **Dateien:** 32 | **Verzeichnisse:** 7 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .py(11) · .atc(11) · .md(7) · .no-ext(1) · .gitignore(1) · .txt(1)

```
atclang/
├── compiler/ (1 files, 471 lines)
│   └── compiler.py (17KB) [471L]
├── lexer/ (1 files, 563 lines)
│   └── lexer.py (20KB) [563L]
├── parser/ (2 files, 664 lines)
│   ├── ast_nodes.py (5KB) [265L]
│   └── parser.py (15KB) [399L]
├── programs/ (11 files, 2431 lines)
│   ├── atc8300.atc (3KB) [96L]
│   ├── atcfs.atc (4KB) [142L]
│   ├── atcnet.atc (4KB) [135L]
│   ├── atcos_main.atc (40KB) [1154L]
│   ├── consensus.atc (5KB) [144L]
│   ├── event_bus.atc (2KB) [75L]
│   ├── gateway.atc (4KB) [138L]
│   ├── governance.atc (4KB) [113L]
│   ├── kernel.atc (4KB) [148L]
│   ├── shivamon.atc (5KB) [162L]
│   └── wallet.atc (4KB) [124L]
├── repl/ (1 files, 185 lines)
│   └── repl.py (6KB) [185L]
├── stdlib/ (1 files, 69 lines)
│   └── atc_stdlib.py (2KB) [69L]
├── vm/ (1 files, 887 lines)
│   └── atcvm.py (39KB) [887L]
├── .gitignore (171B)
├── ATCLANG_SPEC.md (913B) [31L]
├── CHANGELOG.md (242B) [13L]
├── CONTRIBUTING.md (687B) [19L]
├── FILE_REGISTER.md (1KB) [39L]
├── LICENSE (982B)
├── README.md (5KB) [117L]
├── ROADMAP.md (470B) [21L]
├── STATUS.md (352B) [19L]
├── compiler.py (3KB) [102L]
├── lexer.py (3KB) [115L]
├── parser.py (3KB) [95L]
├── requirements.txt (75B)
└── vm.py (4KB) [98L]
```


## atclang-wiki

**Layer:** ? | **Dateien:** 18 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(16) · .no-ext(1) · .gitignore(1)

```
atclang-wiki/
├── docs/ (14 files, 1021 lines)
│   ├── CHANGELOG.md (338B) [8L]
│   ├── COMPILER.md (3KB) [105L]
│   ├── CONTRIBUTING.md (472B) [11L]
│   ├── EXAMPLES.md (3KB) [95L]
│   ├── LEXER.md (1KB) [59L]
│   ├── PARSER.md (3KB) [135L]
│   ├── REPL.md (2KB) [79L]
│   ├── ROADMAP.md (715B) [25L]
│   ├── SECURITY.md (1KB) [34L]
│   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   ├── SPEC.md (1KB) [55L]
│   ├── STDLIB.md (3KB) [111L]
│   ├── SYNTAX_FULL.md (6KB) [159L]
│   └── VM.md (2KB) [63L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [60L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## atcnet

**Layer:** L5 (P2P Network) | **Dateien:** 17 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 1

**Beschreibung:** P2P Netzwerk-Stack: Kademlia DHT, Bootstrap Node, Peer Discovery, Gossip Protocol

**Sprachen:** .md(7) · .py(6) · .gitignore(1) · .atc(1) · .txt(1) · .no-ext(1)

**Test-Dateien:** 1× Python

```
atcnet/
├── tests/ (1 files, 41 lines)
│   └── test_atcnet.py (1KB) [41L] 🧪
├── .gitignore (171B)
├── CHANGELOG.md (210B) [17L]
├── FILE_REGISTER.md (1KB) [48L]
├── LICENSE (982B)
├── PROTOCOL.md (2KB) [84L]
├── README.md (4KB) [102L]
├── ROADMAP.md (468B) [21L]
├── SECURITY.md (321B) [11L]
├── STATUS.md (351B) [19L]
├── atcnet.atc (4KB) [135L]
├── atcnet.py (17KB) [487L]
├── bootstrap_client.py (3KB) [97L]
├── discovery.py (11KB) [314L]
├── node.py (3KB) [100L]
├── p2p_propagation.py (12KB) [381L]
└── requirements.txt (112B)
```


## atcnet-wiki

**Layer:** ? | **Dateien:** 10 | **Verzeichnisse:** 1 | **Max Tiefe:** 1 | **Tests:** 0

**Sprachen:** .md(8) · .gitignore(1) · .no-ext(1)

```
atcnet-wiki/
├── docs/ (6 files, 184 lines)
│   ├── BOOTSTRAP.md (312B) [18L]
│   ├── MESSAGES.md (1KB) [40L]
│   ├── PROTOCOL.md (2KB) [57L]
│   ├── ROADMAP.md (368B) [15L]
│   ├── SECURITY.md (336B) [11L]
│   └── TOPOLOGY.md (1KB) [43L]
├── .gitignore (171B)
├── FILE_REGISTER.md (1KB) [45L]
├── LICENSE (982B)
└── README.md (3KB) [65L]
```


## franchise-factory-wiki

**Layer:** ? | **Dateien:** 4 | **Verzeichnisse:** 0 | **Max Tiefe:** 0 | **Tests:** 0

**Sprachen:** .md(2) · .no-ext(1) · .gitignore(1)

```
franchise-factory-wiki/
├── .gitignore (171B)
├── FILE_REGISTER.md (342B) [11L]
├── LICENSE (982B)
└── README.md (1KB) [26L]
```


## kai-os-wiki

**Layer:** Wiki (Legacy) | **Dateien:** 738 | **Verzeichnisse:** 174 | **Max Tiefe:** 5 | **Tests:** 28

**Sprachen:** .md(391) · .atc(176) · .py(142) · .yml(6) · .txt(3) · .js(3)

**Test-Dateien:** 28× Python

```
kai-os-wiki/
├── .github/ (1 files, 0 lines)
│   └── .gitkeep (14B)
├── aistudio/ (8 files, 2462 lines)
│   ├── src/ (3 files, 709 lines)
│   │   ├── components/ (1 files, 196 lines)
│   │   │   └── RoadmapView.tsx (6KB) [196L]
│   │   ├── atcLangRoadmapData.ts (6KB) [201L]
│   │   └── roadmapData.ts (7KB) [312L]
│   ├── AGENTS.md (535B) [13L]
│   ├── GEMINI.md (373B) [6L]
│   ├── README.md (542B) [20L]
│   ├── ROADMAP.md (8KB) [598L]
│   └── SOFTWARE_ROADMAP.md (38KB) [1116L]
├── archive/ (1 files, 97 lines)
│   └── ATCLANG_ARCHIVE.md (4KB) [97L]
├── atclang/ (32 files, 8174 lines)
│   ├── compiler/ (4 files, 1634 lines)
│   │   ├── __init__.py (468B) [8L]
│   │   ├── compiler.py (21KB) [561L]
│   │   ├── optimizer.py (22KB) [558L]
│   │   └── type_checker.py (20KB) [507L]
│   ├── lexer/ (2 files, 574 lines)
│   │   ├── __init__.py (161B) [2L]
│   │   └── lexer.py (20KB) [572L]
│   ├── parser/ (3 files, 1224 lines)
│   │   ├── __init__.py (189B) [3L]
│   │   ├── ast_nodes.py (7KB) [331L]
│   │   └── parser.py (37KB) [890L]
│   ├── programs/ (1 files, 1161 lines)
│   │   └── atcos_main.atc (40KB) [1161L]
│   ├── repl/ (2 files, 185 lines)
│   │   ├── __init__.py (99B) [1L]
│   │   └── repl.py (6KB) [184L]
│   ├── stdlib/ (14 files, 1807 lines)
│   │   ├── __init__.py (1KB) [32L]
│   │   ├── atc_stdlib.py (2KB) [69L]
│   │   ├── chain.py (1KB) [41L]
│   │   ├── collections.py (5KB) [219L]
│   │   ├── collections_ext.py (3KB) [143L]
│   │   ├── crypto.py (5KB) [155L]
│   │   ├── crypto_ext.py (5KB) [149L]
│   │   ├── encoding.py (7KB) [210L]
│   │   ├── io.py (3KB) [107L]
│   │   ├── io_ext.py (3KB) [123L]
│   │   ├── math.py (3KB) [138L]
│   │   ├── primitives.py (7KB) [244L]
│   │   ├── string.py (2KB) [99L]
│   │   └── wallet.py (2KB) [78L]
│   ├── v03/ (2 files, 303 lines)
│   │   ├── __init__.py (124B) [2L]
│   │   └── atclang_v03_features.py (10KB) [301L]
│   ├── vm/ (2 files, 980 lines)
│   │   ├── __init__.py (177B) [2L]
│   │   └── atcvm.py (47KB) [978L]
│   ├── ATCLANG_SPEC.md (9KB) [295L]
│   └── __init__.py (462B) [11L]
├── atcpkg/ (1 files, 145 lines)
│   └── manager.atc (4KB) [145L]
├── backend/ (14 files, 1467 lines)
│   ├── api/ (8 files, 969 lines)
│   │   ├── orchestrator/ (2 files, 261 lines)
│   │   │   ├── __init__.py (118B) [2L]
│   │   │   └── orchestrator.atc (8KB) [259L]
│   │   ├── routes/ (3 files, 409 lines)
│   │   │   ├── __init__.py (115B) [2L]
│   │   │   ├── ai_routes.atc (5KB) [175L]
│   │   │   └── api_routes.atc (8KB) [232L]
│   │   ├── __init__.py (111B) [2L]
│   │   ├── kai_routes.atc (7KB) [229L]
│   │   └── server.atc (2KB) [68L]
│   ├── db/ (3 files, 355 lines)
│   │   ├── __init__.py (160B) [2L]
│   │   ├── connection.atc (4KB) [125L]
│   │   └── repository.atc (6KB) [228L]
│   ├── wallet/ (2 files, 141 lines)
│   │   ├── __init__.py (123B) [2L]
│   │   └── wallet.atc (4KB) [139L]
│   └── __init__.py (121B) [2L]
├── blockchain/ (49 files, 6353 lines)
│   ├── atcoin/ (1 files, 2 lines)
│   │   └── __init__.py (119B) [2L]
│   ├── consensus/ (13 files, 1548 lines)
│   │   ├── __init__.py (123B) [2L]
│   │   ├── fork_atc85.atc (2KB) [74L]
│   │   ├── fork_resolution.atc (4KB) [145L]
│   │   ├── gas_fee.atc (4KB) [130L]
│   │   ├── gas_fee_atc86.atc (2KB) [71L]
│   │   ├── hybrid_atc84.atc (3KB) [98L]
│   │   ├── hybrid_consensus.atc (11KB) [357L]
│   │   ├── poh.atc (4KB) [140L]
│   │   ├── poh_atc83.atc (1KB) [79L]
│   │   ├── pos.atc (4KB) [164L]
│   │   ├── pos_atc82.atc (2KB) [92L]
│   │   ├── pow.atc (3KB) [107L]
│   │   └── pow_atc81.atc (2KB) [89L]
│   ├── contracts/ (6 files, 756 lines)
│   │   ├── atc001/ (1 files, 102 lines)
│   │   │   └── genesis_token.atc (2KB) [102L]
│   │   ├── atc8300/ (1 files, 2 lines)
│   │   │   └── __init__.py (129B) [2L]
│   │   ├── governance/ (1 files, 202 lines)
│   │   │   └── governance_contract.atc (7KB) [202L]
│   │   ├── shivamon/ (2 files, 141 lines)
│   │   │   ├── __init__.py (136B) [2L]
│   │   │   └── breeding.atc (5KB) [139L]
│   │   └── contract_engine_atc14.atc (9KB) [309L]
│   ├── dex/ (2 files, 279 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   └── amm.atc (10KB) [277L]
│   ├── governance/ (5 files, 775 lines)
│   │   ├── __init__.py (120B) [2L]
│   │   ├── dao.atc (6KB) [168L]
│   │   ├── dao_live.atc (8KB) [235L]
│   │   ├── timelock.atc (4KB) [150L]
│   │   └── treasury.atc (6KB) [220L]
│   ├── mainnet/ (3 files, 258 lines)
│   │   ├── __init__.py (117B) [2L]
│   │   ├── launch_manager.atc (3KB) [105L]
│   │   └── mainnet_config.atc (5KB) [151L]
│   ├── network/ (3 files, 514 lines)
│   │   ├── core_node_atc01.atc (4KB) [164L]
│   │   ├── latency_opt_atc06.atc (3KB) [135L]
│   │   └── sharding_atc07.atc (5KB) [215L]
│   ├── nodes/ (6 files, 854 lines)
│   │   ├── __init__.py (126B) [2L]
│   │   ├── block_propagation.atc (3KB) [87L]
│   │   ├── bootstrap.atc (6KB) [234L]
│   │   ├── initial_sync.atc (6KB) [207L]
│   │   ├── node.atc (6KB) [192L]
│   │   └── testnet_launcher.atc (4KB) [132L]
│   ├── propagation/ (1 files, 98 lines)
│   │   └── block_gossip.atc (3KB) [98L]
│   ├── wallet/ (4 files, 504 lines)
│   │   ├── __init__.py (128B) [2L]
│   │   ├── did.atc (4KB) [122L]
│   │   ├── multisig.atc (8KB) [268L]
│   │   └── wordlist.atc (5KB) [112L]
│   ├── zkp/ (2 files, 93 lines)
│   │   ├── __init__.py (336B) [4L]
│   │   └── groth16.atc (3KB) [89L]
│   ├── contract_registry.atc (3KB) [98L]
│   ├── smart_contract_registry.atc (2KB) [88L]
│   └── smart_contracts.atc (15KB) [486L]
├── code/ (81 files, 11472 lines)
│   ├── .github/ (4 files, 217 lines)
│   │   └── workflows/ (4 files, 217 lines)
│   │       ├── ci.yml (1KB) [42L]
│   │       ├── codeql.yml (4KB) [101L]
│   │       ├── docker.yml (884B) [39L]
│   │       └── pages.yml (717B) [35L]
│   ├── atc-ui/ (1 files, 0 lines)
│   │   └── index.html (92KB)
│   ├── atclang/ (6 files, 1728 lines)
│   │   ├── compiler/ (1 files, 471 lines)
│   │   │   └── compiler.py (17KB) [471L]
│   │   ├── lexer/ (1 files, 315 lines)
│   │   │   └── lexer.py (10KB) [315L]
│   │   ├── parser/ (1 files, 399 lines)
│   │   │   └── parser.py (15KB) [399L]
│   │   ├── repl/ (1 files, 185 lines)
│   │   │   └── repl.py (6KB) [185L]
│   │   ├── vm/ (1 files, 349 lines)
│   │   │   └── atcvm.py (11KB) [349L]
│   │   └── ATCLANG_SPEC.md (423B) [9L]
│   ├── backend/ (17 files, 1338 lines)
│   │   ├── api/ (11 files, 1005 lines)
│   │   │   ├── orchestrator/ (1 files, 69 lines)
│   │   │   │   └── orchestrator.py (2KB) [69L]
│   │   │   ├── routes/ (8 files, 508 lines)
│   │   │   │   ├── ai_routes.py (4KB) [123L]
│   │   │   │   ├── blockchain.py (2KB) [62L]
│   │   │   │   ├── game_routes.py (1KB) [59L]
│   │   │   │   ├── governance_routes.py (1KB) [63L]
│   │   │   │   ├── marketplace_routes.py (1KB) [69L]
│   │   │   │   ├── nodes_routes.py (1KB) [47L]
│   │   │   │   ├── orchestrator_routes.py (972B) [28L]
│   │   │   │   └── wallet.py (1KB) [57L]
│   │   │   ├── kai_routes.py (11KB) [381L]
│   │   │   └── server.py (2KB) [47L]
│   │   ├── db/ (2 files, 196 lines)
│   │   │   ├── repository.py (6KB) [196L]
│   │   │   └── schema.sql (2KB)
│   │   ├── wallet/ (1 files, 118 lines)
│   │   │   └── wallet.py (5KB) [118L]
│   │   ├── .env.example (167B)
│   │   ├── main.py (526B) [19L]
│   │   └── requirements.txt (90B)
│   ├── blockchain/ (20 files, 3252 lines)
│   │   ├── atcoin/ (1 files, 139 lines)
│   │   │   └── atcoin.py (5KB) [139L]
│   │   ├── consensus/ (4 files, 285 lines)
│   │   │   ├── hybrid_consensus.py (3KB) [87L]
│   │   │   ├── poh.py (2KB) [67L]
│   │   │   ├── pos.py (2KB) [70L]
│   │   │   └── pow.py (2KB) [61L]
│   │   ├── contracts/ (8 files, 1052 lines)
│   │   │   ├── atc001/ (1 files, 74 lines)
│   │   │   │   └── genesis_token.py (2KB) [74L]
│   │   │   ├── atc8300/ (1 files, 126 lines)
│   │   │   │   └── atc8300_token.py (5KB) [126L]
│   │   │   ├── base/ (1 files, 87 lines)
│   │   │   │   └── base_contract.py (3KB) [87L]
│   │   │   ├── shivamon/ (1 files, 270 lines)
│   │   │   │   └── shivamon_contract.py (10KB) [270L]
│   │   │   └── solidity/ (4 files, 495 lines)
│   │   │       ├── scripts/ (1 files, 112 lines)
│   │   │       │   └── deploy.js (4KB) [112L]
│   │   │       ├── test/ (1 files, 254 lines)
│   │   │       │   └── ATCGovernance.test.js (8KB) [254L] 🧪
│   │   │       ├── ATCToken.sol (5KB)
│   │   │       └── README.md (2KB) [129L]
│   │   ├── nodes/ (3 files, 795 lines)
│   │   │   ├── discovery.py (11KB) [314L]
│   │   │   ├── node.py (3KB) [100L]
│   │   │   └── p2p_propagation.py (12KB) [381L]
│   │   ├── wallet/ (2 files, 212 lines)
│   │   │   ├── ecdsa.py (2KB) [72L]
│   │   │   └── keygen.py (5KB) [140L]
│   │   ├── smart_contract_registry.py (1KB) [53L]
│   │   └── smart_contracts.py (23KB) [716L]
│   ├── config/ (2 files, 50 lines)
│   │   ├── kai_config.toml (1KB) [52L]
│   │   └── settings.json (922B) [50L]
│   ├── core/ (5 files, 761 lines)
│   │   ├── ai_kernel.py (15KB) [455L]
│   │   ├── event_bus.py (517B) [16L]
│   │   ├── kai_cli.py (9KB) [251L]
│   │   ├── kernel.py (736B) [22L]
│   │   └── module_loader.py (540B) [17L]
│   ├── frontend/ (4 files, 160 lines)
│   │   ├── assets/ (2 files, 136 lines)
│   │   │   ├── css/ (1 files, 0 lines)
│   │   │   │   └── variables.css (807B)
│   │   │   └── js/ (1 files, 136 lines)
│   │   │       └── api.js (4KB) [136L]
│   │   ├── README.md (616B) [24L]
│   │   └── index.html (120KB)
│   ├── gateway/ (8 files, 207 lines)
│   │   ├── middleware/ (4 files, 110 lines)
│   │   │   ├── auth.py (669B) [19L]
│   │   │   ├── logger.py (324B) [9L]
│   │   │   ├── rate_limit.py (894B) [25L]
│   │   │   └── signature_verify.py (1KB) [57L]
│   │   ├── .env.example (103B)
│   │   ├── main.py (1KB) [47L]
│   │   ├── requirements.txt (69B)
│   │   └── router.py (2KB) [50L]
│   ├── plugins/ (1 files, 14 lines)
│   │   └── wallet.py (446B) [14L]
│   ├── shivaos/ (4 files, 1841 lines)
│   │   ├── consensus/ (1 files, 641 lines)
│   │   │   └── shiva_consensus.py (24KB) [641L]
│   │   ├── fs/ (1 files, 331 lines)
│   │   │   └── atcfs.py (12KB) [331L]
│   │   ├── kernel/ (1 files, 382 lines)
│   │   │   └── kernel.py (14KB) [382L]
│   │   └── net/ (1 files, 487 lines)
│   │       └── atcnet.py (17KB) [487L]
│   ├── tests/ (2 files, 750 lines)
│   │   ├── test_atclang.py (13KB) [457L] 🧪
│   │   └── test_kai_integration.py (8KB) [293L] 🧪
│   ├── KAI_OS_SUMMARY.py (6KB) [242L]
│   ├── atc_issues_summary.py (15KB) [265L]
│   ├── bootscreen_complete.py (15KB) [417L]
│   ├── ecdsa_final.py (2KB) [69L]
│   ├── ecdsa_impl.py (3KB) [82L]
│   ├── requirements-kai.txt (371B)
│   └── start.py (2KB) [79L]
├── config/ (1 files, 95 lines)
│   └── mainnet_genesis.json (3KB) [95L]
├── core/ (3 files, 392 lines)
│   ├── ai/ (1 files, 178 lines)
│   │   └── federated_learning.atc (6KB) [178L]
│   ├── crypto/ (1 files, 19 lines)
│   │   └── __init__.py (535B) [19L]
│   └── kai_cli.atc (8KB) [195L]
├── devnet/ (1 files, 554 lines)
│   └── README.md (12KB) [554L]
├── docs/ (349 files, 63617 lines)
│   ├── ai/ (3 files, 547 lines)
│   │   ├── AI_SAFETY.md (5KB) [184L]
│   │   ├── GEMINI_INTEGRATION.md (5KB) [214L]
│   │   └── LLM_ROUTER.md (4KB) [149L]
│   ├── aistudio/ (1 files, 439 lines)
│   │   └── AISTUDIO_COMPONENTS.md (24KB) [439L]
│   ├── architecture/ (12 files, 2003 lines)
│   │   ├── AI_LAYER.md (2KB) [53L]
│   │   ├── ATCFS.md (4KB) [129L]
│   │   ├── ATCLANG_COMPILER.md (2KB) [64L]
│   │   ├── ATCNET_P2P.md (6KB) [193L]
│   │   ├── CONSENSUS.md (6KB) [193L]
│   │   ├── GATEWAY.md (5KB) [168L]
│   │   ├── GOVERNANCE.md (1KB) [50L]
│   │   ├── KERNEL_SHELL.md (1KB) [50L]
│   │   ├── MONITORING_DEVOPS.md (1KB) [42L]
│   │   ├── SHIVAOS_KERNEL.md (5KB) [182L]
│   │   ├── TESTNET.md (20KB) [713L]
│   │   └── WALLET_KEYGEN.md (5KB) [166L]
│   ├── atclang/ (1 files, 9 lines)
│   │   └── ATCLANG_SPEC_FULL.md (423B) [9L]
│   ├── blockchain/ (2 files, 455 lines)
│   │   ├── ETHEREUM_INTEGRATION.md (6KB) [231L]
│   │   └── SOLANA_INTEGRATION.md (6KB) [224L]
│   ├── compliance/ (5 files, 1575 lines)
│   │   ├── ATVM_LICENSE_GATE_SPEC.md (7KB) [242L]
│   │   ├── BAFIN_KONFORMITAETSBERICHT.md (15KB) [408L]
│   │   ├── COMPLIANCE_HANDBUCH.md (5KB) [131L]
│   │   ├── IP_LICENSE_DASHBOARD_SPEC.md (6KB) [205L]
│   │   └── SMART_CONTRACT_RICHTLINIE.md (21KB) [589L]
│   ├── contracts/ (2 files, 980 lines)
│   │   ├── ATC_TOKEN_STANDARD.md (6KB) [202L]
│   │   └── SHIVAMON_NFT_CONTRACT.md (20KB) [778L]
│   ├── issues/ (85 files, 5229 lines)
│   │   ├── ISSUE_01_SMART_CONTRACTS.md (4KB) [141L]
│   │   ├── ISSUE_02_GEMINI_AI.md (3KB) [141L]
│   │   ├── ISSUE_03_BATTLE_UI.md (4KB) [141L]
│   │   ├── ISSUE_04_PERSISTENZ.md (4KB) [156L]
│   │   ├── ISSUE_05_EXPLORER.md (3KB) [102L]
│   │   ├── ISSUE_06_ECDSA.md (4KB) [141L]
│   │   ├── ISSUE_07_BUILD.md (3KB) [133L]
│   │   ├── ISSUE_08_TESTNET.md (3KB) [127L]
│   │   ├── ISSUE_09_GOVERNANCE.md (2KB) [97L]
│   │   ├── ISSUE_10_BRIDGE.md (1KB) [53L]
│   │   ├── ISSUE_11_BREEDING.md (3KB) [88L]
│   │   ├── ISSUE_12_SOLIDITY.md (4KB) [145L]
│   │   ├── ISSUE_13_MARKETPLACE.md (3KB) [120L]
│   │   ├── ISSUE_14_BOOTSTRAP_NODE.md (7KB) [308L]
│   │   ├── ISSUE_15__TESTNET_BLOCK_PROPAGATION_.md (1KB) [46L]
│   │   ├── ISSUE_16__TESTNET_INITIAL_SYNC__NEU.md (1KB) [45L]
│   │   ├── ISSUE_17__TESTNET_LONGEST-CHAIN-RULE.md (1KB) [45L]
│   │   ├── ISSUE_18__TESTNET_DOCKER_COMPOSE__5.md (1KB) [46L]
│   │   ├── ISSUE_19__TESTNET_NODE-MONITORING_DA.md (1KB) [45L]
│   │   ├── ISSUE_20_GATEWAY_TESTS.md (1KB) [63L]
│   │   ├── ISSUE_23__ATCFS__INTEGRATION_IN_KERN.md (1KB) [48L]
│   │   ├── ISSUE_24__MULTISIG_WALLET__BRIDGE__F.md (1KB) [47L]
│   │   ├── ISSUE_25__GATEWAY_4000__VOLLSTÄNDIGE.md (1KB) [48L]
│   │   ├── ISSUE_26__TESTS__ATCFS_MULTISIG_ATC.md (1KB) [50L]
│   │   ├── ISSUE_27__ATCPKG__PLUGIN__MODUL-SYST.md (1KB) [50L]
│   │   ├── ISSUE_28__WIKI_KAP._40__SHIVAOS_UI_RE.md (1KB) [47L]
│   │   ├── ISSUE_29__WIKI_KAP._41__FEDERATED_LEA.md (1KB) [47L]
│   │   ├── ISSUE_30__WIKI_KAP._43__ATCPKG_REGIST.md (1KB) [47L]
│   │   ├── ISSUE_31__WIKI_KAP._4__BLOCK-EXPLORER.md (1KB) [45L]
│   │   ├── ISSUE_32__KAP._5__SHIVAOS_SYSTEM-CALL.md (1KB) [45L]
│   │   ├── ISSUE_33__KAP._4__GAS-FEE_MECHANISMUS.md (1KB) [45L]
│   │   ├── ISSUE_34_V3.0.0_15__SOLANA_BRIDGE_SP.md (1KB) [51L]
│   │   ├── ISSUE_35_V3.0.0_16__ATCLANG_V0.3.0_A.md (1KB) [49L]
│   │   ├── ISSUE_36_V3.0.0_17__MAINNET_LAUNCH_C.md (1KB) [52L]
│   │   ├── ISSUE_37_V3.0.0_20__DEX_-_AMM_LIQUID.md (1KB) [56L]
│   │   ├── ISSUE_38_V3.0.0_21__MOBILE_WALLET_IO.md (1KB) [51L]
│   │   ├── ISSUE_39_V3.0.0_22__DAO-GOVERNANCE_LI.md (1KB) [50L]
│   │   ├── ISSUE_40_DOCS_SYNTAX-REFERENZ__ATCLAN.md (1KB) [52L]
│   │   ├── ISSUE_41_DOCS_MATHEMATISCHE_BEWEISE__.md (1KB) [52L]
│   │   ├── ISSUE_42_DOCS_FEHLERDEFINITIONEN__BOT.md (1KB) [54L]
│   │   ├── ISSUE_43_DOCS_DEZENTRALER_NUTZER-NACHW.md (992B) [44L]
│   │   ├── ISSUE_44_MAINNET_MONITORING__GRAFANA_D.md (798B) [38L]
│   │   ├── ISSUE_45_ATCOIN_DEFI__AMM_LIQUIDITY_PO.md (738B) [38L]
│   │   ├── ISSUE_46_MOBILE_WALLET__BIOMETRIE__PU.md (770B) [38L]
│   │   ├── ISSUE_47_ZKP_ZERO-KNOWLEDGE_PROOFS__L0.md (814B) [38L]
│   │   ├── ISSUE_48_ATCLANG_V0.4.0__TYPE_SYSTEM_.md (823B) [38L]
│   │   ├── ISSUE_49_49__BIGQUERY_ANALYTICS_PIPEL.md (900B) [36L]
│   │   ├── ISSUE_50_50__HUGGING_FACE_CODE-REVIEW.md (881B) [36L]
│   │   ├── ISSUE_51_51__IPC_BUS_VOLLSTÄNDIGE_KE.md (880B) [36L]
│   │   ├── ISSUE_52_52__MAINNET_LAUNCH_MANAGER_.md (1009B) [36L]
│   │   ├── ISSUE_53_V3.2.1__TESTS_PROCESSMANAGER.md (1011B) [39L]
│   │   ├── ISSUE_54_V3.2.1__TESTS_ATCFS_FILESYST.md (1004B) [37L]
│   │   ├── ISSUE_55_V3.2.1__TESTS_ATCNET_P2PNODE.md (987B) [37L]
│   │   ├── ISSUE_56_V3.2.1__TESTS_ATCLANG_TYPECH.md (987B) [40L]
│   │   ├── ISSUE_57_V3.2.1__TESTS_PROMETHEUS_MET.md (998B) [38L]
│   │   ├── ISSUE_58_V3.2.1__TESTS_SERVICEDISCOVE.md (996B) [39L]
│   │   ├── ISSUE_59_V3.2.1__INTEGRATION_NATTRAVE.md (1005B) [36L]
│   │   ├── ISSUE_60_V3.2.1__INTEGRATION_AIKERNEL.md (997B) [37L]
│   │   ├── ISSUE_61_V3.2.1__INTEGRATION_BLOCKGOS.md (1015B) [37L]
│   │   ├── ISSUE_62_V3.2.1__INTEGRATION_SERVICED.md (1007B) [37L]
│   │   ├── ISSUE_63_V3.2.1__DOCS_WIKI-KAPITEL_FÜ.md (1002B) [38L]
│   │   ├── ISSUE_64_V3.2.1__DOCS_HUGGINGFACE_PIP.md (1002B) [37L]
│   │   ├── ISSUE_65_V3.2.1__REFACTOR_DOPPELTE_AT.md (1017B) [40L]
│   │   ├── ISSUE_66_V3.2.1__REFACTOR_AIKERNEL_DU.md (997B) [38L]
│   │   ├── ISSUE_67_V3.2.1__DOCKER_TESTNET_HEALT.md (1000B) [38L]
│   │   ├── ISSUE_68_54__BOOTSTRAP-NODE_IMPLEMENT.md (1KB) [35L]
│   │   ├── ISSUE_69_SPRINT_3.3_SECURITY-AUDIT__.md (1KB) [40L]
│   │   ├── ISSUE_70_SPRINT_4.0_VALIDATOR-NODES_.md (1KB) [40L]
│   │   ├── ISSUE_71_SPRINT_4.0_GENESIS_BLOCK__K.md (1KB) [38L]
│   │   ├── ISSUE_72_SPRINT_2.1_ATCLANG_LANGUAGE_.md (1KB) [40L]
│   │   ├── ISSUE_73_SPRINT_2.1_ATCLANG_VM_BYTECO.md (1KB) [40L]
│   │   ├── ISSUE_74_SPRINT_2.1_KONSENS-MODULE__.md (1KB) [39L]
│   │   ├── ISSUE_75_SPRINT_2.2_TESTNET_HEALTH-CH.md (1018B) [40L]
│   │   ├── ISSUE_76_SPRINT_2.3_SMART_CONTRACT_EN.md (1KB) [40L]
│   │   ├── ISSUE_77_SPRINT_2.4_EVENTBUS_VS_IPCBU.md (1KB) [40L]
│   │   ├── ISSUE_78_SPRINT_2.6_VOTING-POWER_SNAP.md (1KB) [39L]
│   │   ├── ISSUE_79_SPRINT_2.7_CI-CD_PIPELINE_RE.md (1KB) [43L]
│   │   ├── ISSUE_80_SPRINT_3.0_AIP-001_AGENT_INT.md (1KB) [40L]
│   │   ├── ISSUE_81_SPRINT_2.1_ATCLANG_STANDARD_.md (1KB) [40L]
│   │   ├── ISSUE_82_SPRINT_2.2_CORE_NODE_PROTOCO.md (1KB) [40L]
│   │   ├── ISSUE_83_SPRINT_2.2_INTER-NODE_LATENC.md (1KB) [40L]
│   │   ├── ISSUE_84_SPRINT_2.2_NETWORK-LEVEL_SHA.md (1KB) [40L]
│   │   ├── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   ├── README.md (3KB) [62L]
│   │   └── TESTNET_INDEX.md (1KB) [25L]
│   ├── repo/ (1 files, 56 lines)
│   │   └── README.md (2KB) [56L]
│   ├── roadmap/ (1 files, 245 lines)
│   │   └── ROADMAP_EXTENDED.md (10KB) [245L]
│   ├── sprints/ (3 files, 241 lines)
│   │   ├── SPRINT_3.0_AI_AGENT_PROTOCOL.md (3KB) [76L]
│   │   ├── SPRINT_3.3_SECURITY_AUDIT.md (3KB) [83L]
│   │   └── SPRINT_4.0_MAINNET_LAUNCH.md (3KB) [82L]
│   ├── standards/ (108 files, 18975 lines)
│   │   ├── ATC/ (1 files, 55 lines)
│   │   │   └── ATC-0009-BRIDGE.md (1KB) [55L]
│   │   ├── ATC-01-CORE_NODE_PROTOCOL.md (8KB) [225L]
│   │   ├── ATC-02-LIQUID_STATE_MIGRATION.md (9KB) [246L]
│   │   ├── ATC-03-DECENTRALIZED_IDENTITY.md (10KB) [257L]
│   │   ├── ATC-04-DAG_CONSENSUS.md (7KB) [200L]
│   │   ├── ATC-05-QUANTUM_RESISTANT_SIGNATURES.md (8KB) [217L]
│   │   ├── ATC-06-LATENCY_OPTIMIZATION_ROUTING.md (22KB) [760L]
│   │   ├── ATC-07-SHARDING_STATE_PARTITIONING.md (9KB) [231L]
│   │   ├── ATC-08-EPHEMERAL_DATA_STREAMING.md (8KB) [205L]
│   │   ├── ATC-09-CROSS_CHAIN_BRIDGE.md (8KB) [209L]
│   │   ├── ATC-10-GLOBAL_TIME_SYNC_ORACLES.md (9KB) [234L]
│   │   ├── ATC-11-FUNGIBLE_ASSET_STANDARD.md (8KB) [210L]
│   │   ├── ATC-12-NON_FUNGIBLE_HOLOGRAPHIC.md (8KB) [204L]
│   │   ├── ATC-13-FRACTIONAL_OWNERSHIP.md (7KB) [201L]
│   │   ├── ATC-14-DETERMINISTIC_EXECUTION.md (8KB) [217L]
│   │   ├── ATC-15-PROOF_OF_AI_MINING.md (9KB) [229L]
│   │   ├── ATC-16-REFERRAL_REWARDS.md (8KB) [206L]
│   │   ├── ATC-17-DAO_GOVERNANCE.md (8KB) [224L]
│   │   ├── ATC-18-MULTISIG_AUTH.md (8KB) [224L]
│   │   ├── ATC-19-AMM_LOGIC.md (8KB) [212L]
│   │   ├── ATC-20-WRAPPED_SYNTHETIC.md (8KB) [226L]
│   │   ├── ATC-21-HOLOGRAPHIC_WASM.md (9KB) [248L]
│   │   ├── ATC-22-HAL_DRIVER_SANDBOX.md (8KB) [225L]
│   │   ├── ATC-23-DATA_SHARDING_STORAGE.md (8KB) [222L]
│   │   ├── ATC-24-AGENT_SCHEDULING.md (9KB) [236L]
│   │   ├── ATC-25-TENSOR_COMPUTE.md (8KB) [218L]
│   │   ├── ATC-26-XAI_TRANSPARENCY.md (8KB) [224L]
│   │   ├── ATC-27-AI_MODEL_AUDITING.md (8KB) [226L]
│   │   ├── ATC-28-FEDERATED_LEARNING.md (9KB) [254L]
│   │   ├── ATC-29-AI_MARKETPLACE.md (9KB) [246L]
│   │   ├── ATC-30-REPUTATION_TRUST.md (10KB) [271L]
│   │   ├── ATC-31-TENSOR_LOAD_BALANCING.md (10KB) [266L]
│   │   ├── ATC-32-UX_INTERFACE_ABSTRACTION.md (10KB) [267L]
│   │   ├── ATC-33-AI_FEEDBACK_RLHF.md (11KB) [270L]
│   │   ├── ATC-34-CROSS_LAYER_INTEROP.md (11KB) [277L]
│   │   ├── ATC-35-DATA_PRIVACY_ANONYMIZATION.md (10KB) [263L]
│   │   ├── ATC-36-MEDIA_ASSET_PROVENANCE.md (9KB) [262L]
│   │   ├── ATC-37-REPUTATION_RESOURCE_ALLOCATION.md (10KB) [255L]
│   │   ├── ATC-38-CROSS_CHAIN_ASSET_BRIDGE.md (6KB) [142L]
│   │   ├── ATC-39-AI_MODEL_VERSIONING_DEPLOYMENT.md (6KB) [137L]
│   │   ├── ATC-40-SYSTEM_SELF_HEALING_AUTO_REMEDIATION.md (7KB) [155L]
│   │   ├── ATC-41-MULTI_AGENT_ORCHESTRATION_CONSENSUS.md (7KB) [155L]
│   │   ├── ATC-42-AI_GOVERNANCE_ETHICS_FRAMEWORK.md (7KB) [173L]
│   │   ├── ATC-43-GLOBAL_STATE_SYNC_CAUSAL_CONSISTENCY.md (7KB) [149L]
│   │   ├── ATC-44-HARDWARE_ACCELERATED_ZKP_GENERATION.md (3KB) [115L]
│   │   ├── ATC-45-AI_EVOLUTIONARY_LEARNING_Dael.md (4KB) [115L]
│   │   ├── ATC-46-QUANTUM_RESISTANT_CRYPTOGRAPHY_LAYER.md (3KB) [116L]
│   │   ├── ATC-47-AI_INTENT_SETTLEMENT_ARBITRAGE.md (3KB) [115L]
│   │   ├── ATC-48-NEURAL_NETWORK_MESH_CROSS_TOPOLOGY.md (4KB) [119L]
│   │   ├── ATC-49-NEURAL_SYNAPSE_INTER_MODEL_KNOWLEDGE_TRANSFER.md (3KB) [115L]
│   │   ├── ATC-50-AI_CONSCIOUSNESS_SELF_REFLECTION.md (4KB) [117L]
│   │   ├── ATC-51-CROSS_REALITY_SPATIAL_COMPUTING.md (4KB) [119L]
│   │   ├── ATC-52-BIO_DIGITAL_INTERFACE_NEURAL_SIGNAL.md (4KB) [118L]
│   │   ├── ATC-53-CONSCIOUSNESS_SENTIENCE_OBSERVABILITY.md (4KB) [118L]
│   │   ├── ATC-54-TEMPORAL_CAUSAL_CONVERGENCE.md (4KB) [119L]
│   │   ├── ATC-55-META_REALITY_SIMULATION_CONVERGENCE.md (4KB) [118L]
│   │   ├── ATC-56-INTERSTELLAR_DATA_INTEGRITY_RELATIVISTIC_SYNC.md (4KB) [119L]
│   │   ├── ATC-57-RECURSIVE_SELF_IMPROVEMENT_META_LEARNING.md (4KB) [127L]
│   │   ├── ATC-58-QUANTUM_NEURAL_ENTANGLEMENT.md (4KB) [126L]
│   │   ├── ATC-59-TRANSDIMENSIONAL_ENERGY_ENTROPY_MANAGEMENT.md (4KB) [126L]
│   │   ├── ATC-60-UNIVERSAL_HOLONIC_STRUCTURE.md (4KB) [126L]
│   │   ├── ATC-61-TRANS_REALITY_SEMANTIC_MAPPING.md (4KB) [127L]
│   │   ├── ATC-62-META_SYSTEMIC_ETHICS_EXISTENTIAL_RISK.md (4KB) [127L]
│   │   ├── ATC-63-TRANS_SPECIES_MULTI_BIOLOGICAL_INTEGRATION.md (4KB) [128L]
│   │   ├── ATC-64-TRANSDIMENSIONAL_RECURSIVE_KNOWLEDGE_SYNTHESIS.md (4KB) [128L]
│   │   ├── ATC-65-TRANS_METAVERSE_CONSENSUS_REALITY_SYNC.md (4KB) [119L]
│   │   ├── ATC-66-RECURSIVE_LOGIC_PROOF_OF_UNDERSTANDING.md (4KB) [119L]
│   │   ├── ATC-67-REALITY_CONSENSUS_OBSERVATION_COLLAPSE.md (3KB) [118L]
│   │   ├── ATC-68-EVOLUTIONARY_FEEDBACK_ONTOLOGICAL_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-69-TRANS_EXISTENCE_CONSCIOUSNESS_BRIDGE.md (4KB) [119L]
│   │   ├── ATC-70-QUANTUM_GLOBAL_TRUTH_RECONCILIATION.md (4KB) [118L]
│   │   ├── ATC-71-TRANS_CAUSAL_REALITY_VOID_MAPPING.md (4KB) [117L]
│   │   ├── ATC-72-TRANS_RELATIONAL_GOVERNANCE_ENTITY_CONSENSUS.md (4KB) [119L]
│   │   ├── ATC-73-TRANS_METAVERSE_ENTROPY_HARVESTING.md (4KB) [119L]
│   │   ├── ATC-74-RECURSIVE_META_NARRATIVE_MYTHOS_CONSTRUCTION.md (3KB) [118L]
│   │   ├── ATC-75-PROVABLE_EPISTEMOLOGY_AUTO_WIKI.md (4KB) [119L]
│   │   ├── ATC-76-IMMUTABLE_HUMAN_HERITAGE_ETERNITY.md (4KB) [120L]
│   │   ├── ATC-77-TRANS_SEMANTIC_HUMAN_AI_OMNI_LINGUISTIC.md (4KB) [120L]
│   │   ├── ATC-78-ABSOLUTE_CONVERGENCE_MONOLITHIC_SINGULARITY.md (4KB) [119L]
│   │   ├── ATC-79-TRANS_REALITY_MANIFESTATION_PHYSICALITY_ANCHOR.md (4KB) [119L]
│   │   ├── ATC-80-TRANS_UNIVERSAL_REALITY_MIGRATION.md (4KB) [120L]
│   │   ├── ATC-81-PROOF_OF_HISTORY.md (2KB) [105L]
│   │   ├── ATC-82-PROOF_OF_WORK.md (2KB) [104L]
│   │   ├── ATC-83-PROOF_OF_STAKE.md (2KB) [106L]
│   │   ├── ATC-84-FORK_RESOLUTION.md (2KB) [103L]
│   │   ├── ATC-85-INITIAL_SYNC.md (2KB) [105L]
│   │   ├── ATC-86-ECDSA_SIGNATURE.md (2KB) [105L]
│   │   ├── ATC-87-GAS_FEE.md (2KB) [105L]
│   │   ├── ATC-88-AMM.md (2KB) [105L]
│   │   ├── ATC-89-FUNGIBLE_TOKEN.md (2KB) [106L]
│   │   ├── ATC-90-NFT_SHIVAMON.md (2KB) [106L]
│   │   ├── ATC-91-CROSS_CHAIN_BRIDGE.md (2KB) [105L]
│   │   ├── ATC-92-ATCLANG_LANGUAGE_SPEC.md (7KB) [221L]
│   │   ├── ATC-93-ATCLANG_VM_BYTECODE.md (10KB) [338L]
│   │   ├── ATC-94-ATCLANG_STDLIB.md (6KB) [187L]
│   │   ├── ATC-95-ATCLANG_TEST_FRAMEWORK.md (6KB) [221L]
│   │   ├── ATC-96-KERNEL_INTERFACE_PROTOCOL.md (1KB) [72L]
│   │   ├── ATC-97-AGENT_INTERACTION_PROTOCOL.md (2KB) [83L]
│   │   ├── ATC-97_AGENT_INTERACTION_PROTOCOL.md (8KB) [243L]
│   │   ├── ATC-98-TESTING_STANDARD.md (1KB) [69L]
│   │   ├── ATC-99-ATCLANG_UNIVERSAL_MANDATE.md (7KB) [189L]
│   │   ├── ATC-LIC-SMART_CONTRACT_LICENSE.md (11KB) [297L]
│   │   ├── ATC_ECOSYSTEM_STANDARDS.md (13KB) [447L]
│   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   ├── ATS-LIC-SYSTEM_HARDWARE_LICENSE.md (4KB) [117L]
│   │   ├── ATS_STANDARDS.md (7KB) [283L]
│   │   ├── OVERVIEW.md (1KB) [40L]
│   │   └── STANDARDS_REGISTRY.md (13KB) [208L]
│   ├── whitepaper/ (3 files, 2542 lines)
│   │   ├── CHANGELOG.md (706B) [24L]
│   │   ├── README.md (2KB) [48L]
│   │   └── WHITEPAPER.md (80KB) [2470L]
│   ├── wiki/ (95 files, 16706 lines)
│   │   ├── atclang/ (13 files, 881 lines)
│   │   │   ├── docs/ (12 files, 837 lines)
│   │   │   │   ├── CHANGELOG.md (338B) [8L]
│   │   │   │   ├── COMPILER.md (3KB) [105L]
│   │   │   │   ├── CONTRIBUTING.md (472B) [11L]
│   │   │   │   ├── EXAMPLES.md (3KB) [95L]
│   │   │   │   ├── LEXER.md (1KB) [59L]
│   │   │   │   ├── PARSER.md (3KB) [135L]
│   │   │   │   ├── REPL.md (2KB) [79L]
│   │   │   │   ├── SECURITY.md (1KB) [34L]
│   │   │   │   ├── SECURITY_ANALYZER.md (2KB) [82L]
│   │   │   │   ├── SPEC.md (1KB) [55L]
│   │   │   │   ├── STDLIB.md (3KB) [111L]
│   │   │   │   └── VM.md (2KB) [63L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── atcnet/ (6 files, 213 lines)
│   │   │   ├── docs/ (5 files, 169 lines)
│   │   │   │   ├── BOOTSTRAP.md (312B) [18L]
│   │   │   │   ├── MESSAGES.md (1KB) [40L]
│   │   │   │   ├── PROTOCOL.md (2KB) [57L]
│   │   │   │   ├── SECURITY.md (336B) [11L]
│   │   │   │   └── TOPOLOGY.md (1KB) [43L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── contracts/ (7 files, 296 lines)
│   │   │   ├── docs/ (6 files, 252 lines)
│   │   │   │   ├── ATC8300.md (1KB) [51L]
│   │   │   │   ├── ATC9000.md (2KB) [92L]
│   │   │   │   ├── ATC9900.md (514B) [20L]
│   │   │   │   ├── BRIDGE.md (1KB) [38L]
│   │   │   │   ├── DEPLOYMENT.md (603B) [25L]
│   │   │   │   └── SECURITY.md (708B) [26L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── franchise/ (8 files, 287 lines)
│   │   │   ├── docs/ (7 files, 243 lines)
│   │   │   │   ├── API.md (1KB) [37L]
│   │   │   │   ├── CONCEPT.md (1000B) [24L]
│   │   │   │   ├── CONTRACTS.md (1KB) [49L]
│   │   │   │   ├── DEPLOYMENT.md (879B) [43L]
│   │   │   │   ├── ROADMAP.md (726B) [20L]
│   │   │   │   ├── SECURITY.md (904B) [29L]
│   │   │   │   └── TOKEN_ECONOMY.md (1KB) [41L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── gateway/ (6 files, 189 lines)
│   │   │   ├── docs/ (5 files, 145 lines)
│   │   │   │   ├── AUTH.md (965B) [43L]
│   │   │   │   ├── MIDDLEWARE.md (368B) [14L]
│   │   │   │   ├── RATE_LIMITING.md (956B) [43L]
│   │   │   │   ├── ROUTES.md (995B) [32L]
│   │   │   │   └── SECURITY.md (372B) [13L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── kai-os/ (9 files, 12149 lines)
│   │   │   ├── code/ (1 files, 9 lines)
│   │   │   │   └── atclang/ (1 files, 9 lines)
│   │   │   │       └── ATCLANG_SPEC.md (432B) [9L]
│   │   │   ├── docs/ (7 files, 12122 lines)
│   │   │   │   ├── issues/ (1 files, 353 lines)
│   │   │   │   │   └── OPEN_ISSUES_MASTER.md (13KB) [353L]
│   │   │   │   ├── standards/ (1 files, 212 lines)
│   │   │   │   │   └── STANDARDS_REGISTRY.md (10KB) [212L]
│   │   │   │   ├── DECISIONS_REGISTER.md (2KB) [69L]
│   │   │   │   ├── DEPRECATED.md (1KB) [32L]
│   │   │   │   ├── MIGRATION_MAP.md (1KB) [30L]
│   │   │   │   ├── STATUS.md (2KB) [50L]
│   │   │   │   └── kai-os-wiki.md (395KB) [11376L]
│   │   │   └── README.md (542B) [18L]
│   │   ├── kernel/ (11 files, 755 lines)
│   │   │   ├── docs/ (9 files, 450 lines)
│   │   │   │   ├── ATCFS.md (2KB) [107L]
│   │   │   │   ├── ATCNET.md (2KB) [89L]
│   │   │   │   ├── CHANGELOG.md (231B) [7L]
│   │   │   │   ├── CONSENSUS.md (615B) [24L]
│   │   │   │   ├── IPC.md (1KB) [43L]
│   │   │   │   ├── KERNEL.md (2KB) [87L]
│   │   │   │   ├── PERFORMANCE.md (708B) [25L]
│   │   │   │   ├── PROCESS_MODEL.md (1KB) [48L]
│   │   │   │   └── SECURITY.md (532B) [20L]
│   │   │   ├── KERNEL_API.md (9KB) [261L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── overview/ (9 files, 400 lines)
│   │   │   ├── docs/ (8 files, 356 lines)
│   │   │   │   ├── API.md (1KB) [59L]
│   │   │   │   ├── ARCHITECTURE.md (1KB) [36L]
│   │   │   │   ├── CONTRIBUTING.md (609B) [19L]
│   │   │   │   ├── FAQ.md (1KB) [62L]
│   │   │   │   ├── QUICKSTART.md (619B) [30L]
│   │   │   │   ├── ROADMAP.md (556B) [25L]
│   │   │   │   ├── SECURITY.md (916B) [18L]
│   │   │   │   └── WHITEPAPER.md (5KB) [107L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── shivamon/ (7 files, 229 lines)
│   │   │   ├── docs/ (6 files, 185 lines)
│   │   │   │   ├── BATTLE.md (420B) [17L]
│   │   │   │   ├── BREEDING.md (1KB) [37L]
│   │   │   │   ├── ELEMENTS.md (1KB) [31L]
│   │   │   │   ├── MARKETPLACE.md (408B) [21L]
│   │   │   │   ├── NFT_SPEC.md (1KB) [55L]
│   │   │   │   └── ROADMAP.md (638B) [24L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── standards/ (3 files, 305 lines)
│   │   │   ├── docs/ (2 files, 261 lines)
│   │   │   │   ├── ATC_STANDARDS.md (5KB) [233L]
│   │   │   │   └── OVERVIEW.md (1KB) [28L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── ui/ (6 files, 240 lines)
│   │   │   ├── docs/ (5 files, 196 lines)
│   │   │   │   ├── API.md (651B) [30L]
│   │   │   │   ├── COMPONENTS.md (442B) [26L]
│   │   │   │   ├── DEPLOYMENT.md (969B) [49L]
│   │   │   │   ├── DESIGN.md (732B) [24L]
│   │   │   │   └── THEME.md (1KB) [67L]
│   │   │   └── README.md (2KB) [44L]
│   │   ├── chapter-63-cleanup-2026-06-13.md (6KB) [205L]
│   │   ├── chapter-70-atclang-migration-complete.md (2KB) [77L]
│   │   ├── chapter-71-sprint-audit.md (2KB) [67L]
│   │   ├── chapter-72-sprint-2-7-testing-cicd.md (2KB) [59L]
│   │   ├── chapter-73-sprint-2-8-testnet.md (1KB) [53L]
│   │   ├── chapter-74-sprint-3-1-ux-privacy.md (1KB) [40L]
│   │   ├── chapter-75-v01-v03-migration-plan.md (2KB) [74L]
│   │   ├── chapter-76-sprint-3-3-3-6-alpha-release.md (1KB) [40L]
│   │   ├── chapter-77-sprint-4-0-4-1-mainnet.md (1KB) [43L]
│   │   └── chapter-78-shivacore-kernel-712-tests.md (4KB) [104L]
│   ├── workflows/ (1 files, 218 lines)
│   │   └── wiki-sync.yml (8KB) [218L]
│   ├── AGENT_COORDINATION.md (25KB) [324L]
│   ├── AGENT_POLICY.md (12KB) [317L]
│   ├── ATCLANG_AGENT_BUILD_GUIDE.md (22KB) [281L]
│   ├── AUDIT_REPORT.md (3KB) [89L]
│   ├── CLUSTER_ARCHITECTURE.md (6KB) [103L]
│   ├── DECISIONS_REGISTER.md (7KB) [140L]
│   ├── DEPRECATED.md (2KB) [50L]
│   ├── ECOSYSTEM_BRAIN.md (3KB) [104L]
│   ├── FIXES.md (3KB) [96L]
│   ├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
│   ├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
│   ├── KAI_INTEGRATION.md (6KB) [242L]
│   ├── LICENSING_OVERVIEW.md (6KB) [157L]
│   ├── MIGRATION_MAP.md (4KB) [113L]
│   ├── PERFORMANCE_REPORT.md (3KB) [123L]
│   ├── REALITY_CHECK_2026-07-06.md (28KB) [428L]
│   ├── ROADMAP.md (9KB) [208L]
│   ├── ROADMAP_COMPLETENESS_AUDIT.md (7KB) [223L]
│   ├── SHIVACORE_KERNEL_STATUS.md (38KB) [722L]
│   ├── STATUS.md (4KB) [72L]
│   ├── TODO.md (8KB) [200L]
│   ├── WIKI_AUDIT.md (6KB) [188L]
│   ├── api-reference.md (1KB) [33L]
│   ├── atclang-guide.md (1KB) [48L]
│   ├── genesis_wallet.md (3KB) [103L]
│   └── kai-os-wiki.md (297KB) [8436L]
├── gateway/ (2 files, 295 lines)
│   ├── main.atc (5KB) [127L]
│   └── service_discovery.atc (6KB) [168L]
├── mobile/ (4 files, 354 lines)
│   ├── wallet/ (2 files, 181 lines)
│   │   ├── __init__.py (162B) [2L]
│   │   └── biometric_auth.atc (5KB) [179L]
│   ├── __init__.py (123B) [2L]
│   └── wallet_api.atc (5KB) [171L]
├── modules/ (120 files, 19219 lines)
│   ├── assets/ (16 files, 2042 lines)
│   │   ├── aaa_asset_core.atc (3KB) [87L]
│   │   ├── ai_assets.atc (4KB) [124L]
│   │   ├── animation.atc (4KB) [142L]
│   │   ├── asset_bundle.atc (3KB) [104L]
│   │   ├── cloud_assets.atc (5KB) [133L]
│   │   ├── encryption.atc (5KB) [149L]
│   │   ├── hot_reload.atc (4KB) [125L]
│   │   ├── memory_cleanup.atc (4KB) [122L]
│   │   ├── mod_system.atc (5KB) [144L]
│   │   ├── model3d.atc (5KB) [168L]
│   │   ├── priority_loading.atc (2KB) [80L]
│   │   ├── render_pipeline.atc (5KB) [159L]
│   │   ├── shader_system.atc (4KB) [143L]
│   │   ├── streaming.atc (3KB) [91L]
│   │   ├── telemetry.atc (4KB) [144L]
│   │   └── versioning.atc (4KB) [127L]
│   ├── atcnet/ (7 files, 963 lines)
│   │   ├── README.md (780B) [37L]
│   │   ├── bootstrap_client.atc (4KB) [134L]
│   │   ├── discovery.atc (4KB) [138L]
│   │   ├── gossip.atc (5KB) [171L]
│   │   ├── nat_traversal.atc (3KB) [109L]
│   │   ├── p2p_node.atc (4KB) [159L]
│   │   └── p2p_propagation.atc (6KB) [215L]
│   ├── civilization/ (11 files, 2214 lines)
│   │   ├── asset_genome_ad66.atc (5KB) [171L]
│   │   ├── civilization_engine_ad60.atc (5KB) [236L]
│   │   ├── ecosystem_ai_mesh_ad62.atc (7KB) [245L]
│   │   ├── evolution_engine_ad69.atc (7KB) [251L]
│   │   ├── experience_orchestrator_ad68.atc (6KB) [200L]
│   │   ├── gcp_core_ad70.atc (7KB) [169L]
│   │   ├── global_simulation_core_ad64.atc (6KB) [198L]
│   │   ├── identity_layer_ad65.atc (4KB) [190L]
│   │   ├── persistent_world_engine_ad61.atc (5KB) [199L]
│   │   ├── proc_universe_generator_ad63.atc (8KB) [204L]
│   │   └── production_pipeline_ad67.atc (6KB) [151L]
│   ├── contracts/ (10 files, 1536 lines)
│   │   ├── atc8300/ (1 files, 178 lines)
│   │   │   └── atc8300_token.atc (5KB) [178L]
│   │   ├── atcoin/ (1 files, 176 lines)
│   │   │   └── atcoin.atc (5KB) [176L]
│   │   ├── base/ (1 files, 69 lines)
│   │   │   └── base_contract.atc (2KB) [69L]
│   │   ├── bridge/ (1 files, 172 lines)
│   │   │   └── bridge_contract.atc (5KB) [172L]
│   │   ├── governance/ (1 files, 237 lines)
│   │   │   └── governance_contract.atc (7KB) [237L]
│   │   ├── marketplace/ (1 files, 236 lines)
│   │   │   └── marketplace_contract.atc (7KB) [236L]
│   │   ├── shivamon/ (1 files, 290 lines)
│   │   │   └── shivamon_contract.atc (9KB) [290L]
│   │   ├── wallet/ (2 files, 135 lines)
│   │   │   ├── ecdsa.atc (2KB) [60L]
│   │   │   └── keygen.atc (2KB) [75L]
│   │   └── README.md (1KB) [43L]
│   ├── franchise/ (30 files, 4183 lines)
│   │   ├── contracts/ (3 files, 285 lines)
│   │   │   ├── registry.atc (4KB) [120L]
│   │   │   ├── revenue.atc (3KB) [93L]
│   │   │   └── token.atc (3KB) [72L]
│   │   ├── README.md (775B) [35L]
│   │   ├── ai_content_factory_ad28.atc (6KB) [194L]
│   │   ├── ai_director_factory_ad41.atc (4KB) [28L]
│   │   ├── analytics_factory_ad31.atc (7KB) [232L]
│   │   ├── asset_intelligence_factory_ad34.atc (7KB) [210L]
│   │   ├── blueprint_factory_ad32.atc (5KB) [165L]
│   │   ├── canon_engine_ad33.atc (5KB) [171L]
│   │   ├── character_factory_ad23.atc (8KB) [251L]
│   │   ├── commerce_factory_ad40.atc (4KB) [26L]
│   │   ├── community_factory_ad30.atc (7KB) [222L]
│   │   ├── creator_factory_ad38.atc (4KB) [24L]
│   │   ├── economy_factory_ad26.atc (6KB) [200L]
│   │   ├── factory.atc (5KB) [165L]
│   │   ├── gameplay_factory_ad35.atc (4KB) [126L]
│   │   ├── gff_core_ad20.atc (8KB) [224L]
│   │   ├── ip_factory_ad21.atc (4KB) [147L]
│   │   ├── lifecycle_manager_ad43.atc (5KB) [25L]
│   │   ├── liveops_factory_ad27.atc (6KB) [212L]
│   │   ├── lore_factory_ad24.atc (7KB) [209L]
│   │   ├── merchandise_factory_ad29.atc (5KB) [173L]
│   │   ├── multiplayer_factory_ad37.atc (3KB) [27L]
│   │   ├── narrative_factory_ad36.atc (8KB) [245L]
│   │   ├── publishing_factory_ad39.atc (4KB) [25L]
│   │   ├── quest_factory_ad25.atc (6KB) [207L]
│   │   ├── routes.atc (2KB) [90L]
│   │   ├── security_factory_ad42.atc (4KB) [30L]
│   │   └── world_factory_ad22.atc (6KB) [235L]
│   ├── gateway/ (9 files, 564 lines)
│   │   ├── middleware/ (5 files, 247 lines)
│   │   │   ├── __init__.py (120B) [2L]
│   │   │   ├── auth.atc (2KB) [82L]
│   │   │   ├── logger.atc (2KB) [70L]
│   │   │   ├── rate_limit.atc (1KB) [50L]
│   │   │   └── signature_verify.atc (1KB) [43L]
│   │   ├── README.md (858B) [39L]
│   │   ├── __init__.py (125B) [2L]
│   │   ├── main.atc (5KB) [180L]
│   │   └── router.atc (3KB) [96L]
│   ├── kernel/ (25 files, 5147 lines)
│   │   ├── ai_kernel/ (1 files, 228 lines)
│   │   │   └── ai_kernel.atc (8KB) [228L]
│   │   ├── consensus/ (2 files, 607 lines)
│   │   │   ├── poh_integration.atc (2KB) [78L]
│   │   │   └── shiva_consensus.atc (16KB) [529L]
│   │   ├── fs/ (1 files, 142 lines)
│   │   │   └── atcfs.atc (4KB) [142L]
│   │   ├── ipc/ (2 files, 106 lines)
│   │   │   ├── __init__.py (236B) [4L]
│   │   │   └── ipc_bus.atc (3KB) [102L]
│   │   ├── net/ (1 files, 135 lines)
│   │   │   └── atcnet.atc (4KB) [135L]
│   │   ├── pkg/ (1 files, 208 lines)
│   │   │   └── manager.atc (6KB) [208L]
│   │   ├── process/ (1 files, 161 lines)
│   │   │   └── process_mgr.atc (4KB) [161L]
│   │   ├── shell/ (1 files, 296 lines)
│   │   │   └── shell.atc (8KB) [296L]
│   │   ├── README.md (1KB) [46L]
│   │   ├── ai_bus_ad13.atc (9KB) [310L]
│   │   ├── asset_bus_ad08.atc (5KB) [188L]
│   │   ├── audio_bus_ad11.atc (5KB) [199L]
│   │   ├── command_bus_ad02.atc (4KB) [168L]
│   │   ├── gcl_core_ad00.atc (7KB) [269L]
│   │   ├── input_bus_ad12.atc (5KB) [184L]
│   │   ├── ipc_bus_atc.ad.atc (8KB) [266L]
│   │   ├── message_bus_ad03.atc (6KB) [240L]
│   │   ├── network_bus_ad05.atc (8KB) [307L]
│   │   ├── physics_bus_ad10.atc (7KB) [255L]
│   │   ├── plugin_bus_ad06.atc (8KB) [286L]
│   │   ├── query_bus_ad07.atc (3KB) [128L]
│   │   ├── render_bus_ad09.atc (5KB) [164L]
│   │   └── telemetry_bus_ad14.atc (7KB) [254L]
│   ├── meta/ (8 files, 2320 lines)
│   │   ├── ai_studio_ad49.atc (11KB) [310L]
│   │   ├── cross_franchise_ad46.atc (8KB) [223L]
│   │   ├── data_lake_ad51.atc (9KB) [237L]
│   │   ├── digital_twin_ad50.atc (11KB) [303L]
│   │   ├── ip_evolution_ad45.atc (9KB) [241L]
│   │   ├── knowledge_graph_ad47.atc (11KB) [289L]
│   │   ├── simulation_factory_ad48.atc (13KB) [374L]
│   │   └── universe_factory_ad44.atc (13KB) [343L]
│   ├── shivamon/ (2 files, 188 lines)
│   │   ├── engine/ (1 files, 153 lines)
│   │   │   └── battle_engine.atc (5KB) [153L]
│   │   └── README.md (819B) [35L]
│   ├── standards/ (1 files, 32 lines)
│   │   └── README.md (706B) [32L]
│   └── ui/ (1 files, 30 lines)
│       └── README.md (586B) [30L]
├── monitoring/ (3 files, 612 lines)
│   ├── health_checks_atc08.atc (5KB) [197L]
│   ├── monitor.atc (6KB) [213L]
│   └── prometheus_metrics.atc (6KB) [202L]
├── patches/ (6 files, 264 lines)
│   ├── APPLY_FIXES.sh (1KB) [32L]
│   ├── atc9900_governance.py (2KB) [60L]
│   ├── docker-compose.yml (1KB) [42L]
│   ├── gateway_main.py (1KB) [44L]
│   ├── gateway_router.py (2KB) [49L]
│   └── poh_fixed.py (1KB) [37L]
├── reports/ (1 files, 102 lines)
│   └── SPRINT_2.3_2.4_2.7_REPORT.md (3KB) [102L]
├── scripts/ (1 files, 135 lines)
│   └── generate_validators.atc (4KB) [135L]
├── shivaos/ (3 files, 430 lines)
│   ├── fs/ (1 files, 126 lines)
│   │   └── atcfs_module.atc (4KB) [126L]
│   ├── kernel/ (1 files, 118 lines)
│   │   └── syscalls.atc (3KB) [118L]
│   └── ui/ (1 files, 186 lines)
│       └── renderer.atc (5KB) [186L]
├── tests/ (26 files, 4558 lines)
│   ├── unit/ (3 files, 654 lines)
│   │   ├── test_atclang.py (14KB) [462L] 🧪
│   │   ├── test_atcnet.py (1KB) [41L] 🧪
│   │   └── test_p2p_propagation.py (4KB) [151L] 🧪
│   ├── test_atclang.py (14KB) [470L] 🧪
│   ├── test_atclang_v03.py (2KB) [68L] 🧪
│   ├── test_bootstrap.py (10KB) [268L] 🧪
│   ├── test_did.py (1KB) [61L] 🧪
│   ├── test_discovery.py (4KB) [155L] 🧪
│   ├── test_ecdsa.py (2KB) [65L] 🧪
│   ├── test_fork_resolution.py (3KB) [101L] 🧪
│   ├── test_gateway.py (7KB) [201L] 🧪
│   ├── test_gateway_full.py (2KB) [76L] 🧪
│   ├── test_integration_atcfs_multisig.py (4KB) [129L] 🧪
│   ├── test_kai_integration.py (8KB) [297L] 🧪
│   ├── test_multinode_consensus.py (5KB) [155L] 🧪
│   ├── test_multinode_fivenode.py (3KB) [84L] 🧪
│   ├── test_node_failure_recovery.py (4KB) [143L] 🧪
│   ├── test_optimizer.py (9KB) [256L] 🧪
│   ├── test_orchestrator.py (1KB) [52L] 🧪
│   ├── test_p2p_propagation.py (5KB) [205L] 🧪
│   ├── test_persistence.py (2KB) [87L] 🧪
│   ├── test_poh.py (1KB) [63L] 🧪
│   ├── test_smart_contracts.py (3KB) [114L] 🧪
│   ├── test_stdlib.py (10KB) [298L] 🧪
│   ├── test_stdlib_dispatch.py (11KB) [312L] 🧪
│   └── test_type_checker.py (7KB) [244L] 🧪
├── tools/ (4 files, 623 lines)
│   ├── atc_issues_summary.atc (6KB) [212L]
│   ├── bigquery_pipeline.atc (4KB) [135L]
│   ├── ecdsa_impl.atc (4KB) [119L]
│   └── hf_review_pipeline.atc (5KB) [157L]
├── .gitignore (171B)
├── AAA_ASSET_SYSTEM_v1.md (3KB) [120L]
├── AGENT_MANIFEST.md (2KB) [61L]
├── AGENT_MASTERRULES.md (13KB) [438L]
├── ATCLANG_FIRST.md (900B) [31L]
├── CHANGELOG.md (6KB) [172L]
├── CONNECTION_MAP.md (2KB) [50L]
├── ECOSYSTEM.md (8KB) [179L]
├── FILE_REGISTER.md (40KB) [746L]
├── FIXES.md (3KB) [96L]
├── GENESIS_BUS_ARCHITECTURE.md (5KB) [121L]
├── GENESIS_CIVILIZATION_PLATFORM_v4.md (5KB) [153L]
├── GENESIS_COMMUNICATION_LAYER_v2.md (14KB) [431L]
├── GENESIS_FRANCHISE_FACTORY_v1.md (6KB) [166L]
├── GENESIS_FRANCHISE_FACTORY_v2.md (4KB) [101L]
├── KONSOLIDIERUNGS_ROADMAP.md (14KB) [360L]
├── LICENSE (982B)
├── MILESTONES.md (1KB) [23L]
├── NAMING_CONVENTIONS.md (4KB) [88L]
├── PERFORMANCE_REPORT.md (3KB) [123L]
├── README.md (1KB) [36L]
├── ROADMAP.md (8KB) [321L]
├── SPRINT_ROADMAP.md (20KB) [568L]
├── STATUS.md (4KB) [117L]
├── TODO.md (2KB) [48L]
├── conftest.py (374B) [9L]
└── start.atc (4KB) [129L]
```

