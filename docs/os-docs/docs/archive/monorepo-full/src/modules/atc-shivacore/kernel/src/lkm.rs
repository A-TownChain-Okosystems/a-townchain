// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ══════════════════════════════════════════════════════════════════════════════
// K-Sprint 48 — Loadable Kernel Modules (LKM)
// Dynamic module loading/unloading with init/exit functions, dependency graph,
// symbol exports, reference counting, module parameters, safety checks.
// insmod / rmmod / modprobe equivalent.
// ══════════════════════════════════════════════════════════════════════════════

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

// ══════════════════════════════════════════════════════════════════════════════
// GLOBAL COUNTERS
// ══════════════════════════════════════════════════════════════════════════════

static MODULE_SEQ: AtomicU64 = AtomicU64::new(0);

fn next_module_id() -> u64 { MODULE_SEQ.fetch_add(1, Ordering::SeqCst) }

// ══════════════════════════════════════════════════════════════════════════════
// MODULE STATE MACHINE
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ModuleState {
    Registered,  // Discovered but not loaded
    Loading,      // init() in progress
    Active,       // init() succeeded, module running
    Unloading,    // exit() in progress
    Failed,        // init() returned error
    Unloaded,      // Successfully unloaded
}

impl ModuleState {
    pub fn name(&self) -> &'static str {
        match self {
            ModuleState::Registered => "REGISTERED",
            ModuleState::Loading => "LOADING",
            ModuleState::Active => "ACTIVE",
            ModuleState::Unloading => "UNLOADING",
            ModuleState::Failed => "FAILED",
            ModuleState::Unloaded => "UNLOADED",
        }
    }

    pub fn is_active(&self) -> bool { matches!(self, ModuleState::Active) }
    pub fn is_loading(&self) -> bool { matches!(self, ModuleState::Loading) }
    pub fn is_unloading(&self) -> bool { matches!(self, ModuleState::Unloading) }
    pub fn is_terminal(&self) -> bool { matches!(self, ModuleState::Unloaded) }
}

impl std::fmt::Display for ModuleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE PRIORITY
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModulePriority {
    Core = 0,       // Core kernel subsystems (must load first)
    Driver = 1,      // Hardware drivers
    FileSystem = 2,  // File system modules
    Network = 3,     // Network protocol modules
    Security = 4,     // Security modules (SELinux-like)
    Utility = 5,       // Utility/helper modules
    Custom = 9,        // User-defined modules
}

