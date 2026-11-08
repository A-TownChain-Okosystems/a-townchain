# A-TownChain OS — AI Studio Komponenten-Wiki

> **Quelle:** `A-TownChain-Okosystems/A-TownChain--kosystems` (Google AI Studio App)
> **Sync-Datum:** 2026-07-05
> **App-URL:** https://ai.studio/apps/7546bfc1-4139-434e-aef9-3c24e6578bb2
> **Gemini Model:** `gemini-3.1-pro-preview`

## Übersicht

Das persönliche Repository enthält die vollständige Google AI Studio Anwendung für A-TownChain OS. Alle 148 React/TypeScript-Komponenten, 4 Contexts, 2 Hooks und 44 Python-Backend-Module wurden in dieses Wiki synchronisiert.

**Struktur:**
- `aistudio/src/` — TypeScript Frontend (190 Dateien)
- `aistudio/temp_repo/` — Python Backend (133 Dateien)
- `aistudio/` (root) — Config & Scripts (44 Dateien)
- `aistudio/tests/` — Tests (2 Dateien)
- `aistudio/workspace/` — Workspace (8 Dateien)

---

## TypeScript Komponenten (148)

### ⚙️ Settings & User (6 Komponenten, 192,115 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `SettingsView.tsx` | `src/components/SettingsView.tsx` | 108,294 bytes |
| `LoginOverlay.tsx` | `src/components/LoginOverlay.tsx` | 39,836 bytes |
| `SpecificSettingsViews.tsx` | `src/components/SpecificSettingsViews.tsx` | 17,451 bytes |
| `UserProfileView.tsx` | `src/components/UserProfileView.tsx` | 12,569 bytes |
| `SessionExportView.tsx` | `src/components/SessionExportView.tsx` | 9,160 bytes |
| `ThemeSwitcher.tsx` | `src/components/ThemeSwitcher.tsx` | 4,805 bytes |

### ⛓️ Blockchain & Crypto (5 Komponenten, 126,341 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `ConsensusIntegrationGuide.tsx` | `src/components/ConsensusIntegrationGuide.tsx` | 72,510 bytes |
| `ATCWalletView.tsx` | `src/components/ATCWalletView.tsx` | 26,927 bytes |
| `DeFiLiquidityPoolView.tsx` | `src/components/DeFiLiquidityPoolView.tsx` | 14,246 bytes |
| `GenesisBlockGeneratorView.tsx` | `src/components/GenesisBlockGeneratorView.tsx` | 7,143 bytes |
| `TxOrchestratorView.tsx` | `src/components/TxOrchestratorView.tsx` | 5,515 bytes |

### 🌐 Network & Sync (15 Komponenten, 262,117 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `ATownOSNode.tsx` | `src/components/ATownOSNode.tsx` | 73,358 bytes |
| `GitHubRepoSyncView.tsx` | `src/components/GitHubRepoSyncView.tsx` | 65,260 bytes |
| `GitHubStatusDashboard.tsx` | `src/components/GitHubStatusDashboard.tsx` | 34,928 bytes |
| `NetworkExplorerView.tsx` | `src/components/NetworkExplorerView.tsx` | 18,101 bytes |
| `P2PChatView.tsx` | `src/components/P2PChatView.tsx` | 13,136 bytes |
| `SyncHistoryModal.tsx` | `src/components/SyncHistoryModal.tsx` | 11,990 bytes |
| `SyncMetricsView.tsx` | `src/components/SyncMetricsView.tsx` | 7,412 bytes |
| `GitOpsView.tsx` | `src/components/GitOpsView.tsx` | 7,359 bytes |
| `SyncStatusOverview.tsx` | `src/components/SyncStatusOverview.tsx` | 7,299 bytes |
| `SyncDashboardModal.tsx` | `src/components/SyncDashboardModal.tsx` | 4,831 bytes |
| `GitGraphVisualization.tsx` | `src/components/GitGraphVisualization.tsx` | 4,749 bytes |
| `NodeHealthMonitor.tsx` | `src/components/NodeHealthMonitor.tsx` | 4,530 bytes |
| `NetworkExplorerView.test.tsx` | `src/components/NetworkExplorerView.test.tsx` | 4,114 bytes |
| `SyncStatusDonutChart.tsx` | `src/components/SyncStatusDonutChart.tsx` | 2,939 bytes |
| `NetworkTopologyView.tsx` | `src/components/NetworkTopologyView.tsx` | 2,111 bytes |

