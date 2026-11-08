import {
  ProjectAuditSummary,
  AuditFinding,
  AuditSeverity,
  LinkageNode,
  LinkageEdge,
  CircularDependency,
  TodoItem,
  LiveArchitectureDoc,
  LiveArchitectureModule
} from '../types/audit';

export interface FileState {
  name: string;
  content: string;
  iconColor: string;
  iconShape: string;
}

// Helper to normalize file names for path matching
function normalizePath(path: string): string {
  return path.replace(/^\.\//, '').replace(/^\//, '').trim();
}

/**
 * Executes a deep multi-phase project audit across all files in the workspace:
 * 1. Security Audit
 * 2. Completeness Audit
 * 3. Functionality Audit
 * 4. Linkage (Cross-File) Audit
 */
export function runProjectAudit(files: FileState[]): ProjectAuditSummary {
  const findings: AuditFinding[] = [];
  const fileMap = new Map<string, FileState>();
  files.forEach(f => fileMap.set(f.name, f));

  let totalLines = 0;

  // -------------------------------------------------------------
  // Phase 1, 2, 3: Per-File Audits (Security, Completeness, Function)
  // -------------------------------------------------------------
  files.forEach(file => {
    const lines = file.content.split('\n');
    totalLines += lines.length;
    const isDocOrJson = file.name.endsWith('.json') || file.name.endsWith('.md') || file.name.endsWith('.wiki');

    if (!isDocOrJson) {
      auditSecurity(file, lines, findings);
      auditCompleteness(file, lines, findings);
      auditFunctionality(file, lines, findings);
    }
  });

  // -------------------------------------------------------------
  // Phase 4: Linkage Audit (Cross-file dependencies, unresolved imports, orphans)
  // -------------------------------------------------------------
  const { linkageNodes, linkageEdges, circularDeps, orphanedFiles } = auditLinkage(files, fileMap, findings);

  // -------------------------------------------------------------
  // Calculate Health Score & Grades
  // -------------------------------------------------------------
  let penalty = 0;
  let criticalCount = 0;
  let highCount = 0;
  let mediumCount = 0;
  let lowCount = 0;
  let infoCount = 0;

  findings.forEach(f => {
    if (f.severity === 'critical') {
      penalty += 18;
      criticalCount++;
    } else if (f.severity === 'high') {
      penalty += 10;
      highCount++;
    } else if (f.severity === 'medium') {
      penalty += 4;
      mediumCount++;
    } else if (f.severity === 'low') {
      penalty += 1.5;
      lowCount++;
    } else {
      penalty += 0.5;
      infoCount++;
    }
  });

  const healthScore = Math.max(0, Math.min(100, Math.round(100 - penalty)));

  let grade: 'A+' | 'A' | 'B' | 'C' | 'D' | 'F' = 'A+';
  if (healthScore >= 95) grade = 'A+';
  else if (healthScore >= 85) grade = 'A';
  else if (healthScore >= 70) grade = 'B';
  else if (healthScore >= 50) grade = 'C';
  else if (healthScore >= 35) grade = 'D';
  else grade = 'F';

  return {
    healthScore,
    grade,
    totalFindings: findings.length,
    criticalCount,
    highCount,
    mediumCount,
    lowCount,
    infoCount,
    byCategory: {
      security: findings.filter(f => f.category === 'security').length,
      completeness: findings.filter(f => f.category === 'completeness').length,
      functionality: findings.filter(f => f.category === 'functionality').length,
      linkage: findings.filter(f => f.category === 'linkage').length,
    },
    findings,
    linkageGraph: {
      nodes: linkageNodes,
      edges: linkageEdges,
    },
    orphanedFiles,
    circularDeps,
    analyzedFilesCount: files.length,
    analyzedLinesCount: totalLines,
    timestamp: Date.now(),
  };
}

// =========================================================================
// 1. SECURITY AUDIT
// =========================================================================
function auditSecurity(file: FileState, lines: string[], findings: AuditFinding[]) {
  // A. Hardcoded Secrets & Private Keys
  const secretPattern = /(?:secret|private_key|api_key|access_token|password|auth_token)\s*[:=]\s*["']([^"']{4,})["']/i;
  // B. Unsafe / Raw memory access
  const unsafeBlockPattern = /\bunsafe\s*\{/i;
  const rawPtrPattern = /\b(ptr::read|ptr::write|mem::transmute|raw_alloc)\b/i;
  // C. Unbounded loops (Denial of Service)
  const infiniteLoopPattern = /\bwhile\s*(?:\(\s*(?:true|1)\s*\)|true|1)\s*\{/i;
  // D. Unchecked dangerous execution (eval/exec)
  const dangerousExecPattern = /\b(eval|exec|system|spawn_process|dangerously_run)\s*\(/i;
  // E. Sensitive data exposure in logs
  const sensitiveLogPattern = /\b(print|console\.log|println!)\s*\([^)]*(password|secret|privateKey|token)[^)]*\)/i;
  // F. Kernel Syscall without capability check
  const syscallPattern = /\b(syscall|kernel_call|sys_invoke)\s*\(/i;

  let inFunction = false;
  let currentFunctionName = '';
  let functionHasCapCheck = false;

  lines.forEach((line, idx) => {
    const lineNum = idx + 1;
    const trimmed = line.trim();
    if (trimmed.startsWith('//') || trimmed.startsWith('#') || trimmed.startsWith('/*')) return;

    // Track function context
    const fnMatch = trimmed.match(/\b(?:fn|function)\s+([a-zA-Z_]\w*)/);
    if (fnMatch) {
      inFunction = true;
      currentFunctionName = fnMatch[1];
      functionHasCapCheck = false;
    }
    if (trimmed.includes('capability') || trimmed.includes('permission') || trimmed.includes('auth') || trimmed.includes('is_admin') || trimmed.includes('Ring0')) {
      functionHasCapCheck = true;
    }

    // 1. Hardcoded Secrets
    const secretMatch = trimmed.match(secretPattern);
    if (secretMatch) {
      findings.push({
        id: `sec-secret-${file.name}-${lineNum}`,
        category: 'security',
        ruleId: 'SEC-001',
        title: 'Klartext-Geheimnis / Private Key entdeckt',
        description: `Klartext-Geheimnis oder API-Schlüssel direkt im Quellcode hinterlegt ('${secretMatch[1].slice(0, 3)}***'). Dies birgt ein enormes Risiko für Credential-Leakage.`,
        severity: 'critical',
        fileName: file.name,
        line: lineNum,
        match: secretMatch[0],
        recommendation: 'Geheimnisse über Umgebungsvariablen (process.env) oder Hardware-Security-Module (HSM/TPM) beziehen.',
        impact: 'Gefahr des unbefugten Zugriffs und der Kompromittierung des Netzwerks.',
        autoFixable: true,
        autoFix: {
          label: 'In Umgebungsvariable umwandeln',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: secretMatch[0],
          replacement: secretMatch[0].replace(secretMatch[1], `env("ENV_${secretMatch[0].split(/[=:]/)[0].trim().toUpperCase()}")`),
        }
      });
    }

    // 2. Unsafe Memory Access
    if (unsafeBlockPattern.test(trimmed) || rawPtrPattern.test(trimmed)) {
      const matchText = trimmed.match(unsafeBlockPattern)?.[0] || trimmed.match(rawPtrPattern)?.[0] || 'unsafe';
      findings.push({
        id: `sec-unsafe-${file.name}-${lineNum}`,
        category: 'security',
        ruleId: 'SEC-002',
        title: 'Ungeschützter Speicherzugriff (Unsafe Block)',
        description: 'Direkter oder nativer Speicherzugriff hebelt Speichersicherheits- und Sandbox-Garantien aus.',
        severity: 'high',
        fileName: file.name,
        line: lineNum,
        match: matchText,
        recommendation: 'Speicherzugriffe durch Memory-Protection-Unit (MPU) oder geprüfte Lumino-Safe-Buffer kapseln.',
        impact: 'Buffer Overflows, Use-After-Free und potenzielle Memory-Corruption-Angriffe.',
        autoFixable: true,
        autoFix: {
          label: 'Mit Safe-Memory-Guard kapseln',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: matchText,
          replacement: `/* MPU_GUARD */ guarded_memory_scope {`,
        }
      });
    }

    // 3. Infinite Loops (Denial of Service)
    if (infiniteLoopPattern.test(trimmed)) {
      // Check if subsequent lines have break
      const remainingCode = lines.slice(idx, idx + 20).join('\n');
      if (!remainingCode.includes('break') && !remainingCode.includes('return')) {
        findings.push({
          id: `sec-dos-loop-${file.name}-${lineNum}`,
          category: 'security',
          ruleId: 'SEC-003',
          title: 'Potenzielle Endlosschleife / DoS-Risiko',
          description: 'While-Schleife mit konstanter Wahrheitsbedingung ohne ersichtlichen Abbruch (break/return).',
          severity: 'high',
          fileName: file.name,
          line: lineNum,
          match: trimmed,
          recommendation: 'Einen Schleifenabbruch-Zähler oder ein deterministisches Gas-/Tick-Limit einfügen.',
          impact: 'Thread-Blockade, Denial of Service (DoS) und CPU-Aushungerung des Host-Systems.',
          autoFixable: true,
          autoFix: {
            label: 'Timeout- & Tick-Guard einfügen',
            action: 'replace_match',
            targetFile: file.name,
            line: lineNum,
            match: trimmed,
            replacement: `let mut _ticks = 0;\nwhile _ticks < 10000 {\n    _ticks = _ticks + 1;`,
          }
        });
      }
    }

    // 4. Dangerous Arbitrary Execution
    if (dangerousExecPattern.test(trimmed)) {
      findings.push({
        id: `sec-eval-${file.name}-${lineNum}`,
        category: 'security',
        ruleId: 'SEC-004',
        title: 'Dynamische Code-Ausführung (Code Injection Risiko)',
        description: 'Verwendung von eval/exec zur dynamischen Ausführung ungeprüfter String-Inhalte.',
        severity: 'critical',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Dynamische Ausführung durch typsichere Interpreter-AST-Parsing und Sandbox-Parser ersetzen.',
        impact: 'Ermöglicht Remote Code Execution (RCE) und Escaping der Ausführungsumgebung.',
        autoFixable: false,
      });
    }

    // 5. Sensitive Data Logging
    if (sensitiveLogPattern.test(trimmed)) {
      findings.push({
        id: `sec-log-${file.name}-${lineNum}`,
        category: 'security',
        ruleId: 'SEC-005',
        title: 'Sensible Daten in Ausgabeprotokoll (Log Exposure)',
        description: 'Sensible Identifikatoren (Passwort/Token/Key) werden in Standard-Logs ausgegeben.',
        severity: 'medium',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Sensible Ausgaben entfernen oder maskieren (z. B. hash() oder [REDACTED]).',
        impact: 'Datenleckage durch Log-Infiltration oder unverschlüsselte Telemetrie.',
        autoFixable: true,
        autoFix: {
          label: 'Ausgabe maskieren',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: trimmed,
          replacement: `// [REDACTED SENSITIVE LOG]`,
        }
      });
    }

    // 6. Syscall without Capability Check
    if (syscallPattern.test(trimmed) && inFunction && !functionHasCapCheck) {
      findings.push({
        id: `sec-syscall-${file.name}-${lineNum}`,
        category: 'security',
        ruleId: 'SEC-006',
        title: 'Systemaufruf ohne Privilegien-Prüfung',
        description: `Funktion '${currentFunctionName || 'anonym'}' ruft Kernel-Syscalls auf, ohne vorher Berechtigungen oder Ring-Privilegien zu validieren.`,
        severity: 'high',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Vor Syscall-Ausführung Capability-Tokens prüfen (z. B. assert_capability(CAP_SYS_ADMIN)).',
        impact: 'Privilege Escalation und unberechtigte Kernel-Zustandsänderungen.',
        autoFixable: true,
        autoFix: {
          label: 'Capability-Check voranstellen',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: trimmed,
          replacement: `if !verify_caller_cap(CAP_KERNEL_EXEC) { return Err("PermissionDenied"); }\n    ${trimmed}`,
        }
      });
    }
  });
}

// =========================================================================
// 2. VOLLSTÄNDIGKEITS-PRÜFUNG-AUDIT (Completeness Audit)
// =========================================================================
function auditCompleteness(file: FileState, lines: string[], findings: AuditFinding[]) {
  // A. Uninitialized Variables: let x;
  const uninitVarPattern = /^\s*(?:let|var)\s+([a-zA-Z_]\w*)\s*;$/;
  // B. Empty Function Bodies: fn foo() {}
  const emptyFnPattern = /^\s*(?:fn|function)\s+([a-zA-Z_]\w*)\s*\([^)]*\)\s*\{\s*\}$/;
  // C. Incomplete Branching: if without else where assignment happens
  const ifEmptyBlockPattern = /^\s*if\s*\([^)]*\)\s*\{\s*\}$/;
  // D. Swallowed Catch Blocks: catch (...) {}
  const swallowedCatchPattern = /catch\s*(?:\([^)]*\))?\s*\{\s*\}/;
  // E. TODO / Incomplete placeholders
  const todoStubPattern = /\b(?:TODO|FIXME|STUB|UNIMPLEMENTED)\b/i;

  let inFn = false;
  let currentFnName = '';
  let currentFnStartLine = 1;
  let fnBraceDepth = 0;
  let fnHasReturn = false;
  let fnHasBodyCode = false;

  lines.forEach((line, idx) => {
    const lineNum = idx + 1;
    const trimmed = line.trim();

    // 1. Uninitialized Variables
    const uninitMatch = trimmed.match(uninitVarPattern);
    if (uninitMatch) {
      const varName = uninitMatch[1];
      findings.push({
        id: `comp-uninit-${file.name}-${lineNum}`,
        category: 'completeness',
        ruleId: 'COMP-001',
        title: `Unvollständige Variablen-Initialisierung ('${varName}')`,
        description: `Variable '${varName}' wurde ohne Initialwert deklariert. Kann zu Undefined-State oder Speicher-Nullwerten führen.`,
        severity: 'medium',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: `Initialisieren Sie '${varName}' mit einem sicheren Standardwert (z.B. 0, "", null oder None).`,
        impact: 'Laufzeitfehler durch Dereferenzierung von undefinierten Werten.',
        autoFixable: true,
        autoFix: {
          label: `Initialisieren mit Null-Wert`,
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: trimmed,
          replacement: `let ${varName} = null;`,
        }
      });
    }

    // 2. Empty Function Body
    const emptyFnMatch = trimmed.match(emptyFnPattern);
    if (emptyFnMatch) {
      findings.push({
        id: `comp-emptyfn-${file.name}-${lineNum}`,
        category: 'completeness',
        ruleId: 'COMP-002',
        title: `Leere Funktionsdefinition ('${emptyFnMatch[1]}')`,
        description: `Funktion '${emptyFnMatch[1]}' besitzt keinen Implementierungs-Körper.`,
        severity: 'low',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Implementieren Sie die Funktionslogik oder fügen Sie eine Standard-Rückgabe ein.',
        impact: 'Fehlende Geschäftslogik oder stillschweigendes Nichtstun.',
        autoFixable: true,
        autoFix: {
          label: 'Default-Rumpf ergänzen',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: trimmed,
          replacement: trimmed.replace(/\{\s*\}$/, '{\n    return true;\n}'),
        }
      });
    }

    // 3. Swallowed Error Catch
    if (swallowedCatchPattern.test(trimmed)) {
      findings.push({
        id: `comp-swallow-${file.name}-${lineNum}`,
        category: 'completeness',
        ruleId: 'COMP-003',
        title: 'Verschluckter Fehlerblock (Leeres Catch)',
        description: 'Fehler wird in einem catch-Block abgefangen, aber komplett ignoriert und nicht geloggt oder behandelt.',
        severity: 'medium',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Fehler protokolliert ausgeben oder strukturiert behandeln.',
        impact: 'Schwer auffindbare Folgefehler und unterdrückte Systemabstürze.',
        autoFixable: true,
        autoFix: {
          label: 'Fehlerprotokollierung einfügen',
          action: 'replace_match',
          targetFile: file.name,
          line: lineNum,
          match: trimmed,
          replacement: `catch (err) {\n    print("Handled Exception: " + err);\n}`,
        }
      });
    }

    // 4. Empty If-Block
    if (ifEmptyBlockPattern.test(trimmed)) {
      findings.push({
        id: `comp-emptyif-${file.name}-${lineNum}`,
        category: 'completeness',
        ruleId: 'COMP-004',
        title: 'Leere Bedingungsverzweigung (Leerer If-Block)',
        description: 'Bedingung wird geprüft, führt aber keinen Code aus.',
        severity: 'low',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Block implementieren oder ungenutzte Bedingung entfernen.',
        impact: 'Überflüssige CPU-Zyklen und tote Verzweigung.',
        autoFixable: false,
      });
    }

    // Track function return completeness
    const fnStartMatch = trimmed.match(/\b(?:fn|function)\s+([a-zA-Z_]\w*)/);
    if (fnStartMatch && !inFn) {
      inFn = true;
      currentFnName = fnStartMatch[1];
      currentFnStartLine = lineNum;
      fnBraceDepth = 0;
      fnHasReturn = false;
      fnHasBodyCode = false;
    }

    if (inFn) {
      if (trimmed.includes('return')) fnHasReturn = true;
      if (trimmed !== '{' && trimmed !== '}' && !trimmed.startsWith('//')) fnHasBodyCode = true;

      const openBraces = (line.match(/\{/g) || []).length;
      const closeBraces = (line.match(/\}/g) || []).length;
      fnBraceDepth += openBraces - closeBraces;

      if (fnBraceDepth <= 0 && (openBraces > 0 || closeBraces > 0)) {
        // Function ended
        const isGetterOrCalculator = /^(get|calc|find|is|has|compute|eval|fetch)/i.test(currentFnName);
        if (isGetterOrCalculator && !fnHasReturn && fnHasBodyCode) {
          findings.push({
            id: `comp-missingreturn-${file.name}-${currentFnStartLine}`,
            category: 'completeness',
            ruleId: 'COMP-005',
            title: `Fehlende Rückgabeanweisung ('${currentFnName}')`,
            description: `Funktion '${currentFnName}' deutet durch ihren Namen auf einen Rückgabewert hin, besitzt jedoch kein 'return'.`,
            severity: 'medium',
            fileName: file.name,
            line: currentFnStartLine,
            match: `fn ${currentFnName}`,
            recommendation: `Fügen Sie ein explizites 'return ...;' am Ende der Funktion ein.`,
            impact: 'Funktion liefert implizit undefined/null zurück, was zu Fehlern bei Aufrufern führt.',
            autoFixable: true,
            autoFix: {
              label: 'Return-Statement am Funktionsende einfügen',
              action: 'insert_line_after',
              targetFile: file.name,
              line: lineNum - 1,
              replacement: '    return null;',
            }
          });
        }
        inFn = false;
      }
    }
  });
}

