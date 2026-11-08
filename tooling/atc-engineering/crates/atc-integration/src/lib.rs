//! Deterministic integration graph for the A-TownChain ecosystem.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum System {
    Standards,
    Engineering,
    OrgGithub,
    OsDocs,
    ShivaCore,
    GlobusOs,
    AtcLang,
    AtcVm,
    AtcTownchain,
    AuroraAi,
    GenesisEngine,
    GenesisFranchiseFactory,
    AtcWallet,
    AtcSdk,
    AtcNode,
    AtcContracts,
    AtcStorage,
    AtcIndexer,
    AtcOracle,
    AtcInterop,
    AtcZkp,
    AtcCompute,
    AtcAlgorithm,
    AtcMining,
    AtcExplorer,
    AtcMarketplace,
    AtcLaunchpad,
    AtcIde,
}

impl System {
    pub const fn repository(self) -> &'static str {
        match self {
            Self::Standards => "atc-standards",
            Self::Engineering => "atc-engineering",
            Self::OrgGithub => ".github",
            Self::OsDocs => "a-townchain-os-docs",
            Self::ShivaCore => "atc-shivacore",
            Self::GlobusOs => "globus-os",
            Self::AtcLang => "atclang",
            Self::AtcVm => "atc-vm",
            Self::AtcTownchain => "a-townchain",
            Self::AuroraAi => "aurora-ai",
            Self::GenesisEngine => "genesis-engine",
            Self::GenesisFranchiseFactory => "genesis-franchise-factory",
            Self::AtcWallet => "atc-wallet",
            Self::AtcSdk => "atc-sdk",
            Self::AtcNode => "atc-node",
            Self::AtcContracts => "atc-contracts",
            Self::AtcStorage => "atc-storage",
            Self::AtcIndexer => "atc-indexer",
            Self::AtcOracle => "atc-oracle",
            Self::AtcInterop => "atc-interop",
            Self::AtcZkp => "atc-zkp",
            Self::AtcCompute => "atc-compute",
            Self::AtcAlgorithm => "atc-algorithm",
            Self::AtcMining => "atc-mining",
            Self::AtcExplorer => "atc-explorer",
            Self::AtcMarketplace => "atc-marketplace",
            Self::AtcLaunchpad => "atc-launchpad",
            Self::AtcIde => "atc-ide",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readiness {
    Planned,
    Integrated,
    Verified,
    Production,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: System,
    pub to: System,
    pub contract: &'static str,
}

#[derive(Debug, Default)]
pub struct IntegrationGraph {
    nodes: BTreeSet<System>,
    edges: Vec<Edge>,
    readiness: BTreeMap<System, Readiness>,
}

impl IntegrationGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_system(&mut self, system: System, readiness: Readiness) {
        self.nodes.insert(system);
        self.readiness.insert(system, readiness);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.nodes.insert(edge.from);
        self.nodes.insert(edge.to);
        self.edges.push(edge);
    }

    pub fn systems(&self) -> impl Iterator<Item = System> + '_ {
        self.nodes.iter().copied()
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn readiness(&self, system: System) -> Option<Readiness> {
        self.readiness.get(&system).copied()
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        for edge in &self.edges {
            if edge.contract.is_empty() {
                return Err("integration edge has no contract");
            }
            if !self.nodes.contains(&edge.from) || !self.nodes.contains(&edge.to) {
                return Err("integration edge references an unknown system");
            }
        }
        Ok(())
    }

    pub fn dependency_order(&self) -> Result<Vec<System>, &'static str> {
        self.validate()?;
        let mut indegree: BTreeMap<System, usize> = self.nodes.iter().map(|n| (*n, 0)).collect();
        let mut outgoing: BTreeMap<System, Vec<System>> = BTreeMap::new();
        for edge in &self.edges {
            *indegree.get_mut(&edge.to).ok_or("missing edge target")? += 1;
            outgoing.entry(edge.from).or_default().push(edge.to);
        }
        for values in outgoing.values_mut() {
            values.sort();
        }
        let mut queue = VecDeque::new();
        for (node, degree) in &indegree {
            if *degree == 0 {
                queue.push_back(*node);
            }
        }
        let mut result = Vec::with_capacity(self.nodes.len());
        while let Some(node) = queue.pop_front() {
            result.push(node);
            if let Some(children) = outgoing.get(&node) {
                for child in children {
                    let degree = indegree.get_mut(child).ok_or("missing child")?;
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(*child);
                    }
                }
            }
        }
        if result.len() != self.nodes.len() {
            return Err("integration graph contains a dependency cycle");
        }
        Ok(result)
    }

    pub fn canonical() -> Self {
        let mut g = Self::new();
        for (system, readiness) in [
            (System::Standards, Readiness::Verified),
            (System::Engineering, Readiness::Integrated),
            (System::OrgGithub, Readiness::Integrated),
            (System::OsDocs, Readiness::Integrated),
            (System::ShivaCore, Readiness::Integrated),
            (System::GlobusOs, Readiness::Integrated),
            (System::AtcLang, Readiness::Integrated),
            (System::AtcVm, Readiness::Integrated),
            (System::AtcTownchain, Readiness::Integrated),
            (System::AuroraAi, Readiness::Integrated),
            (System::GenesisEngine, Readiness::Integrated),
            (System::GenesisFranchiseFactory, Readiness::Integrated),
            (System::AtcWallet, Readiness::Integrated),
            (System::AtcSdk, Readiness::Integrated),
            (System::AtcNode, Readiness::Integrated),
            (System::AtcContracts, Readiness::Integrated),
            (System::AtcStorage, Readiness::Integrated),
            (System::AtcIndexer, Readiness::Integrated),
            (System::AtcOracle, Readiness::Integrated),
            (System::AtcInterop, Readiness::Integrated),
            (System::AtcZkp, Readiness::Integrated),
            (System::AtcCompute, Readiness::Integrated),
            (System::AtcAlgorithm, Readiness::Integrated),
            (System::AtcMining, Readiness::Integrated),
            (System::AtcExplorer, Readiness::Integrated),
            (System::AtcMarketplace, Readiness::Integrated),
            (System::AtcLaunchpad, Readiness::Integrated),
            (System::AtcIde, Readiness::Integrated),
        ] {
            g.add_system(system, readiness);
        }
        for edge in [
            Edge {
                from: System::Standards,
                to: System::Engineering,
                contract: "standards-registry",
            },
            Edge {
                from: System::OrgGithub,
                to: System::Engineering,
                contract: "org-governance",
            },
            Edge {
                from: System::Standards,
                to: System::OsDocs,
                contract: "standards-documentation",
            },
            Edge {
                from: System::Engineering,
                to: System::GlobusOs,
                contract: "governed-build-evidence",
            },
            Edge {
                from: System::ShivaCore,
                to: System::GlobusOs,
                contract: "kernel-boundary",
            },
            Edge {
                from: System::AtcLang,
                to: System::AtcVm,
                contract: "bytecode-abi",
            },
            Edge {
                from: System::AtcVm,
                to: System::AtcTownchain,
                contract: "deterministic-execution",
            },
            Edge {
                from: System::AtcNode,
                to: System::AtcTownchain,
                contract: "node-runtime",
            },
            Edge {
                from: System::AtcContracts,
                to: System::AtcVm,
                contract: "contract-execution",
            },
            Edge {
                from: System::AtcStorage,
                to: System::AtcNode,
                contract: "persistent-state",
            },
            Edge {
                from: System::AtcIndexer,
                to: System::AtcTownchain,
                contract: "chain-events",
            },
            Edge {
                from: System::AtcOracle,
                to: System::AtcTownchain,
                contract: "external-data",
            },
            Edge {
                from: System::AtcInterop,
                to: System::AtcTownchain,
                contract: "cross-system-messaging",
            },
            Edge {
                from: System::AtcZkp,
                to: System::AtcVm,
                contract: "proof-verification",
            },
            Edge {
                from: System::AtcWallet,
                to: System::AtcTownchain,
                contract: "account-signing",
            },
            Edge {
                from: System::AtcSdk,
                to: System::AtcNode,
                contract: "client-api",
            },
            Edge {
                from: System::AuroraAi,
                to: System::GlobusOs,
                contract: "ai-runtime",
            },
            Edge {
                from: System::GenesisEngine,
                to: System::AtcSdk,
                contract: "game-chain-sdk",
            },
            Edge {
                from: System::GenesisFranchiseFactory,
                to: System::GenesisEngine,
                contract: "engine-content-pipeline",
            },
            Edge {
                from: System::AtcCompute,
                to: System::AuroraAi,
                contract: "ai-compute",
            },
            Edge {
                from: System::AtcAlgorithm,
                to: System::AtcTownchain,
                contract: "consensus-algorithms",
            },
            Edge {
                from: System::AtcMining,
                to: System::AtcTownchain,
                contract: "network-participation",
            },
            Edge {
                from: System::AtcExplorer,
                to: System::AtcIndexer,
                contract: "indexed-chain-api",
            },
            Edge {
                from: System::AtcMarketplace,
                to: System::AtcSdk,
                contract: "marketplace-sdk",
            },
            Edge {
                from: System::AtcLaunchpad,
                to: System::AtcSdk,
                contract: "launchpad-sdk",
            },
            Edge {
                from: System::AtcIde,
                to: System::AtcLang,
                contract: "language-toolchain",
            },
        ] {
            g.add_edge(edge);
        }
        g
    }
}
