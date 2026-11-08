//! IOMMU/DMA isolation contract.
//!
//! The implementation must be backed by the platform IOMMU; device services
//! receive only explicitly authorized DMA domains.

use super::DeviceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DmaDomainId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaRegion {
    pub iova: u64,
    pub physical: u64,
    pub length: u64,
    pub writable: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IommuError {
    InvalidRegion,
    DomainNotFound,
    MappingDenied,
    DeviceAlreadyAssigned,
}

pub trait Iommu {
    fn create_domain(&mut self, domain: DmaDomainId) -> Result<(), IommuError>;
    fn assign_device(&mut self, domain: DmaDomainId, device: DeviceId) -> Result<(), IommuError>;
    fn map(&mut self, domain: DmaDomainId, region: DmaRegion) -> Result<(), IommuError>;
    fn unmap(&mut self, domain: DmaDomainId, iova: u64) -> Result<(), IommuError>;
}

pub fn validate_region(region: DmaRegion) -> Result<(), IommuError> {
    if region.length == 0
        || region.iova.checked_add(region.length).is_none()
        || region.physical.checked_add(region.length).is_none()
    {
        return Err(IommuError::InvalidRegion);
    }
    Ok(())
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SoftwareIommu {
    domains: Vec<DomainState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DomainState {
    id: DmaDomainId,
    devices: Vec<DeviceId>,
    mappings: Vec<DmaRegion>,
}

impl SoftwareIommu {
    fn domain_mut(&mut self, id: DmaDomainId) -> Result<&mut DomainState, IommuError> {
        self.domains
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or(IommuError::DomainNotFound)
    }
    fn domain(&self, id: DmaDomainId) -> Result<&DomainState, IommuError> {
        self.domains
            .iter()
            .find(|d| d.id == id)
            .ok_or(IommuError::DomainNotFound)
    }
}

impl Iommu for SoftwareIommu {
    fn create_domain(&mut self, domain: DmaDomainId) -> Result<(), IommuError> {
        if self.domains.iter().any(|d| d.id == domain) {
            return Err(IommuError::MappingDenied);
        }
        self.domains.push(DomainState {
            id: domain,
            devices: Vec::new(),
            mappings: Vec::new(),
        });
        self.domains.sort_by_key(|d| d.id);
        Ok(())
    }
    fn assign_device(&mut self, domain: DmaDomainId, device: DeviceId) -> Result<(), IommuError> {
        if self.domains.iter().any(|d| d.devices.contains(&device)) {
            return Err(IommuError::DeviceAlreadyAssigned);
        }
        let state = self.domain_mut(domain)?;
        state.devices.push(device);
        state.devices.sort();
        Ok(())
    }
    fn map(&mut self, domain: DmaDomainId, region: DmaRegion) -> Result<(), IommuError> {
        validate_region(region)?;
        let state = self.domain_mut(domain)?;
        let end = region.iova + region.length;
        if state
            .mappings
            .iter()
            .any(|m| region.iova < m.iova + m.length && m.iova < end)
        {
            return Err(IommuError::MappingDenied);
        }
        state.mappings.push(region);
        state.mappings.sort_by_key(|m| m.iova);
        Ok(())
    }
    fn unmap(&mut self, domain: DmaDomainId, iova: u64) -> Result<(), IommuError> {
        let state = self.domain_mut(domain)?;
        let before = state.mappings.len();
        state.mappings.retain(|m| m.iova != iova);
        if before == state.mappings.len() {
            return Err(IommuError::MappingDenied);
        }
        Ok(())
    }
}

impl SoftwareIommu {
    pub fn mappings(&self, domain: DmaDomainId) -> Result<&[DmaRegion], IommuError> {
        Ok(&self.domain(domain)?.mappings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_zero_length_dma() {
        assert_eq!(
            validate_region(DmaRegion {
                iova: 0,
                physical: 0,
                length: 0,
                writable: false
            }),
            Err(IommuError::InvalidRegion)
        );
    }
    #[test]
    fn isolates_domains_and_devices() {
        let mut iommu = SoftwareIommu::default();
        iommu.create_domain(DmaDomainId(1)).unwrap();
        iommu.create_domain(DmaDomainId(2)).unwrap();
        iommu.assign_device(DmaDomainId(1), DeviceId(7)).unwrap();
        assert_eq!(
            iommu.assign_device(DmaDomainId(2), DeviceId(7)),
            Err(IommuError::DeviceAlreadyAssigned)
        );
        iommu
            .map(
                DmaDomainId(1),
                DmaRegion {
                    iova: 0x1000,
                    physical: 0x8000,
                    length: 0x1000,
                    writable: true,
                },
            )
            .unwrap();
        assert_eq!(
            iommu.map(
                DmaDomainId(1),
                DmaRegion {
                    iova: 0x1800,
                    physical: 0x9000,
                    length: 0x1000,
                    writable: true
                }
            ),
            Err(IommuError::MappingDenied)
        );
        assert_eq!(iommu.mappings(DmaDomainId(2)).unwrap(), &[]);
    }
}
