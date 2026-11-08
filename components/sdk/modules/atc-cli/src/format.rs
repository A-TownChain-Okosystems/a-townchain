//! Stable human-readable CLI formatting.
pub fn chain_id(v:&str)->String{v.to_owned()}
pub fn boot_hash(v:&str)->String{v.to_owned()}
pub fn peers(v:usize)->String{v.to_string()}
pub fn ping(v:&str)->String{v.to_owned()}
#[cfg(test)] mod tests{use super::*;#[test]fn format_is_exact(){assert_eq!(chain_id("atc"),"atc");assert_eq!(peers(2),"2");assert_eq!(ping("pong"),"pong");assert_eq!(boot_hash("00ff"),"00ff");}}
