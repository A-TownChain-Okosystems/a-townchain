export type AuditCategory = 'security' | 'completeness' | 'functionality' | 'linkage';

export type AuditSeverity = 'critical' | 'high' | 'medium' | 'low' | 'info';

export interface AuditAutoFix {
  label: string;
  action: 'replace_match' | 'insert_line_after' | 'remove_line' | 'create_file' | 'wrap_guarded' | 'init_variable';
  targetFile: string;
  line: number;
  match?: string;
  replacement?: string;
  newFileContent?: string;
}

export interface AuditFinding {
  id: string;
  category: AuditCategory;
  ruleId: string;
  title: string;
  description: string;
  severity: AuditSeverity;
  fileName: string;
  line: number; // 1-indexed
  match: string;
  recommendation: string;
  impact: string;
  autoFixable: boolean;
  autoFix?: AuditAutoFix;
}

export interface LinkageNode {
  id: string;
  name: string;
  extension: string;
  size: number;
  isEntry: boolean;
  isOrphaned: boolean;
  linesCount: number;
}

export interface LinkageEdge {
  from: string;
  to: string;
  importStatement: string;
  line: number;
  resolved: boolean;
  symbols: string[];
}

export interface CircularDependency {
  cycle: string[];
  description: string;
}

export interface ProjectAuditSummary {
  healthScore: number; // 0 - 100
  grade: 'A+' | 'A' | 'B' | 'C' | 'D' | 'F';
  totalFindings: number;
  criticalCount: number;
  highCount: number;
  mediumCount: number;
  lowCount: number;
  infoCount: number;
  byCategory: {
    security: number;
    completeness: number;
    functionality: number;
    linkage: number;
  };
  findings: AuditFinding[];
  linkageGraph: {
    nodes: LinkageNode[];
    edges: LinkageEdge[];
  };
  orphanedFiles: string[];
  circularDeps: CircularDependency[];
  analyzedFilesCount: number;
  analyzedLinesCount: number;
  timestamp: number;
}

export type TodoPriority = 'critical' | 'high' | 'medium' | 'low';
export type TodoSource = 'code_comment' | 'audit_finding';

export interface TodoItem {
  id: string;
  fileName: string;
  line: number;
  tag: 'TODO' | 'FIXME' | 'BUG' | 'HACK' | 'REVIEW' | 'AUDIT';
  text: string;
  priority: TodoPriority;
  completed: boolean;
  source: TodoSource;
  findingId?: string;
}

export interface LiveArchitectureModule {
  fileName: string;
  role: 'Core / Kernel' | 'Smart Contract / Token' | 'Networking / Protocol' | 'User Interface' | 'Configuration' | 'Documentation' | 'Utility';
  exports: string[];
  imports: string[];
  description: string;
  lines: number;
  securityRating: 'Secure' | 'Needs Review' | 'Vulnerable';
}

export interface LiveArchitectureDoc {
  projectName: string;
  version: string;
  overview: string;
  modules: LiveArchitectureModule[];
  layers: {
    name: string;
    description: string;
    files: string[];
  }[];
  dependencySummary: string;
  securitySummary: string;
  asciiDiagram: string;
  lastUpdated: string;
}
