//! Repository audit primitives for the organization-wide engineering gate.
//!
//! The scanner is dependency-free and fail-closed. Findings are derived from
//! the checked-out repository tree rather than documentation claims.

use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::{collections::hash_map::DefaultHasher, ffi::OsStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingKind {
    Error,
    Security,
    Consistency,
    Connectivity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub kind: FindingKind,
    pub code: &'static str,
    pub path: Option<PathBuf>,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub findings: Vec<Finding>,
}

impl AuditReport {
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn has_kind(&self, kind: FindingKind) -> bool {
        self.findings.iter().any(|finding| finding.kind == kind)
    }
}

pub fn audit_repository(root: &Path) -> Result<AuditReport, std::io::Error> {
    let mut report = AuditReport::default();
    require_file(root, "README.md", "AUDIT-001", &mut report)?;
    require_file(root, "docs/ENGINEERING_AUDIT.md", "AUDIT-002", &mut report)?;
    audit_readme_identity(root, &mut report)?;
    let mut duplicate_candidates = HashMap::<u64, PathBuf>::new();
    scan_tree(root, root, &mut duplicate_candidates, &mut report)?;
    Ok(report)
}

fn require_file(
    root: &Path,
    relative: &str,
    code: &'static str,
    report: &mut AuditReport,
) -> Result<(), std::io::Error> {
    if !root.join(relative).is_file() {
        report.findings.push(Finding {
            kind: FindingKind::Error,
            code,
            path: Some(PathBuf::from(relative)),
            message: format!("required governance file is missing: {relative}"),
        });
    }
    Ok(())
}

fn audit_readme_identity(root: &Path, report: &mut AuditReport) -> Result<(), std::io::Error> {
    let Some(repo_name) = root.file_name().and_then(OsStr::to_str) else {
        return Ok(());
    };
    let readme = fs::read_to_string(root.join("README.md"))?;
    if !readme
        .to_ascii_lowercase()
        .contains(&repo_name.to_ascii_lowercase())
    {
        report.findings.push(Finding {
            kind: FindingKind::Consistency,
            code: "CONS-AUDIT-002",
            path: Some(PathBuf::from("README.md")),
            message: format!("README does not identify repository `{repo_name}`"),
        });
    }
    Ok(())
}

fn scan_tree(
    root: &Path,
    current: &Path,
    duplicates: &mut HashMap<u64, PathBuf>,
    report: &mut AuditReport,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(current)? {
        let path = entry?.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);
        if relative
            .components()
            .any(|component| component.as_os_str() == ".git")
        {
            continue;
        }
        if path.is_dir() {
            scan_tree(root, &path, duplicates, report)?;
            continue;
        }
        if !is_text_candidate(&path) {
            continue;
        }
        let bytes = fs::read(&path)?;
        let Ok(text) = std::str::from_utf8(&bytes) else {
            continue;
        };
        let archived_or_documentation = is_non_production_path(relative);

        if !archived_or_documentation && contains_secret_pattern(text) {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: "high-confidence credential material pattern detected".into(),
            });
        }
        if !archived_or_documentation
            && contains_dangerous_shell_pattern(path.extension().and_then(OsStr::to_str), text)
        {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-002",
                path: Some(relative.to_path_buf()),
                message: "potentially unsafe command execution/download pattern detected".into(),
            });
        }
        if !archived_or_documentation
            && path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
            && text.contains("[workspace")
            && !root.join("Cargo.lock").is_file()
        {
            report.findings.push(Finding {
                kind: FindingKind::Consistency,
                code: "CONS-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: "Rust workspace has no committed Cargo.lock evidence".into(),
            });
        }
        if !archived_or_documentation
            && path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
        {
            audit_cargo_path_dependencies(root, relative, text, report);
        }
        if !archived_or_documentation
            && relative.starts_with(Path::new(".github/workflows"))
            && text.contains("uses:")
            && !text.contains("permissions:")
        {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-003",
                path: Some(relative.to_path_buf()),
                message:
                    "GitHub Actions workflow uses actions without an explicit permissions policy"
                        .into(),
            });
        }
        let is_source = matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("rs" | "py" | "ts" | "tsx" | "js" | "jsx" | "sh" | "bash")
        );
        if !archived_or_documentation
            && is_source
            && relative != Path::new("docs/ENGINEERING_AUDIT.md")
            && contains_todo_marker(text, comment_syntax_for(relative))
        {
            report.findings.push(Finding {
                kind: FindingKind::Error,
                code: "AUDIT-003",
                path: Some(relative.to_path_buf()),
                message: "unresolved TODO/FIXME marker requires triage".into(),
            });
        }
        if !archived_or_documentation && is_duplicate_candidate(&path) {
            let mut hasher = DefaultHasher::new();
            bytes.hash(&mut hasher);
            let fingerprint = hasher.finish();
            if let Some(first) = duplicates.insert(fingerprint, relative.to_path_buf()) {
                report.findings.push(Finding {
                    kind: FindingKind::Consistency,
                    code: "CONS-AUDIT-003",
                    path: Some(relative.to_path_buf()),
                    message: format!("duplicate file content matches {}", first.display()),
                });
            }
        }
    }
    Ok(())
}

