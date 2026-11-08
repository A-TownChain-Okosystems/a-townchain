import React, { useState } from 'react';
import { 
  Monitor, Layers, Globe, Box, Workflow, ChevronRight, ChevronDown, 
  Settings, Network, Lock, Cpu, Globe2, LayoutTemplate, Share2, 
  ShieldCheck, ArrowRight, Zap, PlayCircle, Smartphone, Computer, Webhook 
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface TreeNode {
  id: string;
  name: string;
  icon?: React.ReactNode;
  children?: TreeNode[];
}

const ECOSYSTEM_TREE: TreeNode[] = [
  {
    id: "globus-os",
    name: "GlobusOS",
    icon: <Monitor className="w-5 h-5 text-blue-400" />,
    children: [
      {
        id: "kernfunktionen",
        name: "Kernfunktionen",
        children: [
          {
            id: "multi-device",
            name: "Multi-Device OS",
            icon: <Smartphone className="w-4 h-4 text-slate-400" />,
            children: [
              { id: "md-1", name: "Touchscreen Support" },
              { id: "md-2", name: "Maus & Tastatur Input" },
              { id: "md-3", name: "Controller Support" },
              { id: "md-4", name: "Remote-Steuerung (Smartphone ↔ PC)" }
            ]
          },
          {
            id: "user-management",
            name: "User Management",
            icon: <Settings className="w-4 h-4 text-slate-400" />,
            children: [
              { id: "um-1", name: "Nutzerprofil & Einstellungen" },
              { id: "um-2", name: "Wallet-Integration (A-TownChain)" },
              { id: "um-3", name: "Lokale + Cloud-Daten-Synchronisierung" }
            ]
          },
          {
            id: "security-privacy",
            name: "Security & Privacy",
            icon: <Lock className="w-4 h-4 text-slate-400" />,
            children: [
              { id: "sp-1", name: "End-to-End Verschlüsselung" },
              { id: "sp-2", name: "Dezentralisierte Datenhaltung" },
              { id: "sp-3", name: "Light Node im User Wallet" }
            ]
          },
          {
            id: "asset-management",
            name: "Asset Management",
            icon: <Box className="w-4 h-4 text-slate-400" />,
            children: [
              { id: "am-1", name: "Standard Templates DB für KI-Generierung" },
              { id: "am-2", name: "Asset-Maß Berechnung" },
              { id: "am-3", name: "Vergleich mit Template Datenbanken" },
              { id: "am-4", name: "Lokale + Online Template Integration" }
            ]
          },
          {
            id: "ki-integration",
            name: "KI Integration",
            icon: <Cpu className="w-4 h-4 text-slate-400" />,
            children: [
              { id: "ki-1", name: "Generative KI für Software & Assets" },
              { id: "ki-2", name: "Automatisches Routing & Task-Delegation" },
              { id: "ki-3", name: "Multi-Modal Input (Text, Audio, Bild)" }
            ]
          }
        ]
      }
    ]
  },
  {
    id: "a-town-chain",
    name: "A-Town Blockchain-Technologie",
    icon: <Network className="w-5 h-5 text-purple-400" />,
    children: [
      {
        id: "layer-architektur",
        name: "Layer-Architektur",
        children: [
          {
            id: "layer-0",
            name: "Layer 0: Netzwerk",
            children: [
              { id: "l0-1", name: "Peer-to-Peer Mesh" },
              { id: "l0-2", name: "Light Nodes in User Wallets" },
              { id: "l0-3", name: "Validator Rotation" },
              { id: "l0-4", name: "Dezentrale Task-Verteilung" }
            ]
          },
          {
            id: "layer-1",
            name: "Layer 1: Konsensus",
            children: [
              { id: "l1-1", name: "Proof-of-Work + Proof-of-Stake Hybrid" },
              { id: "l1-2", name: "Automatisches Pool-Matching" },
              { id: "l1-3", name: "Team-Mining + Leaderboards" }
            ]
          },
          {
            id: "layer-2",
            name: "Layer 2: Smart Contracts / dApps",
            children: [
              { id: "l2-1", name: "Wallet-Integration" },
              { id: "l2-2", name: "NFT Marketplace" },
              { id: "l2-3", name: "Referral / Reward-System" }
            ]
          },
          {
            id: "layer-3",
            name: "Layer 3: Orchestrator & KI",
            children: [
              { id: "l3-1", name: "Task-Routing nach Spezialität" },
              { id: "l3-2", name: "Token- & Limit-Management" },
              { id: "l3-3", name: "Cross-KI-Kommunikation für Software/Asset-Generierung" }
            ]
          }
        ]
      }
    ]
  },
  {
    id: "gate-to-hell",
    name: "GateToHell Browser",
    icon: <Globe2 className="w-5 h-5 text-red-400" />,
    children: [
      {
        id: "gth-funktionen",
        name: "Funktionen",
        children: [
          { id: "gth-1", name: "Web3-kompatibel (A-TownChain Wallet)" },
          {
            id: "gth-2",
            name: "Lesezeichen + Bookmark Sync",
            children: [
              { id: "gth-2-1", name: "Gespeichert in Nutzer Wallet" },
              { id: "gth-2-2", name: "Zugriff via GlobusOS & A-Town Browser" },
              { id: "gth-2-3", name: "Synchronisation nur bei Online-Hardware" }
            ]
          },
          { id: "gth-3", name: "KI-gestützte Suche & Auto-Fill" },
          {
            id: "gth-4",
            name: "Dezentralisierte Datenströme",
            children: [
              { id: "gth-4-1", name: "Routing über Light Nodes" },
              { id: "gth-4-2", name: "P2P Datentransfer" }
            ]
          },
          {
            id: "gth-5",
            name: "Security Features",
            children: [
              { id: "gth-5-1", name: "Anti-Tracking" },
              { id: "gth-5-2", name: "Verschlüsselte Verbindung" },
              { id: "gth-5-3", name: "Full Node Support für fortgeschrittene Nutzer" }
            ]
          }
        ]
      }
    ]
  },
  {
    id: "franchise-factory",
    name: "Franchise Factury KI",
    icon: <LayoutTemplate className="w-5 h-5 text-green-400" />,
    children: [
      {
        id: "ff-funktionen",
        name: "Funktionen",
        children: [
          {
            id: "ff-1",
            name: "Asset-Generierung",
            children: [
              { id: "ff-1-1", name: "Zugriff auf Standard Template DB" },
              { id: "ff-1-2", name: "Online Asset Templates durchsuchen" },
              { id: "ff-1-3", name: "Automatisches Herunterladen / Import" }
            ]
          },
          {
            id: "ff-2",
            name: "Software-Erstellung",
            children: [
              { id: "ff-2-1", name: "Text → Struktur → Code" },
              { id: "ff-2-2", name: "Multi-Plattform (EXE, APK, Web)" },
              { id: "ff-2-3", name: "Build-Skripte + Deployment-Vorschläge" }
            ]
          },
          {
            id: "ff-3",
            name: "Automatisiertes Routing",
            children: [
              { id: "ff-3-1", name: "Spezialisiertes KI-Modul wählen" },
              { id: "ff-3-2", name: "Token-Limits überwachen" },
              { id: "ff-3-3", name: "Workflow-Optimierung" }
            ]
          },
          {
            id: "ff-4",
            name: "Lokale + Online Integration",
            children: [
              { id: "ff-4-1", name: "Lokale DB zuerst prüfen" },
              { id: "ff-4-2", name: "Online Templates falls nicht vorhanden" },
              { id: "ff-4-3", name: "Vollständige Offline-/Online-Kombination" }
            ]
          },
          {
            id: "ki-workflow",
            name: "Idee zu App (Kostenloser KI-gestützter Software-Workflow)",
            children: [
              {
                id: "kw-1",
                name: "Schritt 1: Idee definieren",
                children: [
                  { id: "kw-1-1", name: "Beschreibung der Funktionen in natürlicher Sprache" }
                ]
              },
              {
                id: "kw-2",
                name: "Schritt 2: Planung & Struktur",
                children: [
                  { id: "kw-2-1", name: "KI (Replit AI / StarCoder) erstellt Ordnerstruktur, Dateien, Klassen" }
                ]
              },
              {
                id: "kw-3",
                name: "Schritt 3: Code-Generierung",
                children: [
                  { id: "kw-3-1", name: "Features einzeln erstellen lassen" },
                  { id: "kw-3-2", name: "Direkt online testen & debuggen" }
                ]
              },
              {
                id: "kw-4",
                name: "Schritt 4: UI Gestaltung",
                children: [
                  { id: "kw-4-1", name: "Widgets, Buttons, Menüs" },
                  { id: "kw-4-2", name: "Theme & Interaktive Elemente" }
                ]
              },
              {
                id: "kw-5",
                name: "Schritt 5: Build / fertige Software",
                children: [
                  { id: "kw-5-1", name: "EXE → PyInstaller / Python" },
                  { id: "kw-5-2", name: "APK → Flutter Build" },
                  { id: "kw-5-3", name: "Web-App → Replit / GitHub Pages" }
                ]
              },
              {
                id: "kw-6",
                name: "Schritt 6: Testen & Feedback",
                children: [
                  { id: "kw-6-1", name: "KI unterstützt Debug & Optimierung" }
                ]
              },
              {
                id: "kw-7",
                name: "Schritt 7: Deployment & Sharing",
                children: [
                  { id: "kw-7-1", name: "Web-Apps → Host kostenlos online" },
                  { id: "kw-7-2", name: "EXE/APK → Download / Cloud teilen" },
                  { id: "kw-7-3", name: "Iterative Updates über KI-Workflow" }
                ]
              }
            ]
          }
        ]
      }
    ]
  }
];

const TreeNodeItem = ({ node, level = 0 }: { node: TreeNode, level?: number }) => {
  const [isOpen, setIsOpen] = useState(true);
  
  const hasChildren = node.children && node.children.length > 0;
  
  return (
    <div className="w-full">
      <div 
        className={`flex items-center py-2 px-3 hover:bg-slate-800/50 rounded-lg cursor-pointer transition-colors ${level === 0 ? 'bg-slate-800/30 border border-slate-700/50 mb-2' : ''}`}
        style={{ paddingLeft: `${level * 20 + 12}px` }}
        onClick={() => hasChildren && setIsOpen(!isOpen)}
      >
        <div className="flex-1 flex items-center gap-3">
          {hasChildren ? (
            isOpen ? <ChevronDown className="w-4 h-4 text-slate-400" /> : <ChevronRight className="w-4 h-4 text-slate-400" />
          ) : (
            <div className="w-4 h-4 flex items-center justify-center">
              <div className="w-1.5 h-1.5 rounded-full bg-slate-500" />
            </div>
          )}
          
          {node.icon && <div>{node.icon}</div>}
          
          <span className={`${level === 0 ? 'text-white font-bold' : level === 1 ? 'text-slate-200 font-semibold' : 'text-slate-400 text-sm'}`}>
            {node.name}
          </span>
        </div>
      </div>
      
      <AnimatePresence>
        {isOpen && hasChildren && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: "auto", opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="overflow-hidden relative"
          >
            {/* Tree Branch Line */}
            <div 
              className="absolute top-0 bottom-0 border-l border-slate-700/30"
              style={{ left: `${level * 20 + 19}px` }}
            />
            {node.children!.map((child) => (
              <TreeNodeItem key={child.id} node={child} level={level + 1} />
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

export const EcosystemTreeOverlay: React.FC = () => {
  return (
    <div className="w-full h-full bg-slate-900 border border-slate-800 rounded-lg p-6 overflow-y-auto">
      <div className="flex items-center gap-3 mb-6">
        <Globe className="w-6 h-6 text-atc-cyan" />
        <h2 className="text-xl font-mono text-white tracking-widest uppercase">GlobusOS & A-Town Ökosystem</h2>
      </div>

      <div className="mb-6 text-slate-400 text-sm leading-relaxed max-w-3xl">
        Das gesamte Ökosystem von GlobusOS und der A-Town Blockchain-Technologie, von der untersten Multi-Device Unterstützung
        bis zum High-Level Software-Workflow für Endnutzer.
      </div>

      <div className="flex flex-col gap-1">
        {ECOSYSTEM_TREE.map(node => (
          <TreeNodeItem key={node.id} node={node} />
        ))}
      </div>
    </div>
  );
};