### 🎨 Apps & Creative (10 Komponenten, 193,960 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `FranchiseFactoryView.tsx` | `src/components/FranchiseFactoryView.tsx` | 85,239 bytes |
| `MarketplaceView.tsx` | `src/components/MarketplaceView.tsx` | 22,708 bytes |
| `MediaApps.tsx` | `src/components/MediaApps.tsx` | 18,651 bytes |
| `ATCDjStudioView.tsx` | `src/components/ATCDjStudioView.tsx` | 17,533 bytes |
| `SocialMediaView.tsx` | `src/components/SocialMediaView.tsx` | 16,614 bytes |
| `OfficeSuiteView.tsx` | `src/components/OfficeSuiteView.tsx` | 12,754 bytes |
| `VideoGeneratorTab.tsx` | `src/components/VideoGeneratorTab.tsx` | 7,809 bytes |
| `TextGeneratorTab.tsx` | `src/components/TextGeneratorTab.tsx` | 6,859 bytes |
| `ImageGeneratorTab.tsx` | `src/components/ImageGeneratorTab.tsx` | 5,441 bytes |
| `OfficeApps.tsx` | `src/components/OfficeApps.tsx` | 352 bytes |

### 🏗️ Architecture & Modules (12 Komponenten, 186,787 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `ArchitectureView.tsx` | `src/components/ArchitectureView.tsx` | 39,103 bytes |
| `StructureView.tsx` | `src/components/StructureView.tsx` | 23,340 bytes |
| `TechTreeView.tsx` | `src/components/TechTreeView.tsx` | 19,165 bytes |
| `ModulesPluginView.tsx` | `src/components/ModulesPluginView.tsx` | 19,118 bytes |
| `TechDocsView.tsx` | `src/components/TechDocsView.tsx` | 17,551 bytes |
| `EcosystemTreeOverlay.tsx` | `src/components/EcosystemTreeOverlay.tsx` | 12,784 bytes |
| `EcosystemVisualizerView.tsx` | `src/components/EcosystemVisualizerView.tsx` | 12,434 bytes |
| `EcosystemInstaller.tsx` | `src/components/EcosystemInstaller.tsx` | 11,573 bytes |
| `StrategicArchitectureMap.tsx` | `src/components/StrategicArchitectureMap.tsx` | 9,391 bytes |
| `EcosystemUmlView.tsx` | `src/components/EcosystemUmlView.tsx` | 7,620 bytes |
| `ArchitectureDependencyGraph.tsx` | `src/components/ArchitectureDependencyGraph.tsx` | 7,598 bytes |
| `RoadmapView.tsx` | `src/components/RoadmapView.tsx` | 7,110 bytes |

### 💻 ATCLang & Development (9 Komponenten, 141,427 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `AtcLangArchitectureView.tsx` | `src/components/AtcLangArchitectureView.tsx` | 34,037 bytes |
| `ATCLangEditor.tsx` | `src/components/ATCLangEditor.tsx` | 27,629 bytes |
| `AtvmSandboxView.tsx` | `src/components/AtvmSandboxView.tsx` | 26,808 bytes |
| `SourceCodeViewer.tsx` | `src/components/SourceCodeViewer.tsx` | 21,034 bytes |
| `AtcLangPlaygroundView.tsx` | `src/components/AtcLangPlaygroundView.tsx` | 12,972 bytes |
| `TerminalView.tsx` | `src/components/TerminalView.tsx` | 6,653 bytes |
| `CodeAnalyzerView.tsx` | `src/components/CodeAnalyzerView.tsx` | 5,072 bytes |
| `AtcLangPresetsView.tsx` | `src/components/AtcLangPresetsView.tsx` | 4,052 bytes |
| `AtvmSandboxView.test.tsx` | `src/components/AtvmSandboxView.test.tsx` | 3,170 bytes |