fn is_non_production_path(path: &Path) -> bool {
    let mut components = path.components();
    (matches!(
        components.next(),
        Some(std::path::Component::Normal(name)) if name == "docs"
    ) && matches!(
        components.next(),
        Some(std::path::Component::Normal(name)) if name == "archive" || name == "wiki"
    )) || path.starts_with(Path::new("wiki"))
}

fn audit_cargo_path_dependencies(
    root: &Path,
    relative: &Path,
    text: &str,
    report: &mut AuditReport,
) {
    let manifest_dir = relative.parent().unwrap_or_else(|| Path::new("."));
    for line in text.lines() {
        let Some(path_pos) = line.find("path") else {
            continue;
        };
        let tail = &line[path_pos + 4..];
        let Some(eq) = tail.find('=') else {
            continue;
        };
        let value = tail[eq + 1..].trim();
        let Some(start) = value.find('"') else {
            continue;
        };
        let rest = &value[start + 1..];
        let Some(end) = rest.find('"') else {
            continue;
        };
        let dep_path = &rest[..end];
        if dep_path.is_empty() {
            continue;
        }
        if !root.join(manifest_dir).join(dep_path).exists() {
            report.findings.push(Finding {
                kind: FindingKind::Connectivity,
                code: "CONN-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: format!("Cargo path dependency does not exist: {dep_path}"),
            });
        }
    }
}

enum CommentSyntax {
    SlashAndBlock,
    Hash,
    All,
}

fn comment_syntax_for(path: &Path) -> CommentSyntax {
    match path.extension().and_then(OsStr::to_str) {
        Some("rs" | "c" | "h" | "cpp" | "cc" | "ts" | "tsx" | "js" | "jsx" | "java" | "go") => {
            CommentSyntax::SlashAndBlock
        }
        Some("py" | "pyw" | "sh" | "bash" | "zsh" | "rb" | "toml" | "yml" | "yaml" | "ini") => {
            CommentSyntax::Hash
        }
        _ => CommentSyntax::All,
    }
}

fn contains_todo_marker(text: &str, syntax: CommentSyntax) -> bool {
    let allows_slash = matches!(syntax, CommentSyntax::SlashAndBlock | CommentSyntax::All);
    let allows_hash = matches!(syntax, CommentSyntax::Hash | CommentSyntax::All);
    let mut in_block_comment = false;

    for line in text.lines() {
        let bytes = line.as_bytes();
        let mut comment = String::new();
        let mut index = 0;

        while index < bytes.len() {
            if in_block_comment {
                if index + 1 < bytes.len() && bytes[index] == b'*' && bytes[index + 1] == b'/' {
                    in_block_comment = false;
                    index += 2;
                    continue;
                }
                comment.push(bytes[index] as char);
                index += 1;
                continue;
            }

            if allows_slash
                && index + 1 < bytes.len()
                && bytes[index] == b'/'
                && bytes[index + 1] == b'*'
            {
                in_block_comment = true;
                index += 2;
                continue;
            }

            if allows_slash
                && index + 1 < bytes.len()
                && bytes[index] == b'/'
                && bytes[index + 1] == b'/'
            {
                comment.push_str(&line[index + 2..]);
                break;
            }

            if allows_hash && bytes[index] == b'#' {
                comment.push_str(&line[index + 1..]);
                break;
            }

            index += 1;
        }

        let upper = comment.to_ascii_uppercase();
        if upper.contains("TODO") || upper.contains("FIXME") {
            return true;
        }
    }

    false
}

