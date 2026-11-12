// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-System — der durchgehende Pfad des Oekosystems.
//!
//! ```text
//! ATCLang-Quelle
//!   -> atc-core    (Lexer, Parser, Lowering, Verifizierer)      Stufe 1
//!   -> atc-system  (Uebersetzer-Backend auf ATCB-Operationen)    Stufe 2
//!   -> atc-vm      (kanonisches Artefakt mit SHA-256)            Stufe 2
//!   -> atc-exec-admission (Freigabe + Signatur)                  Stufe 3
//!   -> atc-vm      (Ausfuehrung mit Gas, transaktional)          Stufe 4
//!   -> atc-node    (Block an die Kette)                          Stufe 5
//! ```
//!
//! Bis zu diesem Crate gab es zwischen diesen Stufen **keine Kante**: der
//! Compiler erzeugte Bytecode, den niemand ausfuehrte; die VM fuehrte
//! Programme aus, die niemand kompilierte; die Kette buchte Payloads ohne
//! Codebezug. Hier ist die Kante gebaut, und zwar nur in dieser Richtung.

pub mod backend;
pub mod block_payload;
pub mod compile;
pub mod seal;
pub mod state;
pub mod system;

pub use backend::{translate, BackendError, Translation, DIVERGENZEN};
pub use block_payload::{
    admit_payload, container_bytes, encode_container, is_container_payload, validate_payload,
    BlockPayloadError, PayloadKind, CONTAINER_TAG, INNER_TAG,
};
pub use compile::{compile, CompileError};
pub use seal::{AuthorizedKeyVerifier, SealError, SealKey};
pub use state::ContractState;
pub use system::{
    deploy_and_run, hex, Deployment, Receipt, System, SystemError, CONTRACT_RIGHTS, DEFAULT_GAS,
    DEVNET_SEAL_SEED, POLICY_TTL_US, POLICY_VERSION,
};
