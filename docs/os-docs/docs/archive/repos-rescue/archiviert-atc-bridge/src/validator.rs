// Bridge validator set
pub struct BridgeValidator {
    validators: Vec<String>,
    threshold: usize,
}

impl BridgeValidator {
    pub fn new(validators: Vec<String>, threshold: usize) -> Self {
        Self { validators, threshold }
    }
    pub fn validate_signatures(&self, sigs: &[(String, Vec<u8>)]) -> Result<(), String> {
        let valid_count = sigs.iter()
            .filter(|(v, _)| self.validators.contains(v))
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
        let bv = BridgeValidator::new(vec!["a".into(),"b".into(),"c".into()], 2);
        assert!(bv.validate_signatures(&[("a".into(), vec![]), ("b".into(), vec![])]).is_ok());
        assert!(bv.validate_signatures(&[("a".into(), vec![])]).is_err());
    }
}
