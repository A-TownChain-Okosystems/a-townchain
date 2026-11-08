# ARCHITECTURE.md — atc-aistudio
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .env.example
├── .gitignore
├── AGENTS.md
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── GEMINI.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SOFTWARE_ROADMAP.md
├── STATUS.md
├── aaa_asset_core.atc
├── ai_assets.atc
├── ai_studio_ad49.atc
├── animation.atc
├── asset_bundle.atc
├── asset_genome_ad66.atc
├── assets/
│   └── .aistudio/
│       └── .gitignore
├── check_dups2.js
├── check_dups_all.js
├── check_dups_desktop.js
├── check_dups_windows_map.js
├── civilization_engine_ad60.atc
├── cloud_assets.atc
├── cross_franchise_ad46.atc
├── data_lake_ad51.atc
├── digital_twin_ad50.atc
├── ecosystem_ai_mesh_ad62.atc
├── encryption.atc
├── evolution_engine_ad69.atc
├── experience_orchestrator_ad68.atc
├── federated_learning.atc
├── fetch.js
├── firebase-applet-config.json
├── fix.js
├── fix2.js
├── fix_react_imports.cjs
├── fix_wiki.cjs
├── fix_wiki.js
├── gcp_core_ad70.atc
├── global_simulation_core_ad64.atc
├── hot_reload.atc
├── identity_layer_ad65.atc
├── index.html
├── ip_evolution_ad45.atc
├── knowledge_graph_ad47.atc
├── mark_completed.ts
├── mark_completed_src.ts
├── memory_cleanup.atc
├── metadata.json
├── mod_system.atc
├── model3d.atc
├── move_back.js
├── output.txt
├── package-lock.json
├── package.json
├── persistent_world_engine_ad61.atc
├── priority_loading.atc
├── proc_universe_generator_ad63.atc
├── production_pipeline_ad67.atc
├── render_pipeline.atc
├── replace.js
├── replace_langs.cjs
├── replace_langs_2.cjs
├── replace_langs_3.cjs
├── replace_langs_4.cjs
├── replace_langs_5.cjs
├── replace_langs_6.cjs
├── script.cjs
├── script.js
├── script2.cjs
├── server.ts
├── shader_system.atc
├── simulation_factory_ad48.atc
├── src/
│   ├── App.tsx
│   ├── DesktopApp.tsx
│   ├── atcLangRoadmapData.ts
│   ├── atcLangWikiData.ts
│   ├── auditData.ts
│   ├── backend/
│   │   └── p2p/
│   │       └── network.ts
│   ├── components/
│   │   ├── ATCAssetView.tsx
│   │   ├── ATCDjStudioView.tsx
│   │   ├── ATCLangEditor.tsx
│   │   ├── ATCWalletView.tsx
│   │   ├── ATownOSNode.tsx
│   │   ├── ATownTestView.tsx
│   │   ├── AgentCivilizationView.tsx
│   │   ├── Ai3DRenderEngineTab.tsx
│   │   ├── AiAnimationEngineTab.tsx
│   │   ├── AiAudioEngineTab.tsx
│   │   ├── AiCharacterBioTab.tsx
│   │   ├── AiGameEngineTab.tsx
│   │   ├── AiKernelView.tsx
│   │   ├── AiOsEngineView.tsx
│   │   ├── AiSoftwareWorkflowView.tsx
│   │   ├── AiTimelineEngineTab.tsx
│   │   ├── AntiCheatView.tsx
│   │   ├── ApiHealthWidget.tsx
│   │   ├── ApiInterfacesView.tsx
│   │   ├── ApiOrchestratorView.tsx
│   │   ├── AppGlobeView.tsx
│   │   ├── ArchitectureDependencyGraph.tsx
│   │   ├── ArchitectureView.tsx
│   │   ├── AssetVaultView.tsx
│   │   ├── AtcAssetsDbView.tsx
│   │   ├── AtcCoreKernelView.tsx
│   │   ├── AtcLangArchitectureView.tsx
│   │   ├── AtcLangPlaygroundView.tsx
│   │   ├── AtcLangPresetsView.tsx
│   │   ├── AtvmSandboxView.test.tsx
│   │   ├── AtvmSandboxView.tsx
│   │   ├── BatteryStatus.tsx
│   │   ├── BattleArenaView.tsx
│   │   ├── BenchmarkCenterView.tsx
│   │   ├── CalculatorView.tsx
│   │   ├── CalendarView.tsx
│   │   ├── ClockView.tsx
│   │   ├── CodeAnalyzerView.tsx
│   │   ├── CommitHeatmap.tsx
│   │   ├── ComplianceEngineView.tsx
│   │   ├── ComplianceView.tsx
│   │   ├── ConflictResolutionModal.tsx
│   │   ├── CryptoVisualizationView.tsx
│   │   ├── DataProcessingView.tsx
│   │   ├── DbOrchestratorView.tsx
│   │   ├── DependencyMapView.tsx
│   │   ├── DevToolsView.tsx
│   │   ├── DeveloperKnowledgeBaseView.tsx
│   │   ├── DistributedDatalakeView.tsx
│   │   ├── EcosystemInstaller.tsx
│   │   ├── EcosystemTreeOverlay.tsx
│   │   ├── EcosystemUmlView.tsx
│   │   ├── EcosystemVisualizerView.tsx
│   │   ├── FileManagerView.tsx
│   │   ├── FolderView.tsx
│   │   ├── GateToHellBrowser.tsx
│   │   ├── GenesisBlockGeneratorView.tsx
│   │   ├── GitGraphVisualization.tsx
│   │   ├── GitHubRepoSyncView.tsx
│   │   ├── GitOpsView.tsx
│   │   ├── GovernanceView.tsx
│   │   ├── GpuPerformanceWidget.tsx
│   │   ├── IdeaToAppFlowchartView.tsx
│   │   ├── ImageGeneratorTab.tsx
│   │   ├── IntegrationsWindow.tsx
│   │   ├── InterfacesView.tsx
│   │   ├── JsExampleRunner.tsx
│   │   ├── LazyMetricsCharts.tsx
│   │   ├── LegalView.tsx
│   │   ├── LoginOverlay.tsx
│   │   ├── MainnetLaunchView.tsx
│   │   ├── MarketplaceView.tsx
│   │   ├── MediaApps.tsx
│   │   ├── MetricsView.tsx
│   │   ├── ModulesPluginView.tsx
│   │   ├── NetworkExplorerView.test.tsx
│   │   ├── NetworkExplorerView.tsx
│   │   ├── NetworkTopologyView.tsx
│   │   ├── NodeHealthMonitor.tsx
│   │   ├── NotepadView.tsx
│   │   ├── OfficeApps.tsx
│   │   ├── P2PChatView.tsx
│   │   ├── Paint3DView.tsx
│   │   ├── PaymentSystemView.tsx
│   │   ├── PipelineGeneratorTab.tsx
│   │   ├── PoAITrainingEngineView.tsx
│   │   ├── ProjectHubView.tsx
│   │   ├── ProtocolsView.tsx
│   │   ├── ReportsView.tsx
│   │   ├── RepositoryActivityChart.tsx
│   │   ├── RepositoryLineChart.tsx
│   │   ├── RescueSystemView.tsx
│   │   ├── RoadmapView.tsx
│   │   ├── SemanticGraphView.tsx
│   │   ├── SessionExportView.tsx
│   │   ├── SettingsView.tsx
│   │   ├── SocialMediaView.tsx
│   │   ├── SoftwareAuditView.tsx
│   │   ├── SoftwareKnowledgeDbView.tsx
│   │   ├── SourceCodeViewer.tsx
│   │   ├── SpecificSettingsViews.tsx
│   │   ├── StorageManagerView.tsx
│   │   ├── StrategicArchitectureMap.tsx
│   │   ├── StructureView.tsx
│   │   ├── SyncHistoryModal.tsx
│   │   ├── SyncMetricsView.tsx
│   │   ├── SyncStatusDonutChart.tsx
│   │   ├── SyncStatusOverview.tsx
│   │   ├── SystemDiagnosticsView.tsx
│   │   ├── SystemFinderView.tsx
│   │   ├── SystemLogsView.tsx
│   │   ├── TaskManagerView.tsx
│   │   ├── TechDocsView.tsx
│   │   ├── TechTreeView.tsx
│   │   ├── TestnetOrchestrationView.tsx
│   │   ├── TestnetSimulationView.tsx
│   │   ├── TextGeneratorTab.tsx
│   │   ├── ThemeSwitcher.tsx
│   │   ├── TodoView.tsx
│   │   ├── TooltipIcon.tsx
│   │   ├── TxOrchestratorView.tsx
│   │   ├── UserProfileView.tsx
│   │   ├── VideoGeneratorTab.tsx
│   │   ├── WebhookMonitor.tsx
│   │   ├── Window.tsx
│   │   ├── WindowExtras.tsx
│   │   ├── ZeroKnowledgeProofView.tsx
│   │   └── ZkVisualizationView.tsx
│   ├── contexts/
│   │   ├── FirebaseContext.tsx
│   │   ├── GoogleWorkspaceContext.tsx
│   │   ├── SyncMetricsContext.tsx
│   │   └── WalletContext.tsx
│   ├── data.ts
│   ├── db/
│   │   ├── drizzle.config.ts
│   │   ├── index.ts
│   │   └── schema.ts
│   ├── ecosystemData.ts
│   ├── fix_translation.cjs
│   ├── hooks/
│   │   ├── useGoogleSheetsSync.ts
│   │   └── useKeyboardShortcut.ts
│   ├── index.css
│   ├── lib/
│   │   ├── CryptoEngine.ts
│   │   ├── firebase-admin.ts
│   │   ├── firebase.ts
│   │   ├── indexedDb.ts
│   │   ├── syncLogic.test.ts
│   │   └── syncLogic.ts
│   ├── main.tsx
│   ├── marketplaceApps.ts
│   ├── middleware/
│   │   └── auth.ts
│   ├── requirementsData.ts
│   ├── roadmapData.ts
│   ├── routes/
│   │   └── notion.ts
│   ├── services/
│   │   ├── SyncService.ts
│   │   └── githubSync.ts
│   ├── standardsData.ts
│   ├── tierData.ts
│   ├── types.ts
│   ├── utils/
│   │   ├── appSync.tsx
│   │   ├── auditUtils.test.ts
│   │   ├── auditUtils.ts
│   │   └── crypto.ts
│   └── wikiData.ts
├── streaming.atc
├── telemetry.atc
├── testChat.js
├── test_know.js
├── tests/
│   ├── GitHubRepoSyncView.test.tsx
│   └── audit_compliance.test.ts
├── tmp.txt
├── tsconfig.json
├── universe_factory_ad44.atc
├── update_wiki_categories.ts
├── versioning.atc
├── vite.config.ts
└── workspace/
    ├── move.js
    ├── rename.js
    ├── replace.js
    ├── replaceEnterprise.js
    ├── replaceGoals.ts
    ├── replaceGoals2.ts
    └── src/
        └── components/
            └── GovernanceView.tsx
```

## Module Descriptions
- **src/**: TypeScript application source containing AI model visual editor components, canvas workflows, node orchestration, and UI views.
- **assets/**: 3D model assets, textures, animations, and sound effects for studio simulations.
- **tests/**: Automated unit and integration test suites for AI workflow evaluation and pipeline validation.
- **package.json** & **tsconfig.json**: Node package configuration, dependency specifications, and TypeScript strict compiler directives.
- **.atc workflow modules**: Specialized AI engine assets (`civilization_engine`, `global_simulation`, `federated_learning`, `knowledge_graph`).

## Build System
Node.js / npm environment with Vite build tool and TypeScript compiler (`tsc`). Supports Hot Module Replacement (HMR) and automated TS build scripts.

## Dependencies
TypeScript 5.0+, React / Canvas UI frameworks, Three.js / WebGL render engines, Vitest / Jest, WebSocket / REST client libraries.
