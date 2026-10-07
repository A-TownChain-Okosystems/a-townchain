// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Bridge fee calculation

pub type Amount = u128;
pub type FeeRateBps = u64;

pub struct FeeCalculator {
    pub rate: FeeRateBps,
}

impl FeeCalculator {
    pub fn new(rate: FeeRateBps) -> Result<Self, String> {
        if rate > 10_000 {
            return Err("fee rate exceeds 10000 bps".into());
        }
        Ok(Self { rate })
    }

    pub fn calculate(&self, amount: Amount) -> Result<Amount, String> {
        let rate = self.rate as Amount;
        let base = amount / 10_000;
        let remainder = amount % 10_000;
        let whole_fee = base
            .checked_mul(rate)
            .ok_or_else(|| "fee calculation overflow".to_string())?;
        let remainder_fee = remainder
            .checked_mul(rate)
            .ok_or_else(|| "fee calculation overflow".to_string())?
            / 10_000;
        whole_fee
            .checked_add(remainder_fee)
            .ok_or_else(|| "fee calculation overflow".to_string())
    }

    pub fn net(&self, amount: Amount) -> Result<Amount, String> {
        let fee = self.calculate(amount)?;
        amount
            .checked_sub(fee)
            .ok_or_else(|| "fee exceeds amount".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee() {
        let f = FeeCalculator::new(30).unwrap();
        assert_eq!(f.calculate(10_000).unwrap(), 30);
        assert_eq!(f.net(10_000).unwrap(), 9_970);
    }

    #[test]
    fn max_amount_is_supported_without_fee_overflow() {
        let f = FeeCalculator::new(0).unwrap();
        assert_eq!(f.calculate(u128::MAX).unwrap(), 0);
        assert_eq!(f.net(u128::MAX).unwrap(), u128::MAX);
    }

    #[test]
    fn max_amount_at_max_rate_is_supported() {
        let f = FeeCalculator::new(10_000).unwrap();
        assert_eq!(f.calculate(u128::MAX).unwrap(), u128::MAX);
        assert_eq!(f.net(u128::MAX).unwrap(), 0);
    }

    #[test]
    fn max_rate_preserves_floor_semantics() {
        let f = FeeCalculator::new(9_999).unwrap();
        let amount = 12_345;
        assert_eq!(f.calculate(amount).unwrap(), 12_343);
    }

    #[test]
    fn invalid_rate_is_rejected() {
        assert!(FeeCalculator::new(10_001).is_err());
    }
}
