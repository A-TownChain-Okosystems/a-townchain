//! Deterministic package dependency resolution.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub package: String,
    pub minimum_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageNode {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveError {
    EmptyName,
    MissingDependency,
    Cycle,
    VersionMismatch,
}

fn version_at_least(actual: &str, minimum: &str) -> bool {
    fn parse(v: &str) -> Option<(u64, u64, u64)> {
        let mut p = v.split('.');
        Some((p.next()?.parse().ok()?, p.next()?.parse().ok()?, p.next()?.parse().ok()?))
    }
    match (parse(actual), parse(minimum)) {
        (Some(a), Some(m)) => a >= m,
        _ => actual == minimum,
    }
}

pub fn resolve(packages: &[PackageNode], root: &str) -> Result<Vec<String>, ResolveError> {
    let mut map = BTreeMap::new();
    for package in packages {
        if package.name.is_empty() { return Err(ResolveError::EmptyName); }
        map.insert(package.name.clone(), package);
    }
    if !map.contains_key(root) { return Err(ResolveError::MissingDependency); }

    fn visit<'a>(name: &str, map: &BTreeMap<String, &'a PackageNode>, active: &mut BTreeSet<String>, done: &mut BTreeSet<String>, out: &mut Vec<String>) -> Result<(), ResolveError> {
        if done.contains(name) { return Ok(()); }
        if !active.insert(name.to_owned()) { return Err(ResolveError::Cycle); }
        let node = *map.get(name).ok_or(ResolveError::MissingDependency)?;
        let mut deps = node.dependencies.clone();
        deps.sort_by(|a, b| a.package.cmp(&b.package).then_with(|| a.minimum_version.cmp(&b.minimum_version)));
        for dep in deps {
            let target = *map.get(&dep.package).ok_or(ResolveError::MissingDependency)?;
            if !version_at_least(&target.version, &dep.minimum_version) { return Err(ResolveError::VersionMismatch); }
            visit(&dep.package, map, active, done, out)?;
        }
        active.remove(name);
        done.insert(name.to_owned());
        out.push(name.to_owned());
        Ok(())
    }

    let mut active = BTreeSet::new();
    let mut done = BTreeSet::new();
    let mut out = Vec::new();
    visit(root, &map, &mut active, &mut done, &mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependencies_are_post_ordered() {
        let packages = [
            PackageNode { name: "app".into(), version: "1.0.0".into(), dependencies: vec![Dependency { package: "lib".into(), minimum_version: "1.0.0".into() }] },
            PackageNode { name: "lib".into(), version: "1.2.0".into(), dependencies: vec![] },
        ];
        assert_eq!(resolve(&packages, "app").unwrap(), vec!["lib", "app"]);
    }
    #[test]
    fn cycles_fail_closed() {
        let packages = [
            PackageNode { name: "a".into(), version: "1.0.0".into(), dependencies: vec![Dependency { package: "b".into(), minimum_version: "1.0.0".into() }] },
            PackageNode { name: "b".into(), version: "1.0.0".into(), dependencies: vec![Dependency { package: "a".into(), minimum_version: "1.0.0".into() }] },
        ];
        assert_eq!(resolve(&packages, "a"), Err(ResolveError::Cycle));
    }
}
