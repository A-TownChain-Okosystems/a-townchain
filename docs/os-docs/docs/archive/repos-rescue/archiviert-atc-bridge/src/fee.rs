// Bridge fee calculation
pub struct FeeCalculator { pub rate: u64 }
impl FeeCalculator {
    pub fn new(rate: u64) -> Self { Self { rate } }
    pub fn calculate(&self, amount: u64) -> u64 { amount * self.rate / 10000 }
    pub fn net(&self, amount: u64) -> u64 { amount - self.calculate(amount) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fee() {
        let f = FeeCalculator::new(30); // 0.3%
        assert_eq!(f.calculate(10000), 30);
        assert_eq!(f.net(10000), 9970);
    }
}
