import React from 'react';
import { PlaySquare, Save, Eye, Shield, Key, Search, FileText, MonitorPlay, Zap, Server, Activity, ArrowLeftRight, Clock, Network, Cpu, Lock, CheckCircle, Database } from 'lucide-react';

export const PlaceholderView = ({ title, icon: Icon, desc }: { title: string, icon: any, desc: string }) => (
  <div className="flex flex-col h-full bg-[#050B14] p-8 text-white">
    <div className="flex items-center gap-4 mb-8">
      <div className="p-3 bg-indigo-500/20 text-indigo-400 rounded-lg">
        <Icon className="w-8 h-8" />
      </div>
      <div>
        <h1 className="text-2xl font-bold font-mono uppercase tracking-wider">{title}</h1>
        <p className="text-slate-400 text-sm">{desc}</p>
      </div>
    </div>
    <div className="flex-1 border border-white/10 rounded-xl bg-[#020408] flex items-center justify-center p-8">
      <div className="text-center max-w-md">
        <Icon className="w-16 h-16 text-slate-700 mx-auto mb-4" />
        <h2 className="text-lg font-bold text-slate-300 mb-2">System Initializing</h2>
        <p className="text-slate-500 text-sm">
          The {title} module is currently being provisioned by the ATS-Kernel.
          All required dependencies and cryptographic state proofs are being synced.
        </p>
      </div>
    </div>
  </div>
);

export const OfficeCanvasUI = () => <PlaceholderView title="ATS-OFFICE" desc="Decentralized Word, Excel, Powerpoint Workspace" icon={FileText} />
export const PlaygroundUI = () => <PlaceholderView title="ATS-PLAY" desc="Gaming & Immersion Engine" icon={PlaySquare} />
export const GenesisCockpitUI = () => <PlaceholderView title="Genesis Cockpit" desc="ATC Genesis Block Generation & Network Ignition" icon={Zap} />
export const DAOGovernanceUI = () => <PlaceholderView title="DAO Governance" desc="ATS Meta-Governance Protocol Interface" icon={Network} />
export const FinalIntegratorUI = () => <PlaceholderView title="Final Integrator" desc="System Check & Final Merge Tool" icon={CheckCircle} />
export const ArchiveGuardianUI = () => <PlaceholderView title="Archive Guardian" desc="Holographic Storage Permanence" icon={Save} />
export const HashManifestUI = () => <PlaceholderView title="Hash Manifest" desc="DNA Data Compression & Serialization" icon={Database} />
export const SingularityMonitorUI = () => <PlaceholderView title="Singularity Monitor" desc="A-TownChain Resonance & Feedback Loop" icon={Activity} />
export const AnonymityShieldUI = () => <PlaceholderView title="Anonymity Shield" desc="ZKP Pseudonym Management" icon={Lock} />
export const TransparencyMonitorUI = () => <PlaceholderView title="Transparency Monitor" desc="Proof-of-Logic Verification" icon={Eye} />
export const PrivacyGatekeeperUI = () => <PlaceholderView title="Privacy Gatekeeper" desc="PII Deletion & Clean Data" icon={Shield} />
export const LEXAIMonitorUI = () => <PlaceholderView title="LEX-AI Monitor" desc="AI Constitution Compliance" icon={FileText} />
export const TruthGateUI = () => <PlaceholderView title="Truth Gate" desc="Fact Verification & ATS-TRUTH-LOCK" icon={CheckCircle} />
export const ATCLangTerminalUI = () => <PlaceholderView title="ATC-LANG Terminal" desc="Intent Syntax Execution CLI" icon={MonitorPlay} />
export const ProtocolIntegratorUI = () => <PlaceholderView title="Protocol Integrator" desc="ATC Layer 1 / ATS Layer 2 Bridge" icon={ArrowLeftRight} />
export const SurvivalControlUI = () => <PlaceholderView title="Survival Control" desc="ATS-LITHOS Hibernation & Resonance" icon={Server} />
export const EncyclopediaNexusUI = () => <PlaceholderView title="Encyclopedia Nexus" desc="ATS Living Codex" icon={Search} />
export const EvolutionNexusUI = () => <PlaceholderView title="Evolution Nexus" desc="System Evolution & Genetic Versioning" icon={Network} />
export const EvolutionJournalUI = () => <PlaceholderView title="Evolution Journal" desc="Self-Optimization Changelog" icon={FileText} />
export const MultiPassMonitorUI = () => <PlaceholderView title="Multi-Pass Monitor" desc="Triple Check Verification Engine" icon={CheckCircle} />
export const RollbackManagerUI = () => <PlaceholderView title="Rollback Manager" desc="0-Second Recovery & Snapshotting" icon={Clock} />
export const AutoOptimizerUI = () => <PlaceholderView title="Auto-Optimizer" desc="Algorithmic Refactoring in Real-time" icon={Cpu} />
export const GovernanceSentinelUI = () => <PlaceholderView title="Governance Sentinel" desc="ATC-006 CEL & ATS-HUMAN-PRIMACY Override" icon={Shield} />
export const CivilizationCommandUI = () => <PlaceholderView title="Civilization Command" desc="Autonomous Civilization Engine (ACE)" icon={Network} />
