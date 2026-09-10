// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Prime-Feld-Arithmetik (MVP; keine finale Kurven-/Feldwahl — ATC-CRYPTO-001 offen).

pub const FIELD_MODULUS: u64 = 0x3ffffffe40000001;

pub fn field_add(a: u64, b: u64) -> u64 {
    (a % FIELD_MODULUS + b % FIELD_MODULUS) % FIELD_MODULUS
}

pub fn field_mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % FIELD_MODULUS as u128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn feld_arithmetik() {
        assert_eq!(field_add(FIELD_MODULUS - 1, 2), 1);
        assert_eq!(field_mul(0, 12345), 0);
        assert_eq!(field_add(2, 3) + field_add(4, 5), field_add(field_add(2, 3), field_add(4, 5)));
    }
}
