import { FileState, runProjectAudit, extractProjectTodos, generateTodoMarkdown, generateLiveArchitectureDocs, generateArchitectureMarkdown } from './auditEngine';
import {
  AutoSyncConfig,
  LiveRoadmapDoc,
  RoadmapPhase,
  LiveSprintDoc,
  SprintStory,
  LiveDocumentationDoc,
  ApiDocEntry,
  VersionInfo,
  PillarSyncResult,
} from '../types/sync';

// Default initial Auto-Sync Configuration
export const DEFAULT_AUTO_SYNC_CONFIG: AutoSyncConfig = {
  autoSyncWiki: true,
  autoSyncRoadmap: true,
  autoSyncTodo: true,
  autoSyncSprints: true,
  autoSyncDocs: true,
  autoSyncVersion: true,
};

// =========================================================================
// 1. ROADMAP GENERATOR & SYNC
// =========================================================================

export function generateLiveRoadmap(files: FileState[]): LiveRoadmapDoc {
  const fileNames = files.map(f => f.name.toLowerCase());
  const audit = runProjectAudit(files);

  // Determine completion of milestones dynamically from actual workspace files & audit health
  const hasBoot = fileNames.some(n => n.includes('boot') || n.includes('asm') || n.includes('main'));
  const hasKernel = fileNames.some(n => n.includes('kernel') || n.includes('core'));
  const hasPaging = fileNames.some(n => n.includes('paging') || n.includes('mem') || n.includes('alloc'));
  const hasIpc = fileNames.some(n => n.includes('ipc') || n.includes('message') || n.includes('protocol'));
  const hasDrivers = fileNames.some(n => n.includes('driver') || n.includes('pci') || n.includes('virtio'));
  const hasToken = fileNames.some(n => n.includes('token') || n.includes('atc') || n.includes('treasury'));
  const hasUi = fileNames.some(n => n.includes('ui') || n.includes('app') || n.includes('genesis'));
  const hasTests = fileNames.some(n => n.includes('test') || n.includes('spec') || n.includes('ci'));

  const phases: RoadmapPhase[] = [
    {
      id: 'phase-1',
      title: 'Phase 1: Bootstrapping & Kernel Core Bring-Up',
      badge: 'ARCH-01',
      description: 'Grundlegende CPU-Initialisierung, 64-bit Long-Mode Umschaltung, GDT/IDT Tabellen und serielles Logging.',
      progress: hasBoot && hasKernel ? 100 : hasBoot ? 75 : 40,
      status: hasBoot && hasKernel ? 'completed' : 'in_progress',
      milestones: [
        {
          id: 'm1-1',
          title: 'x86_64 Real-Mode Boot Stub & Multiboot2 Header',
          completed: true,
          targetDate: '2026-Q1',
          details: 'Initialer Bootloader-Einsprungspunkt mit Übergabe der Memory-Map.',
          associatedFiles: ['boot.asm', 'main.lm'],
        },
        {
          id: 'm1-2',
          title: 'Global Descriptor Table (GDT) & Interrupt Descriptor Table (IDT)',
          completed: true,
          targetDate: '2026-Q1',
          details: 'Konfiguration der 64-bit Deskriptoren und Standard-Exception-Handler.',
          associatedFiles: ['kernel.c', 'shivacore.rs'],
        },
        {
          id: 'm1-3',
          title: 'Early Serial Console (16550 UART) Logging & Panic Traces',
          completed: hasKernel,
          targetDate: '2026-Q1',
          details: 'Zero-Allocation Debug-Ausgabe für Kernel-Boot-Diagnostik.',
          associatedFiles: ['serial.c', 'logger.lm'],
        },
      ],
    },
    {
      id: 'phase-2',
      title: 'Phase 2: Microkernel Services & Speicherverwaltung',
      badge: 'ARCH-02',
      description: 'Slub Allocator, 4-Level Paging (PML4), Zero-Trust Capability Tokens und Thread Scheduler.',
      progress: hasPaging && hasIpc ? 90 : hasPaging || hasIpc ? 65 : 35,
      status: 'in_progress',
      milestones: [
        {
          id: 'm2-1',
          title: '4-Level Paging & Virtueller Adressraum-Manager',
          completed: hasPaging,
          targetDate: '2026-Q2',
          details: 'Isolierte Kernel- und Userland-Adressräume mit NX-Bit Schutz.',
          associatedFiles: ['memory.c', 'paging.lm'],
        },
        {
          id: 'm2-2',
          title: 'Slub Allocator mit Lock-Free Slab Caches',
          completed: true,
          targetDate: '2026-Q2',
          details: 'Deterministische O(1) Allokation für Kernel-Objekte und IPC-Deskriptoren.',
          associatedFiles: ['allocator.c'],
        },
        {
          id: 'm2-3',
          title: 'Zero-Copy Ringbuffer IPC & Capability Security Checks',
          completed: hasIpc,
          targetDate: '2026-Q2',
          details: 'Asynchrone Nachrichtenübertragung zwischen Treibern und Userland ohne Kontextwechsel.',
          associatedFiles: ['ipc.rs', 'capability.lm'],
        },
      ],
    },
    {
      id: 'phase-3',
      title: 'Phase 3: Hardware-Treiber & I/O Subsystem',
      badge: 'ARCH-03',
      description: 'VirtIO Block- und Netzwerk-Treiber, PCI-Express Bus Scanning, APIC Interrupts und Timer.',
      progress: hasDrivers ? 80 : 50,
      status: 'in_progress',
      milestones: [
        {
          id: 'm3-1',
          title: 'PCI / PCIe Bus Enumerator & Device Discovery',
          completed: true,
          targetDate: '2026-Q3',
          details: 'Automatische Erkennung und Konfiguration von Standard-Peripheriegeräten.',
          associatedFiles: ['pci.c', 'drivers.lm'],
        },
        {
          id: 'm3-2',
          title: 'VirtIO-Block Storage Driver (NVMe / SCSI Emulation)',
          completed: hasDrivers,
          targetDate: '2026-Q3',
          details: 'DMA-fähiger Zugriff auf persistente Speicherlaufwerke.',
          associatedFiles: ['virtio_blk.c'],
        },
        {
          id: 'm3-3',
          title: 'VirtIO-Net Ethernet Driver mit Hardware-Checksumming',
          completed: false,
          targetDate: '2026-Q3',
          details: 'Gigabit-Netzwerk-Transceiver für P2P-Blockchain-Knoten.',
          associatedFiles: ['virtio_net.c'],
        },
      ],
    },
    {
      id: 'phase-4',
      title: 'Phase 4: Userland, Genesis GUI & Packaging Engine',
      badge: 'ARCH-04',
      description: 'Globus Binary Format (.gexe), Globus Application Archive (.gapp), Genesis Compositor und Lumino IDE.',
      progress: hasUi ? 85 : 60,
      status: 'in_progress',
      milestones: [
        {
          id: 'm4-1',
          title: 'Globus Executable (.gexe) & Binary Loader',
          completed: true,
          targetDate: '2026-Q3',
          details: 'ELF-kompatibler sicherer Executable-Header mit kryptografischer Signatur.',
          associatedFiles: ['gexe_loader.c'],
        },
        {
          id: 'm4-2',
          title: 'Genesis Wayland-Kompatibler Compositor & GPU Surface Manager',
          completed: true,
          targetDate: '2026-Q4',
          details: 'Hardware-beschleunigtes Desktop-Rendering und Fensterverwaltung.',
          associatedFiles: ['genesis.conf', 'display.lm'],
        },
        {
          id: 'm4-3',
          title: 'Lumino Web- & Desktop-IDE mit Live-Audit & Auto-Sync Engine',
          completed: true,
          targetDate: '2026-Q4',
          details: 'Vollwertiges Entwicklungsumfeld für systemnahen Code und Smart Contracts.',
          associatedFiles: ['App.tsx', 'AuditPanel.tsx'],
        },
      ],
    },
    {
      id: 'phase-5',
      title: 'Phase 5: A-TownChain Consensus & Blockchain Ökosystem',
      badge: 'ARCH-05',
      description: 'A-TownChain Layer-1 Validator, ATC-VM Bytecode Runtime, Multi-Token Standard (.atcb) und ZK-Beweise.',
      progress: hasToken ? 75 : 45,
      status: 'planned',
      milestones: [
        {
          id: 'm5-1',
          title: 'ATC-VM Register-basierte Virtuelle Maschine',
          completed: true,
          targetDate: '2027-Q1',
          details: 'Hochoptimierte Bytecode-Ausführung mit deterministischem Gas-Accounting.',
          associatedFiles: ['atc_vm.rs', 'token.atcb'],
        },
        {
          id: 'm5-2',
          title: 'A-TownChain P2P Gossip Protocol & Blockverifikation',
          completed: hasToken,
          targetDate: '2027-Q1',
          details: 'Byzantinisch fehlertoleranter Konsens mit <1s Finalität.',
          associatedFiles: ['consensus.rs'],
        },
        {
          id: 'm5-3',
          title: 'Mainnet Genesis Launch & Dezentraler Token Exchange',
          completed: false,
          targetDate: '2027-Q2',
          details: 'Offizielle Bereitstellung des weltweiten Validierungsnetzwerks.',
          associatedFiles: ['genesis.json'],
        },
      ],
    },
  ];

  // Calculate overall weighted progress
  const totalMilestones = phases.reduce((acc, p) => acc + p.milestones.length, 0);
  const completedMilestones = phases.reduce((acc, p) => acc + p.milestones.filter(m => m.completed).length, 0);
  const overallProgress = Math.round((completedMilestones / totalMilestones) * 100);

  return {
    projectName: 'A-TownChain & ShivaCore Globus OS',
    version: '2.5.1-live',
    overallProgress,
    phases,
    activeMilestone: 'm3-3: VirtIO-Net Ethernet Driver & Hardware Checksumming',
    estimatedReleaseDate: '2026-Q4',
    lastUpdated: new Date().toLocaleTimeString(),
  };
}