### 📊 Monitoring & Analytics (17 Komponenten, 246,427 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `MetricsView.tsx` | `src/components/MetricsView.tsx` | 57,633 bytes |
| `SoftwareAuditView.tsx` | `src/components/SoftwareAuditView.tsx` | 39,226 bytes |
| `LazyMetricsCharts.tsx` | `src/components/LazyMetricsCharts.tsx` | 31,950 bytes |
| `SystemDiagnosticsView.tsx` | `src/components/SystemDiagnosticsView.tsx` | 18,442 bytes |
| `BenchmarkCenterView.tsx` | `src/components/BenchmarkCenterView.tsx` | 15,624 bytes |
| `ATownDashboardView.tsx` | `src/components/ATownDashboardView.tsx` | 15,263 bytes |
| `ReportsView.tsx` | `src/components/ReportsView.tsx` | 10,574 bytes |
| `SystemHealthDashboard.tsx` | `src/components/SystemHealthDashboard.tsx` | 10,050 bytes |
| `ProjectAuditDashboard.tsx` | `src/components/ProjectAuditDashboard.tsx` | 7,653 bytes |
| `IdeaToAppFlowchartView.tsx` | `src/components/IdeaToAppFlowchartView.tsx` | 7,476 bytes |
| `RepositoryLineChart.tsx` | `src/components/RepositoryLineChart.tsx` | 6,399 bytes |
| `WebhookMonitor.tsx` | `src/components/WebhookMonitor.tsx` | 5,760 bytes |
| `RepositoryActivityChart.tsx` | `src/components/RepositoryActivityChart.tsx` | 5,301 bytes |
| `MetricsDashboard.tsx` | `src/components/MetricsDashboard.tsx` | 4,523 bytes |
| `CommitHeatmap.tsx` | `src/components/CommitHeatmap.tsx` | 4,419 bytes |
| `ApiHealthWidget.tsx` | `src/components/ApiHealthWidget.tsx` | 3,870 bytes |
| `SystemHealthDashboardWidget.tsx` | `src/components/SystemHealthDashboardWidget.tsx` | 2,264 bytes |

### 📦 Other (37 Komponenten, 354,835 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `ProjectHubView.tsx` | `src/components/ProjectHubView.tsx` | 30,810 bytes |
| `HardwareDriversView.tsx` | `src/components/HardwareDriversView.tsx` | 20,866 bytes |
| `PipelineGeneratorTab.tsx` | `src/components/PipelineGeneratorTab.tsx` | 19,631 bytes |
| `TodoView.tsx` | `src/components/TodoView.tsx` | 19,005 bytes |
| `DeveloperKnowledgeBaseView.tsx` | `src/components/DeveloperKnowledgeBaseView.tsx` | 18,914 bytes |
| `SoftwareKnowledgeDbView.tsx` | `src/components/SoftwareKnowledgeDbView.tsx` | 18,703 bytes |
| `CryptoVisualizationView.tsx` | `src/components/CryptoVisualizationView.tsx` | 18,435 bytes |
| `ApiOrchestratorView.tsx` | `src/components/ApiOrchestratorView.tsx` | 17,573 bytes |
| `ConflictResolutionModal.tsx` | `src/components/ConflictResolutionModal.tsx` | 11,970 bytes |
| `AtcAssetsDbView.tsx` | `src/components/AtcAssetsDbView.tsx` | 11,561 bytes |
| `ATCAssetView.tsx` | `src/components/ATCAssetView.tsx` | 11,519 bytes |
| `BatteryStatus.tsx` | `src/components/BatteryStatus.tsx` | 11,266 bytes |
| `AtcWhitepaperView.tsx` | `src/components/AtcWhitepaperView.tsx` | 10,430 bytes |
| `StorageManagerView.tsx` | `src/components/StorageManagerView.tsx` | 10,098 bytes |
| `ApiInterfacesView.tsx` | `src/components/ApiInterfacesView.tsx` | 9,350 bytes |
| `AppGlobeView.tsx` | `src/components/AppGlobeView.tsx` | 9,326 bytes |
| `AssetVaultView.tsx` | `src/components/AssetVaultView.tsx` | 9,077 bytes |
| `ProtocolsView.tsx` | `src/components/ProtocolsView.tsx` | 8,839 bytes |
| `BattleArenaView.tsx` | `src/components/BattleArenaView.tsx` | 8,560 bytes |
| `AgentCivilizationView.tsx` | `src/components/AgentCivilizationView.tsx` | 8,475 bytes |
| `CiCdPipelineView.tsx` | `src/components/CiCdPipelineView.tsx` | 7,273 bytes |
| `DbOrchestratorView.tsx` | `src/components/DbOrchestratorView.tsx` | 6,433 bytes |
| `GateToHellBrowser.tsx` | `src/components/GateToHellBrowser.tsx` | 6,032 bytes |
| `DevToolsView.tsx` | `src/components/DevToolsView.tsx` | 5,428 bytes |
| `PaymentSystemView.tsx` | `src/components/PaymentSystemView.tsx` | 4,730 bytes |
| `AtsSuite.tsx` | `src/components/AtsSuite.tsx` | 4,666 bytes |
| `DataProcessingView.tsx` | `src/components/DataProcessingView.tsx` | 4,113 bytes |
| `CalculatorView.tsx` | `src/components/CalculatorView.tsx` | 3,767 bytes |
| `DependencyMapView.tsx` | `src/components/DependencyMapView.tsx` | 3,737 bytes |
| `DistributedDatalakeView.tsx` | `src/components/DistributedDatalakeView.tsx` | 3,651 bytes |
| `CalendarView.tsx` | `src/components/CalendarView.tsx` | 3,401 bytes |
| `ClockView.tsx` | `src/components/ClockView.tsx` | 3,310 bytes |
| `SystemLogsView.tsx` | `src/components/SystemLogsView.tsx` | 3,122 bytes |
| `InterfacesView.tsx` | `src/components/InterfacesView.tsx` | 3,041 bytes |
| `JsExampleRunner.tsx` | `src/components/JsExampleRunner.tsx` | 2,920 bytes |
| `NotepadView.tsx` | `src/components/NotepadView.tsx` | 2,477 bytes |
| `SystemFinderView.tsx` | `src/components/SystemFinderView.tsx` | 2,326 bytes |

