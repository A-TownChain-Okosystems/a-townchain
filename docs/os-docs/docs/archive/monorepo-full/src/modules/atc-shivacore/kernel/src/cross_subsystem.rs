// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Cross-Subsystem Integration Tests (K-Sprint 23)
//!
//! Testet den kompletten Kernel-Flow:
//!   Boot → Spawn → Memory → IPC → FS → Capability-Isolation
//!
//! Diese Tests beweisen, dass alle Subsysteme zusammenarbeiten,
//! nicht nur isoliert funktionieren.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::ats1000::{MemoryManager, FileSystem};
use crate::capability::{CapabilityTable, ResourceType, Rights};
use crate::memory_manager::{KernelMemoryManager, AllocSource, MemError};
use crate::atcfs::{AtcFileSystem, atc_content_id, FsError};
use crate::process::{ProcessManager, ProcessState};
use crate::ipc::{IpcSubsystem, ChannelId, Message, IpcError};
use crate::kernel_init::KernelState;

/// Test-Harness: erzeugt ein frisches Set aller Subsysteme
pub struct TestHarness {
    pub caps: CapabilityTable,
    pub mem: KernelMemoryManager,
    pub fs: AtcFileSystem,
    pub proc_mgr: ProcessManager,
    pub ipc: IpcSubsystem,
}

impl TestHarness {
    pub fn new() -> Self {
        Self {
            caps: CapabilityTable::new(),
            mem: KernelMemoryManager::new(),
            fs: AtcFileSystem::new(),
            proc_mgr: ProcessManager::new(),
            ipc: IpcSubsystem::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === FLOW 1: Boot → Spawn → Allocate → Write → Read → Free === //

    #[test]
    fn flow_full_process_lifecycle() {
        let mut h = TestHarness::new();

        // 1. Spawn a process (returns capability::Pid)
        let p1 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 128);
        assert!(p1.0 > 0);
        assert_eq!(h.proc_mgr.active_count(), 1);

        // 2. Allocate memory (takes ats1000::Pid = u32)
        let region = h.mem.allocate(&mut h.caps, p1, 4096).unwrap();
        assert_eq!(region.owner_pid, p1);

        // 3. Process should have capabilities for its memory
        assert!(h.caps.check(p1, ResourceType::Memory, region.region_id, Rights::READ));
        assert!(h.caps.check(p1, ResourceType::Memory, region.region_id, Rights::WRITE));

        // 4. Write a file on behalf of the process (takes capability::Pid)
        h.fs.write_file(&h.caps, "/tmp/process1.dat", b"process data", p1).unwrap();

        // 5. Read it back
        let (cid, node) = h.fs.read_file(&h.caps, "/tmp/process1.dat", p1).unwrap();
        assert_eq!(node.size, 12);
        assert_eq!(h.fs.get_content(&cid).unwrap(), b"process data");

        // 6. Free memory (takes ats1000::Pid = u32)
        h.mem.deallocate(&mut h.caps, p1, region.region_id).unwrap();
        assert_eq!(h.mem.region_count(), 0);

        // 7. Kill process
        h.proc_mgr.kill(p1, 0);
        assert_eq!(h.proc_mgr.active_count(), 0);
    }

    // === FLOW 2: IPC between two processes === //

    #[test]
    fn flow_ipc_between_processes() {
        let mut h = TestHarness::new();

        let sender = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);
        let receiver = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);

        // Allocate memory for each (uses ats1000::Pid = u32)
        let mem_s = h.mem.allocate(&mut h.caps, sender, 2048).unwrap();
        let mem_r = h.mem.allocate(&mut h.caps, receiver, 2048).unwrap();

        // Create IPC channel (uses capability::Pid)
        let ch = h.ipc.create_channel(&mut h.caps, receiver, 1024);

        // Grant send access to sender
        h.ipc.grant_access(&mut h.caps, receiver, ch, sender, Rights::WRITE).unwrap();

        // Sender sends a message (data: Vec<u8>, not Message struct)
        h.ipc.send(&h.caps, sender, ch, b"hello from sender".to_vec()).unwrap();

        // Receiver reads the message
        let received = h.ipc.recv(&h.caps, receiver, ch).unwrap();
        assert_eq!(received.data, b"hello from sender");
        assert_eq!(received.sender, sender);

        // Verify no more messages
        assert_eq!(h.ipc.pending_messages(ch).unwrap(), 0);

