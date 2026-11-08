export type LintSeverity = "warning" | "error";

export type LintPatternType = 
  | "regex" 
  | "keyword" 
  | "max_line_length" 
  | "naming_convention" 
  | "missing_pattern";

export interface CustomLintRule {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  severity: LintSeverity;
  category: "Style" | "Best Practices" | "Security" | "Performance" | "Formatting" | "Custom";
  patternType: LintPatternType;
  
  // Regex options
  pattern?: string;
  flags?: string;
  
  // Keyword options
  keyword?: string;
  
  // Line length options
  maxLength?: number;
  
  // Naming convention options
  targetEntity?: "variable" | "function" | "constant";
  convention?: "camelCase" | "snake_case" | "UPPER_CASE";
  
  // Missing pattern options (e.g. requires preceding doc comment for function)
  matchPattern?: string;
  requirePattern?: string;
  
  // Output message
  errorMessage: string;
  
  // Quick Fix
  quickFix?: {
    label: string;
    actionType: "replace" | "delete" | "trim_trailing" | "custom_replace";
    replacement?: string;
  };
  
  isPreset?: boolean;
}

export interface LintQuickFix {
  label: string;
  action: string;
  replacement?: string;
  ruleId?: string;
}

export interface CustomLintError {
  line: number;
  message: string;
  type: LintSeverity;
  match: string;
  quickFix?: LintQuickFix;
  ruleId?: string;
  ruleName?: string;
}

export const DEFAULT_LINT_RULES: CustomLintRule[] = [
  {
    id: "rule-no-var",
    name: "Verbotenes 'var'-Keyword",
    description: "In Lumino sollte ausschließlich 'let' oder 'const' für Variablendeklarationen genutzt werden.",
    enabled: true,
    severity: "error",
    category: "Best Practices",
    patternType: "regex",
    pattern: "\\bvar\\s+([a-zA-Z_]\\w*)",
    flags: "g",
    errorMessage: "Veraltetes Schlüsselwort 'var' erkannt. Nutze 'let' oder 'const'.",
    quickFix: {
      label: "Ersetze 'var' durch 'let'",
      actionType: "replace",
      replacement: "let "
    },
    isPreset: true,
  },
  {
    id: "rule-const-uppercase",
    name: "Konstanten in UPPER_CASE",
    description: "Konstanten sollten konventionsgemäß in GROSSBUCHSTABEN definiert werden.",
    enabled: true,
    severity: "warning",
    category: "Style",
    patternType: "naming_convention",
    targetEntity: "constant",
    convention: "UPPER_CASE",
    errorMessage: "Konstante entspricht nicht der Konvention 'UPPER_CASE'.",
    quickFix: {
      label: "In UPPER_CASE umwandeln",
      actionType: "replace",
    },
    isPreset: true,
  },
  {
    id: "rule-no-todo-fixme",
    name: "Ungelöste TODO & FIXME Kommentare",
    description: "Markiert unvollständige Code-Stellen und Arbeitsmerker vor dem Deployment.",
    enabled: true,
    severity: "warning",
    category: "Best Practices",
    patternType: "regex",
    pattern: "\\/\\/\\s*(TODO|FIXME|HACK|BUG):?",
    flags: "i",
    errorMessage: "Ungelöster Merker (TODO/FIXME) im Code gefunden.",
    isPreset: true,
  },
  {
    id: "rule-max-line-length",
    name: "Maximale Zeilenlänge (120 Zeichen)",
    description: "Verhindert übermäßig lange Zeilen zur Wahrung der Lesbarkeit auf Standard-Displays.",
    enabled: true,
    severity: "warning",
    category: "Style",
    patternType: "max_line_length",
    maxLength: 120,
    errorMessage: "Zeile überschreitet die empfohlene Maximallänge von 120 Zeichen.",
    isPreset: true,
  },
  {
    id: "rule-no-trailing-whitespace",
    name: "Keine nachgestellten Leerzeichen",
    description: "Entfernt überflüssige Leerzeichen am Zeilenende zur Vermeidung von Git-Diff-Konflikten.",
    enabled: true,
    severity: "warning",
    category: "Formatting",
    patternType: "regex",
    pattern: "[ \\t]+$",
    flags: "g",
    errorMessage: "Überflüssiges Whitespace am Zeilenende gefunden.",
    quickFix: {
      label: "Leerzeichen am Zeilenende entfernen",
      actionType: "trim_trailing",
    },
    isPreset: true,
  },
  {
    id: "rule-no-empty-blocks",
    name: "Keine leeren Code-Blöcke",
    description: "Leere Blockklammern '{}' ohne Statements deuten auf unfertige Logik hin.",
    enabled: true,
    severity: "warning",
    category: "Best Practices",
    patternType: "regex",
    pattern: "\\{\\s*\\}",
    flags: "g",
    errorMessage: "Leerer Code-Block '{}' gefunden.",
    isPreset: true,
  },
  {
    id: "rule-restrict-unsafe",
    name: "Sicherheits-Audit: Unsafe & Asm blockieren",
    description: "Verhindert die versehentliche Ausführung von Low-Level Kernel-Befehlen im Anwendungs-Code.",
    enabled: true,
    severity: "error",
    category: "Security",
    patternType: "keyword",
    keyword: "unsafe",
    errorMessage: "Nutzung von 'unsafe' ist in diesem Modul restriktiv blockiert.",
    isPreset: true,
  },
  {
    id: "rule-no-magic-numbers",
    name: "Keine Magic Numbers in Vergleichen",
    description: "Zahlenkonstanten größer 100 in Vergleichen sollten als sprechende Konstanten deklariert werden.",
    enabled: false,
    severity: "warning",
    category: "Style",
    patternType: "regex",
    pattern: "(?:===?|!==?|<|>|<=|>=)\\s*([1-9]\\d{2,})\\b",
    flags: "g",
    errorMessage: "Unbenannte Zahl (Magic Number) im Vergleich gefunden. Erwäge eine 'const'-Deklaration.",
    isPreset: true,
  },
  {
    id: "rule-prefer-triple-equals",
    name: "Strikte Gleichheit bevorzugen",
    description: "Ermutigt zur Nutzung von '===' anstelle von ungenauem '==' zur Vermeidung von Type-Coercion-Bugs.",
    enabled: false,
    severity: "warning",
    category: "Best Practices",
    patternType: "regex",
    pattern: "[^!=<>]={2}[^=]",
    flags: "g",
    errorMessage: "Loses '==' gefunden. Nutze '===' für strikten Typenvergleich.",
    quickFix: {
      label: "Ersetze '==' durch '==='",
      actionType: "replace",
      replacement: "==="
    },
    isPreset: true,
  },
  {
    id: "rule-require-fn-docs",
    name: "Funktions-Dokumentation erforderlich",
    description: "Funktionsdefinitionen sollten mit einem führenden Kommentar '//' dokumentiert werden.",
    enabled: false,
    severity: "warning",
    category: "Style",
    patternType: "missing_pattern",
    matchPattern: "^\\s*fn\\s+[a-zA-Z_]\\w*",
    requirePattern: "^\\s*\\/\\/",
    errorMessage: "Funktion hat keinen vorangestellten Dokumentationskommentar.",
    isPreset: true,
  }
];

