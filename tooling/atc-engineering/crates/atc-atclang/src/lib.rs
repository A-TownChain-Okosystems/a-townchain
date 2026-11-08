//! Governed ATCLang artifact operations.
//!
//! This crate owns repository-level artifact handling. It deliberately does not
//! reimplement the ATCLang parser or compiler. Compilation is delegated to the
//! canonical `atclang` toolchain through an explicit command adapter.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Source,
    Ecosystem,
    VmBytecode,
    Unknown,
}

impl ArtifactKind {
    pub fn from_path(path: &Path) -> Self {
        match path
            .extension()
            .and_then(|x| x.to_str())
            .unwrap_or_default()
        {
            "atc" => Self::Source,
            "aes" => Self::Ecosystem,
            "atvm" => Self::VmBytecode,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInfo {
    pub path: PathBuf,
    pub kind: ArtifactKind,
    pub bytes: u64,
}

pub fn inspect(path: impl AsRef<Path>) -> io::Result<ArtifactInfo> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;
    Ok(ArtifactInfo {
        path: path.to_path_buf(),
        kind: ArtifactKind::from_path(path),
        bytes: metadata.len(),
    })
}

pub fn create_source(path: impl AsRef<Path>, source: &str) -> io::Result<()> {
    let path = path.as_ref();
    if ArtifactKind::from_path(path) != ArtifactKind::Source {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "creation requires a .atc path",
        ));
    }
    if path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "ATCLang artifact already exists",
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, source)
}

pub fn replace_source(path: impl AsRef<Path>, source: &str) -> io::Result<()> {
    let path = path.as_ref();
    if ArtifactKind::from_path(path) != ArtifactKind::Source {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "editing requires a .atc path",
        ));
    }
    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "ATCLang source does not exist",
        ));
    }
    let tmp = path.with_extension("atc.tmp");
    fs::write(&tmp, source)?;
    let result = fs::rename(&tmp, path);
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerRequest {
    pub executable: String,
    pub arguments: Vec<String>,
    pub input: PathBuf,
    pub output: PathBuf,
}

impl CompilerRequest {
    pub fn new(
        executable: impl Into<String>,
        input: impl Into<PathBuf>,
        output: impl Into<PathBuf>,
    ) -> Self {
        let input = input.into();
        let output = output.into();
        Self {
            executable: executable.into(),
            arguments: vec![
                input.display().to_string(),
                "--out".into(),
                output.display().to_string(),
            ],
            input,
            output,
        }
    }

    pub fn run(&self) -> io::Result<()> {
        if ArtifactKind::from_path(&self.input) != ArtifactKind::Source {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "compiler input must be .atc",
            ));
        }
        if ArtifactKind::from_path(&self.output) != ArtifactKind::VmBytecode {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "compiler output must be .atvm",
            ));
        }
        let status = Command::new(&self.executable)
            .args(&self.arguments)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "ATCLang compiler exited with {status}"
            )));
        }
        if !self.output.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "compiler reported success but produced no .atvm artifact",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("atc-atclang-{nonce}-{name}"))
    }

    #[test]
    fn classifies_supported_extensions() {
        assert_eq!(
            ArtifactKind::from_path(Path::new("x.atc")),
            ArtifactKind::Source
        );
        assert_eq!(
            ArtifactKind::from_path(Path::new("x.aes")),
            ArtifactKind::Ecosystem
        );
        assert_eq!(
            ArtifactKind::from_path(Path::new("x.atvm")),
            ArtifactKind::VmBytecode
        );
    }

    #[test]
    fn creates_and_edits_source() {
        let path = temp_path("example.atc");
        create_source(&path, "contract Example {}\n").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "contract Example {}\n");
        replace_source(&path, "contract Example { fn ping() {} }\n").unwrap();
        assert!(fs::read_to_string(&path).unwrap().contains("ping"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn rejects_wrong_extensions() {
        let path = temp_path("example.txt");
        assert!(create_source(&path, "x").is_err());
    }
}