impl ModulePriority {
    pub fn name(&self) -> &'static str {
        match self {
            ModulePriority::Core => "core",
            ModulePriority::Driver => "driver",
            ModulePriority::FileSystem => "filesystem",
            ModulePriority::Network => "network",
            ModulePriority::Security => "security",
            ModulePriority::Utility => "utility",
            ModulePriority::Custom => "custom",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE LICENSE
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModuleLicense {
    Gpl,         // GPL-compatible
    Mit,          // MIT
    Bsd,          // BSD
    Apache,       // Apache 2.0
    Proprietary,  // Proprietary (taints kernel)
}

impl ModuleLicense {
    pub fn name(&self) -> &'static str {
        match self {
            ModuleLicense::Gpl => "GPL",
            ModuleLicense::Mit => "MIT",
            ModuleLicense::Bsd => "BSD",
            ModuleLicense::Apache => "Apache-2.0",
            ModuleLicense::Proprietary => "Proprietary",
        }
    }

    pub fn is_open_source(&self) -> bool { !matches!(self, ModuleLicense::Proprietary) }
    pub fn taints_kernel(&self) -> bool { matches!(self, ModuleLicense::Proprietary) }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE PARAMETER
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ModuleParam {
    pub name: String,
    pub description: String,
    pub default_value: String,
    pub current_value: String,
    pub read_only: bool,
    pub param_type: ParamType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParamType {
    Bool,
    Int,
    Uint,
    String,
    List,
}

impl ParamType {
    pub fn name(&self) -> &'static str {
        match self {
            ParamType::Bool => "bool",
            ParamType::Int => "int",
            ParamType::Uint => "uint",
            ParamType::String => "string",
            ParamType::List => "list",
        }
    }

    pub fn validate(&self, value: &str) -> bool {
        match self {
            ParamType::Bool => value == "true" || value == "false" || value == "1" || value == "0",
            ParamType::Int => value.parse::<i64>().is_ok(),
            ParamType::Uint => value.parse::<u64>().is_ok(),
            ParamType::String => !value.is_empty(),
            ParamType::List => true, // comma-separated values
        }
    }
}

impl ModuleParam {
    pub fn new(name: &str, ptype: ParamType, default: &str, desc: &str) -> Self {
        ModuleParam {
            name: name.to_string(),
            description: desc.to_string(),
            default_value: default.to_string(),
            current_value: default.to_string(),
            read_only: false,
            param_type: ptype,
        }
    }

    pub fn readonly(name: &str, ptype: ParamType, default: &str, desc: &str) -> Self {
        ModuleParam {
            name: name.to_string(),
            description: desc.to_string(),
            default_value: default.to_string(),
            current_value: default.to_string(),
            read_only: true,
            param_type: ptype,
        }
    }

    pub fn set(&mut self, value: &str) -> Result<(), String> {
        if self.read_only {
            return Err(format!("Parameter '{}' is read-only", self.name));
        }
        if !self.param_type.validate(value) {
            return Err(format!(
                "Invalid value '{}' for type {} ({})",
                value, self.param_type.name(), self.name
            ));
        }
        self.current_value = value.to_string();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.current_value = self.default_value.clone();
    }

    pub fn is_default(&self) -> bool {
        self.current_value == self.default_value
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// EXPORTED SYMBOL
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ExportedSymbol {
    pub name: String,
    pub module_id: u64,
    pub module_name: String,
    pub address: u64,
    pub size: usize,
    pub ref_count: u64,
    pub symbol_type: SymbolType,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SymbolType {
    Function,
    Variable,
    Constant,
    Struct,
    Trait,
}

impl SymbolType {
    pub fn name(&self) -> &'static str {
        match self {
            SymbolType::Function => "fn",
            SymbolType::Variable => "var",
            SymbolType::Constant => "const",
            SymbolType::Struct => "struct",
            SymbolType::Trait => "trait",
        }
    }
}

impl ExportedSymbol {
    pub fn new(name: &str, module_id: u64, module_name: &str, stype: SymbolType) -> Self {
        ExportedSymbol {
            name: name.to_string(),
            module_id,
            module_name: module_name.to_string(),
            address: 0xDEAD_0000 + module_id * 0x1000,
            size: 0,
            ref_count: 0,
            symbol_type: stype,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE STATISTICS
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ModuleStats {
    pub load_count: u64,
    pub unload_count: u64,
    pub init_time_us: u64,
    pub exit_time_us: u64,
    pub last_load_timestamp: u64,
    pub last_unload_timestamp: u64,
    pub error_count: u64,
    pub last_error: Option<String>,
    pub memory_used: u64,
    pub symbols_exported: usize,
    pub symbols_imported: usize,
}

impl Default for ModuleStats {
    fn default() -> Self {
        ModuleStats {
            load_count: 0,
            unload_count: 0,
            init_time_us: 0,
            exit_time_us: 0,
            last_load_timestamp: 0,
            last_unload_timestamp: 0,
            error_count: 0,
            last_error: None,
            memory_used: 0,
            symbols_exported: 0,
            symbols_imported: 0,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE DESCRIPTOR
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ModuleDescriptor {
    pub id: u64,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: ModuleLicense,
    pub priority: ModulePriority,
    pub state: ModuleState,
    pub dependencies: Vec<String>,
    pub optional_deps: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub params: Vec<ModuleParam>,
    pub exports: Vec<ExportedSymbol>,
    pub imports: Vec<String>,
    pub stats: ModuleStats,
    pub ref_count: u64,
    pub load_order: u64,
    pub auto_load: bool,
    pub kernel_version: String,
}

impl ModuleDescriptor {
    pub fn new(name: &str, version: &str) -> Self {
        ModuleDescriptor {
            id: next_module_id(),
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            author: String::new(),
            license: ModuleLicense::Gpl,
            priority: ModulePriority::Utility,
            state: ModuleState::Registered,
            dependencies: Vec::new(),
            optional_deps: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            params: Vec::new(),
            exports: Vec::new(),
            imports: Vec::new(),
            stats: ModuleStats::default(),
            ref_count: 0,
            load_order: 0,
            auto_load: false,
            kernel_version: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn with_author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn with_license(mut self, license: ModuleLicense) -> Self {
        self.license = license;
        self
    }

    pub fn with_priority(mut self, priority: ModulePriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_dependencies(mut self, deps: &[&str]) -> Self {
        self.dependencies = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_optional_deps(mut self, deps: &[&str]) -> Self {
        self.optional_deps = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_conflicts(mut self, conflicts: &[&str]) -> Self {
        self.conflicts = conflicts.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_provides(mut self, provides: &[&str]) -> Self {
        self.provides = provides.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_param(mut self, param: ModuleParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_export(mut self, symbol: ExportedSymbol) -> Self {
        self.imports.push(symbol.name.clone());
        self.exports.push(symbol);
        self
    }

    pub fn with_auto_load(mut self, auto: bool) -> Self {
        self.auto_load = auto;
        self
    }

    pub fn add_ref(&mut self) -> u64 {
        self.ref_count += 1;
        self.ref_count
    }

    pub fn release_ref(&mut self) -> Result<u64, String> {
        if self.ref_count == 0 {
            return Err(format!("Module {} has no references to release", self.name));
        }
        self.ref_count -= 1;
        Ok(self.ref_count)
    }

    pub fn can_unload(&self) -> bool {
        self.state == ModuleState::Active && self.ref_count == 0
    }

    pub fn get_param(&self, name: &str) -> Option<&ModuleParam> {
        self.params.iter().find(|p| p.name == name)
    }

    pub fn get_param_mut(&mut self, name: &str) -> Option<&mut ModuleParam> {
        self.params.iter_mut().find(|p| p.name == name)
    }

    pub fn set_param(&mut self, name: &str, value: &str) -> Result<(), String> {
        let param = self.get_param_mut(name)
            .ok_or_else(|| format!("Parameter '{}' not found in module '{}'", name, self.name))?;
        param.set(value)
    }

    pub fn reset_params(&mut self) {
        for p in &mut self.params {
            p.reset();
        }
    }

    pub fn report(&self) -> String {
        let mut r = String::new();
        r.push_str(&format!("Module: {} v{}\n", self.name, self.version));
        r.push_str(&format!("  State: {}\n", self.state));
        r.push_str(&format!("  Priority: {}\n", self.priority.name()));
        r.push_str(&format!("  License: {}\n", self.license.name()));
        r.push_str(&format!("  Author: {}\n", self.author));
        r.push_str(&format!("  Description: {}\n", self.description));
        r.push_str(&format!("  Ref count: {}\n", self.ref_count));
        r.push_str(&format!("  Load order: {}\n", self.load_order));

        if !self.dependencies.is_empty() {
            r.push_str(&format!("  Dependencies: {}\n", self.dependencies.join(", ")));
        }
        if !self.optional_deps.is_empty() {
            r.push_str(&format!("  Optional deps: {}\n", self.optional_deps.join(", ")));
        }
        if !self.conflicts.is_empty() {
            r.push_str(&format!("  Conflicts: {}\n", self.conflicts.join(", ")));
        }
        if !self.provides.is_empty() {
            r.push_str(&format!("  Provides: {}\n", self.provides.join(", ")));
        }

        if !self.params.is_empty() {
            r.push_str("  Parameters:\n");
            for p in &self.params {
                let ro = if p.read_only { " (ro)" } else { "" };
                r.push_str(&format!("    {} = {} [{}]{}\n", p.name, p.current_value, p.param_type.name(), ro));
            }
        }

        if !self.exports.is_empty() {
            r.push_str(&format!("  Exports: {} symbols\n", self.exports.len()));
        }

        r.push_str(&format!("  Stats: loaded={}x, unloaded={}x, errors={}x\n",
            self.stats.load_count, self.stats.unload_count, self.stats.error_count));

        r
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE EVENT / AUDIT
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ModuleEvent {
    pub event_id: u64,
    pub timestamp: u64,
    pub event_type: ModuleEventType,
    pub module_name: String,
    pub module_id: u64,
    pub message: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModuleEventType {
    Registered,
    LoadStarted,
    LoadSucceeded,
    LoadFailed,
    UnloadStarted,
    UnloadSucceeded,
    UnloadFailed,
    ParamChanged,
    SymbolResolved,
    SymbolUnresolved,
    RefAcquired,
    RefReleased,
    AutoLoaded,
    ConflictDetected,
    DependencyMissing,
    CircularDependency,
}

impl ModuleEventType {
    pub fn name(&self) -> &'static str {
        match self {
            ModuleEventType::Registered => "REGISTERED",
            ModuleEventType::LoadStarted => "LOAD_STARTED",
            ModuleEventType::LoadSucceeded => "LOAD_SUCCEEDED",
            ModuleEventType::LoadFailed => "LOAD_FAILED",
            ModuleEventType::UnloadStarted => "UNLOAD_STARTED",
            ModuleEventType::UnloadSucceeded => "UNLOAD_SUCCEEDED",
            ModuleEventType::UnloadFailed => "UNLOAD_FAILED",
            ModuleEventType::ParamChanged => "PARAM_CHANGED",
            ModuleEventType::SymbolResolved => "SYMBOL_RESOLVED",
            ModuleEventType::SymbolUnresolved => "SYMBOL_UNRESOLVED",
            ModuleEventType::RefAcquired => "REF_ACQUIRED",
            ModuleEventType::RefReleased => "REF_RELEASED",
            ModuleEventType::AutoLoaded => "AUTO_LOADED",
            ModuleEventType::ConflictDetected => "CONFLICT_DETECTED",
            ModuleEventType::DependencyMissing => "DEPENDENCY_MISSING",
            ModuleEventType::CircularDependency => "CIRCULAR_DEPENDENCY",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// DEPENDENCY GRAPH
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct DependencyGraph {
    nodes: BTreeSet<String>,
    edges: HashMap<String, BTreeSet<String>>, // module -> set of dependencies
    reverse_edges: HashMap<String, BTreeSet<String>>, // module -> set of dependents
}

impl Default for DependencyGraph {
    fn default() -> Self {
        DependencyGraph {
            nodes: BTreeSet::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }
}

impl DependencyGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, name: &str) {
        self.nodes.insert(name.to_string());
        self.edges.entry(name.to_string()).or_default();
        self.reverse_edges.entry(name.to_string()).or_default();
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.nodes.insert(from.to_string());
        self.nodes.insert(to.to_string());
        self.edges.entry(from.to_string()).or_default().insert(to.to_string());
        self.edges.entry(to.to_string()).or_default();
        self.reverse_edges.entry(to.to_string()).or_default().insert(from.to_string());
        self.reverse_edges.entry(from.to_string()).or_default();
    }

    pub fn remove_node(&mut self, name: &str) {
        self.nodes.remove(name);
        self.edges.remove(name);
        self.reverse_edges.remove(name);
        // Clean up edges pointing to this node
        for deps in self.edges.values_mut() {
            deps.remove(name);
        }
        for dependents in self.reverse_edges.values_mut() {
            dependents.remove(name);
        }
    }

    pub fn has_node(&self, name: &str) -> bool {
        self.nodes.contains(name)
    }

    pub fn dependencies(&self, name: &str) -> &[String] {
        // Return sorted slice
        // Actually BTreeSet doesn't give &[String], need to handle differently
        // For now, collect on demand in callers
        // This is a placeholder — actual callers use the BTreeSet directly
        unimplemented!()
    }

    pub fn get_dependencies(&self, name: &str) -> Vec<String> {
        self.edges.get(name)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get_dependents(&self, name: &str) -> Vec<String> {
        self.reverse_edges.get(name)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn has_circular(&self) -> Option<Vec<String>> {
        // DFS-based cycle detection
        let mut visited = BTreeSet::new();
        let mut rec_stack = BTreeSet::new();
        let mut path = Vec::new();

        for node in &self.nodes {
            if !visited.contains(node) {
                if let Some(cycle) = self.dfs_cycle(node, &mut visited, &mut rec_stack, &mut path) {
                    return Some(cycle);
                }
            }
        }
        None
    }

    fn dfs_cycle(
        &self,
        node: &str,
        visited: &mut BTreeSet<String>,
        rec_stack: &mut BTreeSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.edges.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.dfs_cycle(dep, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found cycle — extract it
                    let cycle_start = path.iter().position(|n| n == dep).unwrap();
                    let cycle = path[cycle_start..].to_vec();
                    return Some(cycle);
                }
            }
        }

        path.pop();
        rec_stack.remove(node);
        None
    }

    pub fn topological_sort(&self) -> Result<Vec<String>, Vec<String>> {
        // Kahn's algorithm
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<String> = VecDeque::new();
        // Process in sorted order for determinism
        let mut sorted_nodes: Vec<String> = self.nodes.iter().cloned().collect();
        sorted_nodes.sort();
        for node in &sorted_nodes {
            if *in_degree.get(node).unwrap_or(&0) == 0 {
                queue.push_back(node.clone());
            }
        }

        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            if let Some(deps) = self.edges.get(&node) {
                let mut sorted_deps: Vec<String> = deps.iter().cloned().collect();
                sorted_deps.sort();
                for dep in &sorted_deps {
                    let d = in_degree.get_mut(dep).unwrap();
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        if result.len() == self.nodes.len() {
            Ok(result)
        } else {
            // Cycle exists — return nodes that couldn't be sorted
            let remaining: Vec<String> = self.nodes.iter()
                .filter(|n| !result.contains(n))
                .cloned()
                .collect();
            Err(remaining)
        }
    }

    pub fn load_order(&self, target: &str) -> Result<Vec<String>, String> {
        // Get the load order for a specific module (all its transitive deps first)
        let mut order = Vec::new();
        let mut visited = BTreeSet::new();
        self.dfs_load_order(target, &mut visited, &mut order)?;

        // Remove the target itself from the order (it should be loaded last)
        if let Some(pos) = order.iter().position(|n| n == target) {
            order.remove(pos);
        }
        order.push(target.to_string());
        Ok(order)
    }

    fn dfs_load_order(
        &self,
        node: &str,
        visited: &mut BTreeSet<String>,
        order: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(node) {
            return Ok(());
        }
        visited.insert(node.to_string());

        if let Some(deps) = self.edges.get(node) {
            let mut sorted_deps: Vec<String> = deps.iter().cloned().collect();
            sorted_deps.sort();
            for dep in &sorted_deps {
                if !self.nodes.contains(dep) {
                    return Err(format!("Dependency '{}' not found (required by '{}')", dep, node));
                }
                self.dfs_load_order(dep, visited, order)?;
            }
        }

        order.push(node.to_string());
        Ok(())
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|s| s.len()).sum()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// SYMBOL TABLE
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, ExportedSymbol>,
    by_module: HashMap<u64, Vec<String>>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            by_module: HashMap::new(),
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, symbol: ExportedSymbol) -> Result<(), String> {
        if self.symbols.contains_key(&symbol.name) {
            let existing = self.symbols.get(&symbol.name).unwrap();
            return Err(format!(
                "Symbol '{}' already exported by module '{}' (conflict with '{}')",
                symbol.name, existing.module_name, symbol.module_name
            ));
        }
        let name = symbol.name.clone();
        let mod_id = symbol.module_id;
        self.symbols.insert(name.clone(), symbol);
        self.by_module.entry(mod_id).or_default().push(name);
        Ok(())
    }

    pub fn unregister_module(&mut self, module_id: u64) -> Vec<String> {
        let names = self.by_module.remove(&module_id).unwrap_or_default();
        for name in &names {
            self.symbols.remove(name);
        }
        names
    }

    pub fn lookup(&self, name: &str) -> Option<&ExportedSymbol> {
        self.symbols.get(name)
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut ExportedSymbol> {
        self.symbols.get_mut(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    pub fn resolve(&mut self, name: &str) -> Result<&ExportedSymbol, String> {
        let symbol = self.symbols.get_mut(name)
            .ok_or_else(|| format!("Symbol '{}' not found in symbol table", name))?;
        symbol.ref_count += 1;
        Ok(self.symbols.get(name).unwrap())
    }

    pub fn release(&mut self, name: &str) -> Result<u64, String> {
        let symbol = self.symbols.get_mut(name)
            .ok_or_else(|| format!("Symbol '{}' not found", name))?;
        if symbol.ref_count == 0 {
            return Err(format!("Symbol '{}' has no references", name));
        }
        symbol.ref_count -= 1;
        Ok(symbol.ref_count)
    }

    pub fn module_symbols(&self, module_id: u64) -> Vec<String> {
        self.by_module.get(&module_id).cloned().unwrap_or_default()
    }

    pub fn symbol_count(&self) -> usize { self.symbols.len() }

    pub fn all_symbols(&self) -> Vec<&ExportedSymbol> {
        self.symbols.values().collect()
    }

    pub fn unresolved_imports(&self, imports: &[String]) -> Vec<String> {
        imports.iter()
            .filter(|name| !self.symbols.contains_key(*name))
            .cloned()
            .collect()
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE REGISTRY
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ModuleRegistry {
    modules: HashMap<String, ModuleDescriptor>,
    by_id: HashMap<u64, String>,
    dep_graph: DependencyGraph,
    symbol_table: SymbolTable,
    events: VecDeque<ModuleEvent>,
    next_event_id: u64,
    load_counter: u64,
    kernel_tainted: bool,
    max_events: usize,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        ModuleRegistry {
            modules: HashMap::new(),
            by_id: HashMap::new(),
            dep_graph: DependencyGraph::new(),
            symbol_table: SymbolTable::new(),
            events: VecDeque::new(),
            next_event_id: 0,
            load_counter: 0,
            kernel_tainted: false,
            max_events: 1000,
        }
    }
}

impl ModuleRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    // ── Event helpers ──

    fn log_event(&mut self, event_type: ModuleEventType, module_name: &str, module_id: u64, message: &str) {
        let event = ModuleEvent {
            event_id: self.next_event_id,
            timestamp: 0, // Would be real timestamp in kernel
            event_type,
            module_name: module_name.to_string(),
            module_id,
            message: message.to_string(),
        };
        self.next_event_id += 1;
        self.events.push_back(event);
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    pub fn events(&self) -> &VecDeque<ModuleEvent> { &self.events }
    pub fn event_count(&self) -> usize { self.events.len() }

    pub fn events_for_module(&self, name: &str) -> Vec<&ModuleEvent> {
        self.events.iter().filter(|e| e.module_name == name).collect()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // ── Registration ──

    pub fn register(&mut self, mut module: ModuleDescriptor) -> Result<u64, String> {
        if self.modules.contains_key(&module.name) {
            return Err(format!("Module '{}' already registered", module.name));
        }

        // Check for conflicts
        for conflict in &module.conflicts {
            if self.modules.contains_key(conflict) {
                let conflict_mod = self.modules.get(conflict).unwrap();
                if conflict_mod.state.is_active() {
                    self.log_event(
                        ModuleEventType::ConflictDetected,
                        &module.name, module.id,
                        &format!("Conflicts with active module '{}'", conflict),
                    );
                    return Err(format!(
                        "Module '{}' conflicts with active module '{}'",
                        module.name, conflict
                    ));
                }
            }
        }

        // Check for circular dependencies
        self.dep_graph.add_node(&module.name);
        for dep in &module.dependencies {
            self.dep_graph.add_edge(&module.name, dep);
        }
        for dep in &module.optional_deps {
            self.dep_graph.add_edge(&module.name, dep);
        }

        if let Some(cycle) = self.dep_graph.has_circular() {
            // Remove the edges we just added
            self.dep_graph.remove_node(&module.name);
            for dep in &module.dependencies {
                self.dep_graph.add_node(dep);
            }
            self.log_event(
                ModuleEventType::CircularDependency,
                &module.name, module.id,
                &format!("Circular dependency detected: {}", cycle.join(" -> ")),
            );
            return Err(format!("Circular dependency detected: {}", cycle.join(" -> ")));
        }

        let id = module.id;
        let name = module.name.clone();
        self.by_id.insert(id, name.clone());
        self.modules.insert(name, module);

        self.log_event(
            ModuleEventType::Registered,
            &self.modules.get(&self.by_id.get(&id).unwrap()).unwrap().name,
            id,
            "Module registered",
        );

        Ok(id)
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), String> {
        let module = self.modules.get(name)
            .ok_or_else(|| format!("Module '{}' not found", name))?;

        if module.state.is_active() {
            return Err(format!("Cannot unregister active module '{}'. Unload first.", name));
        }

        let id = module.id;
        self.dep_graph.remove_node(name);
        self.symbol_table.unregister_module(id);
        self.by_id.remove(&id);
        self.modules.remove(name);

        Ok(())
    }

    // ── Loading ──

    pub fn load(&mut self, name: &str) -> Result<u64, String> {
        let module = self.modules.get(name)
            .ok_or_else(|| format!("Module '{}' not found", name))?;

        if module.state.is_active() {
            return Err(format!("Module '{}' is already active", name));
        }
        if module.state.is_loading() {
            return Err(format!("Module '{}' is already loading", name));
        }

        // Check that all required deps are active
        let deps_to_check: Vec<String> = module.dependencies.clone();
        let module_name = name.to_string();
        let module_id = module.id;

        // Get load order (deps first)
        let load_order = self.dep_graph.load_order(&module_name)
            .map_err(|e| {
                self.log_event(ModuleEventType::DependencyMissing, &module_name, module_id, &e);
                e
            })?;

        // Load all required deps first
        for dep_name in &load_order {
            if dep_name == &module_name {
                continue;
            }
            if let Some(dep) = self.modules.get(dep_name) {
                if !dep.state.is_active() {
                    self.load(dep_name)?;
                }
            } else {
                let msg = format!("Required dependency '{}' not found", dep_name);
                self.log_event(ModuleEventType::DependencyMissing, &module_name, module_id, &msg);
                return Err(msg);
            }
        }

        // Now load the module itself
        let module = self.modules.get_mut(name).unwrap();
        module.state = ModuleState::Loading;
        self.log_event(ModuleEventType::LoadStarted, name, module.id, "Init function called");

        // Check for unresolved symbol imports
        let imports = module.imports.clone();
        let unresolved = self.symbol_table.unresolved_imports(&imports);
        if !unresolved.is_empty() && !module.optional_deps.is_empty() {
            // Only fail if unresolved imports are required (not optional)
            let optional_set: BTreeSet<String> = module.optional_deps.iter().cloned().collect();
            let truly_unresolved: Vec<String> = unresolved.iter()
                .filter(|s| !optional_set.contains(*s))
                .cloned().collect();
            if !truly_unresolved.is_empty() {
                let module = self.modules.get_mut(name).unwrap();
                module.state = ModuleState::Failed;
                module.stats.error_count += 1;
                module.stats.last_error = Some(format!("Unresolved symbols: {}", truly_unresolved.join(", ")));
                self.log_event(ModuleEventType::SymbolUnresolved, name, module.id,
                    &format!("Unresolved symbols: {}", truly_unresolved.join(", ")));
                return Err(format!("Module '{}' has unresolved symbols: {}", name, truly_unresolved.join(", ")));
            }
        }

        // Register exported symbols
        let exports = module.exports.clone();
        let mid = module.id;
        let mname = module.name.clone();
        for export in exports {
            self.symbol_table.register(export)
                .map_err(|e| {
                    let module = self.modules.get_mut(name).unwrap();
                    module.state = ModuleState::Failed;
                    module.stats.error_count += 1;
                    module.stats.last_error = Some(e.clone());
                    e
                })?;
        }

        // Resolve imports
        for import in &imports {
            if self.symbol_table.contains(import) {
                self.symbol_table.resolve(import)
                    .map_err(|e| {
                        let module = self.modules.get_mut(name).unwrap();
                        module.state = ModuleState::Failed;
                        module.stats.error_count += 1;
                        module.stats.last_error = Some(e.clone());
                        e
                    })?;
                self.log_event(ModuleEventType::SymbolResolved, name, mid, import);
            }
        }

        // Taint kernel if proprietary
        let license = module.license;
        if license.taints_kernel() {
            self.kernel_tainted = true;
        }

        // Finalize
        let module = self.modules.get_mut(name).unwrap();
        module.state = ModuleState::Active;
        module.stats.load_count += 1;
        module.load_order = self.load_counter;
        self.load_counter += 1;
        module.stats.symbols_exported = module.exports.len();
        module.stats.symbols_imported = module.imports.len();

        self.log_event(ModuleEventType::LoadSucceeded, name, module.id, "Module loaded successfully");

        Ok(module.id)
    }

    pub fn unload(&mut self, name: &str) -> Result<(), String> {
        let module = self.modules.get(name)
            .ok_or_else(|| format!("Module '{}' not found", name))?;

        if !module.state.is_active() {
            return Err(format!("Module '{}' is not active (state: {})", name, module.state));
        }

        let module_id = module.id;

        // Check if other modules depend on this one
        let dependents = self.dep_graph.get_dependents(name);
        let active_dependents: Vec<String> = dependents.iter()
            .filter(|dep| {
                self.modules.get(*dep)
                    .map(|m| m.state.is_active())
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if !active_dependents.is_empty() {
            return Err(format!(
                "Cannot unload '{}': active dependents: {}",
                name, active_dependents.join(", ")
            ));
        }

        // Check reference count
        if module.ref_count > 0 {
            return Err(format!("Cannot unload '{}': {} references still held", name, module.ref_count));
        }

        // Set state to unloading
        let module = self.modules.get_mut(name).unwrap();
        module.state = ModuleState::Unloading;
        self.log_event(ModuleEventType::UnloadStarted, name, module_id, "Exit function called");

        // Unregister exported symbols
        let removed_symbols = self.symbol_table.unregister_module(module_id);

        // Release imported symbols
        let imports = module.imports.clone();
        for import in &imports {
            if self.symbol_table.contains(import) {
                let _ = self.symbol_table.release(import);
            }
        }

        // Finalize
        let module = self.modules.get_mut(name).unwrap();
        module.state = ModuleState::Unloaded;
        module.stats.unload_count += 1;
        module.stats.symbols_exported = 0;
        module.stats.symbols_imported = 0;
        // Reset params to defaults
        for p in &mut module.params {
            p.reset();
        }

        self.log_event(ModuleEventType::UnloadSucceeded, name, module_id, "Module unloaded successfully");

        Ok(())
    }

    pub fn reload(&mut self, name: &str) -> Result<(), String> {
        self.unload(name)?;
        // Reset state back to Registered so we can load again
        let module = self.modules.get_mut(name).unwrap();
        module.state = ModuleState::Registered;
        self.load(name)?;
        Ok(())
    }

    // ── Auto-load ──

    pub fn auto_load_all(&mut self) -> Vec<Result<u64, String>> {
        let auto_modules: Vec<String> = self.modules.iter()
            .filter(|(_, m)| m.auto_load && !m.state.is_active())
            .map(|(name, _)| name.clone())
            .collect();

        // Sort by priority
        let mut sorted: Vec<(String, ModulePriority)> = auto_modules.into_iter()
            .map(|name| {
                let priority = self.modules.get(&name).unwrap().priority;
                (name, priority)
            })
            .collect();
        sorted.sort_by_key(|(_, p)| *p);

        sorted.into_iter()
            .map(|(name, _)| self.load(&name))
            .collect()
    }

    // ── Reference management ──

    pub fn acquire_ref(&mut self, name: &str) -> Result<u64, String> {
        let module = self.modules.get_mut(name)
            .ok_or_else(|| format!("Module '{}' not found", name))?;

        if !module.state.is_active() {
            return Err(format!("Module '{}' is not active", name));
        }

        let new_count = module.add_ref();
        self.log_event(ModuleEventType::RefAcquired, name, module.id,
            &format!("Ref acquired (count={})", new_count));
        Ok(new_count)
    }

    pub fn release_ref(&mut self, name: &str) -> Result<u64, String> {
        let module = self.modules.get_mut(name)
            .ok_or_else(|| format!("Module '{}' not found", name))?;

        let new_count = module.release_ref()
            .map_err(|e| {
                let module = self.modules.get_mut(name).unwrap();
                module.stats.error_count += 1;
                module.stats.last_error = Some(e.clone());
                e
            })?;

        self.log_event(ModuleEventType::RefReleased, name, module.id,
            &format!("Ref released (count={})", new_count));
        Ok(new_count)
    }

    // ── Symbol lookup ──

    pub fn lookup_symbol(&self, name: &str) -> Option<&ExportedSymbol> {
        self.symbol_table.lookup(name)
    }

    pub fn resolve_symbol(&mut self, name: &str) -> Result<(), String> {
        self.symbol_table.resolve(name)?;
        Ok(())
    }

    pub fn symbol_count(&self) -> usize {
        self.symbol_table.symbol_count()
    }

    // ── Queries ──

    pub fn get(&self, name: &str) -> Option<&ModuleDescriptor> {
        self.modules.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut ModuleDescriptor> {
        self.modules.get_mut(name)
    }

    pub fn get_by_id(&self, id: u64) -> Option<&ModuleDescriptor> {
        self.by_id.get(&id).and_then(|name| self.modules.get(name))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    pub fn active_count(&self) -> usize {
        self.modules.values().filter(|m| m.state.is_active()).count()
    }

    pub fn loaded_count(&self) -> usize {
        self.modules.values().filter(|m| m.state.is_active() || m.state == ModuleState::Unloaded).count()
    }

    pub fn is_tainted(&self) -> bool {
        self.kernel_tainted
    }

    pub fn list_modules(&self) -> Vec<&ModuleDescriptor> {
        let mut mods: Vec<&ModuleDescriptor> = self.modules.values().collect();
        mods.sort_by(|a, b| a.name.cmp(&b.name));
        mods
    }

    pub fn list_active(&self) -> Vec<&ModuleDescriptor> {
        let mut mods: Vec<&ModuleDescriptor> = self.modules.values()
            .filter(|m| m.state.is_active())
            .collect();
        mods.sort_by(|a, b| a.load_order.cmp(&b.load_order));
        mods
    }

    pub fn list_by_priority(&self, priority: ModulePriority) -> Vec<&ModuleDescriptor> {
        self.modules.values()
            .filter(|m| m.priority == priority)
            .collect()
    }

    pub fn list_by_state(&self, state: ModuleState) -> Vec<&ModuleDescriptor> {
        self.modules.values()
            .filter(|m| m.state == state)
            .collect()
    }

    pub fn set_param(&mut self, module: &str, param: &str, value: &str) -> Result<(), String> {
        let mod_desc = self.modules.get_mut(module)
            .ok_or_else(|| format!("Module '{}' not found", module))?;

        if !mod_desc.state.is_active() {
            return Err(format!("Module '{}' is not active", module));
        }

        mod_desc.set_param(param, value)?;
        self.log_event(ModuleEventType::ParamChanged, module, mod_desc.id,
            &format!("Parameter '{}' set to '{}'", param, value));
        Ok(())
    }

    pub fn get_param(&self, module: &str, param: &str) -> Result<&str, String> {
        let mod_desc = self.modules.get(module)
            .ok_or_else(|| format!("Module '{}' not found", module))?;

        let p = mod_desc.get_param(param)
            .ok_or_else(|| format!("Parameter '{}' not found", param))?;

        Ok(&p.current_value)
    }

    pub fn dependency_graph(&self) -> &DependencyGraph { &self.dep_graph }
    pub fn symbol_table(&self) -> &SymbolTable { &self.symbol_table }

    // ── Reports ──

    pub fn report(&self) -> String {
        let mut r = String::new();
        r.push_str("=== Module Registry Report ===\n\n");
        r.push_str(&format!("Total modules: {}\n", self.module_count()));
        r.push_str(&format!("Active: {}\n", self.active_count()));
        r.push_str(&format!("Symbols: {}\n", self.symbol_count()));
        r.push_str(&format!("Kernel tainted: {}\n", if self.is_tainted() { "yes" } else { "no" }));
        r.push_str(&format!("Events: {}\n\n", self.event_count()));

        r.push_str("--- Active Modules (by load order) ---\n");
        for m in self.list_active() {
            r.push_str(&format!(
                "  [{:>3}] {:20} v{:10} [{}] refs={}\n",
                m.load_order, m.name, m.version, m.priority.name(), m.ref_count
            ));
        }

        r.push_str("\n--- All Modules ---\n");
        for m in self.list_modules() {
            r.push_str(&format!(
                "  {:20} v{:10} {:10} exports={}\n",
                m.name, m.version, m.state, m.exports.len()
            ));
        }

        r
    }

    pub fn dependency_report(&self) -> String {
        let mut r = String::new();
        r.push_str("=== Dependency Graph ===\n\n");
        r.push_str(&format!("Nodes: {}\n", self.dep_graph.node_count()));
        r.push_str(&format!("Edges: {}\n", self.dep_graph.edge_count()));

        if let Some(cycle) = self.dep_graph.has_circular() {
            r.push_str(&format!("\n⚠ CIRCULAR DEPENDENCY: {}\n", cycle.join(" → ")));
        } else {
            r.push_str("\nNo circular dependencies detected.\n");
        }

        r.push_str("\n--- Dependencies ---\n");
        for name in &self.dep_graph.nodes {
            let deps = self.dep_graph.get_dependencies(name);
            let dependents = self.dep_graph.get_dependents(name);
            r.push_str(&format!("  {}:\n", name));
            r.push_str(&format!("    deps:       {}\n", if deps.is_empty() { "(none)".to_string() } else { deps.join(", ") }));
            r.push_str(&format!("    dependents: {}\n", if dependents.is_empty() { "(none)".to_string() } else { dependents.join(", ") }));
        }

        r
    }

    pub fn symbol_report(&self) -> String {
        let mut r = String::new();
        r.push_str("=== Symbol Table ===\n\n");
        r.push_str(&format!("Total symbols: {}\n\n", self.symbol_count()));

        let mut symbols: Vec<&ExportedSymbol> = self.symbol_table.all_symbols();
        symbols.sort_by(|a, b| a.name.cmp(&b.name));

        r.push_str("Symbol                              Module              Type  Refs\n");
        r.push_str("─────────────────────────────────── ─────────────────── ──── ────\n");
        for s in symbols {
            r.push_str(&format!(
                "{:35} {:19} {:4} {}\n",
                s.name, s.module_name, s.symbol_type.name(), s.ref_count
            ));
        }

        r
    }

    pub fn event_report(&self, last_n: usize) -> String {
        let mut r = String::new();
        r.push_str("=== Module Events ===\n\n");

        let events: Vec<&ModuleEvent> = self.events.iter().rev().take(last_n).collect();
        for e in events.iter().rev() {
            r.push_str(&format!(
                "[{:4}] {:20} {:20} {}\n",
                e.event_id, e.module_name, e.event_type.name(), e.message
            ));
        }

        r
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// MODULE BUILDER HELPER
// ══════════════════════════════════════════════════════════════════════════════

pub struct ModuleBuilder {
    module: ModuleDescriptor,
}

impl ModuleBuilder {
    pub fn new(name: &str, version: &str) -> Self {
        ModuleBuilder {
            module: ModuleDescriptor::new(name, version),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.module.description = desc.to_string();
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.module.author = author.to_string();
        self
    }

    pub fn license(mut self, license: ModuleLicense) -> Self {
        self.module.license = license;
        self
    }

    pub fn priority(mut self, priority: ModulePriority) -> Self {
        self.module.priority = priority;
        self
    }

    pub fn depends_on(mut self, deps: &[&str]) -> Self {
        self.module.dependencies = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn optional_depends_on(mut self, deps: &[&str]) -> Self {
        self.module.optional_deps = deps.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn conflicts_with(mut self, modules: &[&str]) -> Self {
        self.module.conflicts = modules.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn provides(mut self, services: &[&str]) -> Self {
        self.module.provides = services.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn param(mut self, name: &str, ptype: ParamType, default: &str, desc: &str) -> Self {
        self.module.params.push(ModuleParam::new(name, ptype, default, desc));
        self
    }

    pub fn readonly_param(mut self, name: &str, ptype: ParamType, default: &str, desc: &str) -> Self {
        self.module.params.push(ModuleParam::readonly(name, ptype, default, desc));
        self
    }

    pub fn export(mut self, name: &str, stype: SymbolType) -> Self {
        let symbol = ExportedSymbol::new(name, self.module.id, &self.module.name, stype);
        self.module.imports.push(name.to_string());
        self.module.exports.push(symbol);
        self
    }

    pub fn import_symbol(mut self, name: &str) -> Self {
        self.module.imports.push(name.to_string());
        self
    }

    pub fn auto_load(mut self) -> Self {
        self.module.auto_load = true;
        self
    }

    pub fn build(self) -> ModuleDescriptor {
        self.module
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// BUILT-IN MODULE DEFINITIONS
// ══════════════════════════════════════════════════════════════════════════════

pub fn create_builtin_modules() -> Vec<ModuleDescriptor> {
    let mut modules = Vec::new();

    // Core: Memory Allocator
    modules.push(
        ModuleBuilder::new("kalloc", "1.0.0")
            .description("Kernel slab/page allocator")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Core)
            .provides(&["kmalloc", "kfree"])
            .export("kmalloc", SymbolType::Function)
            .export("kfree", SymbolType::Function)
            .param("slab_size", ParamType::Uint, "4096", "Default slab size in bytes")
            .param("debug", ParamType::Bool, "false", "Enable debug allocations")
            .auto_load()
            .build()
    );

    // Core: Scheduler
    modules.push(
        ModuleBuilder::new("ksched", "1.0.0")
            .description("Kernel process scheduler (CFS)")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Core)
            .depends_on(&["kalloc"])
            .provides(&["schedule", "wake_up"])
            .export("schedule", SymbolType::Function)
            .export("wake_up", SymbolType::Function)
            .param("timeslice_us", ParamType::Uint, "1000", "Scheduler timeslice in microseconds")
            .param("min_granularity", ParamType::Uint, "100", "Minimum granularity")
            .auto_load()
            .build()
    );

    // Driver: Block Device
    modules.push(
        ModuleBuilder::new("blkdev", "1.0.0")
            .description("Block device layer with caching")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Driver)
            .depends_on(&["kalloc"])
            .provides(&["block_read", "block_write"])
            .export("block_read", SymbolType::Function)
            .export("block_write", SymbolType::Function)
            .param("cache_size", ParamType::Uint, "256", "Block cache size in blocks")
            .param("writeback", ParamType::Bool, "false", "Enable writeback caching")
            .auto_load()
            .build()
    );

    // Driver: Network Device
    modules.push(
        ModuleBuilder::new("netdev", "1.0.0")
            .description("Network device driver framework")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Driver)
            .depends_on(&["kalloc"])
            .provides(&["net_send", "net_recv"])
            .export("net_send", SymbolType::Function)
            .export("net_recv", SymbolType::Function)
            .param("mtu", ParamType::Uint, "1500", "Maximum transmission unit")
            .param("rx_queue_len", ParamType::Uint, "1000", "RX queue length")
            .auto_load()
            .build()
    );

    // Filesystem: ATCFS
    modules.push(
        ModuleBuilder::new("atcfs", "1.0.0")
            .description("A-TownChain filesystem")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::FileSystem)
            .depends_on(&["kalloc", "blkdev"])
            .provides(&["atcfs_mount", "atcfs_open", "atcfs_read"])
            .export("atcfs_mount", SymbolType::Function)
            .export("atcfs_open", SymbolType::Function)
            .export("atcfs_read", SymbolType::Function)
            .param("block_size", ParamType::Uint, "4096", "Filesystem block size")
            .param("journal", ParamType::Bool, "true", "Enable journaling")
            .auto_load()
            .build()
    );

    // Network: TCP/IP Stack
    modules.push(
        ModuleBuilder::new("tcpip", "1.0.0")
            .description("TCP/IP protocol stack")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Network)
            .depends_on(&["kalloc", "netdev"])
            .provides(&["tcp_connect", "tcp_send", "udp_send"])
            .export("tcp_connect", SymbolType::Function)
            .export("tcp_send", SymbolType::Function)
            .export("udp_send", SymbolType::Function)
            .param("tcp_window", ParamType::Uint, "65535", "TCP window size")
            .param("tcp_keepalive", ParamType::Bool, "true", "Enable TCP keepalive")
            .param("max_connections", ParamType::Uint, "10000", "Max concurrent connections")
            .auto_load()
            .build()
    );

    // Security: Capability System
    modules.push(
        ModuleBuilder::new("cap", "1.0.0")
            .description("Capability-based security module")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Security)
            .depends_on(&["kalloc"])
            .provides(&["cap_check", "cap_grant", "cap_revoke"])
            .export("cap_check", SymbolType::Function)
            .export("cap_grant", SymbolType::Function)
            .export("cap_revoke", SymbolType::Function)
            .param("strict_mode", ParamType::Bool, "true", "Enforce strict capability checks")
            .param("audit", ParamType::Bool, "true", "Enable capability audit log")
            .auto_load()
            .build()
    );

    // Security: Audit Module
    modules.push(
        ModuleBuilder::new("kaudit", "1.0.0")
            .description("Kernel security audit log")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Security)
            .depends_on(&["kalloc"])
            .optional_depends_on(&["cap"])
            .provides(&["audit_log", "audit_query"])
            .export("audit_log", SymbolType::Function)
            .export("audit_query", SymbolType::Function)
            .param("log_size", ParamType::Uint, "10000", "Max audit log entries")
            .param("log_syscalls", ParamType::Bool, "true", "Log system calls")
            .auto_load()
            .build()
    );

    // Utility: Kernel Tracing
    modules.push(
        ModuleBuilder::new("ktrace", "1.0.0")
            .description("Kernel function/syscall tracing")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Utility)
            .depends_on(&["kalloc"])
            .provides(&["trace_enable", "trace_disable"])
            .export("trace_enable", SymbolType::Function)
            .export("trace_disable", SymbolType::Function)
            .param("buffer_size", ParamType::Uint, "4096", "Trace buffer size in events")
            .param("filter", ParamType::String, "*", "Trace filter pattern")
            .build()
    );

    // Utility: Container Runtime
    modules.push(
        ModuleBuilder::new("kcontainer", "1.0.0")
            .description("Container isolation and runtime")
            .author("ShivaCore")
            .license(ModuleLicense::Gpl)
            .priority(ModulePriority::Utility)
            .depends_on(&["kalloc", "ksched", "cap"])
            .provides(&["container_create", "container_destroy"])
            .export("container_create", SymbolType::Function)
            .export("container_destroy", SymbolType::Function)
            .param("max_containers", ParamType::Uint, "100", "Maximum containers")
            .param("namespace_types", ParamType::List, "pid,mount,net,ipc,uts", "Namespace types to isolate")
            .build()
    );

    modules
}

// ══════════════════════════════════════════════════════════════════════════════
// TESTS
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── ModuleState Tests ──

    #[test]
    fn test_module_state_transitions() {
        assert_eq!(ModuleState::Registered.name(), "REGISTERED");
        assert_eq!(ModuleState::Loading.name(), "LOADING");
        assert_eq!(ModuleState::Active.name(), "ACTIVE");
        assert_eq!(ModuleState::Unloading.name(), "UNLOADING");
        assert_eq!(ModuleState::Failed.name(), "FAILED");
        assert_eq!(ModuleState::Unloaded.name(), "UNLOADED");
    }

    #[test]
    fn test_module_state_flags() {
        assert!(ModuleState::Active.is_active());
        assert!(!ModuleState::Registered.is_active());
        assert!(ModuleState::Loading.is_loading());
        assert!(!ModuleState::Active.is_loading());
        assert!(ModuleState::Unloading.is_unloading());
        assert!(ModuleState::Unloaded.is_terminal());
        assert!(!ModuleState::Active.is_terminal());
    }

    #[test]
    fn test_module_state_display() {
        let s = ModuleState::Active;
        assert_eq!(format!("{}", s), "ACTIVE");
    }

    // ── ModulePriority Tests ──

    #[test]
    fn test_module_priority_ordering() {
        assert!(ModulePriority::Core < ModulePriority::Driver);
        assert!(ModulePriority::Driver < ModulePriority::FileSystem);
        assert!(ModulePriority::FileSystem < ModulePriority::Network);
        assert!(ModulePriority::Network < ModulePriority::Security);
        assert!(ModulePriority::Security < ModulePriority::Utility);
        assert!(ModulePriority::Utility < ModulePriority::Custom);
    }

    #[test]
    fn test_module_priority_names() {
        assert_eq!(ModulePriority::Core.name(), "core");
        assert_eq!(ModulePriority::Driver.name(), "driver");
        assert_eq!(ModulePriority::FileSystem.name(), "filesystem");
        assert_eq!(ModulePriority::Network.name(), "network");
        assert_eq!(ModulePriority::Security.name(), "security");
        assert_eq!(ModulePriority::Utility.name(), "utility");
        assert_eq!(ModulePriority::Custom.name(), "custom");
    }

    // ── ModuleLicense Tests ──

    #[test]
    fn test_module_license() {
        assert!(ModuleLicense::Gpl.is_open_source());
        assert!(ModuleLicense::Mit.is_open_source());
        assert!(ModuleLicense::Bsd.is_open_source());
        assert!(ModuleLicense::Apache.is_open_source());
        assert!(!ModuleLicense::Proprietary.is_open_source());
    }

    #[test]
    fn test_license_taints() {
        assert!(!ModuleLicense::Gpl.taints_kernel());
        assert!(!ModuleLicense::Mit.taints_kernel());
        assert!(ModuleLicense::Proprietary.taints_kernel());
    }

    #[test]
    fn test_license_names() {
        assert_eq!(ModuleLicense::Gpl.name(), "GPL");
        assert_eq!(ModuleLicense::Mit.name(), "MIT");
        assert_eq!(ModuleLicense::Bsd.name(), "BSD");
        assert_eq!(ModuleLicense::Apache.name(), "Apache-2.0");
        assert_eq!(ModuleLicense::Proprietary.name(), "Proprietary");
    }

    // ── ModuleParam Tests ──

    #[test]
    fn test_param_bool_valid() {
        let mut p = ModuleParam::new("debug", ParamType::Bool, "false", "Debug mode");
        assert!(p.set("true").is_ok());
        assert_eq!(p.current_value, "true");
        assert!(p.set("false").is_ok());
        assert!(p.set("1").is_ok());
        assert!(p.set("0").is_ok());
    }

    #[test]
    fn test_param_bool_invalid() {
        let mut p = ModuleParam::new("debug", ParamType::Bool, "false", "Debug mode");
        assert!(p.set("yes").is_err());
        assert!(p.set("maybe").is_err());
        assert!(p.set("").is_err());
    }

    #[test]
    fn test_param_int_valid() {
        let mut p = ModuleParam::new("size", ParamType::Int, "1024", "Buffer size");
        assert!(p.set("2048").is_ok());
        assert!(p.set("-1").is_ok());
        assert!(p.set("0").is_ok());
    }

    #[test]
    fn test_param_int_invalid() {
        let mut p = ModuleParam::new("size", ParamType::Int, "1024", "Buffer size");
        assert!(p.set("abc").is_err());
        assert!(p.set("12.5").is_err());
    }

    #[test]
    fn test_param_uint_valid() {
        let mut p = ModuleParam::new("count", ParamType::Uint, "100", "Count");
        assert!(p.set("0").is_ok());
        assert!(p.set("999999").is_ok());
    }

    #[test]
    fn test_param_uint_invalid() {
        let mut p = ModuleParam::new("count", ParamType::Uint, "100", "Count");
        assert!(p.set("-1").is_err());
        assert!(p.set("abc").is_err());
    }

    #[test]
    fn test_param_string_valid() {
        let mut p = ModuleParam::new("name", ParamType::String, "default", "Module name");
        assert!(p.set("hello").is_ok());
        assert_eq!(p.current_value, "hello");
    }

    #[test]
    fn test_param_string_invalid() {
        let mut p = ModuleParam::new("name", ParamType::String, "default", "Module name");
        assert!(p.set("").is_err());
    }

    #[test]
    fn test_param_readonly() {
        let mut p = ModuleParam::readonly("version", ParamType::String, "1.0.0", "Module version");
        assert!(p.set("2.0.0").is_err());
        assert_eq!(p.current_value, "1.0.0");
    }

    #[test]
    fn test_param_reset() {
        let mut p = ModuleParam::new("size", ParamType::Uint, "1024", "Buffer size");
        p.set("2048").unwrap();
        assert!(!p.is_default());
        p.reset();
        assert!(p.is_default());
        assert_eq!(p.current_value, "1024");
    }

    #[test]
    fn test_param_type_validate() {
        assert!(ParamType::Bool.validate("true"));
        assert!(!ParamType::Bool.validate("yes"));
        assert!(ParamType::Int.validate("42"));
        assert!(!ParamType::Int.validate("xyz"));
        assert!(ParamType::Uint.validate("42"));
        assert!(!ParamType::Uint.validate("-5"));
        assert!(ParamType::String.validate("hello"));
        assert!(!ParamType::String.validate(""));
        assert!(ParamType::List.validate("a,b,c"));
    }

    // ── ExportedSymbol Tests ──

    #[test]
    fn test_exported_symbol() {
        let s = ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function);
        assert_eq!(s.name, "kmalloc");
        assert_eq!(s.module_id, 1);
        assert_eq!(s.module_name, "kalloc");
        assert_eq!(s.symbol_type, SymbolType::Function);
        assert_eq!(s.ref_count, 0);
    }

    #[test]
    fn test_symbol_types() {
        assert_eq!(SymbolType::Function.name(), "fn");
        assert_eq!(SymbolType::Variable.name(), "var");
        assert_eq!(SymbolType::Constant.name(), "const");
        assert_eq!(SymbolType::Struct.name(), "struct");
        assert_eq!(SymbolType::Trait.name(), "trait");
    }

    // ── ModuleDescriptor Tests ──

    #[test]
    fn test_descriptor_basic() {
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        assert_eq!(m.name, "testmod");
        assert_eq!(m.version, "1.0.0");
        assert_eq!(m.state, ModuleState::Registered);
        assert_eq!(m.priority, ModulePriority::Utility);
        assert_eq!(m.license, ModuleLicense::Gpl);
    }

    #[test]
    fn test_descriptor_builder_pattern() {
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_description("A test module")
            .with_author("Tester")
            .with_license(ModuleLicense::Mit)
            .with_priority(ModulePriority::Driver)
            .with_dependencies(&["dep1", "dep2"])
            .with_optional_deps(&["opt1"])
            .with_conflicts(&["badmod"])
            .with_provides(&["service1"])
            .with_auto_load(true);

        assert_eq!(m.description, "A test module");
        assert_eq!(m.author, "Tester");
        assert_eq!(m.license, ModuleLicense::Mit);
        assert_eq!(m.priority, ModulePriority::Driver);
        assert_eq!(m.dependencies, vec!["dep1", "dep2"]);
        assert_eq!(m.optional_deps, vec!["opt1"]);
        assert_eq!(m.conflicts, vec!["badmod"]);
        assert_eq!(m.provides, vec!["service1"]);
        assert!(m.auto_load);
    }

    #[test]
    fn test_descriptor_ref_counting() {
        let mut m = ModuleDescriptor::new("testmod", "1.0.0");
        assert_eq!(m.ref_count, 0);
        assert_eq!(m.add_ref(), 1);
        assert_eq!(m.add_ref(), 2);
        assert_eq!(m.add_ref(), 3);
        assert_eq!(m.release_ref().unwrap(), 2);
        assert_eq!(m.release_ref().unwrap(), 1);
        assert_eq!(m.release_ref().unwrap(), 0);
        assert!(m.release_ref().is_err());
    }

    #[test]
    fn test_descriptor_can_unload() {
        let mut m = ModuleDescriptor::new("testmod", "1.0.0");
        assert!(!m.can_unload()); // state is Registered, not Active
        m.state = ModuleState::Active;
        assert!(m.can_unload()); // Active + 0 refs
        m.add_ref();
        assert!(!m.can_unload()); // Has refs
        m.release_ref().unwrap();
        assert!(m.can_unload());
    }

    #[test]
    fn test_descriptor_params() {
        let mut m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_param(ModuleParam::new("debug", ParamType::Bool, "false", "Debug mode"))
            .with_param(ModuleParam::new("size", ParamType::Uint, "1024", "Buffer size"));

        assert!(m.set_param("debug", "true").is_ok());
        assert_eq!(m.get_param("debug").unwrap().current_value, "true");

        assert!(m.set_param("size", "2048").is_ok());
        assert_eq!(m.get_param("size").unwrap().current_value, "2048");

        assert!(m.set_param("nonexistent", "value").is_err());
        assert!(m.set_param("debug", "invalid").is_err());
    }

    #[test]
    fn test_descriptor_reset_params() {
        let mut m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_param(ModuleParam::new("size", ParamType::Uint, "1024", "Size"));

        m.set_param("size", "8192").unwrap();
        assert!(!m.get_param("size").unwrap().is_default());

        m.reset_params();
        assert!(m.get_param("size").unwrap().is_default());
    }

    #[test]
    fn test_descriptor_report() {
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_description("A test module")
            .with_author("Tester")
            .with_priority(ModulePriority::Core)
            .with_dependencies(&["dep1"]);

        let report = m.report();
        assert!(report.contains("testmod v1.0.0"));
        assert!(report.contains("A test module"));
        assert!(report.contains("core"));
        assert!(report.contains("dep1"));
    }

    // ── DependencyGraph Tests ──

    #[test]
    fn test_dep_graph_basic() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert!(g.has_node("a"));
        assert!(g.has_node("b"));
        assert!(g.has_node("c"));
    }

    #[test]
    fn test_dep_graph_dependencies() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("a", "c");
        g.add_edge("b", "c");

        let deps_a = g.get_dependencies("a");
        assert_eq!(deps_a.len(), 2);
        assert!(deps_a.contains(&"b".to_string()));
        assert!(deps_a.contains(&"c".to_string()));

        let deps_b = g.get_dependencies("b");
        assert_eq!(deps_b, vec!["c"]);
    }

    #[test]
    fn test_dep_graph_dependents() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("c", "b");

        let dependents_b = g.get_dependents("b");
        assert_eq!(dependents_b.len(), 2);
        assert!(dependents_b.contains(&"a".to_string()));
        assert!(dependents_b.contains(&"c".to_string()));
    }

    #[test]
    fn test_dep_graph_no_circular() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        g.add_edge("c", "d");
        assert!(g.has_circular().is_none());
    }

    #[test]
    fn test_dep_graph_circular_simple() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "a");
        let cycle = g.has_circular();
        assert!(cycle.is_some());
        let cycle = cycle.unwrap();
        assert!(cycle.len() >= 2);
        assert!(cycle.contains(&"a".to_string()));
        assert!(cycle.contains(&"b".to_string()));
    }

    #[test]
    fn test_dep_graph_circular_complex() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        g.add_edge("c", "d");
        g.add_edge("d", "b"); // b → c → d → b
        let cycle = g.has_circular();
        assert!(cycle.is_some());
    }

    #[test]
    fn test_dep_graph_topo_sort() {
        let mut g = DependencyGraph::new();
        g.add_edge("c", "b"); // c depends on b
        g.add_edge("b", "a"); // b depends on a
        let order = g.topological_sort().unwrap();
        let pos_a = order.iter().position(|n| n == "a").unwrap();
        let pos_b = order.iter().position(|n| n == "b").unwrap();
        let pos_c = order.iter().position(|n| n == "c").unwrap();
        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }

    #[test]
    fn test_dep_graph_topo_sort_with_cycle() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("b", "a");
        assert!(g.topological_sort().is_err());
    }

    #[test]
    fn test_dep_graph_load_order() {
        let mut g = DependencyGraph::new();
        g.add_edge("c", "b");
        g.add_edge("b", "a");
        let order = g.load_order("c").unwrap();
        assert_eq!(order.len(), 3);
        assert_eq!(order[0], "a");
        assert_eq!(order[1], "b");
        assert_eq!(order[2], "c");
    }

    #[test]
    fn test_dep_graph_load_order_missing() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "nonexistent");
        assert!(g.load_order("a").is_err());
    }

    #[test]
    fn test_dep_graph_remove_node() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "b");
        g.add_edge("c", "b");
        g.remove_node("b");
        assert!(!g.has_node("b"));
        assert!(!g.get_dependencies("a").contains(&"b".to_string()));
        assert!(!g.get_dependencies("c").contains(&"b".to_string()));
    }

    #[test]
    fn test_dep_graph_self_loop() {
        let mut g = DependencyGraph::new();
        g.add_edge("a", "a");
        let cycle = g.has_circular();
        assert!(cycle.is_some());
    }

    // ── SymbolTable Tests ──

    #[test]
    fn test_symbol_table_register() {
        let mut st = SymbolTable::new();
        let s = ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function);
        assert!(st.register(s).is_ok());
        assert_eq!(st.symbol_count(), 1);
        assert!(st.contains("kmalloc"));
    }

    #[test]
    fn test_symbol_table_duplicate() {
        let mut st = SymbolTable::new();
        let s1 = ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function);
        let s2 = ExportedSymbol::new("kmalloc", 2, "other", SymbolType::Function);
        assert!(st.register(s1).is_ok());
        assert!(st.register(s2).is_err());
    }

    #[test]
    fn test_symbol_table_lookup() {
        let mut st = SymbolTable::new();
        let s = ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function);
        st.register(s).unwrap();
        let found = st.lookup("kmalloc");
        assert!(found.is_some());
        assert_eq!(found.unwrap().module_name, "kalloc");
        assert!(st.lookup("nonexistent").is_none());
    }

    #[test]
    fn test_symbol_table_resolve_release() {
        let mut st = SymbolTable::new();
        let s = ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function);
        st.register(s).unwrap();
        assert!(st.resolve("kmalloc").is_ok());
        assert_eq!(st.lookup("kmalloc").unwrap().ref_count, 1);
        assert!(st.resolve("kmalloc").is_ok());
        assert_eq!(st.lookup("kmalloc").unwrap().ref_count, 2);
        assert_eq!(st.release("kmalloc").unwrap(), 1);
        assert_eq!(st.release("kmalloc").unwrap(), 0);
        assert!(st.release("kmalloc").is_err());
    }

    #[test]
    fn test_symbol_table_unregister_module() {
        let mut st = SymbolTable::new();
        st.register(ExportedSymbol::new("fn1", 1, "mod1", SymbolType::Function)).unwrap();
        st.register(ExportedSymbol::new("fn2", 1, "mod1", SymbolType::Function)).unwrap();
        st.register(ExportedSymbol::new("fn3", 2, "mod2", SymbolType::Function)).unwrap();
        assert_eq!(st.symbol_count(), 3);

        let removed = st.unregister_module(1);
        assert_eq!(removed.len(), 2);
        assert_eq!(st.symbol_count(), 1);
        assert!(!st.contains("fn1"));
        assert!(!st.contains("fn2"));
        assert!(st.contains("fn3"));
    }

    #[test]
    fn test_symbol_table_unresolved_imports() {
        let mut st = SymbolTable::new();
        st.register(ExportedSymbol::new("kmalloc", 1, "kalloc", SymbolType::Function)).unwrap();
        st.register(ExportedSymbol::new("kfree", 1, "kalloc", SymbolType::Function)).unwrap();

        let imports = vec!["kmalloc".to_string(), "kfree".to_string(), "missing".to_string()];
        let unresolved = st.unresolved_imports(&imports);
        assert_eq!(unresolved, vec!["missing"]);
    }

    #[test]
    fn test_symbol_table_module_symbols() {
        let mut st = SymbolTable::new();
        st.register(ExportedSymbol::new("fn1", 1, "mod1", SymbolType::Function)).unwrap();
        st.register(ExportedSymbol::new("fn2", 1, "mod1", SymbolType::Function)).unwrap();
        st.register(ExportedSymbol::new("fn3", 2, "mod2", SymbolType::Function)).unwrap();

        let mod1_syms = st.module_symbols(1);
        assert_eq!(mod1_syms.len(), 2);
        assert!(mod1_syms.contains(&"fn1".to_string()));
        assert!(mod1_syms.contains(&"fn2".to_string()));
    }

    // ── ModuleRegistry Registration Tests ──

    #[test]
    fn test_registry_register() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_description("Test module");
        let id = reg.register(m).unwrap();
        assert!(id > 0);
        assert_eq!(reg.module_count(), 1);
        assert!(reg.contains("testmod"));
    }

    #[test]
    fn test_registry_register_duplicate() {
        let mut reg = ModuleRegistry::new();
        let m1 = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m1).unwrap();
        let m2 = ModuleDescriptor::new("testmod", "2.0.0");
        assert!(reg.register(m2).is_err());
    }

    #[test]
    fn test_registry_register_with_circular() {
        let mut reg = ModuleRegistry::new();
        // Register a → b → a circular
        let a = ModuleDescriptor::new("mod_a", "1.0.0").with_dependencies(&["mod_b"]);
        let b = ModuleDescriptor::new("mod_b", "1.0.0").with_dependencies(&["mod_a"]);
        reg.register(a).unwrap();
        assert!(reg.register(b).is_err());
    }

    #[test]
    fn test_registry_unregister() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        assert!(reg.unregister("testmod").is_ok());
        assert_eq!(reg.module_count(), 0);
    }

    #[test]
    fn test_registry_unregister_active_fails() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        let id = reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        assert!(reg.unregister("testmod").is_err());
    }

    #[test]
    fn test_registry_get_by_id() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        let id = reg.register(m).unwrap();
        let found = reg.get_by_id(id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "testmod");
    }

    // ── ModuleRegistry Loading Tests ──

    #[test]
    fn test_registry_load_simple() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        let id = reg.load("testmod").unwrap();
        assert_eq!(reg.get("testmod").unwrap().state, ModuleState::Active);
        assert_eq!(reg.active_count(), 1);
        assert_eq!(reg.get("testmod").unwrap().stats.load_count, 1);
    }

    #[test]
    fn test_registry_load_already_active() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        assert!(reg.load("testmod").is_err());
    }

    #[test]
    fn test_registry_load_with_deps() {
        let mut reg = ModuleRegistry::new();
        let dep = ModuleDescriptor::new("dep_mod", "1.0.0");
        let main = ModuleDescriptor::new("main_mod", "1.0.0")
            .with_dependencies(&["dep_mod"]);
        reg.register(dep).unwrap();
        reg.register(main).unwrap();

        // Load main — should auto-load dep first
        reg.load("main_mod").unwrap();
        assert_eq!(reg.get("dep_mod").unwrap().state, ModuleState::Active);
        assert_eq!(reg.get("main_mod").unwrap().state, ModuleState::Active);
    }

    #[test]
    fn test_registry_load_missing_dep() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_dependencies(&["nonexistent"]);
        reg.register(m).unwrap();
        assert!(reg.load("testmod").is_err());
    }

    #[test]
    fn test_registry_load_chain_deps() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0").with_dependencies(&["mod_a"]);
        let c = ModuleDescriptor::new("mod_c", "1.0.0").with_dependencies(&["mod_b"]);
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.register(c).unwrap();

        reg.load("mod_c").unwrap();
        assert!(reg.get("mod_a").unwrap().state.is_active());
        assert!(reg.get("mod_b").unwrap().state.is_active());
        assert!(reg.get("mod_c").unwrap().state.is_active());
    }

    #[test]
    fn test_registry_unload_simple() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        assert!(reg.unload("testmod").is_ok());
        assert_eq!(reg.get("testmod").unwrap().state, ModuleState::Unloaded);
        assert_eq!(reg.get("testmod").unwrap().stats.unload_count, 1);
    }

    #[test]
    fn test_registry_unload_not_active() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        assert!(reg.unload("testmod").is_err());
    }

    #[test]
    fn test_registry_unload_with_dependents() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0").with_dependencies(&["mod_a"]);
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.load("mod_b").unwrap();

        // Can't unload mod_a while mod_b is active
        assert!(reg.unload("mod_a").is_err());
    }

    #[test]
    fn test_registry_unload_with_refs() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        reg.acquire_ref("testmod").unwrap();
        assert!(reg.unload("testmod").is_err());
        reg.release_ref("testmod").unwrap();
        assert!(reg.unload("testmod").is_ok());
    }

    #[test]
    fn test_registry_reload() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        assert!(reg.reload("testmod").is_ok());
        assert_eq!(reg.get("testmod").unwrap().state, ModuleState::Active);
        assert_eq!(reg.get("testmod").unwrap().stats.load_count, 2);
        assert_eq!(reg.get("testmod").unwrap().stats.unload_count, 1);
    }