        // Cleanup
        h.mem.deallocate(&mut h.caps, sender, mem_s.region_id).unwrap();
        h.mem.deallocate(&mut h.caps, receiver, mem_r.region_id).unwrap();
        h.proc_mgr.kill(sender, 0);
        h.proc_mgr.kill(receiver, 0);
    }

    // === FLOW 3: Capability isolation === //

    #[test]
    fn flow_capability_isolation() {
        let mut h = TestHarness::new();

        let alice = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let bob = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);

        // Alice allocates memory (u32 Pid)
        let alice_mem = h.mem.allocate(&mut h.caps, alice, 4096).unwrap();

        // Bob tries to read Alice's memory — should fail (u32 Pid)
        assert_eq!(
            h.mem.read_check(&h.caps, bob, alice_mem.region_id),
            Err(MemError::NoCapability)
        );

        // Bob tries to free Alice's memory — should fail
        assert_eq!(
            h.mem.deallocate(&mut h.caps, bob, alice_mem.region_id),
            Err(MemError::NoCapability)
        );

        // Alice writes a private file (capability::Pid)
        h.fs.write_file(&h.caps, "/home/alice/secret.txt", b"secret", alice).unwrap();

        // Bob cannot read Alice's private file
        assert_eq!(
            h.fs.read_file(&h.caps, "/home/alice/secret.txt", bob),
            Err(FsError::PermissionDenied)
        );

        // But Bob can read public files
        h.fs.write_file(&h.caps, "/atc/public.txt", b"public", alice).unwrap();
        let (_, node) = h.fs.read_file(&h.caps, "/atc/public.txt", bob).unwrap();
        assert_eq!(node.size, 6);
    }

    // === FLOW 4: Parent-child with capability delegation === //

    #[test]
    fn flow_parent_child_delegation() {
        let mut h = TestHarness::new();

        let parent = h.proc_mgr.spawn(crate::process::ProcessType::Service, 200);
        let parent_mem = h.mem.allocate(&mut h.caps, parent, 8192).unwrap();

        let child = h.proc_mgr.spawn_child(parent, crate::process::ProcessType::Agent, 50).unwrap();

        // Child initially cannot access parent's memory
        assert_eq!(
            h.mem.read_check(&h.caps, child, parent_mem.region_id),
            Err(MemError::NoCapability)
        );

        // Parent delegates READ capability to child (use shared caps table)
        let cap_id = h.caps.list_for(parent).iter()
            .find(|c| c.resource_type == ResourceType::Memory && c.resource_id == parent_mem.region_id)
            .map(|c| c.id)
            .unwrap();
        h.caps.delegate(parent, cap_id, child, Rights::READ).unwrap();

        // Now child can read parent's memory
        assert!(h.mem.read_check(&h.caps, child, parent_mem.region_id).is_ok());

        // But child still cannot write (only READ was delegated)
        assert_eq!(
            h.mem.write_check(&h.caps, child, parent_mem.region_id),
            Err(MemError::NoCapability)
        );
    }

    // === FLOW 5: Multiple processes sharing an IPC channel === //

    #[test]
    fn flow_broadcast_channel() {
        let mut h = TestHarness::new();

        let p1 = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);
        let p2 = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);
        let p3 = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);

        // p1 creates a channel
        let ch = h.ipc.create_channel(&mut h.caps, p1, 4096);

        // p1 grants send to p2 and p3
        h.ipc.grant_access(&mut h.caps, p1, ch, p2, Rights::WRITE).unwrap();
        h.ipc.grant_access(&mut h.caps, p1, ch, p3, Rights::WRITE).unwrap();

        // p2 and p3 send messages
        h.ipc.send(&h.caps, p2, ch, b"from p2".to_vec()).unwrap();
        h.ipc.send(&h.caps, p3, ch, b"from p3".to_vec()).unwrap();

        // p1 reads both
        let m1 = h.ipc.recv(&h.caps, p1, ch).unwrap();
        let m2 = h.ipc.recv(&h.caps, p1, ch).unwrap();

        assert_eq!(m1.data, b"from p2");
        assert_eq!(m2.data, b"from p3");
    }

    // === FLOW 6: Content-addressed sharing === //

    #[test]
    fn flow_content_addressed_sharing() {
        let mut h = TestHarness::new();

        let writer = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let reader = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);

        let data = b"shared content for verification";
        h.fs.write_file(&h.caps, "/atc/shared.bin", data, writer).unwrap();

        let (cid_w, node_w) = h.fs.read_file(&h.caps, "/atc/shared.bin", writer).unwrap();
        let (cid_r, node_r) = h.fs.read_file(&h.caps, "/atc/shared.bin", reader).unwrap();

        assert_eq!(cid_w, cid_r);
        assert_eq!(node_w.size, node_r.size);

        let content = h.fs.get_content(&cid_w).unwrap();
        assert_eq!(content, data);

        let expected_cid = atc_content_id(data);
        assert_eq!(cid_w, expected_cid);
    }

    // === FLOW 7: Memory stats across multiple processes === //

    #[test]
    fn flow_memory_stats_multi_process() {
        let mut h = TestHarness::new();

        let p1 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let p2 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let p3 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);

        h.mem.allocate(&mut h.caps, p1, 256).unwrap();
        h.mem.allocate(&mut h.caps, p1, 8192).unwrap();
        h.mem.allocate(&mut h.caps, p2, 1024).unwrap();
        h.mem.allocate(&mut h.caps, p3, 4096).unwrap();

        let stats = h.mem.stats();
        assert_eq!(stats.total_allocated, 256 + 8192 + 1024 + 4096);
        assert_eq!(stats.active_regions, 4);

        assert_eq!(h.mem.regions_for(p1).len(), 2);
        assert_eq!(h.mem.regions_for(p2).len(), 1);
        assert_eq!(h.mem.regions_for(p3).len(), 1);

        let p1_regions = h.mem.regions_for(p1);
        h.mem.deallocate(&mut h.caps, p1, p1_regions[0].region_id).unwrap();

        let stats2 = h.mem.stats();
        assert_eq!(stats2.active_regions, 3);
        assert_eq!(stats2.peak_allocated, stats.total_allocated);
    }

    // === FLOW 8: Process priorities === //

    #[test]
    fn flow_process_priorities() {
        let mut h = TestHarness::new();

        let low = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 10);
        let high = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 250);
        let system = h.proc_mgr.spawn(crate::process::ProcessType::System, 255);

        let low_pcb = h.proc_mgr.get(low).unwrap();
        let high_pcb = h.proc_mgr.get(high).unwrap();
        let sys_pcb = h.proc_mgr.get(system).unwrap();

        assert!(low_pcb.priority < high_pcb.priority);
        assert!(high_pcb.priority <= sys_pcb.priority);
    }

    // === FLOW 9: Process state transitions === //

    #[test]
    fn flow_process_state_transitions() {
        let mut h = TestHarness::new();

        let p = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        
        assert_eq!(h.proc_mgr.get(p).unwrap().state, ProcessState::Ready);

        h.proc_mgr.set_running(p);
        assert_eq!(h.proc_mgr.get(p).unwrap().state, ProcessState::Running);

        h.proc_mgr.set_blocked(p);
        assert_eq!(h.proc_mgr.get(p).unwrap().state, ProcessState::Blocked);

        h.proc_mgr.unblock(p);
        assert_eq!(h.proc_mgr.get(p).unwrap().state, ProcessState::Ready);

        h.proc_mgr.kill(p, 42);
        assert_eq!(h.proc_mgr.get(p).unwrap().state, ProcessState::Terminated(42));

        assert_eq!(h.proc_mgr.wait(p), Some(42));
    }

    // === FLOW 10: KernelState boot + smoke === //

    #[test]
    fn flow_kernel_state_boot_and_smoke() {
        let mut state = KernelState::boot().unwrap();
        let log = state.boot_log();
        assert!(log.contains("Boot Complete"));
        state.smoke_test().unwrap();
        assert_eq!(state.memory.stats().total_allocated, 0);
        assert_eq!(state.memory.stats().active_regions, 0);
    }

    // === FLOW 11: IPC cleanup on process kill === //

    #[test]
    fn flow_ipc_cleanup_on_kill() {
        let mut h = TestHarness::new();

        let owner = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let _user = h.proc_mgr.spawn(crate::process::ProcessType::Agent, 100);

        let ch1 = h.ipc.create_channel(&mut h.caps, owner, 1024);
        let ch2 = h.ipc.create_channel(&mut h.caps, owner, 1024);
        let ch3 = h.ipc.create_channel(&mut h.caps, owner, 2048);

        assert_eq!(h.ipc.channel_count(), 3);

        h.ipc.grant_access(&mut h.caps, owner, ch1, _user, Rights::WRITE).unwrap();

        // close_all_for closes capabilities and channels owned by this PID
        let cleaned = h.ipc.close_all_for(&mut h.caps, owner);
        assert_eq!(cleaned, 3);
    }

    // === FLOW 12: Mixed heap and userspace allocations === //

    #[test]
    fn flow_mixed_allocations() {
        let mut h = TestHarness::new();

        let p1 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let p2 = h.proc_mgr.spawn(crate::process::ProcessType::Contract, 200);

        let small1 = h.mem.allocate(&mut h.caps, p1, 512).unwrap();
        let large1 = h.mem.allocate(&mut h.caps, p1, 65536).unwrap();
        let small2 = h.mem.allocate(&mut h.caps, p2, 1024).unwrap();
        let large2 = h.mem.allocate(&mut h.caps, p2, 131072).unwrap();

        assert_eq!(small1.source, AllocSource::KernelHeap);
        assert_eq!(large1.source, AllocSource::UserspaceBump);
        assert_eq!(small2.source, AllocSource::KernelHeap);
        assert_eq!(large2.source, AllocSource::UserspaceBump);

        h.mem.deallocate(&mut h.caps, p1, large1.region_id).unwrap();
        h.mem.deallocate(&mut h.caps, p2, small2.region_id).unwrap();
        h.mem.deallocate(&mut h.caps, p1, small1.region_id).unwrap();
        h.mem.deallocate(&mut h.caps, p2, large2.region_id).unwrap();

        assert_eq!(h.mem.region_count(), 0);
        assert_eq!(h.mem.stats().total_allocated, 0);
    }

    // === FLOW 13: ats1000 Trait compatibility === //

    #[test]
    fn flow_ats1000_memory_trait() {
        let mut mm = KernelMemoryManager::new();
        let r = MemoryManager::alloc(&mut mm, 4096, crate::ats1000::Pid(1));
        assert!(r.is_some());
        let region = r.unwrap();
        assert_eq!(region.size, 4096);
        assert_eq!(region.pid, crate::ats1000::Pid(1));

        assert!(MemoryManager::free(&mut mm, region));
        let r2 = MemoryManager::mmap(&mut mm, 0x5000, 8192);
        assert!(r2.is_some());
        assert_eq!(r2.unwrap().size, 8192);
    }

    #[test]
    fn flow_ats1000_fs_trait() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();
        
        fs.write_file(&caps, "/tmp/trait_test.bin", b"trait test data", crate::ats1000::Pid(1)).unwrap();

        let fh = FileSystem::open(&mut fs, "/tmp/trait_test.bin", 0).unwrap();
        assert!(fh > 0);

        let mut buf = [0u8; 100];
        let n = FileSystem::read(&mut fs, fh, &mut buf);
        assert_eq!(n, 15);
        assert_eq!(&buf[..15], b"trait test data");

        assert!(FileSystem::close(&mut fs, fh));
        assert!(!FileSystem::close(&mut fs, fh));
    }

    // === FLOW 14: FS manifest after multi-process writes === //

    #[test]
    fn flow_fs_manifest_multi_process() {
        let mut h = TestHarness::new();

        let p1 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);
        let p2 = h.proc_mgr.spawn(crate::process::ProcessType::Service, 100);

        h.fs.write_file(&h.caps, "/atc/file1.txt", b"file1 from p1", p1).unwrap();
        h.fs.write_file(&h.caps, "/atc/file2.txt", b"file2 from p1", p1).unwrap();
        h.fs.write_file(&h.caps, "/atc/file3.txt", b"file3 from p2", p2).unwrap();

        let manifest = h.fs.export_manifest();
        assert_eq!(manifest.file_count, 3);
        assert_eq!(manifest.total_size, 39);
        assert!(!manifest.root_hash.is_empty());

        h.fs.write_file(&h.caps, "/atc/file4.txt", b"file4", p2).unwrap();
        let manifest2 = h.fs.export_manifest();
        assert_ne!(manifest.root_hash, manifest2.root_hash);
        assert_eq!(manifest2.file_count, 4);
    }

    // === FLOW 15: Stress test — 50 processes === //

    #[test]
    fn flow_stress_50_processes() {
        let mut h = TestHarness::new();

        let mut pids = Vec::new();
        let mut regions = Vec::new();

        for i in 0..50u32 {
            let p = h.proc_mgr.spawn(crate::process::ProcessType::Agent, ((i % 200) + 1) as u8);
            pids.push(p);

            let r = h.mem.allocate(&mut h.caps, p, 128).unwrap();
            regions.push(r);

            let path = format!("/tmp/p_{}.dat", i);
            let data = format!("data for process iteration {}", i);
            h.fs.write_file(&h.caps, &path, data.as_bytes(), p).unwrap();
        }

        assert_eq!(h.proc_mgr.active_count(), 50);
        assert_eq!(h.mem.region_count(), 50);
        let stats = h.mem.stats();
        assert_eq!(stats.total_allocated, 50 * 128);

        for r in &regions {
            h.mem.deallocate(&mut h.caps, r.owner_pid, r.region_id).unwrap();
        }
        assert_eq!(h.mem.region_count(), 0);

        for p in &pids {
            h.proc_mgr.kill(*p, 0);
        }
        assert_eq!(h.proc_mgr.active_count(), 0);

        let manifest = h.fs.export_manifest();
        assert_eq!(manifest.file_count, 50);
    }
}
