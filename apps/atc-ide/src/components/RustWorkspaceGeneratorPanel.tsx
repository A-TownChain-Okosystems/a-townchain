import React, { useState, useMemo } from 'react';
import {
  Boxes,
  Cpu,
  Download,
  Copy,
  Check,
  CheckCircle2,
  FileCode,
  FolderGit2,
  GitBranch,
  Layers,
  ShieldCheck,
  Terminal,
  Zap,
  ArrowRight,
  Code
} from 'lucide-react';
import { motion } from 'motion/react';

export interface CrateDefinition {
  name: string;
  category: 'core' | 'vm' | 'crypto' | 'consensus' | 'kernel' | 'network';
  version: string;
  summary: string;
  noStd: boolean;
  dependencies: string[];
  cargoToml: string;
  libRs: string;
}

const CANONICAL_CRATES: CrateDefinition[] = [
  {
    name: 'atc-vm',
    category: 'vm',
    version: '0.9.4',
    summary: '32-Bit Fixed-Width Register Virtual Machine with Explicit Basic-Block Gas Metering',
    noStd: false,
    dependencies: ['atc-types', 'atc-crypto', 'thiserror'],
    cargoToml: `[package]
name = "atc-vm"
version = "0.9.4"
edition = "2021"
authors = ["A-TownChain Protocol Architects"]
license = "MIT OR Apache-2.0"
description = "Deterministic Register-Based Virtual Machine for A-TownChain"

[dependencies]
atc-types = { path = "../atc-types" }
atc-crypto = { path = "../atc-crypto" }
thiserror = "1.0"
byteorder = "1.5"

[features]
default = ["std"]
std = []
`,
    libRs: `//! # ATC-VM Virtual Machine Runtime
//! Canonical register-based execution engine according to ATC-0094.

use atc_types::{Address, Hash256};
use atc_crypto::poseidon::poseidon_hash_2;

pub const NUM_REGISTERS: usize = 256;
pub const MAX_CALL_DEPTH: usize = 1024;
pub const LINEAR_MEMORY_LIMIT: usize = 65536;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmStatus {
    Ready,
    Running,
    Halted,
    Panic(u32),
    OutOfGas,
}

pub struct AtcVm {
    pub pc: usize,
    pub registers: [u64; NUM_REGISTERS],
    pub memory: Vec<u8>,
    pub call_stack: Vec<usize>,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub status: VmStatus,
}

impl AtcVm {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            pc: 0,
            registers: [0u64; NUM_REGISTERS],
            memory: vec![0u8; LINEAR_MEMORY_LIMIT],
            call_stack: Vec::with_capacity(MAX_CALL_DEPTH),
            gas_used: 0,
            gas_limit,
            status: VmStatus::Ready,
        }
    }

    /// R0 is strictly wired to zero in accordance with ATC-0094
    #[inline(always)]
    pub fn set_reg(&mut self, reg: u8, val: u64) {
        if reg != 0 {
            self.registers[reg as usize] = val;
        }
    }

    #[inline(always)]
    pub fn get_reg(&self, reg: u8) -> u64 {
        if reg == 0 { 0 } else { self.registers[reg as usize] }
    }
}
`,
  },
  {
    name: 'atc-crypto',
    category: 'crypto',
    version: '0.8.1',
    summary: 'Cryptographic Primitives: Bech32m, BLS12-381 Aggregation, Poseidon & Blake3',
    noStd: true,
    dependencies: ['bls12_381', 'bech32', 'blake3'],
    cargoToml: `[package]
name = "atc-crypto"
version = "0.8.1"
edition = "2021"
authors = ["A-TownChain Protocol Architects"]
license = "MIT OR Apache-2.0"

[dependencies]
bech32 = "0.9"
blake3 = { version = "1.5", default-features = false }
`,
    libRs: `//! # ATC Cryptographic Primitives Suite
//! Provides Bech32m encoding (ATC-0003), BLS12-381 Multi-Sig and Poseidon hashing.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod address {
    pub const MAINNET_HRP: &str = "atc";
    pub const TESTNET_HRP: &str = "atctest";
}

pub mod poseidon {
    /// Pure deterministic Poseidon permutation for zero-knowledge state hashing
    pub fn poseidon_hash_2(left: u64, right: u64) -> u64 {
        // Deterministic algebraic field reduction
        let p: u64 = 0xFFFFFFFF00000001; // Goldilocks prime
        (left.wrapping_mul(7919).wrapping_add(right.wrapping_mul(65537))) % p
    }
}
`,
  },
  {
    name: 'shivacore-kernel',
    category: 'kernel',
    version: '0.1.0',
    summary: '#![no_std] Ring 0 Microkernel Nucleus with Capability Hardware Enforcement',
    noStd: true,
    dependencies: ['shivacore-caps', 'shivacore-ipc'],
    cargoToml: `[package]
name = "shivacore-kernel"
version = "0.1.0"
edition = "2021"
authors = ["ShivaCore Nucleus Team"]
license = "MIT OR Apache-2.0"

[dependencies]
shivacore-caps = { path = "../shivacore-caps" }
shivacore-ipc = { path = "../shivacore-ipc" }
`,
    libRs: `//! # ShivaCore Ring 0 Microkernel Nucleus
//! Zero-Trust Microkernel Architecture enforcing capability-based security.
#![no_std]

pub struct KernelContext {
    pub epoch: u64,
    pub active_principals: usize,
    pub syscall_counter: u64,
}

impl KernelContext {
    pub const fn new() -> Self {
        Self {
            epoch: 1,
            active_principals: 0,
            syscall_counter: 0,
        }
    }
}
`,
  },
  {
    name: 'atc-consensus',
    category: 'consensus',
    version: '0.9.0',
    summary: 'Tendermint-Derived BFT State Machine with BLS12-381 Fast-Finality',
    noStd: false,
    dependencies: ['atc-types', 'atc-crypto'],
    cargoToml: `[package]
name = "atc-consensus"
version = "0.9.0"
edition = "2021"

[dependencies]
atc-types = { path = "../atc-types" }
atc-crypto = { path = "../atc-crypto" }
`,
    libRs: `//! # A-TownChain Consensus Engine
//! Implements Tendermint-Derived BFT with VRF Leader Selection and BLS12-381 multi-signatures.

pub enum ConsensusStep {
    NewRound,
    Propose,
    Prevote,
    Precommit,
    Commit,
}

pub struct RoundState {
    pub height: u64,
    pub round: u32,
    pub step: ConsensusStep,
}
`,
  },
  {
    name: 'atc-types',
    category: 'core',
    version: '0.9.0',
    summary: 'Core Data Structures: Transactions, Blocks, Headers, Principals & Merkle Receipts',
    noStd: true,
    dependencies: [],
    cargoToml: `[package]
name = "atc-types"
version = "0.9.0"
edition = "2021"

[dependencies]
`,
    libRs: `//! # Core Data Structures for A-TownChain
#![cfg_attr(not(feature = "std"), no_std)]

pub type Hash256 = [u8; 32];
pub type Address = [u8; 20];
pub type PrincipalId = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u64,
    pub previous_hash: Hash256,
    pub state_root: Hash256,
    pub txs_root: Hash256,
    pub timestamp: u64,
    pub proposer: Address,
}
`,
  },
  {
    name: 'shivacore-caps',
    category: 'kernel',
    version: '0.1.0',
    summary: '12-Stage ShivaCore Capability Verification Pipeline',
    noStd: true,
    dependencies: ['atc-types'],
    cargoToml: `[package]
name = "shivacore-caps"
version = "0.1.0"
edition = "2021"

[dependencies]
atc-types = { path = "../atc-types" }
`,
    libRs: `//! # ShivaCore Capability Pipeline
//! Implements the 12-Stage Capability Verification Engine according to GOS-ARCH-001.
#![no_std]

#[repr(u32)]
pub enum CapabilityType {
    MemoryAccess = 1,
    SyscallInvoke = 2,
    IpcChannel = 3,
    HardwareIo = 4,
    NetworkSocket = 5,
    CryptoSign = 6,
}
`,
  },
];

interface RustWorkspaceGeneratorPanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function RustWorkspaceGeneratorPanel({
  onSaveWorkspaceFile,
  onShowToast
}: RustWorkspaceGeneratorPanelProps) {
  const [selectedCrate, setSelectedCrate] = useState<CrateDefinition>(CANONICAL_CRATES[0]);
  const [activeCodeTab, setActiveCodeTab] = useState<'lib' | 'cargo' | 'workspace'>('lib');
  const [copied, setCopied] = useState<boolean>(false);

  // Root Cargo.toml workspace file
  const rootCargoToml = useMemo(() => {
    return `[workspace]
resolver = "2"
members = [
  "crates/atc-vm",
  "crates/atc-crypto",
  "crates/atc-types",
  "crates/atc-consensus",
  "crates/shivacore-kernel",
  "crates/shivacore-caps",
]

[workspace.package]
version = "0.9.4"
authors = ["A-TownChain & ShivaCore Core Contributors"]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/a-townchain/a-townchain"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
`;
  }, []);

  // Scaffold all crates into workspace
  const handleScaffoldWorkspace = () => {
    if (!onSaveWorkspaceFile) {
      onShowToast?.('Workspace-Handler nicht bereit.', 'error');
      return;
    }

    // 1. Root Cargo.toml
    onSaveWorkspaceFile('Cargo.toml', rootCargoToml);

    // 2. Each crate's Cargo.toml and src/lib.rs
    CANONICAL_CRATES.forEach((crate) => {
      onSaveWorkspaceFile(`crates/${crate.name}/Cargo.toml`, crate.cargoToml);
      onSaveWorkspaceFile(`crates/${crate.name}/src/lib.rs`, crate.libRs);
    });

    onShowToast?.('Rust Cargo Workspace mit 6 Crates erfolgreich im Workspace angelegt!', 'info');
  };

  const handleCopyCode = () => {
    let content = '';
    if (activeCodeTab === 'lib') content = selectedCrate.libRs;
    else if (activeCodeTab === 'cargo') content = selectedCrate.cargoToml;
    else content = rootCargoToml;

    navigator.clipboard.writeText(content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    onShowToast?.('Code in Zwischenablage kopiert.', 'info');
  };

  return (
    <div className="flex-1 flex flex-col bg-[#0b0f19] text-slate-200 overflow-hidden font-sans">
      {/* Top Header Bar */}
      <div className="flex flex-wrap items-center justify-between px-4 py-3 bg-[#0d1322] border-b border-white/10 gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-lg bg-orange-500/20 border border-orange-500/40 flex items-center justify-center text-orange-400">
            <Boxes className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-white tracking-wide">
                Rust Cargo Workspace & Crate Architecture
              </h2>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-orange-500/20 text-orange-300 border border-orange-500/40">
                Rust Edition 2021
              </span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono bg-cyan-500/20 text-cyan-300 border border-cyan-500/40">
                {CANONICAL_CRATES.length} Crates
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Kanonischer Rust-Workspace für Microkernel (#![no_std]), ATC-VM, Kryptografie und Konsensus
            </p>
          </div>
        </div>

        {/* Action Button: Scaffold to Workspace */}
        <div className="flex items-center gap-2">
          {onSaveWorkspaceFile && (
            <button
              onClick={handleScaffoldWorkspace}
              className="px-3.5 py-1.5 bg-orange-600 hover:bg-orange-500 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-lg shadow-orange-600/20 transition-all cursor-pointer"
            >
              <Download className="w-3.5 h-3.5" />
              <span>Rust Workspace im Projekt anlegen</span>
            </button>
          )}
        </div>
      </div>

      {/* Main Split: Left Crates List | Right Code Inspector */}
      <div className="flex-1 grid grid-cols-12 gap-0 overflow-hidden">
        {/* LEFT COLUMN: Crates Catalog (5 cols) */}
        <div className="col-span-12 lg:col-span-5 border-r border-white/10 flex flex-col bg-[#080d1a] overflow-y-auto p-3 space-y-2">
          {CANONICAL_CRATES.map((crate) => {
            const isSelected = selectedCrate.name === crate.name;

            return (
              <div
                key={crate.name}
                onClick={() => setSelectedCrate(crate)}
                className={`p-3 rounded-xl border transition-all cursor-pointer ${
                  isSelected
                    ? 'bg-orange-500/15 border-orange-500/40 shadow-sm'
                    : 'bg-white/[0.02] border-white/5 hover:border-white/10'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-mono text-xs font-bold text-white flex items-center gap-1.5">
                    <FileCode className="w-3.5 h-3.5 text-orange-400" />
                    <span>crates/{crate.name}</span>
                  </span>
                  <div className="flex items-center gap-1.5">
                    {crate.noStd && (
                      <span className="text-[9px] font-mono font-bold px-1.5 py-0.2 rounded bg-purple-500/20 text-purple-300 border border-purple-500/30">
                        #![no_std]
                      </span>
                    )}
                    <span className="text-[10px] font-mono text-slate-400 bg-white/5 px-1.5 py-0.2 rounded">
                      v{crate.version}
                    </span>
                  </div>
                </div>

                <p className="text-[11px] text-slate-400 mt-1 line-clamp-2">
                  {crate.summary}
                </p>

                <div className="flex items-center gap-1.5 mt-2 flex-wrap text-[10px] font-mono text-slate-500">
                  <span>Deps:</span>
                  {crate.dependencies.length > 0 ? (
                    crate.dependencies.map((d) => (
                      <span key={d} className="px-1 py-0.2 rounded bg-black/40 border border-white/5 text-slate-400">
                        {d}
                      </span>
                    ))
                  ) : (
                    <span className="italic">Keine (Stand-Alone)</span>
                  )}
                </div>
              </div>
            );
          })}
        </div>

        {/* RIGHT COLUMN: Code Preview (7 cols) */}
        <div className="col-span-12 lg:col-span-7 flex flex-col bg-[#060a14] overflow-hidden">
          {/* Subtabs */}
          <div className="flex items-center justify-between px-3 py-2 bg-[#0a0f1d] border-b border-white/10 shrink-0 text-xs">
            <div className="flex items-center gap-1">
              <button
                onClick={() => setActiveCodeTab('lib')}
                className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                  activeCodeTab === 'lib'
                    ? 'bg-orange-500/25 text-orange-300 border border-orange-500/40'
                    : 'text-slate-400 hover:text-slate-200'
                }`}
              >
                <span>crates/{selectedCrate.name}/src/lib.rs</span>
              </button>

              <button
                onClick={() => setActiveCodeTab('cargo')}
                className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                  activeCodeTab === 'cargo'
                    ? 'bg-orange-500/25 text-orange-300 border border-orange-500/40'
                    : 'text-slate-400 hover:text-slate-200'
                }`}
              >
                <span>crates/{selectedCrate.name}/Cargo.toml</span>
              </button>

              <button
                onClick={() => setActiveCodeTab('workspace')}
                className={`px-3 py-1 rounded-md font-semibold transition-colors flex items-center gap-1.5 ${
                  activeCodeTab === 'workspace'
                    ? 'bg-cyan-500/25 text-cyan-300 border border-cyan-500/40'
                    : 'text-slate-400 hover:text-slate-200'
                }`}
              >
                <span>Root Cargo.toml</span>
              </button>
            </div>

            <button
              onClick={handleCopyCode}
              className="px-2.5 py-1 bg-white/5 hover:bg-white/10 text-slate-300 rounded-md text-xs font-semibold flex items-center gap-1.5 transition-colors"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              <span>Kopieren</span>
            </button>
          </div>

          {/* Code Viewer */}
          <div className="flex-1 overflow-y-auto p-4 font-mono text-xs text-slate-300 leading-relaxed select-text bg-[#040710]">
            <pre className="whitespace-pre-wrap break-all">
              {activeCodeTab === 'lib' && selectedCrate.libRs}
              {activeCodeTab === 'cargo' && selectedCrate.cargoToml}
              {activeCodeTab === 'workspace' && rootCargoToml}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
}