### 🖥️ Desktop UI (9 Komponenten, 60,718 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `IntegrationsWindow.tsx` | `src/components/IntegrationsWindow.tsx` | 21,628 bytes |
| `FileManagerView.tsx` | `src/components/FileManagerView.tsx` | 6,973 bytes |
| `Window.tsx` | `src/components/Window.tsx` | 6,608 bytes |
| `DeploymentPipelineWidget.tsx` | `src/components/DeploymentPipelineWidget.tsx` | 6,481 bytes |
| `WindowExtras.tsx` | `src/components/WindowExtras.tsx` | 4,957 bytes |
| `FolderView.tsx` | `src/components/FolderView.tsx` | 4,555 bytes |
| `GpuPerformanceWidget.tsx` | `src/components/GpuPerformanceWidget.tsx` | 4,460 bytes |
| `TaskManagerView.tsx` | `src/components/TaskManagerView.tsx` | 3,982 bytes |
| `TooltipIcon.tsx` | `src/components/TooltipIcon.tsx` | 1,074 bytes |

### 🛡️ Governance & Security (10 Komponenten, 94,290 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `GovernanceView.tsx` | `src/components/GovernanceView.tsx` | 23,828 bytes |
| `RescueSystemView.tsx` | `src/components/RescueSystemView.tsx` | 16,547 bytes |
| `AntiCheatView.tsx` | `src/components/AntiCheatView.tsx` | 14,828 bytes |
| `ComplianceView.tsx` | `src/components/ComplianceView.tsx` | 9,207 bytes |
| `ZeroKnowledgeProofView.tsx` | `src/components/ZeroKnowledgeProofView.tsx` | 6,297 bytes |
| `LegalView.tsx` | `src/components/LegalView.tsx` | 6,208 bytes |
| `SemanticGraphView.tsx` | `src/components/SemanticGraphView.tsx` | 4,799 bytes |
| `ZkCircuitEditorView.tsx` | `src/components/ZkCircuitEditorView.tsx` | 4,605 bytes |
| `ComplianceEngineView.tsx` | `src/components/ComplianceEngineView.tsx` | 4,239 bytes |
| `ZkVisualizationView.tsx` | `src/components/ZkVisualizationView.tsx` | 3,732 bytes |

