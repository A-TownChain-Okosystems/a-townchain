//! KAI-OS Keyring (S16-S18, Issue #107).
//!
//! Kernregel (KAI-CORE-RUNTIME-001 §7):
//! **Keys verlassen die Keyring-Boundary NIE.**
//!
//! Der Keyring veröffentlicht ausschließlich Public Keys und Signaturen.
//! Es existiert bewusst KEINE API, die privates Schlüsselmaterial nach
//! außen gibt. Beim Entfernen eines Keys wird das Secret beim Drop
//! gelöscht (ed25519-dalek: Zeroize-on-Drop des Secret-Scalars).

pub mod keyring;
