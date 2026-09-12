// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC Node Runtime — Konfiguration + Peer-Tabelle (MVP, AD-024-Rebuild-Start).
//! Abhaengigkeit: Konsens KANONISCH ueber atc-algorithm — keine eigene
//! Konsens-Implementierung in atc-node (F-105-Regel).

pub mod bootstrap;
pub mod chain;
pub mod rpc;
pub mod config;
pub mod peers;