### 🤖 AI & Kernel (15 Komponenten, 153,069 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `AiOsEngineView.tsx` | `src/components/AiOsEngineView.tsx` | 20,088 bytes |
| `BlockchainLedgerView.tsx` | `src/components/BlockchainLedgerView.tsx` | 13,386 bytes |
| `MainnetLaunchView.tsx` | `src/components/MainnetLaunchView.tsx` | 12,594 bytes |
| `AiSoftwareWorkflowView.tsx` | `src/components/AiSoftwareWorkflowView.tsx` | 11,944 bytes |
| `BlockchainEcosystemView.tsx` | `src/components/BlockchainEcosystemView.tsx` | 9,427 bytes |
| `AiGameEngineTab.tsx` | `src/components/AiGameEngineTab.tsx` | 9,415 bytes |
| `AiCharacterBioTab.tsx` | `src/components/AiCharacterBioTab.tsx` | 9,343 bytes |
| `AiTimelineEngineTab.tsx` | `src/components/AiTimelineEngineTab.tsx` | 9,276 bytes |
| `AiAnimationEngineTab.tsx` | `src/components/AiAnimationEngineTab.tsx` | 9,139 bytes |
| `AiAudioEngineTab.tsx` | `src/components/AiAudioEngineTab.tsx` | 9,132 bytes |
| `Ai3DRenderEngineTab.tsx` | `src/components/Ai3DRenderEngineTab.tsx` | 9,097 bytes |
| `AtcCoreKernelView.tsx` | `src/components/AtcCoreKernelView.tsx` | 9,090 bytes |
| `PoAITrainingEngineView.tsx` | `src/components/PoAITrainingEngineView.tsx` | 8,491 bytes |
| `AiKernelView.tsx` | `src/components/AiKernelView.tsx` | 6,834 bytes |
| `Paint3DView.tsx` | `src/components/Paint3DView.tsx` | 5,813 bytes |

### 🧪 Testing & Testnet (3 Komponenten, 30,469 bytes)

| Komponente | Pfad | Größe |
|------------|------|-------|
| `TestnetSimulationView.tsx` | `src/components/TestnetSimulationView.tsx` | 14,612 bytes |
| `TestnetOrchestrationView.tsx` | `src/components/TestnetOrchestrationView.tsx` | 9,087 bytes |
| `ATownTestView.tsx` | `src/components/ATownTestView.tsx` | 6,770 bytes |

## React Contexts (4)

| Context | Pfad | Größe |
|---------|------|-------|
| `FirebaseContext.tsx` | `src/contexts/FirebaseContext.tsx` | 2,320 bytes |
| `GoogleWorkspaceContext.tsx` | `src/contexts/GoogleWorkspaceContext.tsx` | 2,578 bytes |
| `SyncMetricsContext.tsx` | `src/contexts/SyncMetricsContext.tsx` | 1,256 bytes |
| `WalletContext.tsx` | `src/contexts/WalletContext.tsx` | 1,459 bytes |

## React Hooks (2)

| Hook | Pfad | Größe |
|------|------|-------|
| `useGoogleSheetsSync.ts` | `src/hooks/useGoogleSheetsSync.ts` | 8,831 bytes |
| `useKeyboardShortcut.ts` | `src/hooks/useKeyboardShortcut.ts` | 799 bytes |

## Bibliotheken (6)

| Datei | Pfad | Größe |
|-------|------|-------|
| `CryptoEngine.ts` | `src/lib/CryptoEngine.ts` | 1,429 bytes |
| `firebase-admin.ts` | `src/lib/firebase-admin.ts` | 444 bytes |
| `firebase.ts` | `src/lib/firebase.ts` | 2,207 bytes |
| `indexedDb.ts` | `src/lib/indexedDb.ts` | 2,858 bytes |
| `syncLogic.test.ts` | `src/lib/syncLogic.test.ts` | 2,831 bytes |
| `syncLogic.ts` | `src/lib/syncLogic.ts` | 1,428 bytes |

## Utilities (4)

| Datei | Pfad | Größe |
|-------|------|-------|
| `appSync.tsx` | `src/utils/appSync.tsx` | 3,051 bytes |
| `auditUtils.test.ts` | `src/utils/auditUtils.test.ts` | 1,635 bytes |
| `auditUtils.ts` | `src/utils/auditUtils.ts` | 649 bytes |
| `crypto.ts` | `src/utils/crypto.ts` | 4,705 bytes |

## TypeScript Backend (2)