// =========================================================================
// 3. FUNKTION-AUDIT (Functionality & Complexity Audit)
// =========================================================================
function auditFunctionality(file: FileState, lines: string[], findings: AuditFinding[]) {
  // A. Excessive parameters
  const fnDeclPattern = /\b(?:fn|function)\s+([a-zA-Z_]\w*)\s*\(([^)]*)\)/;
  // B. Dead code after return / break
  let afterUnconditionalExit = false;
  let exitLine = 0;

  lines.forEach((line, idx) => {
    const lineNum = idx + 1;
    const trimmed = line.trim();

    // Reset dead code detection on block close
    if (trimmed === '}' || trimmed.startsWith('}')) {
      afterUnconditionalExit = false;
    }

    if (afterUnconditionalExit && trimmed.length > 0 && !trimmed.startsWith('//') && trimmed !== '}') {
      findings.push({
        id: `func-deadcode-${file.name}-${lineNum}`,
        category: 'functionality',
        ruleId: 'FUNC-001',
        title: 'Unerreichbarer Programmcode (Dead Code)',
        description: `Code in Zeile ${lineNum} steht direkt nach einem unbedingten 'return' oder 'break' (Zeile ${exitLine}) und wird niemals ausgeführt.`,
        severity: 'medium',
        fileName: file.name,
        line: lineNum,
        match: trimmed,
        recommendation: 'Entfernen Sie den unerreichbaren Code oder verschieben Sie ihn vor das Exit-Statement.',
        impact: 'Verschwendeter Speicher, verwirrende Code-Wartung und Code Smells.',
        autoFixable: true,
        autoFix: {
          label: 'Unerreichbaren Code entfernen',
          action: 'remove_line',
          targetFile: file.name,
          line: lineNum,
        }
      });
      afterUnconditionalExit = false;
    }

    if (trimmed.startsWith('return ') || trimmed === 'return;' || trimmed === 'break;') {
      afterUnconditionalExit = true;
      exitLine = lineNum;
    }

    // Check Parameter Count
    const fnDeclMatch = trimmed.match(fnDeclPattern);
    if (fnDeclMatch) {
      const fnName = fnDeclMatch[1];
      const params = fnDeclMatch[2].split(',').map(p => p.trim()).filter(Boolean);
      if (params.length > 4) {
        findings.push({
          id: `func-maxparams-${file.name}-${lineNum}`,
          category: 'functionality',
          ruleId: 'FUNC-002',
          title: `Zu viele Funktionsparameter in '${fnName}' (${params.length} Parameter)`,
          description: `Funktion '${fnName}' deklariert ${params.length} Parameter. Clean Code Best Practices empfehlen maximal 3-4 Parameter.`,
          severity: 'low',
          fileName: file.name,
          line: lineNum,
          match: fnDeclMatch[0],
          recommendation: 'Parameter in einem Konfigurationsobjekt oder Struct zusammenfassen.',
          impact: 'Erhöhte Fehleranfälligkeit bei Funktionsaufrufen und schwerere Testbarkeit.',
          autoFixable: false,
        });
      }
    }
  });

  // Calculate Cyclomatic Complexity approximation
  let complexityCount = 1;
  lines.forEach(l => {
    const t = l.trim();
    if (/\b(if|else if|while|for|match|case|&&|\|\|)\b/.test(t)) {
      complexityCount++;
    }
  });

  if (complexityCount > 15) {
    findings.push({
      id: `func-complexity-${file.name}-1`,
      category: 'functionality',
      ruleId: 'FUNC-003',
      title: `Hohe zyklomatische Komplexität (Score: ${complexityCount})`,
      description: `Datei '${file.name}' weist eine hohe Anzahl an Verzweigungen und Schleifen auf (${complexityCount} Kontrollflusspfade).`,
      severity: 'medium',
      fileName: file.name,
      line: 1,
      match: file.name,
      recommendation: 'Große Funktionen in modularere Teilfunktionen zerlegen (Refactoring).',
      impact: 'Schwere Wartbarkeit und hohes Risiko von Randfallfehlern bei Änderungen.',
      autoFixable: false,
    });
  }
}

