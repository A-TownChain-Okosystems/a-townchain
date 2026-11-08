//! GPU identity profiles and conservative backend selection.
//!
//! Profiles identify PCI display devices; they do not imply that a usable
//! hardware driver exists. Vendor driver names are compatibility references,
//! not drivers that can be loaded directly by GlobusOS. Hardware-specific
//! features must be reported by a successfully initialized userspace driver.

use globus_devices::pci::PciId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Amd,
    Intel,
    Nvidia,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuFamily {
    /// Vendor known, but the PCI device ID is not in the curated family table.
    Unclassified,
    AmdIntegrated,
    AmdRadeonRx5000,
    AmdRadeonRx6000,
    AmdRadeonRx7000,
    AmdRadeonRx9000,
    IntelUhd,
    IntelIrisXe,
    IntelArcA,
    IntelArcB,
    NvidiaGeForceGtx,
    NvidiaGeForceRtx,
    NvidiaProfessional,
    NvidiaDatacenter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverReference {
    Amdgpu,
    IntelI915OrXe,
    NouveauOrVendorNvidia,
    GenericDisplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    /// A profile exists, but no compatible GlobusOS driver has been activated.
    ProfileOnly,
    Initializing,
    Running,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureState {
    Unknown,
    Unsupported,
    Supported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpuCapabilities {
    /// The PCI function is in the display-controller class.
    pub display_controller: bool,
    /// These features must be reported by an initialized driver, not inferred from vendor ID.
    pub vulkan: FeatureState,
    pub open_gl: FeatureState,
    pub compute: FeatureState,
    pub media_engine: FeatureState,
    /// None means not measured or not exposed by the driver.
    pub dedicated_vram_bytes: Option<u64>,
    pub shared_memory: FeatureState,
}

impl GpuCapabilities {
    const fn profile_only() -> Self {
        Self {
            display_controller: true,
            vulkan: FeatureState::Unknown,
            open_gl: FeatureState::Unknown,
            compute: FeatureState::Unknown,
            media_engine: FeatureState::Unknown,
            dedicated_vram_bytes: None,
            shared_memory: FeatureState::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpuProfile {
    pub vendor: GpuVendor,
    pub family: GpuFamily,
    pub pci_vendor_id: u16,
    pub pci_device_id: u16,
    pub pci_revision_id: u8,
    pub driver_reference: DriverReference,
    pub capabilities: GpuCapabilities,
    pub driver_state: DriverState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    Framebuffer,
    Vulkan,
    OpenGl,
}

/// Classifies PCI display-class devices (VGA, 3D, or other display controller).
/// The revision ID comes from PCI config-space offset 0x08; it is supplied by
/// the discovery caller because the current PciId structure does not store it.
pub fn classify_pci_device(id: PciId, revision_id: u8) -> Option<GpuProfile> {
    if id.class != 0x03 {
        return None;
    }

    let (vendor, driver_reference) = match id.vendor {
        0x1002 => (GpuVendor::Amd, DriverReference::Amdgpu),
        0x8086 => (GpuVendor::Intel, DriverReference::IntelI915OrXe),
        0x10de => (GpuVendor::Nvidia, DriverReference::NouveauOrVendorNvidia),
        other => (GpuVendor::Other(other), DriverReference::GenericDisplay),
    };

    let family = match (id.vendor, id.device) {
        (0x1002, 0x15d8 | 0x1638 | 0x164c) => GpuFamily::AmdIntegrated,
        (0x1002, 0x731f) => GpuFamily::AmdRadeonRx5000, // Radeon RX 5700 XT
        (0x1002, 0x73bf | 0x73df | 0x73ff) => GpuFamily::AmdRadeonRx6000,
        (0x1002, 0x744c | 0x747e | 0x7480) => GpuFamily::AmdRadeonRx7000,
        (0x1002, 0x7550 | 0x7551 | 0x7590) => GpuFamily::AmdRadeonRx9000,
        (0x8086, 0x3e92 | 0x4680 | 0x4690) => GpuFamily::IntelUhd,
        (0x8086, 0x9a40 | 0x9a49 | 0x9a60) => GpuFamily::IntelIrisXe,
        (0x8086, 0x56a0 | 0x56a1) => GpuFamily::IntelArcA,
        (0x8086, 0xe209 | 0xe20b | 0xe20c) => GpuFamily::IntelArcB,
        (0x10de, 0x1b80) => GpuFamily::NvidiaGeForceGtx, // GeForce GTX 1080
        (0x10de, 0x2484 | 0x2684) => GpuFamily::NvidiaGeForceRtx, // RTX 3070 / RTX 4090
        (0x10de, 0x2230 | 0x26b1) => GpuFamily::NvidiaProfessional,
        (0x10de, 0x20b0 | 0x2330) => GpuFamily::NvidiaDatacenter,
        _ => GpuFamily::Unclassified,
    };

    Some(GpuProfile {
        vendor,
        family,
        pci_vendor_id: id.vendor,
        pci_device_id: id.device,
        pci_revision_id: revision_id,
        driver_reference,
        capabilities: GpuCapabilities::profile_only(),
        driver_state: DriverState::ProfileOnly,
    })
}

/// Selects only a backend whose support was explicitly confirmed by a running
/// driver. Recognizing a GPU never enables hardware acceleration by itself.
pub fn select_backend(profile: &GpuProfile) -> GpuBackend {
    if profile.driver_state != DriverState::Running {
        return GpuBackend::Framebuffer;
    }
    if profile.capabilities.vulkan == FeatureState::Supported {
        GpuBackend::Vulkan
    } else if profile.capabilities.open_gl == FeatureState::Supported {
        GpuBackend::OpenGl
    } else {
        GpuBackend::Framebuffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use globus_devices::pci::decode_id;

    #[test]
    fn identifies_amd_intel_and_nvidia_vendor_profiles() {
        let amd = classify_pci_device(decode_id(3, 0, 0, 0x1002, 0x747e), 1).unwrap();
        let intel = classify_pci_device(decode_id(3, 0, 0, 0x8086, 0x56a0), 2).unwrap();
        let nvidia = classify_pci_device(decode_id(3, 0, 0, 0x10de, 0x2684), 3).unwrap();

        assert_eq!(amd.vendor, GpuVendor::Amd);
        assert_eq!(amd.family, GpuFamily::AmdRadeonRx7000);
        assert_eq!(amd.driver_reference, DriverReference::Amdgpu);
        assert_eq!(intel.vendor, GpuVendor::Intel);
        assert_eq!(intel.family, GpuFamily::IntelArcA);
        assert_eq!(intel.driver_reference, DriverReference::IntelI915OrXe);
        assert_eq!(nvidia.vendor, GpuVendor::Nvidia);
        assert_eq!(nvidia.family, GpuFamily::NvidiaGeForceRtx);
        assert_eq!(nvidia.pci_revision_id, 3);
    }

    #[test]
    fn recognizes_additional_gpu_families_by_exact_device_id() {
        assert_eq!(
            classify_pci_device(decode_id(3, 0, 0, 0x1002, 0x7550), 0)
                .unwrap()
                .family,
            GpuFamily::AmdRadeonRx9000
        );
        assert_eq!(
            classify_pci_device(decode_id(3, 0, 0, 0x8086, 0xe20b), 0)
                .unwrap()
                .family,
            GpuFamily::IntelArcB
        );
        assert_eq!(
            classify_pci_device(decode_id(3, 0, 0, 0x10de, 0x2230), 0)
                .unwrap()
                .family,
            GpuFamily::NvidiaProfessional
        );
        assert_eq!(
            classify_pci_device(decode_id(3, 0, 0, 0x10de, 0x20b0), 0)
                .unwrap()
                .family,
            GpuFamily::NvidiaDatacenter
        );
    }

    #[test]
    fn rejects_non_display_pci_functions() {
        assert!(classify_pci_device(decode_id(2, 0, 0, 0x10de, 0x2684), 0).is_none());
    }

    #[test]
    fn recognized_hardware_stays_on_framebuffer_without_a_running_driver() {
        let profile = classify_pci_device(decode_id(3, 0, 0, 0x10de, 0x2684), 0).unwrap();
        assert_eq!(profile.driver_state, DriverState::ProfileOnly);
        assert_eq!(select_backend(&profile), GpuBackend::Framebuffer);
        assert_eq!(profile.capabilities.vulkan, FeatureState::Unknown);
        assert_eq!(profile.capabilities.dedicated_vram_bytes, None);
    }

    #[test]
    fn selects_vulkan_only_after_driver_confirms_support() {
        let mut profile = classify_pci_device(decode_id(3, 2, 0, 0x1002, 0x744c), 0).unwrap();
        profile.driver_state = DriverState::Running;
        profile.capabilities.vulkan = FeatureState::Supported;
        assert_eq!(select_backend(&profile), GpuBackend::Vulkan);
    }
}