| Datei | Pfad | Größe |
|-------|------|-------|
| `engine.ts` | `src/backend/blockchain/engine.ts` | 3,572 bytes |
| `network.ts` | `src/backend/p2p/network.ts` | 2,310 bytes |

## Root src/ Dateien (17)

| Datei | Pfad | Größe |
|-------|------|-------|
| `App.tsx` | `src/App.tsx` | 239,558 bytes |
| `DesktopApp.tsx` | `src/DesktopApp.tsx` | 123,928 bytes |
| `atcLangRoadmapData.ts` | `src/atcLangRoadmapData.ts` | 6,283 bytes |
| `atcLangWikiData.ts` | `src/atcLangWikiData.ts` | 16,525 bytes |
| `auditData.ts` | `src/auditData.ts` | 4,756 bytes |
| `data.ts` | `src/data.ts` | 17,749 bytes |
| `ecosystemData.ts` | `src/ecosystemData.ts` | 11,826 bytes |
| `fix_translation.cjs` | `src/fix_translation.cjs` | 463 bytes |
| `index.css` | `src/index.css` | 5,584 bytes |
| `main.tsx` | `src/main.tsx` | 774 bytes |
| `marketplaceApps.ts` | `src/marketplaceApps.ts` | 6,507 bytes |
| `requirementsData.ts` | `src/requirementsData.ts` | 1,105 bytes |
| `roadmapData.ts` | `src/roadmapData.ts` | 7,821 bytes |
| `standardsData.ts` | `src/standardsData.ts` | 4,527 bytes |
| `tierData.ts` | `src/tierData.ts` | 17,205 bytes |
| `types.ts` | `src/types.ts` | 275 bytes |
| `wikiData.ts` | `src/wikiData.ts` | 49,034 bytes |

---

## Python Backend — `temp_repo/`

### Core (6)

| Datei | Pfad | Größe |
|-------|------|-------|
| `ai_kernel.py` | `temp_repo/core/ai_kernel.py` | 15,994 bytes |
| `__init__.py` | `temp_repo/core/crypto/__init__.py` | 436 bytes |
| `event_bus.py` | `temp_repo/core/event_bus.py` | 418 bytes |
| `kai_cli.py` | `temp_repo/core/kai_cli.py` | 9,244 bytes |
| `kernel.py` | `temp_repo/core/kernel.py` | 637 bytes |
| `module_loader.py` | `temp_repo/core/module_loader.py` | 441 bytes |

### Backend API (25)

| Datei | Pfad | Größe |
|-------|------|-------|
| `.env.example` | `temp_repo/backend/.env.example` | 167 bytes |
| `__init__.py` | `temp_repo/backend/__init__.py` | 22 bytes |
| `__init__.py` | `temp_repo/backend/api/__init__.py` | 12 bytes |
| `kai_routes.py` | `temp_repo/backend/api/kai_routes.py` | 11,910 bytes |
| `__init__.py` | `temp_repo/backend/api/orchestrator/__init__.py` | 19 bytes |
| `orchestrator.py` | `temp_repo/backend/api/orchestrator/orchestrator.py` | 2,722 bytes |
| `__init__.py` | `temp_repo/backend/api/routes/__init__.py` | 16 bytes |
| `ai_routes.py` | `temp_repo/backend/api/routes/ai_routes.py` | 4,117 bytes |
| `blockchain.py` | `temp_repo/backend/api/routes/blockchain.py` | 2,085 bytes |
| `game_routes.py` | `temp_repo/backend/api/routes/game_routes.py` | 1,867 bytes |
| `governance_routes.py` | `temp_repo/backend/api/routes/governance_routes.py` | 1,874 bytes |
| `marketplace_routes.py` | `temp_repo/backend/api/routes/marketplace_routes.py` | 1,934 bytes |
| `nodes_routes.py` | `temp_repo/backend/api/routes/nodes_routes.py` | 1,556 bytes |
| `orchestrator_routes.py` | `temp_repo/backend/api/routes/orchestrator_routes.py` | 873 bytes |
| `wallet.py` | `temp_repo/backend/api/routes/wallet.py` | 1,850 bytes |
| `server.py` | `temp_repo/backend/api/server.py` | 1,992 bytes |
| `__init__.py` | `temp_repo/backend/db/__init__.py` | 61 bytes |
| `connection.py` | `temp_repo/backend/db/connection.py` | 1,137 bytes |
| `migrate.py` | `temp_repo/backend/db/migrate.py` | 1,844 bytes |
| `repository.py` | `temp_repo/backend/db/repository.py` | 6,802 bytes |
| `schema.sql` | `temp_repo/backend/db/schema.sql` | 2,181 bytes |
| `main.py` | `temp_repo/backend/main.py` | 427 bytes |
| `requirements.txt` | `temp_repo/backend/requirements.txt` | 90 bytes |
| `__init__.py` | `temp_repo/backend/wallet/__init__.py` | 24 bytes |
| `wallet.py` | `temp_repo/backend/wallet/wallet.py` | 5,078 bytes |