// =========================================================================
// 4. VERKNÜPFT-AUDIT (Cross-File Linkage & Dependency Graph)
// =========================================================================
function auditLinkage(
  files: FileState[],
  fileMap: Map<string, FileState>,
  findings: AuditFinding[]
) {
  const linkageNodes: LinkageNode[] = [];
  const linkageEdges: LinkageEdge[] = [];
  const importedFileCounts = new Map<string, number>();

  files.forEach(f => {
    importedFileCounts.set(f.name, 0);
  });

  // Regex patterns for imports
  // e.g. import "./network.vx"; or import { x, y } from "./core.atc"; or require('./...')
  const esmImportPattern = /import\s+(?:\{([^}]+)\}\s+from\s+)?["']([^"']+)["']/g;
  const requirePattern = /require\s*\(\s*["']([^"']+)["']\s*\)/g;
  const luminoIncludePattern = /include\s+["']([^"']+)["']/g;

  // Track adjacency list for circular dependency detection
  const graph = new Map<string, string[]>();
  files.forEach(f => graph.set(f.name, []));

  files.forEach(file => {
    const lines = file.content.split('\n');

    lines.forEach((line, lineIdx) => {
      const lineNum = lineIdx + 1;
      let match: RegExpExecArray | null;

      // Check ESM imports
      const esmRegex = new RegExp(esmImportPattern);
      while ((match = esmRegex.exec(line)) !== null) {
        const symbols = match[1] ? match[1].split(',').map(s => s.trim()) : [];
        const rawTarget = match[2];
        processImportReference(file, lineNum, rawTarget, symbols, match[0], fileMap, linkageEdges, graph, importedFileCounts, findings);
      }

      // Check CommonJS require
      const reqRegex = new RegExp(requirePattern);
      while ((match = reqRegex.exec(line)) !== null) {
        const rawTarget = match[1];
        processImportReference(file, lineNum, rawTarget, [], match[0], fileMap, linkageEdges, graph, importedFileCounts, findings);
      }

      // Check Lumino include
      const incRegex = new RegExp(luminoIncludePattern);
      while ((match = incRegex.exec(line)) !== null) {
        const rawTarget = match[1];
        processImportReference(file, lineNum, rawTarget, [], match[0], fileMap, linkageEdges, graph, importedFileCounts, findings);
      }
    });
  });

  // Circular Dependency Check (Tarjan's / DFS)
  const circularDeps: CircularDependency[] = [];
  detectCircularDependencies(graph, circularDeps, findings);

  // Identify Orphaned Files
  const orphanedFiles: string[] = [];
  files.forEach(f => {
    const count = importedFileCounts.get(f.name) || 0;
    const isMainOrEntry = f.name === 'main.lm' || f.name === 'index.html' || f.name.includes('config') || f.name.includes('server') || f.name.includes('App');
    const isDoc = f.name.endsWith('.md') || f.name.endsWith('.wiki') || f.name.endsWith('.json');
    const isOrphan = count === 0 && !isMainOrEntry && !isDoc;

    if (isOrphan) {
      orphanedFiles.push(f.name);
      findings.push({
        id: `link-orphan-${f.name}`,
        category: 'linkage',
        ruleId: 'LINK-002',
        title: `Verwaiste Datei im Workspace ('${f.name}')`,
        description: `Datei '${f.name}' wird von keiner anderen Datei im Projekt importiert oder referenziert.`,
        severity: 'low',
        fileName: f.name,
        line: 1,
        match: f.name,
        recommendation: `Prüfen Sie, ob '${f.name}' in 'main.lm' eingebunden oder archiviert werden soll.`,
        impact: 'Toter Code im Repository, der die Build-Größe unnötig vergrößert.',
        autoFixable: true,
        autoFix: {
          label: `In 'main.lm' importieren`,
          action: 'insert_line_after',
          targetFile: 'main.lm',
          line: 1,
          replacement: `import "./${f.name}";`,
        }
      });
    }

    linkageNodes.push({
      id: f.name,
      name: f.name,
      extension: f.name.split('.').pop() || '',
      size: f.content.length,
      isEntry: isMainOrEntry,
      isOrphaned: isOrphan,
      linesCount: f.content.split('\n').length,
    });
  });

  return { linkageNodes, linkageEdges, circularDeps, orphanedFiles };
}

