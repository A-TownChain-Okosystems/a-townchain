//! Canonical userspace VM lifecycle API for GlobusOS.
//!
//! This crate defines policy, typed lifecycle state, resource quotas, and a
//! backend contract. It does not implement a hypervisor or claim guest boot support.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VmId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestProfile {
    LinuxX86_64,
    WindowsX86_64,
    MacOs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VmConfig {
    pub name: String,
    pub guest: GuestProfile,
    pub vcpus: u16,
    pub memory_mib: u64,
    pub disk_mib: u64,
}

/// Last lifecycle state confirmed by a successful backend operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmState {
    Created,
    Running,
    Paused,
    Stopped,
    /// Reserved for a future backend-reconciliation path. The current manager
    /// never sets this variant because an operation error alone cannot prove
    /// that the guest entered a terminal failed state.
    Failed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VmPermissions {
    pub manage: bool,
    pub start: bool,
    pub pause: bool,
    pub stop: bool,
    pub destroy: bool,
    pub inspect: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLimits {
    pub max_vms: usize,
    pub max_total_vcpus: u64,
    pub max_total_memory_mib: u64,
    pub max_total_disk_mib: u64,
    pub host_memory_reserve_mib: u64,
    pub host_memory_total_mib: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VmMetrics {
    pub id: VmId,
    pub state: VmState,
    pub vcpus: u16,
    pub memory_mib: u64,
    pub disk_mib: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmError {
    PermissionDenied,
    InvalidConfig(&'static str),
    ResourceLimit(&'static str),
    UnknownVm(VmId),
    InvalidState {
        id: VmId,
        state: VmState,
        operation: &'static str,
    },
    IdExhausted,
    Backend(String),
}

/// Backend operations must report errors when the requested transition cannot
/// be confirmed. An error does not guarantee that the guest or hypervisor made
/// no partial progress; the manager therefore retains the last confirmed
/// lifecycle state and returns VmError::Backend. Callers must treat the actual
/// backend state as potentially uncertain after an error and reconcile it
/// before retrying or making destructive decisions.
pub trait VmBackend {
    type Handle: Clone;

    fn create(&mut self, config: &VmConfig) -> Result<Self::Handle, String>;
    fn start(&mut self, handle: &Self::Handle) -> Result<(), String>;
    fn pause(&mut self, handle: &Self::Handle) -> Result<(), String>;
    fn resume(&mut self, handle: &Self::Handle) -> Result<(), String>;
    fn stop(&mut self, handle: &Self::Handle) -> Result<(), String>;
    fn destroy(&mut self, handle: &Self::Handle) -> Result<(), String>;
}

struct ManagedVm<H> {
    config: VmConfig,
    state: VmState,
    handle: H,
}

/// Single canonical manager. A real IPC service must derive permissions from
/// authenticated capabilities; callers must not trust UI-supplied permissions.
pub struct VmManager<B: VmBackend> {
    backend: B,
    limits: ResourceLimits,
    vms: BTreeMap<VmId, ManagedVm<B::Handle>>,
    next_id: u64,
}

impl<B: VmBackend> VmManager<B> {
    pub fn new(backend: B, limits: ResourceLimits) -> Result<Self, VmError> {
        if limits.host_memory_reserve_mib > limits.host_memory_total_mib {
            return Err(VmError::InvalidConfig("host reserve exceeds host memory"));
        }
        Ok(Self {
            backend,
            limits,
            vms: BTreeMap::new(),
            next_id: 1,
        })
    }

    pub fn create_vm(
        &mut self,
        config: VmConfig,
        permissions: VmPermissions,
    ) -> Result<VmId, VmError> {
        if !permissions.manage {
            return Err(VmError::PermissionDenied);
        }
        if config.name.trim().is_empty() {
            return Err(VmError::InvalidConfig("name must not be empty"));
        }
        if config.vcpus == 0 {
            return Err(VmError::InvalidConfig("vcpus must be non-zero"));
        }
        if config.memory_mib == 0 {
            return Err(VmError::InvalidConfig("memory must be non-zero"));
        }
        if config.disk_mib == 0 {
            return Err(VmError::InvalidConfig("disk must be non-zero"));
        }
        if self.vms.len() >= self.limits.max_vms {
            return Err(VmError::ResourceLimit("max_vms"));
        }

        let (vcpus, memory, disk) = self.usage();
        if vcpus
            .checked_add(u64::from(config.vcpus))
            .filter(|v| *v <= self.limits.max_total_vcpus)
            .is_none()
        {
            return Err(VmError::ResourceLimit("total vcpus"));
        }
        if memory
            .checked_add(config.memory_mib)
            .filter(|v| *v <= self.limits.max_total_memory_mib)
            .filter(|v| {
                v.checked_add(self.limits.host_memory_reserve_mib)
                    .is_some_and(|required| required <= self.limits.host_memory_total_mib)
            })
            .is_none()
        {
            return Err(VmError::ResourceLimit("total memory / host reserve"));
        }
        if disk
            .checked_add(config.disk_mib)
            .filter(|v| *v <= self.limits.max_total_disk_mib)
            .is_none()
        {
            return Err(VmError::ResourceLimit("total disk"));
        }
        let id = VmId(self.next_id);
        let next_id = self.next_id.checked_add(1).ok_or(VmError::IdExhausted)?;
        let handle = self.backend.create(&config).map_err(VmError::Backend)?;
        self.vms.insert(
            id,
            ManagedVm {
                config,
                state: VmState::Created,
                handle,
            },
        );
        self.next_id = next_id;
        Ok(id)
    }

    pub fn start_vm(&mut self, id: VmId, permissions: VmPermissions) -> Result<(), VmError> {
        if !permissions.start {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get_mut(&id).ok_or(VmError::UnknownVm(id))?;
        if vm.state != VmState::Created {
            return Err(VmError::InvalidState {
                id,
                state: vm.state,
                operation: "start",
            });
        }
        // Update the last-confirmed state only after backend success. On error,
        // the backend may have made partial progress; callers must reconcile.
        self.backend.start(&vm.handle).map_err(VmError::Backend)?;
        vm.state = VmState::Running;
        Ok(())
    }

    pub fn pause_vm(&mut self, id: VmId, permissions: VmPermissions) -> Result<(), VmError> {
        if !permissions.pause {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get_mut(&id).ok_or(VmError::UnknownVm(id))?;
        if vm.state != VmState::Running {
            return Err(VmError::InvalidState {
                id,
                state: vm.state,
                operation: "pause",
            });
        }
        self.backend.pause(&vm.handle).map_err(VmError::Backend)?;
        vm.state = VmState::Paused;
        Ok(())
    }

    pub fn resume_vm(&mut self, id: VmId, permissions: VmPermissions) -> Result<(), VmError> {
        if !permissions.start {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get_mut(&id).ok_or(VmError::UnknownVm(id))?;
        if vm.state != VmState::Paused {
            return Err(VmError::InvalidState {
                id,
                state: vm.state,
                operation: "resume",
            });
        }
        self.backend.resume(&vm.handle).map_err(VmError::Backend)?;
        vm.state = VmState::Running;
        Ok(())
    }

    pub fn stop_vm(&mut self, id: VmId, permissions: VmPermissions) -> Result<(), VmError> {
        if !permissions.stop {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get_mut(&id).ok_or(VmError::UnknownVm(id))?;
        if !matches!(
            vm.state,
            VmState::Running | VmState::Paused | VmState::Created
        ) {
            return Err(VmError::InvalidState {
                id,
                state: vm.state,
                operation: "stop",
            });
        }
        if vm.state != VmState::Created {
            self.backend.stop(&vm.handle).map_err(VmError::Backend)?;
        }
        vm.state = VmState::Stopped;
        Ok(())
    }

    pub fn destroy_vm(&mut self, id: VmId, permissions: VmPermissions) -> Result<(), VmError> {
        if !permissions.destroy {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get(&id).ok_or(VmError::UnknownVm(id))?;
        if !matches!(
            vm.state,
            VmState::Created | VmState::Stopped | VmState::Failed
        ) {
            return Err(VmError::InvalidState {
                id,
                state: vm.state,
                operation: "destroy",
            });
        }
        self.backend.destroy(&vm.handle).map_err(VmError::Backend)?;
        self.vms.remove(&id);
        Ok(())
    }

    pub fn get_vm_state(&self, id: VmId, permissions: VmPermissions) -> Result<VmState, VmError> {
        if !permissions.inspect {
            return Err(VmError::PermissionDenied);
        }
        self.vms
            .get(&id)
            .map(|vm| vm.state)
            .ok_or(VmError::UnknownVm(id))
    }

    pub fn get_vm_metrics(
        &self,
        id: VmId,
        permissions: VmPermissions,
    ) -> Result<VmMetrics, VmError> {
        if !permissions.inspect {
            return Err(VmError::PermissionDenied);
        }
        let vm = self.vms.get(&id).ok_or(VmError::UnknownVm(id))?;
        Ok(VmMetrics {
            id,
            state: vm.state,
            vcpus: vm.config.vcpus,
            memory_mib: vm.config.memory_mib,
            disk_mib: vm.config.disk_mib,
        })
    }

    pub fn list_vms(&self, permissions: VmPermissions) -> Result<Vec<VmMetrics>, VmError> {
        if !permissions.inspect {
            return Err(VmError::PermissionDenied);
        }
        Ok(self
            .vms
            .iter()
            .map(|(id, vm)| VmMetrics {
                id: *id,
                state: vm.state,
                vcpus: vm.config.vcpus,
                memory_mib: vm.config.memory_mib,
                disk_mib: vm.config.disk_mib,
            })
            .collect())
    }

    pub fn into_backend(self) -> B {
        self.backend
    }

    fn usage(&self) -> (u64, u64, u64) {
        self.vms
            .values()
            .fold((0u64, 0u64, 0u64), |(cpu, mem, disk), vm| {
                (
                    cpu.saturating_add(u64::from(vm.config.vcpus)),
                    mem.saturating_add(vm.config.memory_mib),
                    disk.saturating_add(vm.config.disk_mib),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct FakeBackend {
        next: u64,
        fail_start: bool,
        destroyed: usize,
    }
    impl VmBackend for FakeBackend {
        type Handle = u64;
        fn create(&mut self, _: &VmConfig) -> Result<u64, String> {
            self.next += 1;
            Ok(self.next)
        }
        fn start(&mut self, _: &u64) -> Result<(), String> {
            if self.fail_start {
                Err("injected start failure".into())
            } else {
                Ok(())
            }
        }
        fn pause(&mut self, _: &u64) -> Result<(), String> {
            Ok(())
        }
        fn resume(&mut self, _: &u64) -> Result<(), String> {
            Ok(())
        }
        fn stop(&mut self, _: &u64) -> Result<(), String> {
            Ok(())
        }
        fn destroy(&mut self, _: &u64) -> Result<(), String> {
            self.destroyed += 1;
            Ok(())
        }
    }

    fn permissions() -> VmPermissions {
        VmPermissions {
            manage: true,
            start: true,
            pause: true,
            stop: true,
            destroy: true,
            inspect: true,
        }
    }
    fn limits() -> ResourceLimits {
        ResourceLimits {
            max_vms: 2,
            max_total_vcpus: 4,
            max_total_memory_mib: 1536,
            max_total_disk_mib: 8192,
            host_memory_reserve_mib: 1024,
            host_memory_total_mib: 4096,
        }
    }
    fn config(name: &str) -> VmConfig {
        VmConfig {
            name: name.into(),
            guest: GuestProfile::LinuxX86_64,
            vcpus: 2,
            memory_mib: 1024,
            disk_mib: 4096,
        }
    }

    #[test]
    fn lifecycle_transitions_are_enforced() {
        let mut manager = VmManager::new(FakeBackend::default(), limits()).unwrap();
        let id = manager
            .create_vm(config("linux-test"), permissions())
            .unwrap();
        assert_eq!(
            manager.get_vm_state(id, permissions()),
            Ok(VmState::Created)
        );
        manager.start_vm(id, permissions()).unwrap();
        assert_eq!(
            manager.get_vm_state(id, permissions()),
            Ok(VmState::Running)
        );
        manager.pause_vm(id, permissions()).unwrap();
        assert_eq!(manager.get_vm_state(id, permissions()), Ok(VmState::Paused));
        manager.resume_vm(id, permissions()).unwrap();
        manager.stop_vm(id, permissions()).unwrap();
        assert_eq!(
            manager.get_vm_state(id, permissions()),
            Ok(VmState::Stopped)
        );
        manager.destroy_vm(id, permissions()).unwrap();
        assert_eq!(manager.list_vms(permissions()).unwrap().len(), 0);
    }

    #[test]
    fn denied_create_does_not_allocate_vm() {
        let mut manager = VmManager::new(FakeBackend::default(), limits()).unwrap();
        let mut denied = permissions();
        denied.manage = false;
        assert_eq!(
            manager.create_vm(config("denied"), denied),
            Err(VmError::PermissionDenied)
        );
        assert!(manager.list_vms(permissions()).unwrap().is_empty());
    }

    #[test]
    fn resource_limits_reject_overcommit() {
        let mut manager = VmManager::new(FakeBackend::default(), limits()).unwrap();
        manager.create_vm(config("first"), permissions()).unwrap();
        assert_eq!(
            manager.create_vm(config("second"), permissions()),
            Err(VmError::ResourceLimit("total memory / host reserve"))
        );
    }

    #[test]
    fn host_memory_reserve_rejects_create_independently_of_vm_memory_quota() {
        let mut resource_limits = limits();
        resource_limits.max_total_memory_mib = 4096;
        resource_limits.host_memory_reserve_mib = 3584;
        resource_limits.host_memory_total_mib = 4096;

        let mut manager = VmManager::new(FakeBackend::default(), resource_limits).unwrap();
        assert_eq!(
            manager.create_vm(config("reserve-test"), permissions()),
            Err(VmError::ResourceLimit("total memory / host reserve"))
        );

        // The reserve constraint alone rejects the request before backend allocation.
        assert_eq!(manager.into_backend().next, 0);
    }

    #[test]
    fn failed_backend_start_keeps_created_state() {
        let backend = FakeBackend {
            fail_start: true,
            ..FakeBackend::default()
        };
        let mut manager = VmManager::new(backend, limits()).unwrap();
        let id = manager
            .create_vm(config("linux-test"), permissions())
            .unwrap();
        assert_eq!(
            manager.start_vm(id, permissions()),
            Err(VmError::Backend("injected start failure".into()))
        );
        assert_eq!(
            manager.get_vm_state(id, permissions()),
            Ok(VmState::Created)
        );
    }

    #[test]
    fn running_vm_cannot_be_destroyed_directly() {
        let mut manager = VmManager::new(FakeBackend::default(), limits()).unwrap();
        let id = manager
            .create_vm(config("linux-test"), permissions())
            .unwrap();
        manager.start_vm(id, permissions()).unwrap();
        assert!(matches!(
            manager.destroy_vm(id, permissions()),
            Err(VmError::InvalidState {
                operation: "destroy",
                ..
            })
        ));
    }

    #[test]
    fn inspect_requires_permission() {
        let mut manager = VmManager::new(FakeBackend::default(), limits()).unwrap();
        let id = manager
            .create_vm(config("linux-test"), permissions())
            .unwrap();
        assert_eq!(
            manager.get_vm_state(id, VmPermissions::default()),
            Err(VmError::PermissionDenied)
        );
    }
}
