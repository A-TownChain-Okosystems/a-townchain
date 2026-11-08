use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildProfile {
    pub name: String,
    pub optimize: bool,
    pub debug_symbols: bool,
}
impl BuildProfile {
    pub fn debug() -> Self {
        Self {
            name: "debug".into(),
            optimize: false,
            debug_symbols: true,
        }
    }
    pub fn release() -> Self {
        Self {
            name: "release".into(),
            optimize: true,
            debug_symbols: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub target: String,
    pub files: Vec<PathBuf>,
}
impl PackageManifest {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            target: target.into(),
            files: Vec::new(),
        }
    }
    pub fn add_file(&mut self, path: impl AsRef<Path>) {
        self.files.push(path.as_ref().to_path_buf());
        self.files.sort();
        self.files.dedup();
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("package name is empty".into());
        }
        if self.version.trim().is_empty() {
            return Err("package version is empty".into());
        }
        if self.target.trim().is_empty() {
            return Err("package target is empty".into());
        }
        if self.files.iter().any(|p| {
            p.is_absolute()
                || p.components()
                    .any(|c| matches!(c, std::path::Component::ParentDir))
        }) {
            return Err("package contains unsafe path".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Artifact {
    pub path: PathBuf,
    pub size: u64,
    pub checksum: u64,
}
pub fn checksum(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
pub fn collect_files(root: &Path, relative: &Path) -> std::io::Result<Vec<PathBuf>> {
    let dir = root.join(relative);
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            out.extend(collect_files(root, &relative.join(entry.file_name()))?)
        } else {
            out.push(relative.join(entry.file_name()))
        }
    }
    out.sort();
    Ok(out)
}
pub fn build_artifacts(root: &Path, manifest: &PackageManifest) -> std::io::Result<Vec<Artifact>> {
    let mut out = Vec::with_capacity(manifest.files.len());
    for path in &manifest.files {
        let full = root.join(path);
        let bytes = std::fs::read(&full)?;
        out.push(Artifact {
            path: path.clone(),
            size: bytes.len() as u64,
            checksum: checksum(&bytes),
        })
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}
fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
pub fn manifest_json(manifest: &PackageManifest) -> String {
    let files = manifest
        .files
        .iter()
        .map(|p| {
            format!(
                "\"{}\"",
                json_escape(&p.to_string_lossy().replace('\\', "/"))
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"name\":\"{}\",\"version\":\"{}\",\"target\":\"{}\",\"files\":[{}]}}",
        json_escape(&manifest.name),
        json_escape(&manifest.version),
        json_escape(&manifest.target),
        files
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manifest_is_deterministic() {
        let mut m = PackageManifest::new("game", "1", "linux");
        m.add_file("z");
        m.add_file("a");
        m.add_file("a");
        assert_eq!(m.files, vec![PathBuf::from("a"), PathBuf::from("z")]);
        assert!(m.validate().is_ok());
    }
    #[test]
    fn unsafe_paths_rejected() {
        let mut m = PackageManifest::new("g", "1", "x");
        m.add_file("../secret");
        assert!(m.validate().is_err());
    }
}