function processImportReference(
  sourceFile: FileState,
  lineNum: number,
  rawTarget: string,
  symbols: string[],
  fullMatch: string,
  fileMap: Map<string, FileState>,
  linkageEdges: LinkageEdge[],
  graph: Map<string, string[]>,
  importedFileCounts: Map<string, number>,
  findings: AuditFinding[]
) {
  // Normalize target filename
  const cleanTarget = normalizePath(rawTarget);
  // Look for exact match or extension-appended match
  let resolvedTarget: string | null = null;

  if (fileMap.has(cleanTarget)) {
    resolvedTarget = cleanTarget;
  } else {
    // Try extensions: .lm, .atc, .ts, .js, .json
    for (const ext of ['.lm', '.atc', '.ts', '.tsx', '.js', '.json', '.vx']) {
      if (fileMap.has(`${cleanTarget}${ext}`)) {
        resolvedTarget = `${cleanTarget}${ext}`;
        break;
      }
    }
  }

  // Check if target is external library (e.g. 'express', 'react')
  const isExternalPackage = !rawTarget.startsWith('.') && !rawTarget.includes('/') && !resolvedTarget;

  if (resolvedTarget) {
    importedFileCounts.set(resolvedTarget, (importedFileCounts.get(resolvedTarget) || 0) + 1);
    linkageEdges.push({
      from: sourceFile.name,
      to: resolvedTarget,
      importStatement: fullMatch,
      line: lineNum,
      resolved: true,
      symbols,
    });

    const currentTargets = graph.get(sourceFile.name) || [];
    if (!currentTargets.includes(resolvedTarget)) {
      currentTargets.push(resolvedTarget);
      graph.set(sourceFile.name, currentTargets);
    }

    // Verify imported symbols exist in target file
    if (symbols.length > 0) {
      const targetContent = fileMap.get(resolvedTarget)?.content || '';
      symbols.forEach(sym => {
        const hasExport = targetContent.includes(`export const ${sym}`) ||
                          targetContent.includes(`export fn ${sym}`) ||
                          targetContent.includes(`export function ${sym}`) ||
                          targetContent.includes(`export let ${sym}`) ||
                          targetContent.includes(`export ${sym}`);
        if (!hasExport) {
          findings.push({
            id: `link-missingsym-${sourceFile.name}-${sym}`,
            category: 'linkage',
            ruleId: 'LINK-003',
            title: `Nicht-exportiertes Symbol '${sym}' importiert`,
            description: `Datei '${sourceFile.name}' versucht '${sym}' aus '${resolvedTarget}' zu importieren, aber '${sym}' wird dort nicht exportiert.`,
            severity: 'high',
            fileName: sourceFile.name,
            line: lineNum,
            match: sym,
            recommendation: `Stellen Sie sicher, dass '${resolvedTarget}' den Bezeichner 'export const ${sym} = ...;' enthält.`,
            impact: 'Laufzeitfehler "Symbol not found" oder Import-Fehlschlag beim Modul-Laden.',
            autoFixable: true,
            autoFix: {
              label: `Symbol-Export in '${resolvedTarget}' anlegen`,
              action: 'insert_line_after',
              targetFile: resolvedTarget,
              line: 1,
              replacement: `export const ${sym} = null;`,
            }
          });
        }
      });
    }
  } else if (!isExternalPackage) {
    // UNRESOLVED IMPORT
    linkageEdges.push({
      from: sourceFile.name,
      to: cleanTarget,
      importStatement: fullMatch,
      line: lineNum,
      resolved: false,
      symbols,
    });

    findings.push({
      id: `link-unresolved-${sourceFile.name}-${lineNum}`,
      category: 'linkage',
      ruleId: 'LINK-001',
      title: `Nicht auflösbare Modul-Referenz ('${cleanTarget}')`,
      description: `Datei '${sourceFile.name}' importiert '${cleanTarget}', welche im aktuellen Workspace nicht existiert.`,
      severity: 'critical',
      fileName: sourceFile.name,
      line: lineNum,
      match: fullMatch,
      recommendation: `Erstellen Sie die Datei '${cleanTarget}' im Workspace oder korrigieren Sie den Import-Pfad.`,
      impact: 'Build-Abbruch, ModuleNotFoundError oder Dead Link.',
      autoFixable: true,
      autoFix: {
        label: `Datei '${cleanTarget}' automatisch anlegen`,
        action: 'create_file',
        targetFile: cleanTarget,
        line: 1,
        newFileContent: `// Modul: ${cleanTarget}\n// Automatisch generiert durch Lumino Linkage-Audit\n\nexport const VERSION = "1.0.0";\n\nexport fn init() {\n    print("Modul ${cleanTarget} geladen.");\n    return true;\n}\n`,
      }
    });
  }
}