    // ── Reference Management Tests ──

    #[test]
    fn test_registry_acquire_release_ref() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();

        assert_eq!(reg.acquire_ref("testmod").unwrap(), 1);
        assert_eq!(reg.acquire_ref("testmod").unwrap(), 2);
        assert_eq!(reg.get("testmod").unwrap().ref_count, 2);
        assert_eq!(reg.release_ref("testmod").unwrap(), 1);
        assert_eq!(reg.release_ref("testmod").unwrap(), 0);
    }

    #[test]
    fn test_registry_acquire_ref_not_active() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        assert!(reg.acquire_ref("testmod").is_err());
    }

    #[test]
    fn test_registry_release_ref_too_many() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        reg.acquire_ref("testmod").unwrap();
        reg.release_ref("testmod").unwrap();
        assert!(reg.release_ref("testmod").is_err());
    }

    // ── Symbol Management Tests ──

    #[test]
    fn test_registry_symbol_registration() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("kalloc", "1.0.0")
            .with_export("kmalloc", SymbolType::Function)
            .with_export("kfree", SymbolType::Function);
        reg.register(m).unwrap();
        reg.load("kalloc").unwrap();

        assert!(reg.lookup_symbol("kmalloc").is_some());
        assert!(reg.lookup_symbol("kfree").is_some());
        assert_eq!(reg.symbol_count(), 2);
    }

    #[test]
    fn test_registry_symbol_unregister_on_unload() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("kalloc", "1.0.0")
            .with_export("kmalloc", SymbolType::Function);
        reg.register(m).unwrap();
        reg.load("kalloc").unwrap();
        assert_eq!(reg.symbol_count(), 1);

        reg.unload("kalloc").unwrap();
        assert_eq!(reg.symbol_count(), 0);
        assert!(reg.lookup_symbol("kmalloc").is_none());
    }

    #[test]
    fn test_registry_symbol_conflict() {
        let mut reg = ModuleRegistry::new();
        let m1 = ModuleDescriptor::new("mod1", "1.0.0")
            .with_export("shared_fn", SymbolType::Function);
        let m2 = ModuleDescriptor::new("mod2", "1.0.0")
            .with_export("shared_fn", SymbolType::Function);
        reg.register(m1).unwrap();
        reg.register(m2).unwrap();
        reg.load("mod1").unwrap();
        // Loading mod2 should fail — symbol conflict
        assert!(reg.load("mod2").is_err());
        assert_eq!(reg.get("mod2").unwrap().state, ModuleState::Failed);
    }

    // ── Auto-Load Tests ──

    #[test]
    fn test_registry_auto_load_all() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0").with_auto_load(true);
        let b = ModuleDescriptor::new("mod_b", "1.0.0")
            .with_auto_load(true)
            .with_dependencies(&["mod_a"]);
        let c = ModuleDescriptor::new("mod_c", "1.0.0"); // Not auto-load

        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.register(c).unwrap();

        let results = reg.auto_load_all();
        assert_eq!(results.len(), 2);
        assert!(reg.get("mod_a").unwrap().state.is_active());
        assert!(reg.get("mod_b").unwrap().state.is_active());
        assert!(!reg.get("mod_c").unwrap().state.is_active());
    }

    #[test]
    fn test_registry_auto_load_priority_order() {
        let mut reg = ModuleRegistry::new();
        let driver = ModuleDescriptor::new("drv", "1.0.0")
            .with_priority(ModulePriority::Driver)
            .with_auto_load(true);
        let core = ModuleDescriptor::new("core_mod", "1.0.0")
            .with_priority(ModulePriority::Core)
            .with_auto_load(true);
        let net = ModuleDescriptor::new("net_mod", "1.0.0")
            .with_priority(ModulePriority::Network)
            .with_auto_load(true);

        reg.register(driver).unwrap();
        reg.register(core).unwrap();
        reg.register(net).unwrap();

        reg.auto_load_all();

        // Core should have lower load order (loaded first)
        let core_order = reg.get("core_mod").unwrap().load_order;
        let drv_order = reg.get("drv").unwrap().load_order;
        let net_order = reg.get("net_mod").unwrap().load_order;
        assert!(core_order < drv_order);
        assert!(drv_order < net_order);
    }

    // ── Conflict Tests ──

    #[test]
    fn test_registry_conflict_detection() {
        let mut reg = ModuleRegistry::new();
        let m1 = ModuleDescriptor::new("mod1", "1.0.0");
        let m2 = ModuleDescriptor::new("mod2", "1.0.0")
            .with_conflicts(&["mod1"]);
        reg.register(m1).unwrap();
        reg.register(m2).unwrap();

        reg.load("mod1").unwrap();
        // mod2 conflicts with active mod1
        assert!(reg.load("mod2").is_err());
    }

    // ── Parameter Tests ──

    #[test]
    fn test_registry_set_param() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_param(ModuleParam::new("debug", ParamType::Bool, "false", "Debug"));
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();

        assert!(reg.set_param("testmod", "debug", "true").is_ok());
        assert_eq!(reg.get_param("testmod", "debug").unwrap(), "true");
    }

    #[test]
    fn test_registry_set_param_not_active() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_param(ModuleParam::new("debug", ParamType::Bool, "false", "Debug"));
        reg.register(m).unwrap();
        assert!(reg.set_param("testmod", "debug", "true").is_err());
    }

    // ── Taint Tests ──

    #[test]
    fn test_registry_proprietary_taints() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("prop_mod", "1.0.0")
            .with_license(ModuleLicense::Proprietary);
        reg.register(m).unwrap();
        reg.load("prop_mod").unwrap();
        assert!(reg.is_tainted());
    }

    #[test]
    fn test_registry_gpl_no_taint() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("gpl_mod", "1.0.0")
            .with_license(ModuleLicense::Gpl);
        reg.register(m).unwrap();
        reg.load("gpl_mod").unwrap();
        assert!(!reg.is_tainted());
    }

    // ── Query Tests ──

    #[test]
    fn test_registry_list_active() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0");
        let c = ModuleDescriptor::new("mod_c", "1.0.0");
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.register(c).unwrap();
        reg.load("mod_a").unwrap();
        reg.load("mod_c").unwrap();
        let active = reg.list_active();
        assert_eq!(active.len(), 2);
        // Should be sorted by load_order
        assert_eq!(active[0].name, "mod_a");
        assert_eq!(active[1].name, "mod_c");
    }

    #[test]
    fn test_registry_list_by_state() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0");
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.load("mod_a").unwrap();
        reg.unload("mod_a").unwrap();

        let active = reg.list_by_state(ModuleState::Active);
        assert_eq!(active.len(), 0);
        let unloaded = reg.list_by_state(ModuleState::Unloaded);
        assert_eq!(unloaded.len(), 1);
        let registered = reg.list_by_state(ModuleState::Registered);
        assert_eq!(registered.len(), 1);
    }

    #[test]
    fn test_registry_list_by_priority() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0")
            .with_priority(ModulePriority::Core);
        let b = ModuleDescriptor::new("mod_b", "1.0.0")
            .with_priority(ModulePriority::Driver);
        let c = ModuleDescriptor::new("mod_c", "1.0.0")
            .with_priority(ModulePriority::Core);
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.register(c).unwrap();

        let core_mods = reg.list_by_priority(ModulePriority::Core);
        assert_eq!(core_mods.len(), 2);
    }

    // ── Event Tests ──

    #[test]
    fn test_registry_events_logged() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        reg.unload("testmod").unwrap();

        let events = reg.events();
        // Should have: Registered, LoadStarted, LoadSucceeded, UnloadStarted, UnloadSucceeded
        assert!(events.len() >= 5);
    }

    #[test]
    fn test_registry_events_for_module() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0");
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.load("mod_a").unwrap();

        let a_events = reg.events_for_module("mod_a");
        let b_events = reg.events_for_module("mod_b");
        assert!(a_events.len() > b_events.len());
    }

    #[test]
    fn test_registry_clear_events() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        assert!(reg.event_count() > 0);
        reg.clear_events();
        assert_eq!(reg.event_count(), 0);
    }

    #[test]
    fn test_registry_event_overflow() {
        let mut reg = ModuleRegistry::new().with_max_events(5);
        for i in 0..10 {
            let m = ModuleDescriptor::new(&format!("mod_{}", i), "1.0.0");
            reg.register(m).unwrap();
        }
        assert!(reg.event_count() <= 5);
    }

    // ── Report Tests ──

    #[test]
    fn test_registry_report() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0")
            .with_description("Test module");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        let report = reg.report();
        assert!(report.contains("Module Registry Report"));
        assert!(report.contains("testmod"));
        assert!(report.contains("Active: 1"));
    }

    #[test]
    fn test_registry_dependency_report() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("mod_a", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0").with_dependencies(&["mod_a"]);
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        let report = reg.dependency_report();
        assert!(report.contains("Dependency Graph"));
        assert!(report.contains("mod_a"));
        assert!(report.contains("No circular"));
    }

    #[test]
    fn test_registry_symbol_report() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("kalloc", "1.0.0")
            .with_export("kmalloc", SymbolType::Function)
            .with_export("kfree", SymbolType::Function);
        reg.register(m).unwrap();
        reg.load("kalloc").unwrap();
        let report = reg.symbol_report();
        assert!(report.contains("Symbol Table"));
        assert!(report.contains("kmalloc"));
        assert!(report.contains("kfree"));
    }

    #[test]
    fn test_registry_event_report() {
        let mut reg = ModuleRegistry::new();
        let m = ModuleDescriptor::new("testmod", "1.0.0");
        reg.register(m).unwrap();
        reg.load("testmod").unwrap();
        let report = reg.event_report(10);
        assert!(report.contains("Module Events"));
    }

    // ── ModuleBuilder Tests ──

    #[test]
    fn test_module_builder() {
        let m = ModuleBuilder::new("my_mod", "2.0.0")
            .description("My custom module")
            .author("Me")
            .license(ModuleLicense::Mit)
            .priority(ModulePriority::Custom)
            .depends_on(&["dep1"])
            .optional_depends_on(&["opt1"])
            .conflicts_with(&["bad_mod"])
            .provides(&["my_service"])
            .param("debug", ParamType::Bool, "false", "Debug mode")
            .readonly_param("version", ParamType::String, "2.0.0", "Module version")
            .export("my_fn", SymbolType::Function)
            .import_symbol("external_fn")
            .auto_load()
            .build();

        assert_eq!(m.name, "my_mod");
        assert_eq!(m.version, "2.0.0");
        assert_eq!(m.description, "My custom module");
        assert_eq!(m.author, "Me");
        assert_eq!(m.license, ModuleLicense::Mit);
        assert_eq!(m.priority, ModulePriority::Custom);
        assert_eq!(m.dependencies, vec!["dep1"]);
        assert_eq!(m.optional_deps, vec!["opt1"]);
        assert_eq!(m.conflicts, vec!["bad_mod"]);
        assert_eq!(m.provides, vec!["my_service"]);
        assert_eq!(m.params.len(), 2);
        assert_eq!(m.exports.len(), 1);
        assert!(m.auto_load);
    }

    // ── Builtin Modules Tests ──

    #[test]
    fn test_builtin_modules_created() {
        let modules = create_builtin_modules();
        assert_eq!(modules.len(), 10);

        // Check all have unique names
        let names: Vec<&str> = modules.iter().map(|m| m.name.as_str()).collect();
        let unique: BTreeSet<&str> = names.iter().cloned().collect();
        assert_eq!(names.len(), unique.len());
    }

    #[test]
    fn test_builtin_modules_structure() {
        let modules = create_builtin_modules();

        // kalloc — core, no deps
        let kalloc = modules.iter().find(|m| m.name == "kalloc").unwrap();
        assert_eq!(kalloc.priority, ModulePriority::Core);
        assert!(kalloc.dependencies.is_empty());
        assert!(kalloc.auto_load);
        assert!(kalloc.exports.iter().any(|e| e.name == "kmalloc"));

        // ksched — core, depends on kalloc
        let ksched = modules.iter().find(|m| m.name == "ksched").unwrap();
        assert_eq!(ksched.priority, ModulePriority::Core);
        assert_eq!(ksched.dependencies, vec!["kalloc"]);

        // atcfs — filesystem, depends on kalloc + blkdev
        let atcfs = modules.iter().find(|m| m.name == "atcfs").unwrap();
        assert_eq!(atcfs.priority, ModulePriority::FileSystem);
        assert!(atcfs.dependencies.contains(&"blkdev".to_string()));
        assert!(atcfs.dependencies.contains(&"kalloc".to_string()));
    }

    #[test]
    fn test_builtin_modules_auto_load() {
        let modules = create_builtin_modules();
        let auto_count = modules.iter().filter(|m| m.auto_load).count();
        assert!(auto_count >= 8); // Most are auto-load
    }

    #[test]
    fn test_builtin_modules_load_all() {
        let mut reg = ModuleRegistry::new();
        for m in create_builtin_modules() {
            reg.register(m).unwrap();
        }
        let results = reg.auto_load_all();
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        assert!(success_count >= 8);
        assert!(reg.active_count() >= 8);
    }

    #[test]
    fn test_builtin_modules_symbol_table() {
        let mut reg = ModuleRegistry::new();
        for m in create_builtin_modules() {
            reg.register(m).unwrap();
        }
        reg.auto_load_all();

        // kmalloc should be in symbol table
        assert!(reg.lookup_symbol("kmalloc").is_some());
        assert!(reg.lookup_symbol("kfree").is_some());
        assert!(reg.lookup_symbol("schedule").is_some());
        assert!(reg.lookup_symbol("tcp_connect").is_some());
        assert!(reg.lookup_symbol("cap_check").is_some());
    }

    #[test]
    fn test_builtin_modules_dep_chain() {
        let mut reg = ModuleRegistry::new();
        for m in create_builtin_modules() {
            reg.register(m).unwrap();
        }

        // Load tcpip — should cascade: kalloc → netdev → tcpip
        reg.load("tcpip").unwrap();
        assert!(reg.get("kalloc").unwrap().state.is_active());
        assert!(reg.get("netdev").unwrap().state.is_active());
        assert!(reg.get("tcpip").unwrap().state.is_active());
    }

    #[test]
    fn test_builtin_modules_no_circular() {
        let mut reg = ModuleRegistry::new();
        for m in create_builtin_modules() {
            let result = reg.register(m);
            assert!(result.is_ok(), "Failed to register: {:?}", result.err());
        }
        assert!(reg.dependency_graph().has_circular().is_none());
    }

    #[test]
    fn test_builtin_modules_unload_cascade() {
        let mut reg = ModuleRegistry::new();
        for m in create_builtin_modules() {
            reg.register(m).unwrap();
        }
        reg.auto_load_all();

        // Unload netdev — should fail because tcpip depends on it
        assert!(reg.unload("netdev").is_err());

        // Unload tcpip first
        assert!(reg.unload("tcpip").is_ok());
        // Now netdev should be unloadable (if no other deps)
        // kcontainer might depend on kalloc, but not netdev
        assert!(reg.unload("netdev").is_ok());
    }

    // ── Complex Scenario Tests ──

    #[test]
    fn test_full_lifecycle() {
        let mut reg = ModuleRegistry::new();

        // Register
        let m = ModuleDescriptor::new("lifecycle_mod", "1.0.0")
            .with_export("init_fn", SymbolType::Function)
            .with_param(ModuleParam::new("mode", ParamType::String, "normal", "Operation mode"));
        reg.register(m).unwrap();
        assert_eq!(reg.get("lifecycle_mod").unwrap().state, ModuleState::Registered);

        // Load
        reg.load("lifecycle_mod").unwrap();
        assert_eq!(reg.get("lifecycle_mod").unwrap().state, ModuleState::Active);

        // Set param
        reg.set_param("lifecycle_mod", "mode", "turbo").unwrap();
        assert_eq!(reg.get_param("lifecycle_mod", "mode").unwrap(), "turbo");

        // Acquire ref
        reg.acquire_ref("lifecycle_mod").unwrap();
        assert!(reg.unload("lifecycle_mod").is_err()); // Can't unload with refs

        // Release ref
        reg.release_ref("lifecycle_mod").unwrap();

        // Unload
        reg.unload("lifecycle_mod").unwrap();
        assert_eq!(reg.get("lifecycle_mod").unwrap().state, ModuleState::Unloaded);

        // Params should be reset
        assert_eq!(reg.get_param("lifecycle_mod", "mode").unwrap(), "normal");

        // Reload
        let module = reg.get_mut("lifecycle_mod").unwrap();
        module.state = ModuleState::Registered;
        reg.load("lifecycle_mod").unwrap();
        assert_eq!(reg.get("lifecycle_mod").unwrap().state, ModuleState::Active);
        assert_eq!(reg.get("lifecycle_mod").unwrap().stats.load_count, 2);
    }

    #[test]
    fn test_diamond_dependency() {
        // A → B → D, A → C → D
        let mut reg = ModuleRegistry::new();
        let d = ModuleDescriptor::new("mod_d", "1.0.0");
        let b = ModuleDescriptor::new("mod_b", "1.0.0").with_dependencies(&["mod_d"]);
        let c = ModuleDescriptor::new("mod_c", "1.0.0").with_dependencies(&["mod_d"]);
        let a = ModuleDescriptor::new("mod_a", "1.0.0").with_dependencies(&["mod_b", "mod_c"]);

        reg.register(d).unwrap();
        reg.register(b).unwrap();
        reg.register(c).unwrap();
        reg.register(a).unwrap();

        reg.load("mod_a").unwrap();
        assert!(reg.get("mod_d").unwrap().state.is_active());
        assert!(reg.get("mod_b").unwrap().state.is_active());
        assert!(reg.get("mod_c").unwrap().state.is_active());
        assert!(reg.get("mod_a").unwrap().state.is_active());
    }

    #[test]
    fn test_unload_order_validation() {
        let mut reg = ModuleRegistry::new();
        let a = ModuleDescriptor::new("base_mod", "1.0.0")
            .with_export("base_fn", SymbolType::Function);
        let b = ModuleDescriptor::new("user_mod", "1.0.0")
            .with_dependencies(&["base_mod"])
            .import_symbol("base_fn");
        reg.register(a).unwrap();
        reg.register(b).unwrap();
        reg.load("user_mod").unwrap();

        // Can't unload base while user depends on it
        assert!(reg.unload("base_mod").is_err());
        // Can unload user
        assert!(reg.unload("user_mod").is_ok());
        // Now can unload base
        assert!(reg.unload("base_mod").is_ok());
    }

    #[test]
    fn test_multiple_modules_same_priority() {
        let mut reg = ModuleRegistry::new();
        for i in 0..5 {
            let m = ModuleDescriptor::new(&format!("mod_{}", i), "1.0.0")
                .with_priority(ModulePriority::Utility)
                .with_auto_load(true);
            reg.register(m).unwrap();
        }
        reg.auto_load_all();
        assert_eq!(reg.active_count(), 5);
    }
}