//! Package-Manifest mit deterministisch sortierbaren Dependencies.

use crate::version::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub req: VersionReq,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: Version,
    pub deps: Vec<Dependency>,
}
