//! Deterministic economy, trade and currency foundation.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wallet { pub credits: u64 }

impl Wallet {
    pub fn deposit(&mut self, amount: u64) { self.credits = self.credits.saturating_add(amount); }
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.credits < amount { return false; }
        self.credits -= amount; true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeOffer { pub item_id: String, pub quantity: u32, pub price: u64 }
impl TradeOffer {
    pub fn total_price(&self) -> u64 { self.price.saturating_mul(self.quantity as u64) }
    pub fn valid(&self) -> bool { !self.item_id.is_empty() && self.quantity > 0 && self.price > 0 }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Market { pub offers: Vec<TradeOffer> }
impl Market {
    pub fn list(&mut self, offer: TradeOffer) -> bool {
        if !offer.valid() { return false; }
        self.offers.push(offer);
        self.offers.sort_by(|a,b| a.item_id.cmp(&b.item_id).then(a.price.cmp(&b.price)).then(a.quantity.cmp(&b.quantity)));
        true
    }
    pub fn find(&self, item_id: &str) -> Option<&TradeOffer> { self.offers.iter().find(|o| o.item_id == item_id) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn wallet_is_bounded() { let mut w=Wallet{credits:u64::MAX-1}; w.deposit(10); assert_eq!(w.credits,u64::MAX); assert!(!w.withdraw(1_000)); }
    #[test] fn trade_price_is_checked() { let o=TradeOffer{item_id:"wood".into(),quantity:3,price:10}; assert!(o.valid()); assert_eq!(o.total_price(),30); }
    #[test] fn market_order_is_deterministic() { let mut m=Market{offers:Vec::new()}; m.list(TradeOffer{item_id:"ore".into(),quantity:2,price:20}); m.list(TradeOffer{item_id:"ore".into(),quantity:1,price:10}); assert_eq!(m.find("ore").unwrap().price,10); }
}
