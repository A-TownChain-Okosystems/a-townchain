// 32-bit/64-bit entry point
pub fn entry_point() -> u64 { 0x100000 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entry() { assert_eq!(entry_point(), 0x100000); }
}
