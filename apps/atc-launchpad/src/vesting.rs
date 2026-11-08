// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Vesting Module — Token-Freischaltungspläne mit Cliff, Intervallen & Claim-Invarianten (LP-VEST-001).

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VestingError {
    InvalidSchedule,
    CliffNotReached,
    NothingToClaim,
    AllocationExceeded,
    Unauthorized,
}

#[derive(Debug, Clone)]
pub struct VestingSchedule {
    pub total_allocation: u128,
    pub start_slot: u64,
    pub cliff_slot: u64,
    pub duration_slots: u64,
    pub interval_slots: u64,
}

#[derive(Debug, Clone, Default)]
pub struct BeneficiaryState {
    pub total_allocation: u128,
    pub claimed_amount: u128,
    pub nonce: u64,
}

pub struct VestingManager {
    schedule: VestingSchedule,
    beneficiaries: HashMap<u64, BeneficiaryState>,
}

impl VestingManager {
    pub fn new(schedule: VestingSchedule) -> Result<Self, VestingError> {
        if schedule.duration_slots == 0 || schedule.interval_slots == 0 {
            return Err(VestingError::InvalidSchedule);
        }
        if schedule.cliff_slot < schedule.start_slot {
            return Err(VestingError::InvalidSchedule);
        }
        Ok(Self {
            schedule,
            beneficiaries: HashMap::new(),
        })
    }

    pub fn set_beneficiary_allocation(&mut self, beneficiary: u64, allocation: u128) {
        let entry = self.beneficiaries.entry(beneficiary).or_default();
        entry.total_allocation = allocation;
    }

    pub fn calculate_released(&self, beneficiary: u64, current_slot: u64) -> u128 {
        let state = match self.beneficiaries.get(&beneficiary) {
            Some(s) => s,
            None => return 0,
        };

        if current_slot < self.schedule.cliff_slot {
            return 0;
        }

        if current_slot >= self.schedule.start_slot.saturating_add(self.schedule.duration_slots) {
            return state.total_allocation;
        }

        let elapsed = current_slot.saturating_sub(self.schedule.start_slot);
        let intervals_passed = elapsed / self.schedule.interval_slots;
        let effective_elapsed = intervals_passed * self.schedule.interval_slots;

        (state.total_allocation as u128)
            .saturating_mul(effective_elapsed as u128)
            / (self.schedule.duration_slots as u128)
    }

    pub fn calculate_claimable(&self, beneficiary: u64, current_slot: u64) -> u128 {
        let released = self.calculate_released(beneficiary, current_slot);
        let claimed = self
            .beneficiaries
            .get(&beneficiary)
            .map(|s| s.claimed_amount)
            .unwrap_or(0);
        released.saturating_sub(claimed)
    }

    pub fn claim(&mut self, beneficiary: u64, current_slot: u64) -> Result<u128, VestingError> {
        if current_slot < self.schedule.cliff_slot {
            return Err(VestingError::CliffNotReached);
        }

        let claimable = self.calculate_claimable(beneficiary, current_slot);
        if claimable == 0 {
            return Err(VestingError::NothingToClaim);
        }

        let state = self
            .beneficiaries
            .get_mut(&beneficiary)
            .ok_or(VestingError::Unauthorized)?;

        state.claimed_amount = state.claimed_amount.saturating_add(claimable);
        state.nonce += 1;

        Ok(claimable)
    }

    pub fn get_beneficiary_state(&self, beneficiary: u64) -> Option<&BeneficiaryState> {
        self.beneficiaries.get(&beneficiary)
    }

    pub fn check_invariants(&self, beneficiary: u64, current_slot: u64) -> bool {
        if let Some(state) = self.beneficiaries.get(&beneficiary) {
            let released = self.calculate_released(beneficiary, current_slot);
            let claimable = self.calculate_claimable(beneficiary, current_slot);
            released <= state.total_allocation
                && claimable <= state.total_allocation.saturating_sub(state.claimed_amount)
                && state.claimed_amount.saturating_add(claimable) == released
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vesting_cliff_and_incremental_release() {
        let schedule = VestingSchedule {
            total_allocation: 1000,
            start_slot: 100,
            cliff_slot: 150,
            duration_slots: 100,
            interval_slots: 10,
        };
        let mut vm = VestingManager::new(schedule).unwrap();
        vm.set_beneficiary_allocation(1, 1000);

        // Before cliff
        assert_eq!(vm.calculate_released(1, 120), 0);
        assert_eq!(vm.claim(1, 120), Err(VestingError::CliffNotReached));

        // At cliff (slot 150 -> 50 slots passed out of 100 -> 50% released = 500)
        assert_eq!(vm.calculate_released(1, 150), 500);
        assert_eq!(vm.claim(1, 150), Ok(500));
        assert!(vm.check_invariants(1, 150));

        // No new claim at slot 154 (interval is 10, so 150..159 is same step)
        assert_eq!(vm.calculate_claimable(1, 154), 0);
        assert_eq!(vm.claim(1, 154), Err(VestingError::NothingToClaim));

        // Slot 160 -> 60 slots passed -> 600 released, claimed was 500 -> 100 claimable
        assert_eq!(vm.calculate_claimable(1, 160), 100);
        assert_eq!(vm.claim(1, 160), Ok(100));

        // Slot 200 (end of duration -> 100% released)
        assert_eq!(vm.calculate_claimable(1, 200), 400);
        assert_eq!(vm.claim(1, 200), Ok(400));

        let state = vm.get_beneficiary_state(1).unwrap();
        assert_eq!(state.claimed_amount, 1000);
        assert_eq!(state.nonce, 3);
        assert!(vm.check_invariants(1, 200));
    }

    #[test]
    fn test_invalid_schedule() {
        let schedule = VestingSchedule {
            total_allocation: 1000,
            start_slot: 100,
            cliff_slot: 50, // invalid cliff before start
            duration_slots: 100,
            interval_slots: 10,
        };
        assert_eq!(VestingManager::new(schedule).err(), Some(VestingError::InvalidSchedule));
    }
}
