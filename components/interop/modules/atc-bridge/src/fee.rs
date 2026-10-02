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
        amount
            .checked_mul(rate)
            .ok_or_else(|| "fee calculation overflow".into())
            .map(|value| value / 10_000)
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
    fn multiplication_overflow_is_rejected() {
        let f = FeeCalculator::new(10_000).unwrap();
        assert!(f.calculate(u128::MAX).is_err());
    }

    #[test]
    fn invalid_rate_is_rejected() {
        assert!(FeeCalculator::new(10_001).is_err());
    }
}
