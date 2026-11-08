export type AutoSyncPillarId = 'wiki' | 'roadmap' | 'todo' | 'sprints' | 'docs' | 'version';

export interface AutoSyncConfig {
  autoSyncWiki: boolean;
  autoSyncRoadmap: boolean;
  autoSyncTodo: boolean;
  autoSyncSprints: boolean;
  autoSyncDocs: boolean;
  autoSyncVersion: boolean;
}

export interface RoadmapMilestone {
  id: string;
  title: string;
  completed: boolean;
  targetDate: string;
  details: string;
  associatedFiles: string[];
}

export interface RoadmapPhase {
  id: string;
  title: string;
  badge: string;
  description: string;
  progress: number; // 0 - 100
  status: 'completed' | 'in_progress' | 'planned';
  milestones: RoadmapMilestone[];
}

export interface LiveRoadmapDoc {
  projectName: string;
  version: string;
  overallProgress: number;
  phases: RoadmapPhase[];
  activeMilestone: string;
  estimatedReleaseDate: string;
  lastUpdated: string;
}

export interface SprintStory {
  id: string;
  title: string;
  storyPoints: number;
  priority: 'critical' | 'high' | 'medium' | 'low';
  component: string;
  assignee: string;
  status: 'done' | 'in_progress' | 'todo' | 'blocked';
  relatedFindingId?: string;
  associatedFile?: string;
}

export interface PastSprintSummary {
  sprintNumber: number;
  name: string;
  completedPoints: number;
  totalPoints: number;
  deliveredFeatures: string[];
}

export interface LiveSprintDoc {
  sprintNumber: number;
  sprintName: string;
  startDate: string;
  endDate: string;
  goal: string;
  totalPoints: number;
  completedPoints: number;
  remainingPoints: number;
  burndownPercentage: number;
  velocity: number;
  stories: SprintStory[];
  pastSprints: PastSprintSummary[];
  lastUpdated: string;
}

export interface ApiDocEntry {
  fileName: string;
  symbolType: 'function' | 'class' | 'struct' | 'interface' | 'contract' | 'constant' | 'module';
  name: string;
  signature: string;
  description: string;
  line: number;
}

export interface LiveDocumentationDoc {
  projectName: string;
  version: string;
  apiEntries: ApiDocEntry[];
  indexedFiles: {
    name: string;
    category: string;
    lines: number;
    description: string;
  }[];
  matrixVersions: {
    component: string;
    version: string;
    status: 'stable' | 'beta' | 'planned';
    notes: string;
  }[];
  lastUpdated: string;
}

export interface VersionInfo {
  major: number;
  minor: number;
  patch: number;
  build: number;
  prerelease?: string;
  fullVersion: string;
  recommendedBump: 'patch' | 'minor' | 'major' | 'none';
  recommendedReason: string;
  lastBumpReason: string;
  timestamp: string;
}

export interface PillarSyncResult {
  pillar: AutoSyncPillarId;
  targetFiles: string[];
  status: 'synced' | 'skipped' | 'unchanged' | 'error';
  summary: string;
  timestamp: string;
}

export interface ProjectSyncSummary {
  timestamp: string;
  results: PillarSyncResult[];
  allFilesSynced: boolean;
}