fn is_duplicate_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("rs" | "py" | "ts" | "tsx" | "js" | "jsx" | "sh" | "bash" | "toml" | "yaml" | "yml")
    )
}

fn is_text_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some(
            "rs" | "toml"
                | "yaml"
                | "yml"
                | "json"
                | "md"
                | "txt"
                | "py"
                | "ts"
                | "tsx"
                | "js"
                | "jsx"
                | "sh"
                | "bash"
        )
    )
}

fn contains_secret_pattern(text: &str) -> bool {
    for key_kind in ["RSA PRIVATE KEY", "EC PRIVATE KEY", "OPENSSH PRIVATE KEY"] {
        let header = format!("-----BEGIN {key_kind}-----");
        if text.contains(&header) {
            return true;
        }
    }

    text.lines().any(|line| {
        let trimmed = line.trim();
        contains_token_prefix(trimmed, "github_pat_", 20)
            || contains_token_prefix(trimmed, "ghp_", 20)
            || (trimmed.starts_with("AKIA") && trimmed.len() >= 20)
    })
}

fn contains_token_prefix(text: &str, prefix: &str, minimum_suffix_len: usize) -> bool {
    let mut offset = 0;
    while let Some(relative) = text[offset..].find(prefix) {
        let start = offset + relative + prefix.len();
        let suffix_len = text[start..]
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
            .count();
        if suffix_len >= minimum_suffix_len {
            return true;
        }
        offset = start;
        if offset >= text.len() {
            break;
        }
    }
    false
}

