// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Service Space (AD-012/AD-028) — Service-Submodule AUSGELAGERT aus dem Kernel-Crate.
//! Diese Schicht baut auf dem Kernel auf (shivacore), niemals umgekehrt.
#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[cfg(test)]
extern crate std;

extern crate alloc;

pub mod ai;
pub mod atcnet;
pub mod blockchain;
pub mod consensus;
// container_net.rs: dormant (std-basiert, im Kernel ebenfalls nicht registriert).
// Aktivierung erfordert Portierung auf alloc/core + Ord-Derives.
pub mod contract;
pub mod did;
pub mod genesis;
pub mod genesis_bridge;
pub mod gossip_bridge;
pub mod identity_key_hal;
pub mod identity_key_service;
pub mod knowledge_graph;
pub mod mempool;
pub mod net;
pub mod p2p;
pub mod p2p_secure;
pub mod remote_caps;
pub mod security_audit;
pub mod service_init;
pub mod sockets;
pub mod tcpip;
pub mod vm;