### ATCLang Compiler (13)

| Datei | Pfad | Größe |
|-------|------|-------|
| `ATCLANG_SPEC.md` | `temp_repo/atclang/ATCLANG_SPEC.md` | 913 bytes |
| `__init__.py` | `temp_repo/atclang/__init__.py` | 10 bytes |
| `__init__.py` | `temp_repo/atclang/compiler/__init__.py` | 19 bytes |
| `compiler.py` | `temp_repo/atclang/compiler/compiler.py` | 18,057 bytes |
| `__init__.py` | `temp_repo/atclang/lexer/__init__.py` | 10 bytes |
| `lexer.py` | `temp_repo/atclang/lexer/lexer.py` | 11,131 bytes |
| `__init__.py` | `temp_repo/atclang/parser/__init__.py` | 10 bytes |
| `ast_nodes.py` | `temp_repo/atclang/parser/ast_nodes.py` | 5,359 bytes |
| `parser.py` | `temp_repo/atclang/parser/parser.py` | 15,414 bytes |
| `__init__.py` | `temp_repo/atclang/repl/__init__.py` | 15 bytes |
| `repl.py` | `temp_repo/atclang/repl/repl.py` | 6,525 bytes |
| `__init__.py` | `temp_repo/atclang/vm/__init__.py` | 10 bytes |
| `atcvm.py` | `temp_repo/atclang/vm/atcvm.py` | 11,526 bytes |

### Dokumentation (27)

| Datei | Pfad | Größe |
|-------|------|-------|
| `KAI_INTEGRATION.md` | `temp_repo/docs/KAI_INTEGRATION.md` | 6,309 bytes |
| `README.md` | `temp_repo/docs/README.md` | 2,325 bytes |
| `CONSENSUS.md` | `temp_repo/docs/architecture/CONSENSUS.md` | 3,131 bytes |
| `GATEWAY.md` | `temp_repo/docs/architecture/GATEWAY.md` | 2,985 bytes |
| `Testnet.md` | `temp_repo/docs/architecture/TESTNET.md` | 21,439 bytes |
| `WALLET_KEYGEN.md` | `temp_repo/docs/architecture/WALLET_KEYGEN.md` | 2,292 bytes |
| `ATC_TOKEN_STANDARD.md` | `temp_repo/docs/contracts/ATC_TOKEN_STANDARD.md` | 534 bytes |
| `SHIVAMON_NFT_CONTRACT.md` | `temp_repo/docs/contracts/SHIVAMON_NFT_CONTRACT.md` | 20,661 bytes |
| `ISSUE_01_SMART_CONTRACTS.md` | `temp_repo/docs/issues/ISSUE_01_SMART_CONTRACTS.md` | 4,215 bytes |
| `ISSUE_02_GEMINI_AI.md` | `temp_repo/docs/issues/ISSUE_02_GEMINI_AI.md` | 3,929 bytes |
| `ISSUE_03_BATTLE_UI.md` | `temp_repo/docs/issues/ISSUE_03_BATTLE_UI.md` | 5,041 bytes |
| `ISSUE_04_PERSISTENZ.md` | `temp_repo/docs/issues/ISSUE_04_PERSISTENZ.md` | 4,527 bytes |
| `ISSUE_05_EXPLORER.md` | `temp_repo/docs/issues/ISSUE_05_EXPLORER.md` | 3,721 bytes |
| `ISSUE_06_ECDSA.md` | `temp_repo/docs/issues/ISSUE_06_ECDSA.md` | 4,735 bytes |
| `ISSUE_07_BUILD.md` | `temp_repo/docs/issues/ISSUE_07_BUILD.md` | 3,424 bytes |
| `ISSUE_08_TESTNET.md` | `temp_repo/docs/issues/ISSUE_08_TESTNET.md` | 3,136 bytes |
| `ISSUE_09_GOVERNANCE.md` | `temp_repo/docs/issues/ISSUE_09_GOVERNANCE.md` | 2,941 bytes |
| `ISSUE_10_BRIDGE.md` | `temp_repo/docs/issues/ISSUE_10_BRIDGE.md` | 1,926 bytes |
| `ISSUE_11_BREEDING.md` | `temp_repo/docs/issues/ISSUE_11_BREEDING.md` | 3,107 bytes |
| `ISSUE_12_SOLIDITY.md` | `temp_repo/docs/issues/ISSUE_12_SOLIDITY.md` | 4,342 bytes |
| `ISSUE_13_MARKETPLACE.md` | `temp_repo/docs/issues/ISSUE_13_MARKETPLACE.md` | 3,877 bytes |
| `ISSUE_14_BOOTSTRAP_NODE.md` | `temp_repo/docs/issues/ISSUE_14_BOOTSTRAP_NODE.md` | 7,437 bytes |
| `ISSUE_20_GATEWAY_TESTS.md` | `temp_repo/docs/issues/ISSUE_20_GATEWAY_TESTS.md` | 2,039 bytes |
| `README.md` | `temp_repo/docs/issues/README.md` | 3,282 bytes |
| `TESTNET_INDEX.md` | `temp_repo/docs/issues/TESTNET_INDEX.md` | 1,685 bytes |
| `kai-os-wiki.md` | `temp_repo/docs/kai-os-wiki.md` | 266,512 bytes |
| `wiki-sync.yml` | `temp_repo/docs/workflows/wiki-sync.yml` | 8,958 bytes |

