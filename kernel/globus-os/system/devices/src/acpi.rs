//! ACPI table discovery contracts and checksum validation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RsdpV1 {
    pub revision: u8,
    pub rsdt_address: u32,
    pub checksum: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RsdpV2 {
    pub revision: u8,
    pub rsdt_address: u32,
    pub xsdt_address: u64,
    pub length: u32,
    pub checksum: u8,
    pub extended_checksum: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiError {
    InvalidSignature,
    InvalidChecksum,
    InvalidLength,
    InvalidAddress,
}

pub fn checksum_valid(bytes: &[u8]) -> bool {
    bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b)) == 0
}

pub fn validate_rsdp(bytes: &[u8]) -> Result<(), AcpiError> {
    if bytes.len() < 8 {
        return Err(AcpiError::InvalidLength);
    }
    if &bytes[0..8] != b"RSD PTR " {
        return Err(AcpiError::InvalidSignature);
    }
    if bytes.len() < 20 {
        return Err(AcpiError::InvalidLength);
    }
    if !checksum_valid(&bytes[..20]) {
        return Err(AcpiError::InvalidChecksum);
    }
    if bytes[15] >= 2 {
        if bytes.len() < 36 {
            return Err(AcpiError::InvalidLength);
        }
        let length = u32::from_le_bytes(
            bytes[20..24]
                .try_into()
                .map_err(|_| AcpiError::InvalidLength)?,
        );
        if !(36..=4096).contains(&length) || length as usize > bytes.len() {
            return Err(AcpiError::InvalidLength);
        }
        if !checksum_valid(&bytes[..length as usize]) {
            return Err(AcpiError::InvalidChecksum);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McfgWindow {
    pub base_address: u64,
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
}

impl McfgWindow {
    /// Validate that this window describes a non-empty, aligned, non-overflowing ECAM region.
    pub fn validate(&self) -> Result<(), AcpiError> {
        if self.start_bus > self.end_bus {
            return Err(AcpiError::InvalidLength);
        }
        if self.base_address & 0xF_FFFF != 0 {
            return Err(AcpiError::InvalidAddress);
        }
        let bus_count = u64::from(self.end_bus) - u64::from(self.start_bus) + 1;
        let region_size = bus_count.checked_shl(20).ok_or(AcpiError::InvalidAddress)?;
        self.base_address
            .checked_add(region_size - 1)
            .ok_or(AcpiError::InvalidAddress)?;
        Ok(())
    }

    pub fn ecam_address(&self, bus: u8, device: u8, function: u8, offset: u16) -> Option<u64> {
        self.validate().ok()?;
        if bus < self.start_bus
            || bus > self.end_bus
            || device >= 32
            || function >= 8
            || offset >= 4096
            || offset & 3 != 0
        {
            return None;
        }
        let bus_delta = u64::from(bus - self.start_bus);
        let offset = (bus_delta << 20)
            | (u64::from(device) << 15)
            | (u64::from(function) << 12)
            | u64::from(offset);
        let address = self.base_address.checked_add(offset)?;
        let bus_count = u64::from(self.end_bus) - u64::from(self.start_bus) + 1;
        let region_size = bus_count.checked_shl(20)?;
        let region_end = self.base_address.checked_add(region_size - 1)?;
        (address <= region_end).then_some(address)
    }
}

/// Validate the fixed header and entry layout of an ACPI MCFG table.
/// The function intentionally validates bytes only; it never dereferences firmware addresses.
pub fn validate_mcfg_table(bytes: &[u8]) -> Result<(), AcpiError> {
    const ACPI_HEADER_LEN: usize = 36;
    const MCFG_FIXED_LEN: usize = 44;
    const MCFG_ENTRY_LEN: usize = 16;

    if bytes.len() < ACPI_HEADER_LEN {
        return Err(AcpiError::InvalidLength);
    }
    if &bytes[0..4] != b"MCFG" {
        return Err(AcpiError::InvalidSignature);
    }
    let length = u32::from_le_bytes(
        bytes[4..8]
            .try_into()
            .map_err(|_| AcpiError::InvalidLength)?,
    ) as usize;
    if length <= MCFG_FIXED_LEN || length > bytes.len() {
        return Err(AcpiError::InvalidLength);
    }
    if !(length - MCFG_FIXED_LEN).is_multiple_of(MCFG_ENTRY_LEN) {
        return Err(AcpiError::InvalidLength);
    }
    // ACPI MCFG reserves bytes 36..44 of the table header; they must be zero.
    if bytes[ACPI_HEADER_LEN..MCFG_FIXED_LEN]
        .iter()
        .any(|byte| *byte != 0)
    {
        return Err(AcpiError::InvalidLength);
    }
    if !checksum_valid(&bytes[..length]) {
        return Err(AcpiError::InvalidChecksum);
    }

    let (entries, remainder) = bytes[MCFG_FIXED_LEN..length].as_chunks::<MCFG_ENTRY_LEN>();
    if !remainder.is_empty() {
        return Err(AcpiError::InvalidLength);
    }
    for entry in entries {
        // Each MCFG allocation structure reserves bytes 12..16; they must be zero.
        if entry[12..16].iter().any(|byte| *byte != 0) {
            return Err(AcpiError::InvalidLength);
        }
        let base = u64::from_le_bytes(
            entry[0..8]
                .try_into()
                .map_err(|_| AcpiError::InvalidLength)?,
        );
        let start_bus = entry[8];
        let end_bus = entry[9];
        McfgWindow {
            base_address: base,
            segment: u16::from_le_bytes([entry[10], entry[11]]),
            start_bus,
            end_bus,
        }
        .validate()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fix_checksum(bytes: &mut [u8], index: usize, covered_len: usize) {
        bytes[index] = 0;
        let sum = bytes[..covered_len]
            .iter()
            .fold(0u8, |sum, byte| sum.wrapping_add(*byte));
        bytes[index] = 0u8.wrapping_sub(sum);
    }

    fn valid_rsdp_v1() -> Vec<u8> {
        let mut bytes = vec![0u8; 20];
        bytes[0..8].copy_from_slice(b"RSD PTR ");
        bytes[15] = 0;
        fix_checksum(&mut bytes, 8, 20);
        bytes
    }

    fn valid_rsdp_v2(length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        bytes[0..8].copy_from_slice(b"RSD PTR ");
        bytes[15] = 2;
        bytes[20..24].copy_from_slice(&(length as u32).to_le_bytes());
        fix_checksum(&mut bytes, 8, 20);
        fix_checksum(&mut bytes, 32, length);
        bytes
    }

    fn valid_mcfg_with_window(base: u64, start_bus: u8, end_bus: u8) -> Vec<u8> {
        let mut bytes = vec![0u8; 60];
        bytes[0..4].copy_from_slice(b"MCFG");
        bytes[4..8].copy_from_slice(&(60u32).to_le_bytes());
        bytes[44..52].copy_from_slice(&base.to_le_bytes());
        bytes[52] = start_bus;
        bytes[53] = end_bus;
        fix_checksum(&mut bytes, 9, 60);
        bytes
    }

    #[test]
    fn rsdp_rejects_bad_signature_and_truncated_input() {
        let mut bad_signature = valid_rsdp_v1();
        bad_signature[0] = b'X';
        assert_eq!(
            validate_rsdp(&bad_signature),
            Err(AcpiError::InvalidSignature)
        );
        assert_eq!(validate_rsdp(&[]), Err(AcpiError::InvalidLength));
        assert_eq!(validate_rsdp(b"RSD PTR "), Err(AcpiError::InvalidLength));
        assert_eq!(
            validate_rsdp(&valid_rsdp_v1()[..19]),
            Err(AcpiError::InvalidLength)
        );
    }

    #[test]
    fn rsdp_rejects_bad_legacy_checksum() {
        let mut bytes = valid_rsdp_v1();
        bytes[10] ^= 1;
        assert_eq!(validate_rsdp(&bytes), Err(AcpiError::InvalidChecksum));
    }

    #[test]
    fn rsdp_v1_valid_boundary_remains_accepted() {
        assert_eq!(validate_rsdp(&valid_rsdp_v1()), Ok(()));
    }

    #[test]
    fn rsdp_v2_rejects_short_data_invalid_length_and_bad_extended_checksum() {
        let bytes = valid_rsdp_v2(36);
        assert_eq!(validate_rsdp(&bytes[..35]), Err(AcpiError::InvalidLength));
        let mut invalid_length = valid_rsdp_v2(36);
        invalid_length[20..24].copy_from_slice(&35u32.to_le_bytes());
        fix_checksum(&mut invalid_length, 32, 36);
        assert_eq!(
            validate_rsdp(&invalid_length),
            Err(AcpiError::InvalidLength)
        );
        let mut bad_extended_checksum = valid_rsdp_v2(36);
        bad_extended_checksum[35] ^= 1;
        assert_eq!(
            validate_rsdp(&bad_extended_checksum),
            Err(AcpiError::InvalidChecksum)
        );
    }

    #[test]
    fn rsdp_v2_maximum_supported_length_is_accepted() {
        assert_eq!(validate_rsdp(&valid_rsdp_v2(4096)), Ok(()));
    }

    #[test]
    fn mcfg_rejects_empty_short_wrong_signature_and_invalid_table_length() {
        assert_eq!(validate_mcfg_table(&[]), Err(AcpiError::InvalidLength));
        assert_eq!(validate_mcfg_table(&[0; 35]), Err(AcpiError::InvalidLength));
        let mut wrong_signature = vec![0u8; 44];
        wrong_signature[0..4].copy_from_slice(b"BAD!");
        assert_eq!(
            validate_mcfg_table(&wrong_signature),
            Err(AcpiError::InvalidSignature)
        );
        let mut malformed_length = vec![0u8; 44];
        malformed_length[0..4].copy_from_slice(b"MCFG");
        malformed_length[4..8].copy_from_slice(&43u32.to_le_bytes());
        assert_eq!(
            validate_mcfg_table(&malformed_length),
            Err(AcpiError::InvalidLength)
        );
        let mut empty_table = vec![0u8; 44];
        empty_table[0..4].copy_from_slice(b"MCFG");
        empty_table[4..8].copy_from_slice(&44u32.to_le_bytes());
        fix_checksum(&mut empty_table, 9, 44);
        assert_eq!(
            validate_mcfg_table(&empty_table),
            Err(AcpiError::InvalidLength)
        );
    }

    #[test]
    fn mcfg_rejects_nonzero_header_reserved_bytes_with_valid_checksum() {
        let mut bytes = valid_mcfg_with_window(0xE000_0000, 0, 0);
        bytes[36] = 1;
        let length = bytes.len();
        fix_checksum(&mut bytes, 9, length);
        assert_eq!(validate_mcfg_table(&bytes), Err(AcpiError::InvalidLength));
    }

    #[test]
    fn mcfg_rejects_nonzero_allocation_reserved_bytes_with_valid_checksum() {
        let mut bytes = valid_mcfg_with_window(0xE000_0000, 0, 0);
        bytes[56] = 1; // entry-relative byte 12 (absolute table offset 56)
        let length = bytes.len();
        fix_checksum(&mut bytes, 9, length);
        assert_eq!(validate_mcfg_table(&bytes), Err(AcpiError::InvalidLength));
    }

    #[test]
    fn mcfg_rejects_bad_checksum_and_non_integral_entry_layout() {
        let mut bad_checksum = valid_mcfg_with_window(0xE000_0000, 0, 0);
        bad_checksum[45] ^= 1;
        assert_eq!(
            validate_mcfg_table(&bad_checksum),
            Err(AcpiError::InvalidChecksum)
        );
        let mut bad_layout = vec![0u8; 61];
        bad_layout[0..4].copy_from_slice(b"MCFG");
        bad_layout[4..8].copy_from_slice(&61u32.to_le_bytes());
        fix_checksum(&mut bad_layout, 9, 61);
        assert_eq!(
            validate_mcfg_table(&bad_layout),
            Err(AcpiError::InvalidLength)
        );
    }

    #[test]
    fn mcfg_rejects_invalid_bus_range_and_address_overflow() {
        assert_eq!(
            validate_mcfg_table(&valid_mcfg_with_window(0xE000_0000, 8, 7)),
            Err(AcpiError::InvalidLength)
        );
        assert_eq!(
            validate_mcfg_table(&valid_mcfg_with_window(!0xF_FFFF, 0, 1)),
            Err(AcpiError::InvalidAddress)
        );
    }

    #[test]
    fn ecam_valid_boundaries_and_region_end_are_checked() {
        let window = McfgWindow {
            base_address: 0xE000_0000,
            segment: 0,
            start_bus: 0,
            end_bus: 1,
        };
        assert_eq!(window.ecam_address(0, 0, 0, 0), Some(0xE000_0000));
        assert_eq!(window.ecam_address(1, 31, 7, 4092), Some(0xE01F_FFFC));
        assert_eq!(window.ecam_address(2, 0, 0, 0), None);
        assert_eq!(window.ecam_address(0, 32, 0, 0), None);
        assert_eq!(window.ecam_address(0, 0, 8, 0), None);
        assert_eq!(window.ecam_address(0, 0, 0, 4096), None);
        assert_eq!(window.ecam_address(0, 0, 0, 2), None);
    }

    #[test]
    fn ecam_rejects_unaligned_base_invalid_window_and_overflow() {
        let unaligned = McfgWindow {
            base_address: 0xE000_0001,
            segment: 0,
            start_bus: 0,
            end_bus: 0,
        };
        assert_eq!(unaligned.validate(), Err(AcpiError::InvalidAddress));
        let reversed = McfgWindow {
            base_address: 0xE000_0000,
            segment: 0,
            start_bus: 2,
            end_bus: 1,
        };
        assert_eq!(reversed.validate(), Err(AcpiError::InvalidLength));
        let overflow = McfgWindow {
            base_address: !0xF_FFFF,
            segment: 0,
            start_bus: 0,
            end_bus: 1,
        };
        assert_eq!(overflow.validate(), Err(AcpiError::InvalidAddress));
        assert_eq!(overflow.ecam_address(0, 0, 0, 0), None);
    }
}