function detectCircularDependencies(
  graph: Map<string, string[]>,
  circularDeps: CircularDependency[],
  findings: AuditFinding[]
) {
  const visited = new Set<string>();
  const recStack = new Set<string>();
  const path: string[] = [];

  function dfs(node: string) {
    visited.add(node);
    recStack.add(node);
    path.push(node);

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      if (!visited.has(neighbor)) {
        dfs(neighbor);
      } else if (recStack.has(neighbor)) {
        // Cycle found
        const cycleStartIndex = path.indexOf(neighbor);
        const cycle = path.slice(cycleStartIndex).concat(neighbor);
        const cycleStr = cycle.join(' ➔ ');

        circularDeps.push({
          cycle,
          description: `Zyklische Abhängigkeit entdeckt: ${cycleStr}`,
        });

        findings.push({
          id: `link-cycle-${cycle[0]}-${cycle[1]}`,
          category: 'linkage',
          ruleId: 'LINK-004',
          title: `Zyklische Modul-Abhängigkeit (Circular Dependency)`,
          description: `Zirkulärer Import-Kreis gefunden: ${cycleStr}. Führt zu unvollständigen Exports und Lade-Blockaden.`,
          severity: 'high',
          fileName: cycle[0],
          line: 1,
          match: cycleStr,
          recommendation: 'Gemeinsame Typen oder Konstanten in ein separates Basis-Modul auslagern.',
          impact: 'Unvollständige Initialisierung und unvorhersehbare Laufzeit-Nullwerte.',
          autoFixable: false,
        });
      }
    }

    path.pop();
    recStack.delete(node);
  }

  for (const node of graph.keys()) {
    if (!visited.has(node)) {
      dfs(node);
    }
  }
}

