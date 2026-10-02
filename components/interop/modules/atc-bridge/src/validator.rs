// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Bridge validator set
pub struct BridgeValidator {
    validators: Vec<String>,
    threshold: usize,
}

impl BridgeValidator {
    pub fn new(validators: Vec<String>, threshold: usize) -> Result<Self, String> {
        if validators.is_empty() {
            return Err("validator set must not be empty".into());
        }
        if threshold == 0 || threshold > validators.len() {
            return Err("validator threshold is outside validator set".into());
        }
        let mut unique = std::collections::HashSet::with_capacity(validators.len());
        if validators.iter().any(|validator| !unique.insert(validator)) {
            return Err("validator set contains duplicates".into());
        }
        Ok(Self { validators, threshold })
    }
    pub fn validate_signatures(&self, sigs: &[(String, Vec<u8>)]) -> Result<(), String> {
        let mut seen = std::collections::HashSet::new();
        let valid_count = sigs
            .iter()
            .filter(|(v, signature)| {
                !signature.is_empty() && self.validators.contains(v) && seen.insert(v)
            })
            .count();
        if valid_count < self.threshold {
            return Err(format!("Insufficient signatures: {}/{}", valid_count, self.threshold));
        }
        Ok(())
    }
    pub fn count(&self) -> usize { self.validators.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_validator() {
        let bv = BridgeValidator::new(vec!["a".into(),"b".into(),"c".into()], 2).unwrap();
        assert!(bv.validate_signatures(&[("a".into(), vec![1]), ("b".into(), vec![2])]).is_ok());
        assert!(bv.validate_signatures(&[("a".into(), vec![1])]).is_err());
        assert!(bv.validate_signatures(&[("a".into(), vec![]), ("b".into(), vec![2])]).is_err());
        assert!(bv.validate_signatures(&[("a".into(), vec![1]), ("a".into(), vec![2])]).is_err());
        assert!(BridgeValidator::new(vec!["a".into(),"b".into(),"a".into()], 1).is_err());
        assert!(BridgeValidator::new(vec!["a".into()], 0).is_err());
    }
}
