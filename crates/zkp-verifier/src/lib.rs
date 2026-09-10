// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Proof-FORMAT-Verifikation (MVP: Struktur; KEINE Kryptografie — PLONK-Backend offen).

pub const ACCEPTED_SYSTEM_ID: u8 = 1;

pub fn verify_format(system_id: u8, proof_len: usize) -> bool {
    system_id == ACCEPTED_SYSTEM_ID && proof_len >= 41
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_gate() {
        assert!(verify_format(1, 41));
        assert!(!verify_format(2, 100));
        assert!(!verify_format(1, 10));
    }
}
