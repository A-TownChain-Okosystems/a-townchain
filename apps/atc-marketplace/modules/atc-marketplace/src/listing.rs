// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Listing module for atc-marketplace.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListingStatus {
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    pub listing_id: u64,
    pub seller: String,
    pub asset_id: u64,
    pub price: u128,
    pub expiry_height: u64,
    pub status: ListingStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListingError {
    InvalidPrice,
    Expired,
    ListingNotActive,
    Unauthorized,
}

impl Listing {
    pub fn new(
        listing_id: u64,
        seller: impl Into<String>,
        asset_id: u64,
        price: u128,
        expiry_height: u64,
        current_height: u64,
    ) -> Result<Self, ListingError> {
        if price == 0 {
            return Err(ListingError::InvalidPrice);
        }
        if expiry_height <= current_height {
            return Err(ListingError::Expired);
        }
        Ok(Self {
            listing_id,
            seller: seller.into(),
            asset_id,
            price,
            expiry_height,
            status: ListingStatus::Active,
        })
    }

    pub fn cancel(&mut self, caller: &str) -> Result<(), ListingError> {
        if self.seller != caller {
            return Err(ListingError::Unauthorized);
        }
        if self.status != ListingStatus::Active {
            return Err(ListingError::ListingNotActive);
        }
        self.status = ListingStatus::Cancelled;
        Ok(())
    }

    pub fn mark_filled(&mut self) -> Result<(), ListingError> {
        if self.status != ListingStatus::Active {
            return Err(ListingError::ListingNotActive);
        }
        self.status = ListingStatus::Filled;
        Ok(())
    }

    pub fn check_expiry(&mut self, current_height: u64) -> bool {
        if self.status == ListingStatus::Active && current_height >= self.expiry_height {
            self.status = ListingStatus::Expired;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listing_creation_and_cancellation() {
        let mut listing = Listing::new(1, "alice", 100, 1000, 50, 10).unwrap();
        assert_eq!(listing.status, ListingStatus::Active);

        // Unauthorized cancellation fails
        assert_eq!(listing.cancel("bob"), Err(ListingError::Unauthorized));

        // Authorized cancellation succeeds
        assert!(listing.cancel("alice").is_ok());
        assert_eq!(listing.status, ListingStatus::Cancelled);
    }

    #[test]
    fn test_listing_creation_invalid_price() {
        assert_eq!(
            Listing::new(1, "alice", 100, 0, 50, 10),
            Err(ListingError::InvalidPrice)
        );
    }

    #[test]
    fn test_listing_creation_expired_height() {
        assert_eq!(
            Listing::new(1, "alice", 100, 500, 10, 10),
            Err(ListingError::Expired)
        );
    }

    #[test]
    fn test_listing_expiry() {
        let mut listing = Listing::new(2, "alice", 101, 500, 30, 10).unwrap();
        assert!(!listing.check_expiry(20));
        assert_eq!(listing.status, ListingStatus::Active);

        assert!(listing.check_expiry(30));
        assert_eq!(listing.status, ListingStatus::Expired);
    }

    #[test]
    fn test_listing_mark_filled() {
        let mut listing = Listing::new(3, "alice", 102, 1200, 40, 10).unwrap();
        assert!(listing.mark_filled().is_ok());
        assert_eq!(listing.status, ListingStatus::Filled);
        assert_eq!(listing.mark_filled(), Err(ListingError::ListingNotActive));
    }
}