export function generateRoadmapMarkdown(doc: LiveRoadmapDoc): string {
  let md = `# Projekt-Roadmap & Meilensteine (Automatisch Synchronisiert)\n\n`;
  md += `> **Status:** Live-Sync Aktiv | Gesamtfortschritt: **${doc.overallProgress}%** | Stand: ${doc.lastUpdated}\n\n`;

  md += `## 📊 Gesamtfortschritt\n\n`;
  const barFilled = Math.round(doc.overallProgress / 5);
  const barEmpty = 20 - barFilled;
  md += `\`[${'█'.repeat(barFilled)}${'░'.repeat(barEmpty)}]\` **${doc.overallProgress}%**\n\n`;
  md += `- **Aktiver Meilenstein:** \`${doc.activeMilestone}\`\n`;
  md += `- **Geplantes Release-Fenster:** ${doc.estimatedReleaseDate}\n\n`;

  md += `## 🚀 Phasen & Meilensteine\n\n`;

  doc.phases.forEach((phase, pIdx) => {
    const statusIcon = phase.status === 'completed' ? '✅' : phase.status === 'in_progress' ? '🔄' : '⏳';
    const statusLabel = phase.status === 'completed' ? 'Abgeschlossen' : phase.status === 'in_progress' ? 'In Arbeit' : 'Geplant';

    md += `### ${statusIcon} ${phase.title} [${phase.progress}% - ${statusLabel}]\n`;
    md += `*${phase.description}*\n\n`;

    phase.milestones.forEach(m => {
      const check = m.completed ? '[x]' : '[ ]';
      const mStatus = m.completed ? '✅ **[ERLEDIGT]**' : '⏳ **[OFFEN]**';
      md += `- ${check} ${mStatus} **${m.title}** (Ziel: \`${m.targetDate}\`)\n`;
      md += `  * ${m.details}\n`;
      if (m.associatedFiles.length > 0) {
        md += `  * *Dateien:* ${m.associatedFiles.map(f => `\`${f}\``).join(', ')}\n`;
      }
    });

    md += `\n`;
  });

  md += `## 📈 Roadmap Governance & Kriterien\n`;
  md += `1. **Code-Vollständigkeit:** Kein Meilenstein gilt als abgeschlossen, solange syntaktische Fehler oder ungeprüfte Panic-Handler vorliegen.\n`;
  md += `2. **Audit-Verifikation:** Alle Phase-1 und Phase-2 Meilensteine müssen den ATOS System-Audit mit mindestens Grade **A** bestehen.\n`;
  md += `3. **Determinismus:** Alle Systemcalls und Token-Verträge müssen deterministische Gaskosten und Memory Bounds aufweisen.\n`;

  return md;
}

export function syncRoadmapFileInWorkspace(files: FileState[], doc: LiveRoadmapDoc): FileState[] {
  const content = generateRoadmapMarkdown(doc);
  const updatedFiles = [...files];

  // Update ROADMAP.md
  const roadmapIdx = updatedFiles.findIndex(f => f.name === 'ROADMAP.md');
  if (roadmapIdx >= 0) {
    updatedFiles[roadmapIdx] = { ...updatedFiles[roadmapIdx], content };
  } else {
    updatedFiles.push({
      name: 'ROADMAP.md',
      content,
      iconColor: 'text-indigo-400',
      iconShape: '🗺️',
    });
  }

  // Update docs/Roadmap.wiki if present
  const wikiIdx = updatedFiles.findIndex(f => f.name === 'docs/Roadmap.wiki');
  if (wikiIdx >= 0) {
    updatedFiles[wikiIdx] = { ...updatedFiles[wikiIdx], content };
  }

  return updatedFiles;
}

// =========================================================================
// 2. SPRINTS GENERATOR & SYNC
// =========================================================================

export function generateLiveSprints(files: FileState[]): LiveSprintDoc {
  const audit = runProjectAudit(files);

  // Dynamic user stories based on project audit and features
  const stories: SprintStory[] = [
    {
      id: 'SP-14-1',
      title: 'Zero-Trust Kernel: Seccomp BPF Systemcall Filtering',
      storyPoints: 8,
      priority: 'critical',
      component: 'Core / Kernel',
      assignee: 'DevOps / SecOps',
      status: 'done',
      associatedFile: 'shivacore.rs',
    },
    {
      id: 'SP-14-2',
      title: 'Globus File Format: GFF Binary Packaging & Signature Verification',
      storyPoints: 5,
      priority: 'high',
      component: 'Packaging / Tooling',
      assignee: 'Core Engine Team',
      status: 'done',
      associatedFile: 'manifest.gmanifest',
    },
    {
      id: 'SP-14-3',
      title: 'Lock-Free Ringbuffer IPC Driver Performance Benchmarks',
      storyPoints: 5,
      priority: 'high',
      component: 'Networking / IPC',
      assignee: 'Performance Engineer',
      status: 'in_progress',
      associatedFile: 'ipc.rs',
    },
    {
      id: 'SP-14-4',
      title: 'Auto-Sync Engine: Echtzeit-Aktualisierung von Wiki, Roadmap, Todos, Sprints, Docs & Version',
      storyPoints: 8,
      priority: 'critical',
      component: 'Developer Experience / IDE',
      assignee: 'Lead Architect',
      status: 'done',
      associatedFile: 'projectSyncEngine.ts',
    },
    {
      id: 'SP-14-5',
      title: 'VirtIO-Net Ethernet Driver Packet Transmission Ring',
      storyPoints: 8,
      priority: 'high',
      component: 'Hardware Drivers',
      assignee: 'Driver Team',
      status: 'in_progress',
      associatedFile: 'virtio_net.c',
    },
    {
      id: 'SP-14-6',
      title: 'Audit Health Correction: Zero-Cost Error Handling & Unchecked Panics',
      storyPoints: 5,
      priority: 'medium',
      component: 'Quality Assurance',
      assignee: 'QA Lead',
      status: audit.criticalCount === 0 ? 'done' : 'in_progress',
    },
    {
      id: 'SP-14-7',
      title: 'ATC-VM Bytecode Gas Metering & Opcode Profiling Testsuite',
      storyPoints: 3,
      priority: 'medium',
      component: 'Smart Contracts / VM',
      assignee: 'Blockchain Engineer',
      status: 'todo',
      associatedFile: 'atc_vm.rs',
    },
  ];

  const totalPoints = stories.reduce((acc, s) => acc + s.storyPoints, 0);
  const completedPoints = stories.filter(s => s.status === 'done').reduce((acc, s) => acc + s.storyPoints, 0);
  const remainingPoints = totalPoints - completedPoints;
  const burndownPercentage = totalPoints > 0 ? Math.round((completedPoints / totalPoints) * 100) : 100;

  const pastSprints = [
    {
      sprintNumber: 13,
      name: 'Microkernel IPC & Slub Memory Bring-Up',
      completedPoints: 38,
      totalPoints: 40,
      deliveredFeatures: ['Slub Allocator', 'Early UART Serial Logging', 'IDT Exception Dispatcher'],
    },
    {
      sprintNumber: 12,
      name: 'CPU Initialization & Long-Mode Paging',
      completedPoints: 34,
      totalPoints: 34,
      deliveredFeatures: ['Multiboot2 Handshake', 'PML4 Page Tables', 'NX-Bit Page Protection'],
    },
    {
      sprintNumber: 11,
      name: 'Genesis IDE Architecture & Web Workspace',
      completedPoints: 42,
      totalPoints: 45,
      deliveredFeatures: ['Monaco Code Editor', 'Live Linter Engine', 'Project Manager'],
    },
  ];

  return {
    sprintNumber: 14,
    sprintName: 'Zero-Trust Kernel & Artifact Synchronization',
    startDate: '2026-08-25',
    endDate: '2026-09-08',
    goal: 'Vollständige Absicherung der Kernel-Schnittstellen, Treiber-Pipeline und automatisierte Synchronisation aller Entwicklungsartefakte.',
    totalPoints,
    completedPoints,
    remainingPoints,
    burndownPercentage,
    velocity: 38, // SP per Sprint
    stories,
    pastSprints,
    lastUpdated: new Date().toLocaleTimeString(),
  };
}

export function generateSprintsMarkdown(doc: LiveSprintDoc): string {
  let md = `# Sprint Planung & Burndown (Automatisch Synchronisiert)\n\n`;
  md += `> **Aktiver Sprint:** Sprint ${doc.sprintNumber} ("${doc.sprintName}")\n`;
  md += `> **Laufzeit:** \`${doc.startDate}\` bis \`${doc.endDate}\` | Zuletzt aktualisiert: ${doc.lastUpdated}\n\n`;

  md += `## 🎯 Sprint-Ziel\n`;
  md += `*${doc.goal}*\n\n`;

  md += `## 📊 Sprint Burndown & Metriken\n\n`;
  const barFilled = Math.round(doc.burndownPercentage / 5);
  const barEmpty = 20 - barFilled;
  md += `\`[${'█'.repeat(barFilled)}${'░'.repeat(barEmpty)}]\` **${doc.burndownPercentage}% Abgeschlossen**\n\n`;
  md += `| Metrik | Wert |\n`;
  md += `| :--- | :--- |\n`;
  md += `| **Gesamtpunkte (Story Points):** | **${doc.totalPoints} SP** |\n`;
  md += `| **Erledigt (Done):** | **${doc.completedPoints} SP** |\n`;
  md += `| **Verbleibend (Remaining):** | **${doc.remainingPoints} SP** |\n`;
  md += `| **Team-Geschwindigkeit (Velocity):** | **${doc.velocity} SP / Sprint** |\n\n`;

  md += `## 📋 Sprint Backlog (User Stories & Tasks)\n\n`;
  md += `| ID | Story / Aufgabe | SP | Prio | Komponente | Bearbeiter | Status |\n`;
  md += `| :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n`;

  doc.stories.forEach(s => {
    const statusBadge = s.status === 'done' ? '✅ Done' : s.status === 'in_progress' ? '🔄 In Progress' : s.status === 'blocked' ? '⛔ Blocked' : '⏳ Todo';
    const prioIcon = s.priority === 'critical' ? '🔴 Critical' : s.priority === 'high' ? '🟠 High' : s.priority === 'medium' ? '🟡 Medium' : '🟢 Low';
    md += `| \`${s.id}\` | ${s.title} | **${s.storyPoints}** | ${prioIcon} | ${s.component} | ${s.assignee} | ${statusBadge} |\n`;
  });
  md += `\n`;

  md += `## 📜 Historie vergangener Sprints\n\n`;
  doc.pastSprints.forEach(ps => {
    md += `### Sprint ${ps.sprintNumber}: ${ps.name} (${ps.completedPoints}/${ps.totalPoints} SP)\n`;
    md += `- **Gelieferte Features:** ${ps.deliveredFeatures.join(', ')}\n\n`;
  });

  return md;
}

export function syncSprintFileInWorkspace(files: FileState[], doc: LiveSprintDoc): FileState[] {
  const content = generateSprintsMarkdown(doc);
  const existingIdx = files.findIndex(f => f.name === 'SPRINTS.md');

  if (existingIdx >= 0) {
    const copy = [...files];
    copy[existingIdx] = { ...copy[existingIdx], content };
    return copy;
  }

  return [
    ...files,
    {
      name: 'SPRINTS.md',
      content,
      iconColor: 'text-amber-400',
      iconShape: '🏃',
    },
  ];
}

// =========================================================================
// 3. DOCUMENTATION INDEX & API REFERENCE GENERATOR & SYNC
// =========================================================================

export function generateLiveDocumentation(files: FileState[]): LiveDocumentationDoc {
  const apiEntries: ApiDocEntry[] = [];
  const indexedFiles = files.map(f => {
    let category = 'Allgemein';
    if (f.name.endsWith('.ts') || f.name.endsWith('.tsx') || f.name.endsWith('.js')) category = 'Frontend & TypeScript';
    else if (f.name.endsWith('.rs') || f.name.endsWith('.c') || f.name.endsWith('.h')) category = 'Systemkern & Kernel';
    else if (f.name.endsWith('.lm')) category = 'Lumino Quellcode';
    else if (f.name.endsWith('.json') || f.name.endsWith('.conf') || f.name.endsWith('.gmanifest')) category = 'Konfiguration & Manifeste';
    else if (f.name.endsWith('.md') || f.name.endsWith('.wiki')) category = 'Dokumentation';

    // Scan for functions, classes, structs
    const lines = f.content.split('\n');
    lines.forEach((lineStr, lineIdx) => {
      // Functions
      const fnMatch = lineStr.match(/(?:export\s+)?(?:fn|function|pub fn)\s+([a-zA-Z0-9_]+)\s*\(([^)]*)\)/);
      if (fnMatch) {
        apiEntries.push({
          fileName: f.name,
          symbolType: 'function',
          name: fnMatch[1],
          signature: `${fnMatch[1]}(${fnMatch[2].trim()})`,
          description: `Öffentliche Funktion im Modul \`${f.name}\`.`,
          line: lineIdx + 1,
        });
      }

      // Structs / Interfaces / Classes
      const structMatch = lineStr.match(/(?:export\s+)?(?:struct|interface|class|enum)\s+([a-zA-Z0-9_]+)/);
      if (structMatch) {
        apiEntries.push({
          fileName: f.name,
          symbolType: 'struct',
          name: structMatch[1],
          signature: `${structMatch[1]}`,
          description: `Datentyp/Definition in Modul \`${f.name}\`.`,
          line: lineIdx + 1,
        });
      }
    });

    return {
      name: f.name,
      category,
      lines: lines.length,
      description: `Projektdatei '${f.name}' mit ${lines.length} Code-Zeilen.`,
    };
  });

  const matrixVersions = [
    {
      component: 'ShivaCore Microkernel',
      version: 'v0.9.4-alpha',
      status: 'stable' as const,
      notes: 'x86_64 Long Mode, Paging, Ringbuffer IPC',
    },
    {
      component: 'Lumino Compiler Engine',
      version: 'v2.5.1-live',
      status: 'stable' as const,
      notes: 'Parser, AST Linter, WebAssembly Runtime',
    },
    {
      component: 'ATC-VM Register Machine',
      version: 'v1.4.0',
      status: 'beta' as const,
      notes: 'Determinismus, Gas-Accounting, ZK-Verifikation',
    },
    {
      component: 'Globus Desktop Compositor',
      version: 'v0.8.2',
      status: 'beta' as const,
      notes: 'Wayland Surface Protocol, Shader Pipeline',
    },
    {
      component: 'A-TownChain P2P Consensus',
      version: 'v0.6.0',
      status: 'planned' as const,
      notes: 'Byzantinische Finalität, Devnet Genesis',
    },
  ];

  return {
    projectName: 'A-TownChain & ShivaCore Globus OS',
    version: '2.5.1',
    apiEntries,
    indexedFiles,
    matrixVersions,
    lastUpdated: new Date().toLocaleTimeString(),
  };
}

export function generateDocIndexMarkdown(doc: LiveDocumentationDoc): string {
  let md = `# Projekt Dokumentations-Index (Master Documentation Catalog)\n\n`;
  md += `> **Standard:** ATC-DOC Standard-001 | Version: ${doc.version} | Stand: ${doc.lastUpdated}\n\n`;

  md += `## 📚 Repository Dateikatalog (${doc.indexedFiles.length} Dateien)\n\n`;
  md += `| Datei | Kategorie | Zeilen | Beschreibung |\n`;
  md += `| :--- | :--- | :--- | :--- |\n`;

  doc.indexedFiles.forEach(f => {
    md += `| \`${f.name}\` | **${f.category}** | ${f.lines} | ${f.description} |\n`;
  });
  md += `\n`;

  md += `## 📑 Verfügbare Dokumentations-Module\n\n`;
  md += `- **[ARCHITECTURE.md](ARCHITECTURE.md):** Umfassende Systemarchitektur, Schichtenmodell und ASCII-Komponentendiagramm.\n`;
  md += `- **[ROADMAP.md](ROADMAP.md):** 5 Entwicklungsphasen, strategische Meilensteine und Release-Kriterien.\n`;
  md += `- **[SPRINTS.md](SPRINTS.md):** Aktiver Sprint-Backlog, Story Points, Burndown und Velocity.\n`;
  md += `- **[TODO.md](TODO.md):** Aus Quellcode-Kommentaren und Sicherheits-Audits extrahierte Aufgaben.\n`;
  md += `- **[docs/API_REFERENCE.md](docs/API_REFERENCE.md):** Automatisch generierte Schnittstellen- und Funktionsdokumentation.\n`;
  md += `- **[docs/VERSION_MATRIX.md](docs/VERSION_MATRIX.md):** Kompatibilitätsmatrix aller Module und Compiler.\n`;
  md += `- **[CHANGELOG.md](CHANGELOG.md):** Chronologisches Änderungsprotokoll nach Keep-A-Changelog.\n`;

  return md;
}

export function generateApiReferenceMarkdown(doc: LiveDocumentationDoc): string {
  let md = `# API & Funktions-Referenz (Automatisch Generiert)\n\n`;
  md += `*Generiert durch ATOS Code Analyzer | Stand: ${doc.lastUpdated} | Einträge: ${doc.apiEntries.length}*\n\n`;

  if (doc.apiEntries.length === 0) {
    md += `*Keine exportierten Funktionen oder Typen in den aktuellen Workspace-Dateien gefunden.*\n`;
    return md;
  }

  md += `## Übersicht der extrahierten Symbole\n\n`;
  md += `| Symbol | Typ | Datei : Zeile | Signatur |\n`;
  md += `| :--- | :--- | :--- | :--- |\n`;

  doc.apiEntries.forEach(item => {
    md += `| \`${item.name}\` | **${item.symbolType}** | \`${item.fileName}:${item.line}\` | \`${item.signature}\` |\n`;
  });
  md += `\n`;

  return md;
}

export function generateVersionMatrixMarkdown(doc: LiveDocumentationDoc): string {
  let md = `# Version & Kompatibilitäts-Matrix (VERSION_MATRIX.md)\n\n`;
  md += `> **ATOS Engineering Standard ATC-DOC-003** | Zuletzt aktualisiert: ${doc.lastUpdated}\n\n`;

  md += `| Komponente / Subsystem | Version | Status | Spezifikation & Notizen |\n`;
  md += `| :--- | :--- | :--- | :--- |\n`;

  doc.matrixVersions.forEach(v => {
    const statusIcon = v.status === 'stable' ? '🟢 Stable' : v.status === 'beta' ? '🟡 Beta' : '🔵 Planned';
    md += `| **${v.component}** | \`${v.version}\` | ${statusIcon} | ${v.notes} |\n`;
  });
  md += `\n`;

  md += `## ABI- und Interoperabilitäts-Garantien\n`;
  md += `- **Systemcall ABI:** 64-bit Register-Konvention (RAX = Syscall ID, RDI, RSI, RDX, R10, R8, R9).\n`;
  md += `- **Bytecode Determinismus:** Gleiche Bytecode-Hashes führen auf allen Knoten zum identischen Zustand.\n`;
  md += `- **Forward-Compatibility:** Minor-Versionen bleiben vollständig abwärtskompatibel.\n`;

  return md;
}

export function generateChangelogMarkdown(version: string): string {
  const dateStr = new Date().toISOString().split('T')[0];
  let md = `# Changelog\n\n`;
  md += `Alle nennenswerten Änderungen an diesem Projekt werden in dieser Datei dokumentiert.\n`;
  md += `Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/) und dieses Projekt folgt [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n`;

  md += `## [${version}] - ${dateStr}\n\n`;
  md += `### Added (Neu hinzugefügt)\n`;
  md += `- **Echtzeit-Aktualisierung (Auto-Sync Hub):** Automatisches Synchronisieren aller 6 Entwicklungsartefakte:\n`;
  md += `  - 🏛️ **Wiki:** \`ARCHITECTURE.md\` und Architekturschichtenmodell\n`;
  md += `  - 🗺️ **Roadmap:** \`ROADMAP.md\` und 5 strategische Meilenstein-Phasen\n`;
  md += `  - ✓ **Todos:** \`TODO.md\` inklusive Source-Code-Kommentar-Parser und Checkbox-Sync\n`;
  md += `  - 🏃 **Sprints:** \`SPRINTS.md\` mit Story Points, Burndown-Chart und Geschwindigkeitsmetriken\n`;
  md += `  - 📚 **Dokumentation:** \`docs/DOCUMENTATION_INDEX.md\`, \`docs/API_REFERENCE.md\` und \`docs/VERSION_MATRIX.md\`\n`;
  md += `  - 🏷️ **Version:** \`VERSION\` Datei mit SemVer, Build-Hash und automatischer Reversionsberechnung\n`;
  md += `- Interaktive Status-Zentrale mit Einzel- und Master-Schaltern in der Menüleiste.\n\n`;

  md += `### Changed (Geändert)\n`;
  md += `- Verbesserte Linkage-Graph-Generierung mit zirkulärer Abhängigkeitserkennung.\n`;
  md += `- Optimierte Speicherzuweisungen im Slub-Allocator.\n\n`;

  md += `### Security (Sicherheitsverbesserungen)\n`;
  md += `- Automatische Erkennung kritischer Audit-Befunde und Einbindung als Prio-1 Tasks in \`TODO.md\`.\n`;
  md += `- Zero-Cost Error Boundary Validierung.\n\n`;

  return md;
}

export function syncDocumentationFilesInWorkspace(files: FileState[], doc: LiveDocumentationDoc, currentVersion: string): FileState[] {
  const docIndexContent = generateDocIndexMarkdown(doc);
  const apiRefContent = generateApiReferenceMarkdown(doc);
  const versionMatrixContent = generateVersionMatrixMarkdown(doc);
  const changelogContent = generateChangelogMarkdown(currentVersion);

  let updated = [...files];

  const updateOrAdd = (name: string, content: string, iconShape: string, iconColor: string) => {
    const idx = updated.findIndex(f => f.name === name);
    if (idx >= 0) {
      updated[idx] = { ...updated[idx], content };
    } else {
      updated.push({ name, content, iconShape, iconColor });
    }
  };

  updateOrAdd('docs/DOCUMENTATION_INDEX.md', docIndexContent, '📚', 'text-sky-400');
  updateOrAdd('docs/API_REFERENCE.md', apiRefContent, '📑', 'text-indigo-400');
  updateOrAdd('docs/VERSION_MATRIX.md', versionMatrixContent, '🔢', 'text-emerald-400');
  updateOrAdd('CHANGELOG.md', changelogContent, '📜', 'text-amber-400');

  return updated;
}

// =========================================================================
// 4. VERSION MANAGEMENT & SYNC
// =========================================================================

export function extractCurrentVersion(files: FileState[]): VersionInfo {
  const versionFile = files.find(f => f.name === 'VERSION');
  let raw = versionFile ? versionFile.content.trim() : '2.5.1';

  // Parse semver e.g. 2.5.1 or 2.5.1+build.108
  const match = raw.match(/^v?(\d+)\.(\d+)\.(\d+)(?:\+build\.(\d+))?/);
  let major = 2;
  let minor = 5;
  let patch = 1;
  let build = 108;

  if (match) {
    major = parseInt(match[1], 10);
    minor = parseInt(match[2], 10);
    patch = parseInt(match[3], 10);
    if (match[4]) build = parseInt(match[4], 10);
  }

  const audit = runProjectAudit(files);

  // Recommend bump based on audit & files
  let recommendedBump: VersionInfo['recommendedBump'] = 'none';
  let recommendedReason = 'Keine kritischen Änderungen erkannt.';

  if (audit.criticalCount > 0) {
    recommendedBump = 'patch';
    recommendedReason = `${audit.criticalCount} kritische Befunde erfordern einen Sicherheits-Patch.`;
  } else if (files.length > 25) {
    recommendedBump = 'minor';
    recommendedReason = 'Neue Module und Schnittstellen hinzugefügt.';
  } else {
    recommendedBump = 'patch';
    recommendedReason = 'Reguläre Code-Pflege und Optimierungen.';
  }

  return {
    major,
    minor,
    patch,
    build,
    fullVersion: `${major}.${minor}.${patch}+build.${build}`,
    recommendedBump,
    recommendedReason,
    lastBumpReason: 'Auto-Sync Synchronisationszyklus',
    timestamp: new Date().toLocaleTimeString(),
  };
}

export function bumpVersion(
  current: VersionInfo,
  bumpType: 'patch' | 'minor' | 'major' | 'auto'
): VersionInfo {
  const type = bumpType === 'auto' ? (current.recommendedBump === 'none' ? 'patch' : current.recommendedBump) : bumpType;

  let newMajor = current.major;
  let newMinor = current.minor;
  let newPatch = current.patch;
  let newBuild = current.build + 1;

  if (type === 'major') {
    newMajor += 1;
    newMinor = 0;
    newPatch = 0;
  } else if (type === 'minor') {
    newMinor += 1;
    newPatch = 0;
  } else {
    newPatch += 1;
  }

  const fullVersion = `${newMajor}.${newMinor}.${newPatch}+build.${newBuild}`;

  return {
    major: newMajor,
    minor: newMinor,
    patch: newPatch,
    build: newBuild,
    fullVersion,
    recommendedBump: 'none',
    recommendedReason: 'Version wurde soeben aktualisiert.',
    lastBumpReason: `Erfolgreicher ${type.toUpperCase()}-Bump auf v${newMajor}.${newMinor}.${newPatch}`,
    timestamp: new Date().toLocaleTimeString(),
  };
}

export function syncVersionFileInWorkspace(
  files: FileState[],
  versionInfo: VersionInfo
): { updatedFiles: FileState[]; versionString: string } {
  let updated = [...files];
  const versionString = `${versionInfo.major}.${versionInfo.minor}.${versionInfo.patch}`;
  const fullContent = `${versionInfo.fullVersion}\n`;

  // 1. Update VERSION file
  const versionIdx = updated.findIndex(f => f.name === 'VERSION');
  if (versionIdx >= 0) {
    updated[versionIdx] = { ...updated[versionIdx], content: fullContent };
  } else {
    updated.push({
      name: 'VERSION',
      content: fullContent,
      iconShape: '🏷️',
      iconColor: 'text-emerald-400',
    });
  }

  // 2. Update package.json if it exists in workspace
  const pkgIdx = updated.findIndex(f => f.name === 'package.json');
  if (pkgIdx >= 0) {
    try {
      const parsed = JSON.parse(updated[pkgIdx].content);
      parsed.version = versionString;
      updated[pkgIdx] = {
        ...updated[pkgIdx],
        content: JSON.stringify(parsed, null, 2) + '\n',
      };
    } catch {
      // ignore parse issues
    }
  }

  return {
    updatedFiles: updated,
    versionString,
  };
}

// =========================================================================
// 5. MASTER AUTO-SYNC ORCHESTRATOR
// =========================================================================

export function syncAllProjectArtifacts(
  files: FileState[],
  config: AutoSyncConfig,
  options?: {
    versionBumpType?: 'patch' | 'minor' | 'major' | 'auto';
    forceAll?: boolean;
  }
): {
  updatedFiles: FileState[];
  results: PillarSyncResult[];
  newVersionString: string;
} {
  let currentFiles = [...files];
  const results: PillarSyncResult[] = [];
  const timestamp = new Date().toLocaleTimeString();
  const force = options?.forceAll || false;

  // 1. Version Sync
  let versionInfo = extractCurrentVersion(currentFiles);
  if (options?.versionBumpType) {
    versionInfo = bumpVersion(versionInfo, options.versionBumpType);
  }

  if (config.autoSyncVersion || force) {
    const vRes = syncVersionFileInWorkspace(currentFiles, versionInfo);
    currentFiles = vRes.updatedFiles;
    results.push({
      pillar: 'version',
      targetFiles: ['VERSION', 'package.json'],
      status: 'synced',
      summary: `Version aktualisiert auf v${versionInfo.major}.${versionInfo.minor}.${versionInfo.patch} (Build #${versionInfo.build})`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'version',
      targetFiles: ['VERSION'],
      status: 'skipped',
      summary: 'Automatische Versionierung deaktiviert',
      timestamp,
    });
  }

  const currentVersionStr = `${versionInfo.major}.${versionInfo.minor}.${versionInfo.patch}`;

  // 2. Wiki Sync (ARCHITECTURE.md)
  if (config.autoSyncWiki || force) {
    const audit = runProjectAudit(currentFiles);
    const archDoc = generateLiveArchitectureDocs(currentFiles, audit);
    currentFiles = syncWikiFileInWorkspace(currentFiles, archDoc);
    results.push({
      pillar: 'wiki',
      targetFiles: ['ARCHITECTURE.md'],
      status: 'synced',
      summary: `Architektur-Wiki mit ${archDoc.modules.length} Modulen und 5 Schichten synchronisiert`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'wiki',
      targetFiles: ['ARCHITECTURE.md'],
      status: 'skipped',
      summary: 'Wiki Auto-Sync deaktiviert',
      timestamp,
    });
  }

  // 3. Roadmap Sync (ROADMAP.md & docs/Roadmap.wiki)
  if (config.autoSyncRoadmap || force) {
    const roadmapDoc = generateLiveRoadmap(currentFiles);
    currentFiles = syncRoadmapFileInWorkspace(currentFiles, roadmapDoc);
    results.push({
      pillar: 'roadmap',
      targetFiles: ['ROADMAP.md', 'docs/Roadmap.wiki'],
      status: 'synced',
      summary: `Roadmap mit 5 Phasen und ${roadmapDoc.overallProgress}% Gesamtfortschritt synchronisiert`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'roadmap',
      targetFiles: ['ROADMAP.md'],
      status: 'skipped',
      summary: 'Roadmap Auto-Sync deaktiviert',
      timestamp,
    });
  }

  // 4. Todos Sync (TODO.md)
  if (config.autoSyncTodo || force) {
    const audit = runProjectAudit(currentFiles);
    const todos = extractProjectTodos(currentFiles, audit.findings);
    currentFiles = syncTodoFileInWorkspace(currentFiles, todos);
    const pending = todos.filter(t => !t.completed).length;
    const done = todos.filter(t => t.completed).length;
    results.push({
      pillar: 'todo',
      targetFiles: ['TODO.md'],
      status: 'synced',
      summary: `${todos.length} Aufgaben synchronisiert (${pending} offen, ${done} erledigt)`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'todo',
      targetFiles: ['TODO.md'],
      status: 'skipped',
      summary: 'TODO Auto-Sync deaktiviert',
      timestamp,
    });
  }

  // 5. Sprints Sync (SPRINTS.md)
  if (config.autoSyncSprints || force) {
    const sprintDoc = generateLiveSprints(currentFiles);
    currentFiles = syncSprintFileInWorkspace(currentFiles, sprintDoc);
    results.push({
      pillar: 'sprints',
      targetFiles: ['SPRINTS.md'],
      status: 'synced',
      summary: `Sprint #${sprintDoc.sprintNumber} (${sprintDoc.completedPoints}/${sprintDoc.totalPoints} SP, ${sprintDoc.burndownPercentage}%) synchronisiert`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'sprints',
      targetFiles: ['SPRINTS.md'],
      status: 'skipped',
      summary: 'Sprint Auto-Sync deaktiviert',
      timestamp,
    });
  }

  // 6. Dokumentation Sync (docs/DOCUMENTATION_INDEX.md, docs/API_REFERENCE.md, docs/VERSION_MATRIX.md, CHANGELOG.md)
  if (config.autoSyncDocs || force) {
    const liveDoc = generateLiveDocumentation(currentFiles);
    currentFiles = syncDocumentationFilesInWorkspace(currentFiles, liveDoc, currentVersionStr);
    results.push({
      pillar: 'docs',
      targetFiles: ['docs/DOCUMENTATION_INDEX.md', 'docs/API_REFERENCE.md', 'docs/VERSION_MATRIX.md', 'CHANGELOG.md'],
      status: 'synced',
      summary: `Dokumentationsindex, API-Referenz (${liveDoc.apiEntries.length} Symbole) & Changelog synchronisiert`,
      timestamp,
    });
  } else {
    results.push({
      pillar: 'docs',
      targetFiles: ['docs/DOCUMENTATION_INDEX.md', 'CHANGELOG.md'],
      status: 'skipped',
      summary: 'Dokumentations-Auto-Sync deaktiviert',
      timestamp,
    });
  }

  return {
    updatedFiles: currentFiles,
    results,
    newVersionString: currentVersionStr,
  };
}