// =========================================================================
// 5. AUTO-REMEDIATION (Verbesserungen automatisch integrieren)
// =========================================================================

/**
 * Applies a single finding's automatic fix across the files array
 */
export function applyFindingAutoFix(files: FileState[], finding: AuditFinding): FileState[] {
  if (!finding.autoFixable || !finding.autoFix) return files;
  const fix = finding.autoFix;

  // Case 1: Create a missing file (Resolves Unresolved Imports)
  if (fix.action === 'create_file') {
    if (files.some(f => f.name === fix.targetFile)) return files;
    return [
      ...files,
      {
        name: fix.targetFile,
        content: fix.newFileContent || `// ${fix.targetFile}\nexport const OK = true;\n`,
        iconColor: fix.targetFile.endsWith('.lm') ? 'text-cyan-400' : 'text-emerald-400',
        iconShape: '◆',
      }
    ];
  }

  // Case 2: Modify existing file
  return files.map(file => {
    if (file.name !== fix.targetFile) return file;
    const lines = file.content.split('\n');
    const targetIdx = fix.line - 1;

    if (fix.action === 'replace_match' && fix.match && fix.replacement !== undefined) {
      if (lines[targetIdx]) {
        lines[targetIdx] = lines[targetIdx].replace(fix.match, fix.replacement);
      }
    } else if (fix.action === 'insert_line_after' && fix.replacement !== undefined) {
      if (targetIdx >= 0 && targetIdx < lines.length) {
        lines.splice(targetIdx + 1, 0, fix.replacement);
      } else {
        lines.push(fix.replacement);
      }
    } else if (fix.action === 'remove_line') {
      if (targetIdx >= 0 && targetIdx < lines.length) {
        lines.splice(targetIdx, 1);
      }
    }

    return {
      ...file,
      content: lines.join('\n'),
    };
  });
}

/**
 * Automatically applies all auto-fixable findings across the workspace in batch
 */
export function applyAllAutoFixes(
  files: FileState[],
  findings: AuditFinding[]
): { updatedFiles: FileState[]; fixedCount: number } {
  let currentFiles = [...files];
  let fixedCount = 0;

  const fixable = findings.filter(f => f.autoFixable && f.autoFix);

  for (const finding of fixable) {
    currentFiles = applyFindingAutoFix(currentFiles, finding);
    fixedCount++;
  }

  return { updatedFiles: currentFiles, fixedCount };
}

// =========================================================================
// 6. LIVE TODO-LIST SCANNER & AUTO-UPDATE
// =========================================================================

/**
 * Extracts all TODOs from source code comments AND from high-severity audit findings
 */
export function extractProjectTodos(files: FileState[], findings: AuditFinding[] = []): TodoItem[] {
  const todos: TodoItem[] = [];
  const todoRegex = /\/\/\s*(TODO|FIXME|BUG|HACK|REVIEW|AUDIT)(?:\s*[:\-])?\s*(.*)$/i;

  files.forEach(file => {
    // Skip TODO.md itself to avoid feedback loop
    if (file.name === 'TODO.md') return;

    const lines = file.content.split('\n');
    lines.forEach((line, idx) => {
      const match = line.match(todoRegex);
      if (match) {
        const tag = match[1].toUpperCase() as TodoItem['tag'];
        const text = match[2].trim() || 'Keine Beschreibung angegeben';
        const isDone = text.startsWith('[DONE]') || text.startsWith('[x]') || text.startsWith('[X]');
        const cleanText = text.replace(/^\[(?:DONE|x|X)\]\s*/, '');

        let priority: TodoItem['priority'] = 'medium';
        if (tag === 'BUG' || tag === 'FIXME') priority = 'high';
        else if (tag === 'HACK') priority = 'low';

        todos.push({
          id: `todo-${file.name}-${idx + 1}`,
          fileName: file.name,
          line: idx + 1,
          tag,
          text: cleanText,
          priority,
          completed: isDone,
          source: 'code_comment',
        });
      }
    });
  });

  // Promote critical / high audit findings into actionable TODOs
  findings.forEach(f => {
    if (f.severity === 'critical' || f.severity === 'high') {
      todos.push({
        id: `todo-audit-${f.id}`,
        fileName: f.fileName,
        line: f.line,
        tag: 'AUDIT',
        text: `[${f.ruleId}] ${f.title}: ${f.recommendation}`,
        priority: f.severity === 'critical' ? 'critical' : 'high',
        completed: false,
        source: 'audit_finding',
        findingId: f.id,
      });
    }
  });

  return todos;
}

