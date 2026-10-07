//! Stable human-readable CLI formatting.
pub fn chain_id(v:u64)->String{v.to_string()}
pub fn boot_hash(v:u64)->String{v.to_string()}
pub fn peers(v:usize)->String{v.to_string()}
pub fn ping(v:&str)->String{v.to_owned()}
#[cfg(test)] mod tests{use super::*;#[test]fn format_is_exact(){assert_eq!(chain_id(658467),"658467");assert_eq!(peers(2),"2");assert_eq!(ping("pong"),"pong");}}
