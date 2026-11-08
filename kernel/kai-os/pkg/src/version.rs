//! SemVer-artige Versionen mit deterministischem Vergleich und Version-Reqs.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionReq {
    /// Exakte Version ("1.2.3")
    Exact(Version),
    /// Caret: kompatibel innerhalb derselben Major-Version ("^1.2.0")
    Caret(Version),
}

#[derive(Debug, PartialEq)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "version parse error: {}", self.0)
    }
}

impl FromStr for Version {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(ParseError(format!("'{s}' erwartet major.minor.patch")));
        }
        let mut nums = [0u64; 3];
        for (i, p) in parts.iter().enumerate() {
            nums[i] = p
                .parse()
                .map_err(|_| ParseError(format!("'{p}' ist keine Zahl")))?;
        }
        Ok(Version {
            major: nums[0],
            minor: nums[1],
            patch: nums[2],
        })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for VersionReq {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix('^') {
            Ok(VersionReq::Caret(Version::from_str(rest)?))
        } else {
            Ok(VersionReq::Exact(Version::from_str(s)?))
        }
    }
}

impl std::fmt::Display for VersionReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionReq::Exact(v) => write!(f, "{v}"),
            VersionReq::Caret(v) => write!(f, "^{v}"),
        }
    }
}

impl VersionReq {
    pub fn matches(&self, v: Version) -> bool {
        match self {
            VersionReq::Exact(e) => *e == v,
            VersionReq::Caret(c) => {
                v.major == c.major && (v.major, v.minor, v.patch) >= (c.major, c.minor, c.patch)
            }
        }
    }
}