/**
 * Toggles a TODO status in the source code file (e.g. marks [DONE])
 */
export function toggleTodoStatusInFile(files: FileState[], todo: TodoItem): FileState[] {
  if (todo.source !== 'code_comment') return files;

  return files.map(file => {
    if (file.name !== todo.fileName) return file;
    const lines = file.content.split('\n');
    const idx = todo.line - 1;
    if (!lines[idx]) return file;

    const currentLine = lines[idx];
    if (todo.completed) {
      // Uncheck it: remove [DONE]
      lines[idx] = currentLine.replace(/\[DONE\]\s*/i, '');
    } else {
      // Check it: add [DONE]
      lines[idx] = currentLine.replace(/(TODO|FIXME|BUG|HACK|REVIEW)(?:\s*[:\-])?\s*/i, '$1: [DONE] ');
    }

    return {
      ...file,
      content: lines.join('\n'),
    };
  });
}

/**
 * Generates formatted Markdown representation for TODO.md
 */
export function generateTodoMarkdown(todos: TodoItem[]): string {
  const pending = todos.filter(t => !t.completed);
  const done = todos.filter(t => t.completed);

  let md = `# Projekt TODO-Liste (Automatisch Synchronisiert)\n\n`;
  md += `*Generiert durch ATOS Live Audit & Task Engine | Zuletzt aktualisiert: ${new Date().toLocaleTimeString()}*\n\n`;
  md += `### Status-Übersicht\n`;
  md += `- Offene Aufgaben: **${pending.length}**\n`;
  md += `- Erledigte Aufgaben: **${done.length}**\n`;
  md += `- Gesamtzahl: **${todos.length}**\n\n`;

  md += `## 📌 Offene Aufgaben\n\n`;
  if (pending.length === 0) {
    md += `*Keine offenen Aufgaben. Alles erledigt! 🎉*\n\n`;
  } else {
    pending.forEach(t => {
      const icon = t.priority === 'critical' ? '🔴' : t.priority === 'high' ? '🟠' : t.priority === 'medium' ? '🟡' : '🟢';
      md += `- [ ] ${icon} **[${t.tag}]** \`${t.fileName}:${t.line}\` - ${t.text}\n`;
    });
    md += `\n`;
  }

  if (done.length > 0) {
    md += `## ✅ Erledigte Aufgaben\n\n`;
    done.forEach(t => {
      md += `- [x] ~**[${t.tag}]** \`${t.fileName}:${t.line}\` - ${t.text}~\n`;
    });
    md += `\n`;
  }

  return md;
}

/**
 * Updates or creates TODO.md in the files workspace
 */
export function syncTodoFileInWorkspace(files: FileState[], todos: TodoItem[]): FileState[] {
  const content = generateTodoMarkdown(todos);
  const existingIdx = files.findIndex(f => f.name === 'TODO.md');

  if (existingIdx >= 0) {
    const copy = [...files];
    copy[existingIdx] = { ...copy[existingIdx], content };
    return copy;
  }

  return [
    ...files,
    {
      name: 'TODO.md',
      content,
      iconColor: 'text-amber-400',
      iconShape: '✓',
    }
  ];
}

// =========================================================================
// 7. LIVE ARCHITECTURE WIKI GENERATOR & AUTO-UPDATE
// =========================================================================

/**
 * Extracts dynamic project architecture info from workspace files
 */