fn contains_dangerous_shell_pattern(extension: Option<&str>, text: &str) -> bool {
    if !matches!(extension, Some("sh" | "bash" | "yml" | "yaml")) {
        return false;
    }
    let normalized = text.replace('\n', " ");
    (normalized.contains("curl ") || normalized.contains("wget "))
        && (normalized.contains("| sh") || normalized.contains("| bash"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_syntax(text: &str) -> bool {
        contains_todo_marker(text, CommentSyntax::All)
    }

    #[test]
    fn secret_patterns_are_detected() {
        let token = "ghp_".to_owned() + &"1".repeat(40);
        assert!(contains_secret_pattern(&format!("prefix {token}suffix")));
        let openssh_header = format!("-----BEGIN {}-----", "OPENSSH PRIVATE KEY");
        assert!(contains_secret_pattern(&openssh_header));
        assert!(!contains_secret_pattern("public documentation only"));
        assert!(!contains_secret_pattern("ghp_123"));
        assert!(!contains_secret_pattern("github_pat_placeholder"));
    }

    #[test]
    fn extensions_are_restricted_to_text_inputs() {
        assert!(is_text_candidate(Path::new("main.rs")));
        assert!(is_text_candidate(Path::new("README.md")));
        assert!(!is_text_candidate(Path::new("texture.png")));
    }

    #[test]
    fn unsafe_download_pipelines_are_detected() {
        assert!(contains_dangerous_shell_pattern(
            Some("sh"),
            "curl https://example.test/a.sh | sh"
        ));
        assert!(!contains_dangerous_shell_pattern(
            Some("sh"),
            "curl https://example.test/a.sh -o a.sh"
        ));
        assert!(!contains_dangerous_shell_pattern(
            Some("rs"),
            "curl https://example.test/a.sh | sh"
        ));
    }

    #[test]
    fn duplicate_candidates_are_limited_to_source_and_config() {
        assert!(is_duplicate_candidate(Path::new("a.rs")));
        assert!(is_duplicate_candidate(Path::new("a.yml")));
        assert!(!is_duplicate_candidate(Path::new("README.md")));
    }

    #[test]
    fn documentation_and_archives_are_non_production() {
        assert!(is_non_production_path(Path::new(
            "docs/archive/legacy/a.py"
        )));
        assert!(is_non_production_path(Path::new("docs/wiki/a.py")));
        assert!(is_non_production_path(Path::new("wiki/a.py")));
        assert!(!is_non_production_path(Path::new("src/a.py")));
    }

    #[test]
    fn marker_in_comments_fire_across_all_syntaxes() {
        // Fixtures aus Einzelteilen: der Scanner-Quelltext selbst darf keine
        // Marker neben Kommentar-Syntax enthalten (Self-Detection-Disziplin).
        let m1 = ["T", "O", "D", "O"].concat();
        let m2 = ["F", "I", "X", "M", "E"].concat();

        // Zeilenkommentar (Rust/JS-Stil) -> muss feuern
        assert!(any_syntax(&format!("let x = 1; // {m1} fix")));
        assert!(any_syntax(&format!("// {m2} follow-up")));

        // Hash-Kommentar (Shell/Python/YAML-Stil) -> muss feuern
        assert!(any_syntax(&format!("# {m1} shell script")));
        assert!(any_syntax(&format!("echo done # {m2} later")));

        // Block-Kommentar einzeilig -> muss feuern
        assert!(any_syntax(&format!("/* {m1} embedded */ let y = 2;")));

        // Block-Kommentar mehrzeilig, Marker in Folgezeile -> muss feuern
        assert!(any_syntax(&format!(
            "let z = 3; /* start\n{m1} inside block\nend */ let w = 4;"
        )));
    }

    #[test]
    fn marker_outside_comments_do_not_fire() {
        let m1 = ["T", "O", "D", "O"].concat();
        let m2 = ["F", "I", "X", "M", "E"].concat();

        // String-Literale -> kein Finding
        assert!(!any_syntax(&format!("let s = \"{m1} in string\";")));
        assert!(!any_syntax(&format!("let raw = r\"{m1} raw string\";")));
        assert!(!any_syntax(&format!("let msg = \"{m2} inside message\";")));

        // Identifier und Testnamen -> kein Finding
        assert!(!any_syntax(&format!("fn {m1}_marker_helper() {{}}")));
        assert!(!any_syntax(&format!("let {m1}_count = 42;")));
    }

    #[test]
    fn detector_source_snippet_is_not_self_flagged() {
        // Die eigene Detektor-Implementierung referenziert die Marker als
        // String-Literale (contains("...")) — Self-Detection: kein Finding.
        let m1 = ["T", "O", "D", "O"].concat();
        let detector_snippet = format!(
            "let upper = comment.to_ascii_uppercase();\nif upper.contains(\"{m1}\") {{\n    return true;\n}}\n"
        );
        assert!(!any_syntax(&detector_snippet));
    }

    #[test]
    fn comment_syntax_is_language_aware() {
        let m1 = ["T", "O", "D", "O"].concat();

        // Python/Shell: Hash-Kommentar feuert
        assert!(contains_todo_marker(
            &format!("# {m1} shell script"),
            CommentSyntax::Hash
        ));

        // Python-Glob mit /* ist KEIN Kommentar: Code danach bleibt Code
        // (Regression: Giftzeile standards_enforcement_audit.py:88)
        assert!(!contains_todo_marker(
            &format!("if not any(repo.glob(\"**/*test*\")):\n    pattern = \"{m1}\"\n*/\n"),
            CommentSyntax::Hash
        ));

        // Rust: Block-Kommentar feuert, Hash ist kein Kommentar
        assert!(contains_todo_marker(
            &format!("/* {m1} embedded */"),
            CommentSyntax::SlashAndBlock
        ));
        assert!(!contains_todo_marker(
            &format!("let zeile = \"#{m1}\";"),
            CommentSyntax::SlashAndBlock
        ));
    }

    #[test]
    fn comment_syntax_for_maps_extensions() {
        assert!(matches!(
            comment_syntax_for(Path::new("a.rs")),
            CommentSyntax::SlashAndBlock
        ));
        assert!(matches!(
            comment_syntax_for(Path::new("a.ts")),
            CommentSyntax::SlashAndBlock
        ));
        assert!(matches!(
            comment_syntax_for(Path::new("a.py")),
            CommentSyntax::Hash
        ));
        assert!(matches!(
            comment_syntax_for(Path::new("a.sh")),
            CommentSyntax::Hash
        ));
        assert!(matches!(
            comment_syntax_for(Path::new("x.unknown")),
            CommentSyntax::All
        ));
        assert!(matches!(
            comment_syntax_for(Path::new("ohne")),
            CommentSyntax::All
        ));
    }
}
