// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Minimal x86 CPU HAL. Detects Intel/AMD at runtime using CPUID.

use core::arch::x86_64::__cpuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuVendor {
    Intel,
    Amd,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuFeatures {
    pub sse2: bool,
    pub nx: bool,
    pub apic: bool,
    pub x2apic: bool,
    pub syscall: bool,
    pub one_gib_pages: bool,
    pub invariant_tsc: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuInfo {
    pub vendor: CpuVendor,
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
    pub max_basic_leaf: u32,
    pub max_extended_leaf: u32,
    pub features: CpuFeatures,
}

impl CpuInfo {
    pub fn detect() -> Self {
        let basic = __cpuid(0);
        let max_basic_leaf = basic.eax;
        let vendor = vendor_from_regs(basic.ebx, basic.ecx, basic.edx);
        let ext = __cpuid(0x8000_0000);
        let max_extended_leaf = ext.eax;

        let leaf1 = if max_basic_leaf >= 1 {
            __cpuid(1)
        } else {
            zero_cpuid()
        };
        let stepping = (leaf1.eax & 0xF) as u8;
        let base_family = ((leaf1.eax >> 8) & 0xF) as u8;
        let base_model = ((leaf1.eax >> 4) & 0xF) as u8;
        let ext_family = ((leaf1.eax >> 20) & 0xFF) as u8;
        let ext_model = ((leaf1.eax >> 16) & 0xF) as u8;
        let family = if base_family == 0xF {
            base_family.saturating_add(ext_family)
        } else {
            base_family
        };
        let model = if base_family == 0x6 || base_family == 0xF {
            base_model | (ext_model << 4)
        } else {
            base_model
        };

        let ext1 = if max_extended_leaf >= 0x8000_0001 {
            __cpuid(0x8000_0001)
        } else {
            zero_cpuid()
        };
        let ext7 = if max_extended_leaf >= 0x8000_0007 {
            __cpuid(0x8000_0007)
        } else {
            zero_cpuid()
        };

        Self {
            vendor,
            family,
            model,
            stepping,
            max_basic_leaf,
            max_extended_leaf,
            features: features_from_leaves(leaf1, ext1, ext7),
        }
    }

    pub const fn vendor_name(&self) -> &'static str {
        match self.vendor {
            CpuVendor::Intel => "Intel",
            CpuVendor::Amd => "AMD",
            CpuVendor::Other => "Other",
        }
    }

    pub const fn boot_compatible(&self) -> bool {
        self.features.sse2 && self.features.nx && self.features.apic
    }
}

fn features_from_leaves(
    leaf1: core::arch::x86_64::CpuidResult,
    ext1: core::arch::x86_64::CpuidResult,
    ext7: core::arch::x86_64::CpuidResult,
) -> CpuFeatures {
    CpuFeatures {
        sse2: (leaf1.edx & (1 << 26)) != 0,
        apic: (leaf1.edx & (1 << 9)) != 0,
        x2apic: (leaf1.ecx & (1 << 21)) != 0,
        nx: (ext1.edx & (1 << 20)) != 0,
        syscall: (ext1.edx & (1 << 11)) != 0,
        one_gib_pages: (ext1.edx & (1 << 26)) != 0,
        invariant_tsc: (ext7.edx & (1 << 8)) != 0,
    }
}

fn zero_cpuid() -> core::arch::x86_64::CpuidResult {
    core::arch::x86_64::CpuidResult {
        eax: 0,
        ebx: 0,
        ecx: 0,
        edx: 0,
    }
}

fn vendor_from_regs(ebx: u32, ecx: u32, edx: u32) -> CpuVendor {
    let bytes = [ebx.to_le_bytes(), edx.to_le_bytes(), ecx.to_le_bytes()];
    if bytes == [*b"Genu", *b"ineI", *b"ntel"] {
        CpuVendor::Intel
    } else if bytes == [*b"Auth", *b"enti", *b"cAMD"] {
        CpuVendor::Amd
    } else {
        CpuVendor::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_current_cpu_without_panicking() {
        let info = CpuInfo::detect();
        assert!(info.max_basic_leaf >= 1);
        assert!(info.max_extended_leaf >= 0x8000_0000);
    }

    #[test]
    fn vendor_parser_accepts_intel_and_amd() {
        assert_eq!(
            vendor_from_regs(0x756e6547, 0x6c65746e, 0x49656e69),
            CpuVendor::Intel
        );
        assert_eq!(
            vendor_from_regs(0x68747541, 0x444d4163, 0x69746e65),
            CpuVendor::Amd
        );
    }

    #[test]
    fn absent_feature_leaves_decode_as_unsupported() {
        let features = features_from_leaves(zero_cpuid(), zero_cpuid(), zero_cpuid());
        assert_eq!(
            features,
            CpuFeatures {
                sse2: false,
                nx: false,
                apic: false,
                x2apic: false,
                syscall: false,
                one_gib_pages: false,
                invariant_tsc: false,
            }
        );
    }

    #[test]
    fn feature_bits_are_decoded_from_their_defined_leaves() {
        let leaf1 = core::arch::x86_64::CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 1 << 21,
            edx: (1 << 26) | (1 << 9),
        };
        let ext1 = core::arch::x86_64::CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: (1 << 20) | (1 << 11) | (1 << 26),
        };
        let ext7 = core::arch::x86_64::CpuidResult {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 1 << 8,
        };

        assert_eq!(
            features_from_leaves(leaf1, ext1, ext7),
            CpuFeatures {
                sse2: true,
                nx: true,
                apic: true,
                x2apic: true,
                syscall: true,
                one_gib_pages: true,
                invariant_tsc: true,
            }
        );
    }
}