export function generateLiveArchitectureDocs(
  files: FileState[],
  audit: ProjectAuditSummary
): LiveArchitectureDoc {
  const modules: LiveArchitectureModule[] = [];

  files.forEach(f => {
    let role: LiveArchitectureModule['role'] = 'Utility';
    if (f.name.includes('core') || f.name.includes('kernel') || f.name.includes('main')) {
      role = 'Core / Kernel';
    } else if (f.name.includes('token') || f.name.includes('treasury') || f.name.includes('ateco')) {
      role = 'Smart Contract / Token';
    } else if (f.name.includes('network') || f.name.includes('protocol') || f.name.includes('server')) {
      role = 'Networking / Protocol';
    } else if (f.name.includes('ui') || f.name.includes('index') || f.name.includes('App') || f.name.endsWith('.tsx')) {
      role = 'User Interface';
    } else if (f.name.endsWith('.json') || f.name.includes('config')) {
      role = 'Configuration';
    } else if (f.name.endsWith('.md') || f.name.endsWith('.wiki')) {
      role = 'Documentation';
    }

    // Extract exports
    const exports: string[] = [];
    const exportMatches = f.content.matchAll(/export\s+(?:const|fn|function|let)\s+([a-zA-Z_]\w*)/g);
    for (const em of exportMatches) {
      exports.push(em[1]);
    }

    // Extract imports
    const imports: string[] = [];
    const importMatches = f.content.matchAll(/(?:import\s+(?:\{[^}]+\}\s+from\s+)?|include\s+)["']([^"']+)["']/g);
    for (const im of importMatches) {
      imports.push(im[1]);
    }

    // Determine security rating based on audit findings for this file
    const fileFindings = audit.findings.filter(item => item.fileName === f.name);
    const hasCritOrHigh = fileFindings.some(item => item.severity === 'critical' || item.severity === 'high');
    const hasMedOrLow = fileFindings.some(item => item.severity === 'medium' || item.severity === 'low');

    let securityRating: LiveArchitectureModule['securityRating'] = 'Secure';
    if (hasCritOrHigh) securityRating = 'Vulnerable';
    else if (hasMedOrLow) securityRating = 'Needs Review';

    modules.push({
      fileName: f.name,
      role,
      exports,
      imports,
      description: `Modul '${f.name}' mit ${f.content.split('\n').length} Zeilen und ${exports.length} öffentlichen Exports.`,
      lines: f.content.split('\n').length,
      securityRating,
    });
  });

  const layers = [
    {
      name: 'Layer 0: Core Nucleus & Kernel',
      description: 'Laufzeitbasis, Scheduler, Hardware-Abstraktionsschicht und Speicherverwaltung.',
      files: modules.filter(m => m.role === 'Core / Kernel').map(m => m.fileName),
    },
    {
      name: 'Layer 1: Consensus & Tokenomics',
      description: 'A-TownChain Tokenverträge, Transaktionsverifikation und Treasury-Regeln.',
      files: modules.filter(m => m.role === 'Smart Contract / Token').map(m => m.fileName),
    },
    {
      name: 'Layer 2: Network & Protocol',
      description: 'P2P-Nachrichtenweiterleitung, WebSocket-Sync und RPC-Gateways.',
      files: modules.filter(m => m.role === 'Networking / Protocol').map(m => m.fileName),
    },
    {
      name: 'Layer 3: Userland & Interfaces',
      description: 'Frontend-Komponenten, IDE-Views, Dashboard und Benutzerinteraktion.',
      files: modules.filter(m => m.role === 'User Interface').map(m => m.fileName),
    },
    {
      name: 'Layer 4: Config & Environment',
      description: 'Systemkonfigurationen, JSON-Manifeste und Runtime-Parameter.',
      files: modules.filter(m => m.role === 'Configuration' || m.role === 'Documentation').map(m => m.fileName),
    }
  ];

  // Generate ASCII dependency diagram
  let asciiDiagram = `
+-------------------------------------------------------------------+
|                   ATOS LIVE ARCHITEKTUR-DIAGRAMM                  |
+-------------------------------------------------------------------+
| Health-Score: ${audit.healthScore}/100 (Grade ${audit.grade})  |  Module: ${files.length}  |  Befunde: ${audit.totalFindings}  |
+-------------------------------------------------------------------+

`;

  layers.forEach(layer => {
    asciiDiagram += `[ ${layer.name} ]\n`;
    if (layer.files.length === 0) {
      asciiDiagram += `  └─ (Keine Dateien zugewiesen)\n`;
    } else {
      layer.files.forEach((f, idx) => {
        const isLast = idx === layer.files.length - 1;
        const branch = isLast ? '└─' : '├─';
        const mod = modules.find(m => m.fileName === f);
        const secIcon = mod?.securityRating === 'Secure' ? '🛡️ [OK]' : mod?.securityRating === 'Needs Review' ? '⚠️ [REV]' : '🚨 [VULN]';
        const exportStr = mod?.exports.length ? `(exports: ${mod.exports.slice(0, 3).join(', ')})` : '';
        asciiDiagram += `  ${branch} ${f.padEnd(20)} ${secIcon} ${exportStr}\n`;
      });
    }
    asciiDiagram += `        │\n        ▼\n`;
  });

  asciiDiagram += `[ Abschluss / Runtime Gateway ]\n`;

  return {
    projectName: 'ATOS & Lumino Runtime',
    version: '2.5.0-live',
    overview: `Automatisch generierte Architektur-Dokumentation basierend auf ${files.length} Dateien im Workspace. Das Projekt erzielt einen aktuellen System-Health-Score von ${audit.healthScore}/100 (${audit.grade}).`,
    modules,
    layers,
    dependencySummary: `${audit.linkageGraph.edges.length} aktive Modulverknüpfungen analysiert. ${audit.orphanedFiles.length} verwaiste Dateien, ${audit.circularDeps.length} zyklische Abhängigkeiten.`,
    securitySummary: `${audit.byCategory.security} Sicherheitsbefunde (${audit.criticalCount} kritisch, ${audit.highCount} hoch).`,
    asciiDiagram,
    lastUpdated: new Date().toLocaleTimeString(),
  };
}

/**
 * Formats dynamic architecture docs as clean Markdown for ARCHITECTURE.md
 */
export function generateArchitectureMarkdown(doc: LiveArchitectureDoc): string {
  let md = `# ${doc.projectName} - Systemarchitektur & Spezifikation\n\n`;
  md += `> **Live-Status:** Automatisch generiert | Letzte Aktualisierung: ${doc.lastUpdated} | Version: ${doc.version}\n\n`;

  md += `## 1. System-Übersicht\n${doc.overview}\n\n`;

  md += `## 2. Architektur-Schichten (Layer Model)\n\n`;
  doc.layers.forEach(layer => {
    md += `### ${layer.name}\n`;
    md += `${layer.description}\n\n`;
    if (layer.files.length > 0) {
      md += `**Zugehörige Dateien:**\n`;
      layer.files.forEach(f => {
        md += `- \`${f}\`\n`;
      });
    } else {
      md += `*Keine Dateien in diesem Layer.*\n`;
    }
    md += `\n`;
  });

  md += `## 3. Architektur-Diagramm\n\n\`\`\`text\n${doc.asciiDiagram}\n\`\`\`\n\n`;

  md += `## 4. Modul-Katalog & Schnittstellen\n\n`;
  md += `| Modul | Rolle | Zeilen | Exports | Sicherheitsstatus |\n`;
  md += `| :--- | :--- | :--- | :--- | :--- |\n`;
  doc.modules.forEach(m => {
    const exportsStr = m.exports.length > 0 ? m.exports.slice(0, 3).join(', ') + (m.exports.length > 3 ? '...' : '') : '-';
    md += `| \`${m.fileName}\` | ${m.role} | ${m.lines} | \`${exportsStr}\` | ${m.securityRating} |\n`;
  });
  md += `\n`;

  md += `## 5. Audit & Verknüpfungs-Metriken\n`;
  md += `- **Verknüpfungen:** ${doc.dependencySummary}\n`;
  md += `- **Sicherheitslage:** ${doc.securitySummary}\n`;

  return md;
}

/**
 * Synchronizes ARCHITECTURE.md into the files workspace
 */
export function syncWikiFileInWorkspace(files: FileState[], doc: LiveArchitectureDoc): FileState[] {
  const content = generateArchitectureMarkdown(doc);
  const existingIdx = files.findIndex(f => f.name === 'ARCHITECTURE.md');

  if (existingIdx >= 0) {
    const copy = [...files];
    copy[existingIdx] = { ...copy[existingIdx], content };
    return copy;
  }

  return [
    ...files,
    {
      name: 'ARCHITECTURE.md',
      content,
      iconColor: 'text-cyan-400',
      iconShape: '🏛️',
    }
  ];
}