---

## Konfiguration

### Gemini AI
- **Model:** `gemini-3.1-pro-preview`
- **Zweck:** High-security autonomous agent logic, dynamic metrics analysis, smart contract anomaly detection
- **Integration:** AI Services Layer im A-TownOS-Kernel
- **API:** `@google/genai` v2.4.0

### Dependencies (package.json)
- `@google/genai` v2.4.0 — Gemini AI
- `@notionhq/client` v5.22.0 — Notion Integration
- `firebase` v12.15.0 + `firebase-admin` v14.0.0 — Firebase
- `three` v0.184.0 — 3D Rendering
- `recharts` v3.8.1 — Charts
- `drizzle-orm` v0.45.2 + `pg` v8.22.0 — PostgreSQL
- `d3` v7.9.0 — Data Visualization
- `@noble/ed25519` v3.1.0 + `@noble/hashes` v2.2.0 — Kryptographie
- `@uiw/react-codemirror` v4.25.10 — Code Editor
- `motion` v12.23.24 — Animationen
- `jspdf` v4.2.1 — PDF Export
- `qrcode.react` v4.2.0 — QR Codes

### Desktop Wallpapers
- **Aurora Fjord** (Standard) — Nordlichter über Fjord
- **Ember Forest** — Pilzwald bei Sonnenuntergang
- **Genesis Mandala** — Blaues Lotus-Mandala
- **Classic Grid** — Ursprünglicher Unsplash-Hintergrund

### AI Studio App
- **URL:** https://ai.studio/apps/7546bfc1-4139-434e-aef9-3c24e6578bb2
- **Capabilities:** `MAJOR_CAPABILITY_SERVER_SIDE_GEMINI_API`
- **Metadata Name:** "A-TownChain Explorer"

---

## Querverweise

- **Hauptrepo (Code):** `A-TownChain-Okosystems/a-townchain-os`
- **Dokumentationsrepo:** `A-TownChain-Okosystems/a-townchain-os-docs` (dieses Wiki)
- **Persönliches Repo:** `A-TownChain-Okosystems/A-TownChain--kosystems` (AI Studio Source)
- **KAI-OS Wiki:** `A-TownChain-Okosystems/kai-os-wiki`
- **Whitepaper:** `A-TownChain-Okosystems/atc-whitepaper`

---

> **Sync-Status:** ✅ 378/378 Dateien synchronisiert (100%)
> **Letzter Sync:** 2026-07-05 durch Aurora Sync Agent