// Helper functions for naming conventions
export function toCamelCase(str: string): string {
  return str
    .toLowerCase()
    .replace(/[^a-zA-Z0-9]+(.)/g, (_, chr) => chr.toUpperCase());
}

export function toSnakeCase(str: string): string {
  return str
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/[-\s]+/g, "_")
    .toLowerCase();
}

export function toUpperSnakeCase(str: string): string {
  return toSnakeCase(str).toUpperCase();
}

function escapeRegExp(string: string): string {
  return string.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/**
 * Executes custom linting rules on Lumino source code.
 */
export function evaluateCustomLintRules(
  code: string,
  rules: CustomLintRule[]
): CustomLintError[] {
  const errors: CustomLintError[] = [];
  if (!code || !rules || rules.length === 0) return errors;

  const lines = code.split("\n");
  const activeRules = rules.filter((r) => r.enabled);

  for (const rule of activeRules) {
    try {
      if (rule.patternType === "regex" && rule.pattern) {
        let flags = rule.flags || "g";
        if (!flags.includes("g")) flags += "g";
        let rx: RegExp;
        try {
          rx = new RegExp(rule.pattern, flags);
        } catch (e) {
          console.warn(`Ungültige Regex in Lint-Regel '${rule.name}':`, e);
          continue;
        }

        lines.forEach((lineText, lineIdx) => {
          rx.lastIndex = 0;
          let match: RegExpExecArray | null;
          let matchCount = 0;
          while ((match = rx.exec(lineText)) !== null && matchCount < 30) {
            matchCount++;
            const matchStr = match[0];
            if (!matchStr) {
              rx.lastIndex++;
              continue;
            }

            let quickFix = undefined;
            if (rule.quickFix) {
              let replacement = rule.quickFix.replacement;
              if (rule.quickFix.actionType === "trim_trailing") {
                replacement = "";
              }
              quickFix = {
                label: rule.quickFix.label,
                action: rule.quickFix.actionType === "trim_trailing" ? "trim_trailing" : "replace_match",
                replacement,
                ruleId: rule.id
              };
            }

            errors.push({
              line: lineIdx,
              message: `[${rule.name}] ${rule.errorMessage || "Lint-Verletzung"}`,
              type: rule.severity,
              match: matchStr,
              quickFix,
              ruleId: rule.id,
              ruleName: rule.name,
            });

            if (!flags.includes("g")) break;
          }
        });
      } else if (rule.patternType === "keyword" && rule.keyword) {
        const kw = rule.keyword.trim();
        if (!kw) continue;
        const kwRx = new RegExp(`\\b${escapeRegExp(kw)}\\b`, "g");

        lines.forEach((lineText, lineIdx) => {
          kwRx.lastIndex = 0;
          let match: RegExpExecArray | null;
          while ((match = kwRx.exec(lineText)) !== null) {
            errors.push({
              line: lineIdx,
              message: `[${rule.name}] ${rule.errorMessage || `Verbotenes Keyword '${kw}' gefunden`}`,
              type: rule.severity,
              match: match[0],
              ruleId: rule.id,
              ruleName: rule.name,
            });
          }
        });
      } else if (rule.patternType === "max_line_length") {
        const maxLen = rule.maxLength || 120;
        lines.forEach((lineText, lineIdx) => {
          if (lineText.length > maxLen) {
            const excess = lineText.substring(maxLen);
            errors.push({
              line: lineIdx,
              message: `[${rule.name}] ${rule.errorMessage || `Zeile zu lang (${lineText.length}/${maxLen} Zeichen)`}`,
              type: rule.severity,
              match: excess.trim() || lineText.trim().slice(-10) || " ",
              ruleId: rule.id,
              ruleName: rule.name,
            });
          }
        });
      } else if (rule.patternType === "naming_convention" && rule.convention) {
        const target = rule.targetEntity || "variable";
        const conv = rule.convention;

        lines.forEach((lineText, lineIdx) => {
          let idRegex: RegExp | null = null;
          if (target === "constant") {
            idRegex = /\bconst\s+([a-zA-Z_]\w*)/g;
          } else if (target === "variable") {
            idRegex = /\b(?:let|const)\s+([a-zA-Z_]\w*)/g;
          } else if (target === "function") {
            idRegex = /\bfn\s+([a-zA-Z_]\w*)/g;
          }

          if (idRegex) {
            let m: RegExpExecArray | null;
            while ((m = idRegex.exec(lineText)) !== null) {
              const ident = m[1];
              let isCompliant = true;
              let suggested = ident;

              if (conv === "UPPER_CASE") {
                isCompliant = /^[A-Z][A-Z0-9_]*$/.test(ident);
                suggested = toUpperSnakeCase(ident);
              } else if (conv === "camelCase") {
                isCompliant = /^[a-z][a-zA-Z0-9]*$/.test(ident);
                suggested = toCamelCase(ident);
              } else if (conv === "snake_case") {
                isCompliant = /^[a-z][a-z0-9_]*$/.test(ident);
                suggested = toSnakeCase(ident);
              }

              if (!isCompliant) {
                errors.push({
                  line: lineIdx,
                  message: `[${rule.name}] ${rule.errorMessage || `Identifier '${ident}' entspricht nicht '${conv}'.`}`,
                  type: rule.severity,
                  match: ident,
                  quickFix: {
                    label: `In ${conv} konvertieren ('${suggested}')`,
                    action: "replace_match",
                    replacement: suggested,
                    ruleId: rule.id
                  },
                  ruleId: rule.id,
                  ruleName: rule.name,
                });
              }
            }
          }
        });
      } else if (rule.patternType === "missing_pattern" && rule.matchPattern) {
        try {
          const matchRx = new RegExp(rule.matchPattern);
          const reqRx = rule.requirePattern ? new RegExp(rule.requirePattern) : null;

          lines.forEach((lineText, lineIdx) => {
            if (matchRx.test(lineText)) {
              // Check preceding non-empty line
              let foundReq = false;
              if (reqRx) {
                for (let prev = lineIdx - 1; prev >= 0; prev--) {
                  const prevLine = lines[prev].trim();
                  if (!prevLine) continue;
                  if (reqRx.test(prevLine)) {
                    foundReq = true;
                  }
                  break;
                }
              }

              if (!foundReq) {
                errors.push({
                  line: lineIdx,
                  message: `[${rule.name}] ${rule.errorMessage || "Erforderliches Pattern fehlt"}`,
                  type: rule.severity,
                  match: lineText.trim().split(" ")[0] || lineText.trim(),
                  ruleId: rule.id,
                  ruleName: rule.name,
                });
              }
            }
          });
        } catch (err) {
          console.warn(`Fehler in missing_pattern Regel '${rule.name}':`, err);
        }
      }
    } catch (ruleErr) {
      console.error(`Fehler bei Ausführung von Lint-Regel '${rule.name}':`, ruleErr);
    }
  }

  return errors;
}
