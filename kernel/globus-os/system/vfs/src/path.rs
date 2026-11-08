//! Deterministic VFS path normalization and mount resolution.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    NotAbsolute,
    EmptyComponent,
    TraversalAboveRoot,
}

pub fn normalize(path: &str) -> Result<String, PathError> {
    if !path.starts_with('/') {
        return Err(PathError::NotAbsolute);
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(PathError::TraversalAboveRoot);
                }
            }
            value if value.contains('\0') => return Err(PathError::EmptyComponent),
            value => parts.push(value),
        }
    }
    if parts.is_empty() {
        Ok("/".into())
    } else {
        Ok(format!("/{}", parts.join("/")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonicalizes() {
        assert_eq!(normalize("/home/a/../b").unwrap(), "/home/b");
    }
    #[test]
    fn blocks_relative() {
        assert_eq!(normalize("home/a"), Err(PathError::NotAbsolute));
    }
    #[test]
    fn blocks_escape() {
        assert_eq!(normalize("/../../etc"), Err(PathError::TraversalAboveRoot));
    }
}
