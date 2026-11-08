// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 41: Container Isolation + Agent Sandboxing
// Copyright (c) 2026 Michael Wroblewski. All rights reserved.
//
// Container-Runtime für KI-Agenten-Isolation auf dem dezentralen KI-Betriebssystem.
// Implementiert:
//   1. NAMESPACES — PID, Mount, Network, IPC, UTS, User, Cgroup
//   2. RESOURCE LIMITS — CPU, Memory, I/O, PID-Count, FD-Count (Cgroup-Style)
//   3. CONTAINER LIFECYCLE — Create, Start, Pause, Resume, Stop, Destroy
//   4. AGENT SANDBOXING — KI-Agenten isoliert in Containern mit Capability-Restrictions
//   5. CONTAINER IMAGE — RootFS-Pfad, Environment, Entry-Point, Volume-Mounts
//   6. HEALTH CHECKS — Liveness/Readiness Probes für Agent-Container
//   7. SECURITY — Seccomp-ähnliche Syscall-Filter pro Container

#![allow(dead_code)]

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

// ════════════════════════════════════════════════════════════════
//  CONSTANTS
// ════════════════════════════════════════════════════════════════

const MAX_NAMESPACES: usize = 256;
const MAX_CONTAINERS: usize = 128;
const MAX_RESOURCE_LIMITS: usize = 16;
const MAX_ENV_VARS: usize = 64;
const MAX_VOLUME_MOUNTS: usize = 16;
const MAX_PORT_MAPPINGS: usize = 32;
const MAX_AGENT_CAPABILITIES: usize = 32;
const MAX_HEALTH_RETRIES: u8 = 3;
const DEFAULT_CPU_QUOTA_US: u64 = 100_000;     // 100ms in microseconds
const DEFAULT_MEM_LIMIT_BYTES: u64 = 512 * 1024 * 1024; // 512 MB
const DEFAULT_PID_LIMIT: u32 = 256;
const DEFAULT_FD_LIMIT: u32 = 1024;
const DEFAULT_IO_BPS: u64 = 10 * 1024 * 1024;   // 10 MB/s

// ════════════════════════════════════════════════════════════════
//  NAMESPACE TYPES
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NamespaceType {
    Pid = 0,       // PID-Isolation: Container sieht nur eigene Prozesse
    Mount = 1,     // Mount-Isolation: eigenes VFS-Root
    Network = 2,   // Netzwerk-Isolation: eigenes Interface, IP, Routing
    Ipc = 3,       // IPC-Isolation: eigene Channels, SHM
    Uts = 4,       // UTS-Isolation: eigener Hostname
    User = 5,      // User-Isolation: eigener UID/GID-Mapping
    Cgroup = 6,    // Cgroup-Isolation: eigene Resource-Limits
}

impl NamespaceType {
    pub fn name(&self) -> &'static str {
        match self {
            NamespaceType::Pid => "pid",
            NamespaceType::Mount => "mount",
            NamespaceType::Network => "net",
            NamespaceType::Ipc => "ipc",
            NamespaceType::Uts => "uts",
            NamespaceType::User => "user",
            NamespaceType::Cgroup => "cgroup",
        }
    }

    pub fn all() -> [NamespaceType; 7] {
        [
            NamespaceType::Pid,
            NamespaceType::Mount,
            NamespaceType::Network,
            NamespaceType::Ipc,
            NamespaceType::Uts,
            NamespaceType::User,
            NamespaceType::Cgroup,
        ]
    }
}

// ════════════════════════════════════════════════════════════════
//  NAMESPACE INSTANCE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Namespace {
    pub ns_id: u32,
    pub ns_type: NamespaceType,
    pub parent_id: Option<u32>,
    pub owner_container: u32,
    pub process_ids: Vec<u32>,
    pub hostname: String,
    pub network_config: Option<NetworkConfig>,
    pub mount_root: Option<String>,
    pub uid_map: Vec<(u32, u32, u32)>,  // (container_uid, host_uid, count)
    pub gid_map: Vec<(u32, u32, u32)>,
    pub created_at: u64,
}

impl Namespace {
    pub fn new(ns_id: u32, ns_type: NamespaceType, container_id: u32) -> Self {
        Self {
            ns_id,
            ns_type,
            parent_id: None,
            owner_container: container_id,
            process_ids: Vec::new(),
            hostname: String::new(),
            network_config: None,
            mount_root: None,
            uid_map: vec![(0, 0, 1)],
            gid_map: vec![(0, 0, 1)],
            created_at: 0,
        }
    }

    pub fn add_process(&mut self, pid: u32) {
        if !self.process_ids.contains(&pid) {
            self.process_ids.push(pid);
        }
    }

    pub fn remove_process(&mut self, pid: u32) -> bool {
        let before = self.process_ids.len();
        self.process_ids.retain(|&p| p != pid);
        self.process_ids.len() < before
    }

    pub fn has_process(&self, pid: u32) -> bool {
        self.process_ids.contains(&pid)
    }

    pub fn process_count(&self) -> usize {
        self.process_ids.len()
    }

    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = hostname.to_string();
    }

    pub fn set_network(&mut self, config: NetworkConfig) {
        self.network_config = Some(config);
    }

    pub fn set_mount_root(&mut self, path: &str) {
        self.mount_root = Some(path.to_string());
    }

    pub fn set_uid_map(&mut self, mappings: Vec<(u32, u32, u32)>) {
        self.uid_map = mappings;
    }

    pub fn set_gid_map(&mut self, mappings: Vec<(u32, u32, u32)>) {
        self.gid_map = mappings;
    }

    pub fn map_uid(&self, container_uid: u32) -> Option<u32> {
        for (c_uid, h_uid, count) in &self.uid_map {
            if container_uid >= *c_uid && container_uid < *c_uid + *count {
                return Some(h_uid + (container_uid - c_uid));
            }
        }
        None
    }

    pub fn map_gid(&self, container_gid: u32) -> Option<u32> {
        for (c_gid, h_gid, count) in &self.gid_map {
            if container_gid >= *c_gid && container_gid < *c_gid + *count {
                return Some(h_gid + (container_gid - c_gid));
            }
        }
        None
    }

    pub fn reverse_map_uid(&self, host_uid: u32) -> Option<u32> {
        for (c_uid, h_uid, count) in &self.uid_map {
            if host_uid >= *h_uid && host_uid < *h_uid + *count {
                return Some(c_uid + (host_uid - h_uid));
            }
        }
        None
    }
}

// ════════════════════════════════════════════════════════════════
//  NETWORK CONFIGURATION (per Namespace)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkConfig {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
    pub mac_address: String,
    pub mtu: u16,
    pub ports: Vec<PortMapping>,
}

impl NetworkConfig {
    pub fn new(ip: &str, netmask: &str, gateway: &str) -> Self {
        Self {
            ip_address: ip.to_string(),
            netmask: netmask.to_string(),
            gateway: gateway.to_string(),
            dns_servers: vec!["8.8.8.8".to_string()],
            mac_address: generate_mac(),
            mtu: 1500,
            ports: Vec::new(),
        }
    }

    pub fn add_port(&mut self, host_port: u16, container_port: u16, protocol: PortProtocol) {
        self.ports.push(PortMapping {
            host_port,
            container_port,
            protocol,
        });
    }

    pub fn remove_port(&mut self, host_port: u16) -> bool {
        let before = self.ports.len();
        self.ports.retain(|p| p.host_port != host_port);
        self.ports.len() < before
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PortMapping {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: PortProtocol,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortProtocol {
    Tcp,
    Udp,
}

impl PortProtocol {
    pub fn name(&self) -> &'static str {
        match self {
            PortProtocol::Tcp => "tcp",
            PortProtocol::Udp => "udp",
        }
    }
}

fn generate_mac() -> String {
    // Deterministic MAC for container (02:xx:xx:xx:xx:xx — locally administered)
    let ns_id = 0u32; // Will be set by caller context
    format!("02:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        (ns_id >> 16) & 0xFF, (ns_id >> 8) & 0xFF, ns_id & 0xFF,
        (ns_id >> 8) & 0xFF, ns_id & 0xFF)
}

// ════════════════════════════════════════════════════════════════
//  RESOURCE LIMITS (Cgroup-Style)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ResourceLimits {
    // CPU
    pub cpu_quota_us: u64,         // Microseconds per period
    pub cpu_period_us: u64,        // Period length
    pub cpu_shares: u32,           // Relative weight (1024 = default)
    pub cpuset: String,            // CPU affinity mask ("0-3,8")

    // Memory
    pub memory_limit_bytes: u64,   // Hard limit
    pub memory_soft_limit: u64,    // Soft limit (throttle, don't kill)
    pub memory_swap_limit: u64,    // Swap limit (0 = no swap)
    pub memory_oom_kill: bool,     // Kill process on OOM (true) or block (false)

    // I/O
    pub io_read_bps: u64,           // Max read bytes/sec
    pub io_write_bps: u64,          // Max write bytes/sec
    pub io_iops: u32,               // Max I/O operations/sec

    // Process limits
    pub pid_max: u32,               // Max processes in container
    pub fd_max: u32,                // Max file descriptors

    // Network
    pub net_rx_bps: u64,            // Max receive bytes/sec
    pub net_tx_bps: u64,            // Max transmit bytes/sec
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu_quota_us: DEFAULT_CPU_QUOTA_US,
            cpu_period_us: 100_000,
            cpu_shares: 1024,
            cpuset: String::new(),

            memory_limit_bytes: DEFAULT_MEM_LIMIT_BYTES,
            memory_soft_limit: DEFAULT_MEM_LIMIT_BYTES / 2,
            memory_swap_limit: 0,
            memory_oom_kill: true,

            io_read_bps: DEFAULT_IO_BPS,
            io_write_bps: DEFAULT_IO_BPS,
            io_iops: 1000,

            pid_max: DEFAULT_PID_LIMIT,
            fd_max: DEFAULT_FD_LIMIT,

            net_rx_bps: 100 * 1024 * 1024,  // 100 MB/s
            net_tx_bps: 100 * 1024 * 1024,
        }
    }
}

impl ResourceLimits {
    pub fn unlimited() -> Self {
        Self {
            cpu_quota_us: 0,             // 0 = no limit
            cpu_period_us: 100_000,
            cpu_shares: 1024,
            cpuset: String::new(),

            memory_limit_bytes: 0,        // 0 = no limit
            memory_soft_limit: 0,
            memory_swap_limit: 0,
            memory_oom_kill: false,

            io_read_bps: 0,
            io_write_bps: 0,
            io_iops: 0,

            pid_max: 0,
            fd_max: 0,

            net_rx_bps: 0,
            net_tx_bps: 0,
        }
    }

    pub fn minimal() -> Self {
        Self {
            cpu_quota_us: 10_000,          // 10ms
            cpu_period_us: 100_000,
            cpu_shares: 100,
            cpuset: "0".to_string(),

            memory_limit_bytes: 32 * 1024 * 1024,  // 32 MB
            memory_soft_limit: 16 * 1024 * 1024,
            memory_swap_limit: 0,
            memory_oom_kill: true,

            io_read_bps: 1024 * 1024,      // 1 MB/s
            io_write_bps: 512 * 1024,       // 512 KB/s
            io_iops: 100,

            pid_max: 16,
            fd_max: 64,

            net_rx_bps: 1024 * 1024,        // 1 MB/s
            net_tx_bps: 512 * 1024,
        }
    }

    pub fn high_perf() -> Self {
        Self {
            cpu_quota_us: 0,               // No limit
            cpu_period_us: 100_000,
            cpu_shares: 4096,
            cpuset: "0-7".to_string(),      // All 8 cores

            memory_limit_bytes: 4 * 1024 * 1024 * 1024,  // 4 GB
            memory_soft_limit: 2 * 1024 * 1024 * 1024,
            memory_swap_limit: 1 * 1024 * 1024 * 1024,
            memory_oom_kill: true,

            io_read_bps: 500 * 1024 * 1024, // 500 MB/s
            io_write_bps: 200 * 1024 * 1024,
            io_iops: 10000,

            pid_max: 1024,
            fd_max: 8192,

            net_rx_bps: 1024 * 1024 * 1024, // 1 GB/s
            net_tx_bps: 1024 * 1024 * 1024,
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  RESOURCE USAGE TRACKING
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Default)]
pub struct ResourceUsage {
    pub cpu_used_us: u64,
    pub memory_used_bytes: u64,
    pub memory_peak_bytes: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub pid_count: u32,
    pub fd_count: u32,
}

impl ResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_cpu_limit(&self, limits: &ResourceLimits) -> bool {
        if limits.cpu_quota_us == 0 {
            return true;
        }
        self.cpu_used_us < limits.cpu_quota_us
    }

    pub fn check_memory_limit(&self, limits: &ResourceLimits) -> bool {
        if limits.memory_limit_bytes == 0 {
            return true;
        }
        self.memory_used_bytes < limits.memory_limit_bytes
    }

    pub fn check_pid_limit(&self, limits: &ResourceLimits) -> bool {
        if limits.pid_max == 0 {
            return true;
        }
        self.pid_count < limits.pid_max
    }

    pub fn check_fd_limit(&self, limits: &ResourceLimits) -> bool {
        if limits.fd_max == 0 {
            return true;
        }
        self.fd_count < limits.fd_max
    }

    pub fn check_all(&self, limits: &ResourceLimits) -> Result<(), ResourceError> {
        if !self.check_memory_limit(limits) {
            return Err(ResourceError::MemoryExceeded);
        }
        if !self.check_pid_limit(limits) {
            return Err(ResourceError::PidLimitExceeded);
        }
        if !self.check_fd_limit(limits) {
            return Err(ResourceError::FdLimitExceeded);
        }
        if !self.check_cpu_limit(limits) {
            return Err(ResourceError::CpuQuotaExceeded);
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER IMAGE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ContainerImage {
    pub name: String,
    pub version: String,
    pub rootfs_path: String,
    pub entry_point: String,
    pub args: Vec<String>,
    pub env_vars: Vec<(String, String)>,
    pub working_dir: String,
    pub volumes: Vec<VolumeMount>,
    pub exposed_ports: Vec<u16>,
    pub labels: Vec<(String, String)>,
}

impl ContainerImage {
    pub fn new(name: &str, rootfs: &str, entry: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "latest".to_string(),
            rootfs_path: rootfs.to_string(),
            entry_point: entry.to_string(),
            args: Vec::new(),
            env_vars: Vec::new(),
            working_dir: "/".to_string(),
            volumes: Vec::new(),
            exposed_ports: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn add_env(&mut self, key: &str, value: &str) {
        if self.env_vars.len() < MAX_ENV_VARS {
            self.env_vars.push((key.to_string(), value.to_string()));
        }
    }

    pub fn add_arg(&mut self, arg: &str) {
        self.args.push(arg.to_string());
    }

    pub fn add_volume(&mut self, volume: VolumeMount) {
        if self.volumes.len() < MAX_VOLUME_MOUNTS {
            self.volumes.push(volume);
        }
    }

    pub fn add_label(&mut self, key: &str, value: &str) {
        self.labels.push((key.to_string(), value.to_string()));
    }

    pub fn get_env(&self, key: &str) -> Option<&str> {
        for (k, v) in &self.env_vars {
            if k == key {
                return Some(v.as_str());
            }
        }
        None
    }

    pub fn get_label(&self, key: &str) -> Option<&str> {
        for (k, v) in &self.labels {
            if k == key {
                return Some(v.as_str());
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct VolumeMount {
    pub host_path: String,
    pub container_path: String,
    pub read_only: bool,
    pub mount_type: MountType,
}

impl VolumeMount {
    pub fn new(host: &str, container: &str, read_only: bool) -> Self {
        Self {
            host_path: host.to_string(),
            container_path: container.to_string(),
            read_only,
            mount_type: MountType::Bind,
        }
    }

    pub fn tmpfs(container: &str, size: u64) -> Self {
        Self {
            host_path: format!("tmpfs:{}", size),
            container_path: container.to_string(),
            read_only: false,
            mount_type: MountType::Tmpfs,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountType {
    Bind,       // Host directory mounted into container
    Tmpfs,      // In-memory filesystem
    Overlay,    // Union filesystem (copy-on-write)
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER STATE
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Created,    // Image loaded, not started
    Running,    // Process active
    Paused,     // Suspended (frozen)
    Stopped,    // Terminated
    Error,      // Failed state
    Destroyed,  // Fully cleaned up
}

impl ContainerState {
    pub fn name(&self) -> &'static str {
        match self {
            ContainerState::Created => "created",
            ContainerState::Running => "running",
            ContainerState::Paused => "paused",
            ContainerState::Stopped => "stopped",
            ContainerState::Error => "error",
            ContainerState::Destroyed => "destroyed",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ContainerState::Running | ContainerState::Paused)
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Container {
    pub container_id: u32,
    pub name: String,
    pub image: ContainerImage,
    pub state: ContainerState,
    pub namespaces: Vec<Namespace>,
    pub resource_limits: ResourceLimits,
    pub resource_usage: ResourceUsage,
    pub init_pid: Option<u32>,
    pub process_ids: Vec<u32>,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
    pub exit_code: Option<i32>,
    pub hostname: String,
    pub restart_count: u8,
    pub max_restarts: u8,
    pub auto_restart: bool,
    pub health_status: HealthStatus,
    pub syscall_filter: SyscallFilter,
    pub agent_id: Option<String>,     // KI-Agent ID if this is an agent container
    pub network_enabled: bool,
    pub capabilities_granted: Vec<String>,
}

impl Container {
    pub fn new(id: u32, name: &str, image: ContainerImage) -> Self {
        Self {
            container_id: id,
            name: name.to_string(),
            image,
            state: ContainerState::Created,
            namespaces: Vec::new(),
            resource_limits: ResourceLimits::default(),
            resource_usage: ResourceUsage::new(),
            init_pid: None,
            process_ids: Vec::new(),
            created_at: 0,
            started_at: None,
            finished_at: None,
            exit_code: None,
            hostname: format!("container-{}", id),
            restart_count: 0,
            max_restarts: 3,
            auto_restart: false,
            health_status: HealthStatus::Unknown,
            syscall_filter: SyscallFilter::default(),
            agent_id: None,
            network_enabled: true,
            capabilities_granted: Vec::new(),
        }
    }

    pub fn add_namespace(&mut self, ns: Namespace) {
        if self.namespaces.len() < MAX_NAMESPACES {
            self.namespaces.push(ns);
        }
    }

    pub fn get_namespace(&self, ns_type: NamespaceType) -> Option<&Namespace> {
        self.namespaces.iter().find(|ns| ns.ns_type == ns_type)
    }

    pub fn get_namespace_mut(&mut self, ns_type: NamespaceType) -> Option<&mut Namespace> {
        self.namespaces.iter_mut().find(|ns| ns.ns_type == ns_type)
    }

    pub fn add_process(&mut self, pid: u32) {
        if !self.process_ids.contains(&pid) {
            self.process_ids.push(pid);
            self.resource_usage.pid_count = self.process_ids.len() as u32;
        }
    }

    pub fn remove_process(&mut self, pid: u32) -> bool {
        let before = self.process_ids.len();
        self.process_ids.retain(|&p| p != pid);
        self.resource_usage.pid_count = self.process_ids.len() as u32;
        self.process_ids.len() < before
    }

    pub fn has_process(&self, pid: u32) -> bool {
        self.process_ids.contains(&pid)
    }

    pub fn process_count(&self) -> usize {
        self.process_ids.len()
    }

    pub fn set_agent(&mut self, agent_id: &str) {
        self.agent_id = Some(agent_id.to_string());
    }

    pub fn is_agent_container(&self) -> bool {
        self.agent_id.is_some()
    }

    pub fn grant_capability(&mut self, cap: &str) {
        if !self.capabilities_granted.iter().any(|c| c == cap) {
            self.capabilities_granted.push(cap.to_string());
        }
    }

    pub fn revoke_capability(&mut self, cap: &str) -> bool {
        let before = self.capabilities_granted.len();
        self.capabilities_granted.retain(|c| c != cap);
        self.capabilities_granted.len() < before
    }

    pub fn has_capability(&self, cap: &str) -> bool {
        self.capabilities_granted.iter().any(|c| c == cap)
    }

    pub fn set_syscall_filter(&mut self, filter: SyscallFilter) {
        self.syscall_filter = filter;
    }

    pub fn check_syscall(&self, syscall: &str) -> bool {
        self.syscall_filter.check(syscall)
    }

    pub fn uptime_ms(&self) -> u64 {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => end.saturating_sub(start),
            (Some(start), None) => 0,  // Still running — caller provides current time
            _ => 0,
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.health_status, HealthStatus::Healthy)
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.health_status, HealthStatus::Healthy | HealthStatus::Starting)
    }

    pub fn record_resource_usage(&mut self, cpu_us: u64, mem_bytes: u64, io_read: u64, io_write: u64, net_rx: u64, net_tx: u64) {
        self.resource_usage.cpu_used_us += cpu_us;
        self.resource_usage.memory_used_bytes = mem_bytes;
        if mem_bytes > self.resource_usage.memory_peak_bytes {
            self.resource_usage.memory_peak_bytes = mem_bytes;
        }
        self.resource_usage.io_read_bytes += io_read;
        self.resource_usage.io_write_bytes += io_write;
        self.resource_usage.net_rx_bytes += net_rx;
        self.resource_usage.net_tx_bytes += net_tx;
    }

    pub fn check_limits(&self) -> Result<(), ResourceError> {
        self.resource_usage.check_all(&self.resource_limits)
    }

    pub fn snapshot(&self) -> ContainerSnapshot {
        ContainerSnapshot {
            container_id: self.container_id,
            name: self.name.clone(),
            state: self.state,
            pid_count: self.process_count() as u32,
            cpu_used_us: self.resource_usage.cpu_used_us,
            memory_used_bytes: self.resource_usage.memory_used_bytes,
            memory_peak_bytes: self.resource_usage.memory_peak_bytes,
            io_read_bytes: self.resource_usage.io_read_bytes,
            io_write_bytes: self.resource_usage.io_write_bytes,
            net_rx_bytes: self.resource_usage.net_rx_bytes,
            net_tx_bytes: self.resource_usage.net_tx_bytes,
            health: self.health_status,
            uptime_ms: self.uptime_ms(),
            restart_count: self.restart_count,
            agent_id: self.agent_id.clone(),
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER SNAPSHOT (for monitoring/API)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ContainerSnapshot {
    pub container_id: u32,
    pub name: String,
    pub state: ContainerState,
    pub pid_count: u32,
    pub cpu_used_us: u64,
    pub memory_used_bytes: u64,
    pub memory_peak_bytes: u64,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub health: HealthStatus,
    pub uptime_ms: u64,
    pub restart_count: u8,
    pub agent_id: Option<String>,
}

// ════════════════════════════════════════════════════════════════
//  HEALTH STATUS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Unknown,        // No health check configured
    Starting,        // Container starting, not yet healthy
    Healthy,        // Health check passing
    Unhealthy,      // Health check failing
    Degraded,       // Some checks failing, container still running
}

impl HealthStatus {
    pub fn name(&self) -> &'static str {
        match self {
            HealthStatus::Unknown => "unknown",
            HealthStatus::Starting => "starting",
            HealthStatus::Healthy => "healthy",
            HealthStatus::Unhealthy => "unhealthy",
            HealthStatus::Degraded => "degraded",
        }
    }
}

#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub check_type: HealthCheckType,
    pub interval_ms: u64,
    pub timeout_ms: u64,
    pub start_delay_ms: u64,
    pub retries: u8,
    pub consecutive_failures: u8,
    pub last_check_at: Option<u64>,
    pub last_result: Option<bool>,
}

impl HealthCheck {
    pub fn new(check_type: HealthCheckType) -> Self {
        Self {
            check_type,
            interval_ms: 10_000,       // 10 seconds
            timeout_ms: 5_000,          // 5 seconds
            start_delay_ms: 5_000,      // 5 seconds start delay
            retries: MAX_HEALTH_RETRIES,
            consecutive_failures: 0,
            last_check_at: None,
            last_result: None,
        }
    }

    pub fn record_result(&mut self, success: bool) -> HealthStatus {
        self.last_result = Some(success);
        if success {
            self.consecutive_failures = 0;
            HealthStatus::Healthy
        } else {
            self.consecutive_failures += 1;
            if self.consecutive_failures >= self.retries {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Degraded
            }
        }
    }

    pub fn should_check(&self, current_time: u64) -> bool {
        match self.last_check_at {
            None => current_time >= self.start_delay_ms,
            Some(last) => current_time >= last + self.interval_ms,
        }
    }
}

#[derive(Clone, Debug)]
pub enum HealthCheckType {
    Process,          // Check if init process is alive
    TcpPort(u16),     // Check TCP port accepts connections
    HttpPath(String), // HTTP GET to path, expect 200
    Custom(String),   // Custom check command
}

// ════════════════════════════════════════════════════════════════
//  SYSCALL FILTER (Seccomp-Style)
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SyscallFilter {
    pub mode: FilterMode,
    pub allowed: Vec<String>,
    pub blocked: Vec<String>,
}

impl Default for SyscallFilter {
    fn default() -> Self {
        Self {
            mode: FilterMode::AllowAll,
            allowed: Vec::new(),
            blocked: Vec::new(),
        }
    }
}

impl SyscallFilter {
    pub fn allow_all() -> Self {
        Self::default()
    }

    pub fn block_list(blocked: Vec<String>) -> Self {
        Self {
            mode: FilterMode::BlockList,
            allowed: Vec::new(),
            blocked,
        }
    }

    pub fn allow_list(allowed: Vec<String>) -> Self {
        Self {
            mode: FilterMode::AllowList,
            allowed,
            blocked: Vec::new(),
        }
    }

    pub fn check(&self, syscall: &str) -> bool {
        match self.mode {
            FilterMode::AllowAll => true,
            FilterMode::BlockList => !self.blocked.iter().any(|s| s == syscall),
            FilterMode::AllowList => self.allowed.iter().any(|s| s == syscall),
        }
    }

    pub fn add_blocked(&mut self, syscall: &str) {
        if !self.blocked.iter().any(|s| s == syscall) {
            self.blocked.push(syscall.to_string());
        }
    }

    pub fn add_allowed(&mut self, syscall: &str) {
        if !self.allowed.iter().any(|s| s == syscall) {
            self.allowed.push(syscall.to_string());
        }
    }

    pub fn agent_sandbox() -> Self {
        // Secure profile for AI agent containers
        Self::block_list(vec![
            "reboot".to_string(),
            "shutdown".to_string(),
            "mount".to_string(),
            "umount".to_string(),
            "pivot_root".to_string(),
            "swapon".to_string(),
            "swapoff".to_string(),
            "ksettimer".to_string(),
            "init_module".to_string(),
            "finit_module".to_string(),
            "delete_module".to_string(),
            "iopl".to_string(),
            "ioperm".to_string(),
            "kexec_load".to_string(),
            "perf_event_open".to_string(),
            "personality".to_string(),
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilterMode {
    AllowAll,       // No filtering
    BlockList,      // Block specific syscalls
    AllowList,      // Only allow specific syscalls
}

// ════════════════════════════════════════════════════════════════
//  RESOURCE ERRORS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceError {
    MemoryExceeded,
    CpuQuotaExceeded,
    PidLimitExceeded,
    FdLimitExceeded,
    IoLimitExceeded,
    NetLimitExceeded,
}

impl ResourceError {
    pub fn name(&self) -> &'static str {
        match self {
            ResourceError::MemoryExceeded => "memory_limit_exceeded",
            ResourceError::CpuQuotaExceeded => "cpu_quota_exceeded",
            ResourceError::PidLimitExceeded => "pid_limit_exceeded",
            ResourceError::FdLimitExceeded => "fd_limit_exceeded",
            ResourceError::IoLimitExceeded => "io_limit_exceeded",
            ResourceError::NetLimitExceeded => "net_limit_exceeded",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER ERRORS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerError {
    NotFound,
    AlreadyExists,
    NotCreated,
    NotRunning,
    AlreadyRunning,
    Paused,
    MaxContainersReached,
    MaxRestartsReached,
    ResourceLimit(ResourceError),
    SyscallBlocked,
    AgentNotRegistered,
    InvalidImage,
    InvalidNamespace,
}

impl ContainerError {
    pub fn name(&self) -> &'static str {
        match self {
            ContainerError::NotFound => "not_found",
            ContainerError::AlreadyExists => "already_exists",
            ContainerError::NotCreated => "not_created",
            ContainerError::NotRunning => "not_running",
            ContainerError::AlreadyRunning => "already_running",
            ContainerError::Paused => "paused",
            ContainerError::MaxContainersReached => "max_containers_reached",
            ContainerError::MaxRestartsReached => "max_restarts_reached",
            ContainerError::ResourceLimit(_) => "resource_limit",
            ContainerError::SyscallBlocked => "syscall_blocked",
            ContainerError::AgentNotRegistered => "agent_not_registered",
            ContainerError::InvalidImage => "invalid_image",
            ContainerError::InvalidNamespace => "invalid_namespace",
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER MANAGER
// ════════════════════════════════════════════════════════════════

pub struct ContainerManager {
    containers: BTreeMap<u32, Container>,
    namespace_counter: u32,
    container_counter: u32,
    agent_containers: BTreeMap<String, u32>,  // agent_id → container_id
    health_checks: BTreeMap<u32, HealthCheck>,  // container_id → check
    total_created: u64,
    total_started: u64,
    total_stopped: u64,
    total_restarts: u64,
    total_failed: u64,
}

impl ContainerManager {
    pub fn new() -> Self {
        Self {
            containers: BTreeMap::new(),
            namespace_counter: 1,
            container_counter: 1,
            agent_containers: BTreeMap::new(),
            health_checks: BTreeMap::new(),
            total_created: 0,
            total_started: 0,
            total_stopped: 0,
            total_restarts: 0,
            total_failed: 0,
        }
    }

    // ── Container Lifecycle ──────────────────────────────────

    pub fn create(&mut self, name: &str, image: ContainerImage) -> Result<u32, ContainerError> {
        if self.containers.len() >= MAX_CONTAINERS {
            return Err(ContainerError::MaxContainersReached);
        }

        // Check for duplicate name
        if self.containers.values().any(|c| c.name == name) {
            return Err(ContainerError::AlreadyExists);
        }

        let id = self.container_counter;
        self.container_counter += 1;

        let mut container = Container::new(id, name, image);
        container.created_at = 0; // Would use timer in production

        // Create default namespaces (all 7 types)
        for ns_type in NamespaceType::all() {
            let mut ns = Namespace::new(self.namespace_counter, ns_type, id);
            ns.set_hostname(&container.hostname);
            if ns_type == NamespaceType::Network && container.network_enabled {
                let net_config = NetworkConfig::new(
                    &format!("10.{}.{}.2", (id >> 8) & 0xFF, id & 0xFF),
                    "255.255.255.0",
                    &format!("10.{}.{}.1", (id >> 8) & 0xFF, id & 0xFF),
                );
                ns.set_network(net_config);
            }
            if ns_type == NamespaceType::Mount {
                ns.set_mount_root(&container.image.rootfs_path);
            }
            self.namespace_counter += 1;
            container.add_namespace(ns);
        }

        self.containers.insert(id, container);
        self.total_created += 1;
        Ok(id)
    }

    pub fn start(&mut self, container_id: u32, init_pid: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state != ContainerState::Created && container.state != ContainerState::Stopped {
            if container.state == ContainerState::Running {
                return Err(ContainerError::AlreadyRunning);
            }
            return Err(ContainerError::NotCreated);
        }

        container.state = ContainerState::Running;
        container.init_pid = Some(init_pid);
        container.started_at = Some(0); // Would use timer
        container.add_process(init_pid);
        container.health_status = HealthStatus::Starting;

        // Set up health check (process-based by default)
        self.health_checks.insert(container_id, HealthCheck::new(HealthCheckType::Process));

        self.total_started += 1;
        Ok(())
    }

    pub fn stop(&mut self, container_id: u32, exit_code: i32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state != ContainerState::Running && container.state != ContainerState::Paused {
            return Err(ContainerError::NotRunning);
        }

        container.state = ContainerState::Stopped;
        container.finished_at = Some(0);
        container.exit_code = Some(exit_code);
        container.health_status = HealthStatus::Unknown;

        self.health_checks.remove(&container_id);
        self.total_stopped += 1;

        // Check auto-restart
        if container.auto_restart && container.restart_count < container.max_restarts {
            container.restart_count += 1;
            self.total_restarts += 1;
            // In production: trigger restart
        }

        Ok(())
    }

    pub fn pause(&mut self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state != ContainerState::Running {
            return Err(ContainerError::NotRunning);
        }

        container.state = ContainerState::Paused;
        Ok(())
    }

    pub fn resume(&mut self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state != ContainerState::Paused {
            return Err(ContainerError::Paused);
        }

        container.state = ContainerState::Running;
        Ok(())
    }

    pub fn destroy(&mut self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state.is_active() {
            return Err(ContainerError::AlreadyRunning);
        }

        container.state = ContainerState::Destroyed;

        // Clean up agent mapping
        if let Some(ref agent_id) = container.agent_id {
            self.agent_containers.remove(agent_id);
        }

        self.containers.remove(&container_id);
        self.health_checks.remove(&container_id);
        Ok(())
    }

    pub fn kill(&mut self, container_id: u32) -> Result<(), ContainerError> {
        // Force stop
        self.stop(container_id, -9)
    }

    pub fn restart(&mut self, container_id: u32, new_pid: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if container.state != ContainerState::Stopped {
            return Err(ContainerError::NotRunning);
        }

        if container.restart_count >= container.max_restarts {
            self.total_failed += 1;
            return Err(ContainerError::MaxRestartsReached);
        }

        container.restart_count += 1;
        container.state = ContainerState::Running;
        container.init_pid = Some(new_pid);
        container.add_process(new_pid);
        container.health_status = HealthStatus::Starting;
        container.finished_at = None;
        container.exit_code = None;
        container.started_at = Some(0);
        self.total_restarts += 1;
        Ok(())
    }

    // ── Process Management ──────────────────────────────────

    pub fn add_process(&mut self, container_id: u32, pid: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if !container.state.is_active() {
            return Err(ContainerError::NotRunning);
        }

        // Check PID limit
        if !container.resource_usage.check_pid_limit(&container.resource_limits) {
            return Err(ContainerError::ResourceLimit(ResourceError::PidLimitExceeded));
        }

        container.add_process(pid);

        // Add to PID namespace
        if let Some(ns) = container.get_namespace_mut(NamespaceType::Pid) {
            ns.add_process(pid);
        }

        Ok(())
    }

    pub fn remove_process(&mut self, container_id: u32, pid: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        container.remove_process(pid);

        if let Some(ns) = container.get_namespace_mut(NamespaceType::Pid) {
            ns.remove_process(pid);
        }

        // If init process exits, stop container
        if container.init_pid == Some(pid) && container.state.is_active() {
            self.stop(container_id, 0)?;
        }

        Ok(())
    }

    pub fn check_syscall(&self, container_id: u32, syscall: &str) -> Result<bool, ContainerError> {
        let container = self.containers.get(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if !container.check_syscall(syscall) {
            return Err(ContainerError::SyscallBlocked);
        }

        Ok(true)
    }

    // ── Resource Management ─────────────────────────────────

    pub fn set_limits(&mut self, container_id: u32, limits: ResourceLimits) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.resource_limits = limits;
        Ok(())
    }

    pub fn update_usage(&mut self, container_id: u32, cpu_us: u64, mem_bytes: u64,
                        io_read: u64, io_write: u64, net_rx: u64, net_tx: u64) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.record_resource_usage(cpu_us, mem_bytes, io_read, io_write, net_rx, net_tx);

        // Check limits
        if let Err(e) = container.check_limits() {
            return Err(ContainerError::ResourceLimit(e));
        }

        Ok(())
    }

    pub fn check_limits(&self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.check_limits().map_err(ContainerError::ResourceLimit)
    }

    // ── Agent Sandboxing ─────────────────────────────────────

    pub fn create_agent_container(&mut self, agent_id: &str, name: &str,
                                   image: ContainerImage, limits: ResourceLimits) -> Result<u32, ContainerError> {
        if self.agent_containers.contains_key(agent_id) {
            return Err(ContainerError::AlreadyExists);
        }

        let container_id = self.create(name, image)?;
        let container = self.containers.get_mut(&container_id).unwrap();
        container.set_agent(agent_id);
        container.resource_limits = limits;
        container.syscall_filter = SyscallFilter::agent_sandbox();
        container.auto_restart = true;

        self.agent_containers.insert(agent_id.to_string(), container_id);
        Ok(container_id)
    }

    pub fn get_agent_container(&self, agent_id: &str) -> Option<u32> {
        self.agent_containers.get(agent_id).copied()
    }

    pub fn stop_agent(&mut self, agent_id: &str) -> Result<(), ContainerError> {
        let container_id = self.get_agent_container(agent_id)
            .ok_or(ContainerError::AgentNotRegistered)?;
        self.stop(container_id, 0)
    }

    pub fn list_agent_containers(&self) -> Vec<(String, u32)> {
        self.agent_containers.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect()
    }

    // ── Health Checks ───────────────────────────────────────

    pub fn set_health_check(&mut self, container_id: u32, check: HealthCheck) -> Result<(), ContainerError> {
        if !self.containers.contains_key(&container_id) {
            return Err(ContainerError::NotFound);
        }
        self.health_checks.insert(container_id, check);
        Ok(())
    }

    pub fn run_health_check(&mut self, container_id: u32, success: bool) -> Result<HealthStatus, ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        let check = self.health_checks.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        let status = check.record_result(success);
        container.health_status = status;

        if status == HealthStatus::Unhealthy && container.state == ContainerState::Running {
            // Auto-restart on unhealthy
            if container.auto_restart && container.restart_count < container.max_restarts {
                container.restart_count += 1;
                self.total_restarts += 1;
            } else {
                self.total_failed += 1;
            }
        }

        Ok(status)
    }

    // ── Network Management ───────────────────────────────────

    pub fn add_port_mapping(&mut self, container_id: u32, host_port: u16,
                            container_port: u16, protocol: PortProtocol) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;

        if let Some(ns) = container.get_namespace_mut(NamespaceType::Network) {
            if let Some(ref mut net) = ns.network_config {
                net.add_port(host_port, container_port, protocol);
            }
        }
        Ok(())
    }

    pub fn disable_network(&mut self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.network_enabled = false;
        Ok(())
    }

    pub fn enable_network(&mut self, container_id: u32) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.network_enabled = true;
        Ok(())
    }

    // ── Capability Management ───────────────────────────────

    pub fn grant_capability(&mut self, container_id: u32, cap: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.grant_capability(cap);
        Ok(())
    }

    pub fn revoke_capability(&mut self, container_id: u32, cap: &str) -> Result<bool, ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        Ok(container.revoke_capability(cap))
    }

    pub fn check_capability(&self, container_id: u32, cap: &str) -> Result<bool, ContainerError> {
        let container = self.containers.get(&container_id)
            .ok_or(ContainerError::NotFound)?;
        Ok(container.has_capability(cap))
    }

    // ── Queries ──────────────────────────────────────────────

    pub fn get_container(&self, container_id: u32) -> Option<&Container> {
        self.containers.get(&container_id)
    }

    pub fn get_container_mut(&mut self, container_id: u32) -> Option<&mut Container> {
        self.containers.get_mut(&container_id)
    }

    pub fn list_containers(&self) -> Vec<ContainerSnapshot> {
        self.containers.values().map(|c| c.snapshot()).collect()
    }

    pub fn list_by_state(&self, state: ContainerState) -> Vec<u32> {
        self.containers.values()
            .filter(|c| c.state == state)
            .map(|c| c.container_id)
            .collect()
    }

    pub fn list_running(&self) -> Vec<u32> {
        self.list_by_state(ContainerState::Running)
    }

    pub fn list_stopped(&self) -> Vec<u32> {
        self.list_by_state(ContainerState::Stopped)
    }

    pub fn list_agent_containers_snapshots(&self) -> Vec<ContainerSnapshot> {
        self.containers.values()
            .filter(|c| c.is_agent_container())
            .map(|c| c.snapshot())
            .collect()
    }

    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    pub fn running_count(&self) -> usize {
        self.list_running().len()
    }

    pub fn find_by_name(&self, name: &str) -> Option<u32> {
        self.containers.values()
            .find(|c| c.name == name)
            .map(|c| c.container_id)
    }

    // ── Namespace Operations ─────────────────────────────────

    pub fn set_hostname(&mut self, container_id: u32, hostname: &str) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        container.hostname = hostname.to_string();
        if let Some(ns) = container.get_namespace_mut(NamespaceType::Uts) {
            ns.set_hostname(hostname);
        }
        Ok(())
    }

    pub fn set_uid_map(&mut self, container_id: u32, mappings: Vec<(u32, u32, u32)>) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        if let Some(ns) = container.get_namespace_mut(NamespaceType::User) {
            ns.set_uid_map(mappings);
            Ok(())
        } else {
            Err(ContainerError::InvalidNamespace)
        }
    }

    pub fn set_gid_map(&mut self, container_id: u32, mappings: Vec<(u32, u32, u32)>) -> Result<(), ContainerError> {
        let container = self.containers.get_mut(&container_id)
            .ok_or(ContainerError::NotFound)?;
        if let Some(ns) = container.get_namespace_mut(NamespaceType::User) {
            ns.set_gid_map(mappings);
            Ok(())
        } else {
            Err(ContainerError::InvalidNamespace)
        }
    }

    pub fn map_uid(&self, container_id: u32, container_uid: u32) -> Result<Option<u32>, ContainerError> {
        let container = self.containers.get(&container_id)
            .ok_or(ContainerError::NotFound)?;
        if let Some(ns) = container.get_namespace(NamespaceType::User) {
            Ok(ns.map_uid(container_uid))
        } else {
            Ok(None)
        }
    }

    // ── Stats ───────────────────────────────────────────────

    pub fn stats(&self) -> ContainerManagerStats {
        ContainerManagerStats {
            total_containers: self.containers.len() as u32,
            running: self.running_count() as u32,
            stopped: self.list_stopped().len() as u32,
            agent_containers: self.agent_containers.len() as u32,
            total_created: self.total_created,
            total_started: self.total_started,
            total_stopped: self.total_stopped,
            total_restarts: self.total_restarts,
            total_failed: self.total_failed,
        }
    }
}

// ════════════════════════════════════════════════════════════════
//  CONTAINER MANAGER STATS
// ════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct ContainerManagerStats {
    pub total_containers: u32,
    pub running: u32,
    pub stopped: u32,
    pub agent_containers: u32,
    pub total_created: u64,
    pub total_started: u64,
    pub total_stopped: u64,
    pub total_restarts: u64,
    pub total_failed: u64,
}

// ════════════════════════════════════════════════════════════════
//  TESTS
// ════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── Namespace Tests ──────────────────────────────────────

    #[test]
    fn test_namespace_creation() {
        let ns = Namespace::new(1, NamespaceType::Pid, 100);
        assert_eq!(ns.ns_id, 1);
        assert_eq!(ns.ns_type, NamespaceType::Pid);
        assert_eq!(ns.owner_container, 100);
        assert_eq!(ns.process_count(), 0);
    }

    #[test]
    fn test_namespace_process_management() {
        let mut ns = Namespace::new(1, NamespaceType::Pid, 100);
        ns.add_process(42);
        ns.add_process(43);
        ns.add_process(42);  // duplicate
        assert_eq!(ns.process_count(), 2);
        assert!(ns.has_process(42));
        assert!(ns.has_process(43));
        assert!(!ns.has_process(99));

        assert!(ns.remove_process(42));
        assert_eq!(ns.process_count(), 1);
        assert!(!ns.has_process(42));

        // Remove non-existent
        assert!(!ns.remove_process(999));
    }

    #[test]
    fn test_namespace_hostname() {
        let mut ns = Namespace::new(1, NamespaceType::Uts, 100);
        assert_eq!(ns.hostname, "");
        ns.set_hostname("ai-agent-01");
        assert_eq!(ns.hostname, "ai-agent-01");
    }

    #[test]
    fn test_namespace_uid_mapping() {
        let mut ns = Namespace::new(1, NamespaceType::User, 100);
        ns.set_uid_map(vec![(0, 1000, 1), (1000, 2000, 100)]);

        // Container UID 0 → Host UID 1000
        assert_eq!(ns.map_uid(0), Some(1000));
        // Container UID 1000 → Host UID 2000
        assert_eq!(ns.map_uid(1000), Some(2000));
        // Container UID 1050 → Host UID 2050
        assert_eq!(ns.map_uid(1050), Some(2050));
        // Container UID 1100 → unmapped
        assert_eq!(ns.map_uid(1100), None);
    }

    #[test]
    fn test_namespace_gid_mapping() {
        let mut ns = Namespace::new(1, NamespaceType::User, 100);
        ns.set_gid_map(vec![(0, 1000, 1)]);
        assert_eq!(ns.map_gid(0), Some(1000));
        assert_eq!(ns.map_gid(1), None);
    }

    #[test]
    fn test_namespace_reverse_uid_mapping() {
        let mut ns = Namespace::new(1, NamespaceType::User, 100);
        ns.set_uid_map(vec![(0, 1000, 1), (1000, 2000, 100)]);
        assert_eq!(ns.reverse_map_uid(1000), Some(0));
        assert_eq!(ns.reverse_map_uid(2050), Some(1050));
        assert_eq!(ns.reverse_map_uid(999), None);
    }

    #[test]
    fn test_namespace_network_config() {
        let mut ns = Namespace::new(1, NamespaceType::Network, 100);
        let net = NetworkConfig::new("10.0.0.2", "255.255.255.0", "10.0.0.1");
        assert_eq!(net.ip_address, "10.0.0.2");
        assert_eq!(net.gateway, "10.0.0.1");
        ns.set_network(net);
        assert!(ns.network_config.is_some());
        assert_eq!(ns.network_config.as_ref().unwrap().ip_address, "10.0.0.2");
    }

    #[test]
    fn test_namespace_mount_root() {
        let mut ns = Namespace::new(1, NamespaceType::Mount, 100);
        ns.set_mount_root("/var/containers/100/rootfs");
        assert_eq!(ns.mount_root, Some("/var/containers/100/rootfs".to_string()));
    }

    #[test]
    fn test_namespace_type_names() {
        assert_eq!(NamespaceType::Pid.name(), "pid");
        assert_eq!(NamespaceType::Mount.name(), "mount");
        assert_eq!(NamespaceType::Network.name(), "net");
        assert_eq!(NamespaceType::Ipc.name(), "ipc");
        assert_eq!(NamespaceType::Uts.name(), "uts");
        assert_eq!(NamespaceType::User.name(), "user");
        assert_eq!(NamespaceType::Cgroup.name(), "cgroup");
    }

    #[test]
    fn test_namespace_all_types() {
        let all = NamespaceType::all();
        assert_eq!(all.len(), 7);
    }

    // ── Network Config Tests ────────────────────────────────

    #[test]
    fn test_network_config_port_mapping() {
        let mut net = NetworkConfig::new("10.0.0.2", "255.255.255.0", "10.0.0.1");
        net.add_port(8080, 80, PortProtocol::Tcp);
        assert_eq!(net.ports.len(), 1);
        assert_eq!(net.ports[0].host_port, 8080);
        assert_eq!(net.ports[0].container_port, 80);
        assert_eq!(net.ports[0].protocol, PortProtocol::Tcp);

        net.add_port(8443, 443, PortProtocol::Tcp);
        assert_eq!(net.ports.len(), 2);

        assert!(net.remove_port(8080));
        assert_eq!(net.ports.len(), 1);
        assert!(!net.remove_port(9999));
    }

    #[test]
    fn test_port_protocol_names() {
        assert_eq!(PortProtocol::Tcp.name(), "tcp");
        assert_eq!(PortProtocol::Udp.name(), "udp");
    }

    // ── Resource Limits Tests ───────────────────────────────

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.cpu_quota_us, DEFAULT_CPU_QUOTA_US);
        assert_eq!(limits.memory_limit_bytes, DEFAULT_MEM_LIMIT_BYTES);
        assert_eq!(limits.pid_max, DEFAULT_PID_LIMIT);
        assert_eq!(limits.fd_max, DEFAULT_FD_LIMIT);
    }

    #[test]
    fn test_resource_limits_unlimited() {
        let limits = ResourceLimits::unlimited();
        assert_eq!(limits.cpu_quota_us, 0);
        assert_eq!(limits.memory_limit_bytes, 0);
        assert_eq!(limits.pid_max, 0);
    }

    #[test]
    fn test_resource_limits_minimal() {
        let limits = ResourceLimits::minimal();
        assert_eq!(limits.memory_limit_bytes, 32 * 1024 * 1024);
        assert_eq!(limits.pid_max, 16);
        assert_eq!(limits.fd_max, 64);
    }

    #[test]
    fn test_resource_limits_high_perf() {
        let limits = ResourceLimits::high_perf();
        assert_eq!(limits.memory_limit_bytes, 4 * 1024 * 1024 * 1024);
        assert_eq!(limits.cpu_shares, 4096);
        assert_eq!(limits.pid_max, 1024);
    }

    // ── Resource Usage Tests ────────────────────────────────

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::new();
        assert_eq!(usage.cpu_used_us, 0);
        assert_eq!(usage.memory_used_bytes, 0);
        assert_eq!(usage.pid_count, 0);
    }

    #[test]
    fn test_resource_usage_check_cpu() {
        let limits = ResourceLimits::default();
        let mut usage = ResourceUsage::new();
        usage.cpu_used_us = 50_000;
        assert!(usage.check_cpu_limit(&limits));

        usage.cpu_used_us = 150_000;
        assert!(!usage.check_cpu_limit(&limits));

        // Unlimited
        let unlimited = ResourceLimits::unlimited();
        assert!(usage.check_cpu_limit(&unlimited));
    }

    #[test]
    fn test_resource_usage_check_memory() {
        let limits = ResourceLimits::default();
        let mut usage = ResourceUsage::new();
        usage.memory_used_bytes = 256 * 1024 * 1024;
        assert!(usage.check_memory_limit(&limits));

        usage.memory_used_bytes = 1024 * 1024 * 1024;
        assert!(!usage.check_memory_limit(&limits));

        // Unlimited
        let unlimited = ResourceLimits::unlimited();
        assert!(usage.check_memory_limit(&unlimited));
    }

    #[test]
    fn test_resource_usage_check_pid_limit() {
        let limits = ResourceLimits::default();
        let mut usage = ResourceUsage::new();
        usage.pid_count = 100;
        assert!(usage.check_pid_limit(&limits));

        usage.pid_count = 300;
        assert!(!usage.check_pid_limit(&limits));
    }

    #[test]
    fn test_resource_usage_check_fd_limit() {
        let limits = ResourceLimits::default();
        let mut usage = ResourceUsage::new();
        usage.fd_count = 500;
        assert!(usage.check_fd_limit(&limits));

        usage.fd_count = 2048;
        assert!(!usage.check_fd_limit(&limits));
    }

    #[test]
    fn test_resource_usage_check_all_ok() {
        let limits = ResourceLimits::default();
        let usage = ResourceUsage::new();
        assert!(usage.check_all(&limits).is_ok());
    }

    #[test]
    fn test_resource_usage_check_all_memory_exceeded() {
        let limits = ResourceLimits::default();
        let mut usage = ResourceUsage::new();
        usage.memory_used_bytes = 1024 * 1024 * 1024;
        assert_eq!(usage.check_all(&limits), Err(ResourceError::MemoryExceeded));
    }

    #[test]
    fn test_resource_usage_reset() {
        let mut usage = ResourceUsage::new();
        usage.cpu_used_us = 1000;
        usage.memory_used_bytes = 5000;
        usage.reset();
        assert_eq!(usage.cpu_used_us, 0);
        assert_eq!(usage.memory_used_bytes, 0);
    }

    // ── Container Image Tests ───────────────────────────────

    #[test]
    fn test_container_image_creation() {
        let image = ContainerImage::new("ai-agent", "/var/images/agent", "/bin/agent");
        assert_eq!(image.name, "ai-agent");
        assert_eq!(image.rootfs_path, "/var/images/agent");
        assert_eq!(image.entry_point, "/bin/agent");
        assert_eq!(image.version, "latest");
    }

    #[test]
    fn test_container_image_env() {
        let mut image = ContainerImage::new("test", "/root", "/bin/test");
        image.add_env("PATH", "/usr/bin:/bin");
        image.add_env("HOME", "/root");
        assert_eq!(image.env_vars.len(), 2);
        assert_eq!(image.get_env("PATH"), Some("/usr/bin:/bin"));
        assert_eq!(image.get_env("HOME"), Some("/root"));
        assert_eq!(image.get_env("NONEXIST"), None);
    }

    #[test]
    fn test_container_image_args() {
        let mut image = ContainerImage::new("test", "/root", "/bin/test");
        image.add_arg("--verbose");
        image.add_arg("--port");
        image.add_arg("8080");
        assert_eq!(image.args.len(), 3);
    }

    #[test]
    fn test_container_image_volume() {
        let mut image = ContainerImage::new("test", "/root", "/bin/test");
        image.add_volume(VolumeMount::new("/host/data", "/data", false));
        image.add_volume(VolumeMount::new("/host/config", "/config", true));
        assert_eq!(image.volumes.len(), 2);
        assert!(!image.volumes[0].read_only);
        assert!(image.volumes[1].read_only);
    }

    #[test]
    fn test_container_image_labels() {
        let mut image = ContainerImage::new("test", "/root", "/bin/test");
        image.add_label("app", "ai-agent");
        image.add_label("version", "1.0");
        assert_eq!(image.get_label("app"), Some("ai-agent"));
        assert_eq!(image.get_label("version"), Some("1.0"));
        assert_eq!(image.get_label("missing"), None);
    }

    #[test]
    fn test_volume_mount_tmpfs() {
        let mount = VolumeMount::tmpfs("/tmp", 64 * 1024 * 1024);
        assert_eq!(mount.mount_type, MountType::Tmpfs);
        assert!(!mount.read_only);
        assert_eq!(mount.container_path, "/tmp");
    }

    // ── Container State Tests ───────────────────────────────

    #[test]
    fn test_container_state_is_active() {
        assert!(ContainerState::Running.is_active());
        assert!(ContainerState::Paused.is_active());
        assert!(!ContainerState::Created.is_active());
        assert!(!ContainerState::Stopped.is_active());
        assert!(!ContainerState::Error.is_active());
        assert!(!ContainerState::Destroyed.is_active());
    }

    #[test]
    fn test_container_state_names() {
        assert_eq!(ContainerState::Created.name(), "created");
        assert_eq!(ContainerState::Running.name(), "running");
        assert_eq!(ContainerState::Paused.name(), "paused");
        assert_eq!(ContainerState::Stopped.name(), "stopped");
        assert_eq!(ContainerState::Error.name(), "error");
        assert_eq!(ContainerState::Destroyed.name(), "destroyed");
    }

    // ── Container Tests ─────────────────────────────────────

    #[test]
    fn test_container_creation() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let container = Container::new(1, "test-container", image);
        assert_eq!(container.container_id, 1);
        assert_eq!(container.name, "test-container");
        assert_eq!(container.state, ContainerState::Created);
        assert_eq!(container.process_count(), 0);
        assert!(!container.is_agent_container());
    }

    #[test]
    fn test_container_add_remove_process() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let mut container = Container::new(1, "test", image);
        container.add_process(100);
        container.add_process(101);
        container.add_process(100);  // duplicate
        assert_eq!(container.process_count(), 2);
        assert!(container.has_process(100));
        assert!(container.has_process(101));

        assert!(container.remove_process(100));
        assert_eq!(container.process_count(), 1);
        assert!(!container.has_process(100));
    }

    #[test]
    fn test_container_agent() {
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let mut container = Container::new(1, "agent-1", image);
        assert!(!container.is_agent_container());

        container.set_agent("aurora-001");
        assert!(container.is_agent_container());
        assert_eq!(container.agent_id, Some("aurora-001".to_string()));
    }

    #[test]
    fn test_container_capabilities() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let mut container = Container::new(1, "test", image);

        container.grant_capability("net.access");
        container.grant_capability("fs.read");
        container.grant_capability("net.access");  // duplicate
        assert_eq!(container.capabilities_granted.len(), 2);
        assert!(container.has_capability("net.access"));
        assert!(!container.has_capability("fs.write"));

        assert!(container.revoke_capability("net.access"));
        assert!(!container.has_capability("net.access"));
        assert!(!container.revoke_capability("nonexistent"));
    }

    #[test]
    fn test_container_resource_recording() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let mut container = Container::new(1, "test", image);
        container.record_resource_usage(1000, 50_000_000, 1024, 512, 2048, 1024);
        assert_eq!(container.resource_usage.cpu_used_us, 1000);
        assert_eq!(container.resource_usage.memory_used_bytes, 50_000_000);
        assert_eq!(container.resource_usage.memory_peak_bytes, 50_000_000);
        assert_eq!(container.resource_usage.io_read_bytes, 1024);
        assert_eq!(container.resource_usage.io_write_bytes, 512);
        assert_eq!(container.resource_usage.net_rx_bytes, 2048);
        assert_eq!(container.resource_usage.net_tx_bytes, 1024);

        // Peak tracking
        container.record_resource_usage(500, 30_000_000, 0, 0, 0, 0);
        assert_eq!(container.resource_usage.memory_peak_bytes, 50_000_000);
        assert_eq!(container.resource_usage.cpu_used_us, 1500);
    }

    #[test]
    fn test_container_check_limits_ok() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let container = Container::new(1, "test", image);
        assert!(container.check_limits().is_ok());
    }

    #[test]
    fn test_container_check_limits_exceeded() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let mut container = Container::new(1, "test", image);
        container.resource_usage.memory_used_bytes = 1024 * 1024 * 1024;
        assert_eq!(container.check_limits(), Err(ResourceError::MemoryExceeded));
    }

    #[test]
    fn test_container_snapshot() {
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let mut container = Container::new(42, "snap-test", image);
        container.set_agent("agent-42");
        container.add_process(100);
        container.record_resource_usage(5000, 100_000_000, 4096, 2048, 8192, 4096);
        let snap = container.snapshot();
        assert_eq!(snap.container_id, 42);
        assert_eq!(snap.name, "snap-test");
        assert_eq!(snap.pid_count, 1);
        assert_eq!(snap.cpu_used_us, 5000);
        assert_eq!(snap.memory_used_bytes, 100_000_000);
        assert_eq!(snap.agent_id, Some("agent-42".to_string()));
    }

    // ── Syscall Filter Tests ─────────────────────────────────

    #[test]
    fn test_syscall_filter_allow_all() {
        let filter = SyscallFilter::allow_all();
        assert!(filter.check("read"));
        assert!(filter.check("write"));
        assert!(filter.check("reboot"));
    }

    #[test]
    fn test_syscall_filter_block_list() {
        let filter = SyscallFilter::block_list(vec!["reboot".to_string(), "shutdown".to_string()]);
        assert!(filter.check("read"));
        assert!(filter.check("write"));
        assert!(!filter.check("reboot"));
        assert!(!filter.check("shutdown"));
    }

    #[test]
    fn test_syscall_filter_allow_list() {
        let filter = SyscallFilter::allow_list(vec!["read".to_string(), "write".to_string()]);
        assert!(filter.check("read"));
        assert!(filter.check("write"));
        assert!(!filter.check("execve"));
        assert!(!filter.check("fork"));
    }

    #[test]
    fn test_syscall_filter_agent_sandbox() {
        let filter = SyscallFilter::agent_sandbox();
        assert!(filter.check("read"));
        assert!(filter.check("write"));
        assert!(!filter.check("reboot"));
        assert!(!filter.check("mount"));
        assert!(!filter.check("init_module"));
        assert!(!filter.check("kexec_load"));
    }

    #[test]
    fn test_syscall_filter_add_blocked() {
        let mut filter = SyscallFilter::allow_all();
        filter.add_blocked("ptrace");
        assert!(!filter.check("ptrace"));
        assert!(filter.check("read"));
    }

    #[test]
    fn test_syscall_filter_add_allowed() {
        let mut filter = SyscallFilter::allow_list(vec!["read".to_string()]);
        filter.add_allowed("write");
        assert!(filter.check("read"));
        assert!(filter.check("write"));
        assert!(!filter.check("execve"));
    }

    // ── Health Check Tests ──────────────────────────────────

    #[test]
    fn test_health_check_healthy() {
        let mut check = HealthCheck::new(HealthCheckType::Process);
        let status = check.record_result(true);
        assert_eq!(status, HealthStatus::Healthy);
        assert_eq!(check.consecutive_failures, 0);
    }

    #[test]
    fn test_health_check_degraded() {
        let mut check = HealthCheck::new(HealthCheckType::Process);
        check.record_result(false);
        assert_eq!(check.consecutive_failures, 1);
        let status = check.record_result(true);
        assert_eq!(status, HealthStatus::Healthy);
        assert_eq!(check.consecutive_failures, 0);
    }

    #[test]
    fn test_health_check_unhealthy() {
        let mut check = HealthCheck::new(HealthCheckType::Process);
        check.retries = 3;
        check.record_result(false);
        check.record_result(false);
        let status = check.record_result(false);
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_check_recovery() {
        let mut check = HealthCheck::new(HealthCheckType::Process);
        check.retries = 3;
        check.record_result(false);
        check.record_result(false);
        // Recovery before hitting unhealthy
        let status = check.record_result(true);
        assert_eq!(status, HealthStatus::Healthy);
        assert_eq!(check.consecutive_failures, 0);
    }

    #[test]
    fn test_health_check_should_check_initial() {
        let check = HealthCheck::new(HealthCheckType::Process);
        check.start_delay_ms = 5000;
        assert!(!check.should_check(3000));
        assert!(check.should_check(5000));
        assert!(check.should_check(6000));
    }

    #[test]
    fn test_health_check_should_check_interval() {
        let mut check = HealthCheck::new(HealthCheckType::Process);
        check.start_delay_ms = 0;
        check.interval_ms = 10000;
        check.last_check_at = Some(15000);
        assert!(!check.should_check(20000));
        assert!(check.should_check(25000));
        assert!(check.should_check(30000));
    }

    #[test]
    fn test_health_status_names() {
        assert_eq!(HealthStatus::Unknown.name(), "unknown");
        assert_eq!(HealthStatus::Starting.name(), "starting");
        assert_eq!(HealthStatus::Healthy.name(), "healthy");
        assert_eq!(HealthStatus::Unhealthy.name(), "unhealthy");
        assert_eq!(HealthStatus::Degraded.name(), "degraded");
    }

    // ── Container Manager: Lifecycle Tests ─────────────────

    #[test]
    fn test_manager_create() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("container-1", image).unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.container_count(), 1);
        assert!(mgr.get_container(id).is_some());
    }

    #[test]
    fn test_manager_create_duplicate_name() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        mgr.create("dup", image.clone()).unwrap();
        let result = mgr.create("dup", image);
        assert_eq!(result, Err(ContainerError::AlreadyExists));
    }

    #[test]
    fn test_manager_start() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Running);
        assert_eq!(container.init_pid, Some(100));
        assert!(container.has_process(100));
    }

    #[test]
    fn test_manager_start_already_running() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        let result = mgr.start(id, 101);
        assert_eq!(result, Err(ContainerError::AlreadyRunning));
    }

    #[test]
    fn test_manager_stop() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.stop(id, 0).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Stopped);
        assert_eq!(container.exit_code, Some(0));
    }

    #[test]
    fn test_manager_stop_not_running() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let result = mgr.stop(id, 0);
        assert_eq!(result, Err(ContainerError::NotRunning));
    }

    #[test]
    fn test_manager_pause_resume() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.pause(id).unwrap();
        assert_eq!(mgr.get_container(id).unwrap().state, ContainerState::Paused);
        mgr.resume(id).unwrap();
        assert_eq!(mgr.get_container(id).unwrap().state, ContainerState::Running);
    }

    #[test]
    fn test_manager_pause_not_running() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let result = mgr.pause(id);
        assert_eq!(result, Err(ContainerError::NotRunning));
    }

    #[test]
    fn test_manager_destroy() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.stop(id, 0).unwrap();
        mgr.destroy(id).unwrap();
        assert_eq!(mgr.container_count(), 0);
    }

    #[test]
    fn test_manager_destroy_running_fails() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        let result = mgr.destroy(id);
        assert_eq!(result, Err(ContainerError::AlreadyRunning));
    }

    #[test]
    fn test_manager_kill() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.kill(id).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Stopped);
        assert_eq!(container.exit_code, Some(-9));
    }

    #[test]
    fn test_manager_not_found() {
        let mut mgr = ContainerManager::new();
        let result = mgr.start(999, 100);
        assert_eq!(result, Err(ContainerError::NotFound));
    }

    // ── Container Manager: Process Tests ────────────────────

    #[test]
    fn test_manager_add_process() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.add_process(id, 101).unwrap();
        mgr.add_process(id, 102).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.process_count(), 3);
    }

    #[test]
    fn test_manager_add_process_not_running() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let result = mgr.add_process(id, 101);
        assert_eq!(result, Err(ContainerError::NotRunning));
    }

    #[test]
    fn test_manager_add_process_pid_limit() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let mut limits = ResourceLimits::minimal();
        limits.pid_max = 2;
        mgr.set_limits(id, limits).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.add_process(id, 101).unwrap();
        let result = mgr.add_process(id, 102);
        assert_eq!(result, Err(ContainerError::ResourceLimit(ResourceError::PidLimitExceeded)));
    }

    #[test]
    fn test_manager_remove_process() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.add_process(id, 101).unwrap();
        mgr.remove_process(id, 101).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.process_count(), 1);
        assert!(!container.has_process(101));
    }

    #[test]
    fn test_manager_remove_init_stops_container() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.remove_process(id, 100).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Stopped);
    }

    // ── Container Manager: Resource Tests ───────────────────

    #[test]
    fn test_manager_set_limits() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let limits = ResourceLimits::minimal();
        mgr.set_limits(id, limits).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.resource_limits.memory_limit_bytes, 32 * 1024 * 1024);
    }

    #[test]
    fn test_manager_update_usage() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.update_usage(id, 5000, 100_000_000, 4096, 2048, 8192, 4096).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.resource_usage.cpu_used_us, 5000);
        assert_eq!(container.resource_usage.memory_used_bytes, 100_000_000);
    }

    #[test]
    fn test_manager_update_usage_exceeds_memory() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.set_limits(id, ResourceLimits::minimal()).unwrap();
        mgr.start(id, 100).unwrap();
        let result = mgr.update_usage(id, 0, 64 * 1024 * 1024, 0, 0, 0, 0);
        assert_eq!(result, Err(ContainerError::ResourceLimit(ResourceError::MemoryExceeded)));
    }

    #[test]
    fn test_manager_check_limits() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        assert!(mgr.check_limits(id).is_ok());
    }

    // ── Container Manager: Agent Tests ──────────────────────

    #[test]
    fn test_manager_create_agent_container() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let id = mgr.create_agent_container("aurora-001", "agent-1", image, ResourceLimits::default()).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert!(container.is_agent_container());
        assert_eq!(container.agent_id, Some("aurora-001".to_string()));
        assert!(container.auto_restart);
        // Agent sandbox should block dangerous syscalls
        assert!(!container.check_syscall("reboot"));
        assert!(!container.check_syscall("mount"));
    }

    #[test]
    fn test_manager_create_agent_duplicate() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        mgr.create_agent_container("aurora-001", "agent-1", image.clone(), ResourceLimits::default()).unwrap();
        let result = mgr.create_agent_container("aurora-001", "agent-2", image, ResourceLimits::default());
        assert_eq!(result, Err(ContainerError::AlreadyExists));
    }

    #[test]
    fn test_manager_get_agent_container() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let id = mgr.create_agent_container("aurora-001", "agent-1", image, ResourceLimits::default()).unwrap();
        assert_eq!(mgr.get_agent_container("aurora-001"), Some(id));
        assert_eq!(mgr.get_agent_container("nonexistent"), None);
    }

    #[test]
    fn test_manager_stop_agent() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let id = mgr.create_agent_container("aurora-001", "agent-1", image, ResourceLimits::default()).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.stop_agent("aurora-001").unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Stopped);
    }

    #[test]
    fn test_manager_list_agent_containers() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        mgr.create_agent_container("aurora-001", "agent-1", image.clone(), ResourceLimits::default()).unwrap();
        mgr.create_agent_container("aurora-002", "agent-2", image, ResourceLimits::default()).unwrap();
        let agents = mgr.list_agent_containers();
        assert_eq!(agents.len(), 2);
    }

    #[test]
    fn test_manager_list_agent_snapshots() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let id = mgr.create_agent_container("aurora-001", "agent-1", image, ResourceLimits::default()).unwrap();
        mgr.start(id, 100).unwrap();
        let snaps = mgr.list_agent_containers_snapshots();
        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].agent_id, Some("aurora-001".to_string()));
    }

    // ── Container Manager: Capability Tests ─────────────────

    #[test]
    fn test_manager_grant_capability() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.grant_capability(id, "net.access").unwrap();
        assert!(mgr.check_capability(id, "net.access").unwrap());
        assert!(!mgr.check_capability(id, "fs.write").unwrap());
    }

    #[test]
    fn test_manager_revoke_capability() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.grant_capability(id, "net.access").unwrap();
        assert!(mgr.revoke_capability(id, "net.access").unwrap());
        assert!(!mgr.check_capability(id, "net.access").unwrap());
    }

    // ── Container Manager: Syscall Filter Tests ──────────────

    #[test]
    fn test_manager_check_syscall_allowed() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let result = mgr.check_syscall(id, "read").unwrap();
        assert!(result);
    }

    #[test]
    fn test_manager_check_syscall_blocked() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("ai-agent", "/root", "/bin/agent");
        let id = mgr.create_agent_container("aurora-001", "agent", image, ResourceLimits::default()).unwrap();
        let result = mgr.check_syscall(id, "reboot");
        assert_eq!(result, Err(ContainerError::SyscallBlocked));
    }

    // ── Container Manager: Health Check Tests ───────────────

    #[test]
    fn test_manager_health_check_healthy() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        let status = mgr.run_health_check(id, true).unwrap();
        assert_eq!(status, HealthStatus::Healthy);
    }

    #[test]
    fn test_manager_health_check_unhealthy() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.run_health_check(id, false).unwrap();
        mgr.run_health_check(id, false).unwrap();
        let status = mgr.run_health_check(id, false).unwrap();
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_manager_set_custom_health_check() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let check = HealthCheck::new(HealthCheckType::TcpPort(8080));
        mgr.set_health_check(id, check).unwrap();
        mgr.start(id, 100).unwrap();
        let status = mgr.run_health_check(id, true).unwrap();
        assert_eq!(status, HealthStatus::Healthy);
    }

    // ── Container Manager: Network Tests ────────────────────

    #[test]
    fn test_manager_port_mapping() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.add_port_mapping(id, 8080, 80, PortProtocol::Tcp).unwrap();
        let container = mgr.get_container(id).unwrap();
        let ns = container.get_namespace(NamespaceType::Network).unwrap();
        let net = ns.network_config.as_ref().unwrap();
        assert_eq!(net.ports.len(), 1);
        assert_eq!(net.ports[0].host_port, 8080);
    }

    #[test]
    fn test_manager_disable_enable_network() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.disable_network(id).unwrap();
        assert!(!mgr.get_container(id).unwrap().network_enabled);
        mgr.enable_network(id).unwrap();
        assert!(mgr.get_container(id).unwrap().network_enabled);
    }

    // ── Container Manager: Namespace Tests ──────────────────

    #[test]
    fn test_manager_set_hostname() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.set_hostname(id, "ai-node-01").unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.hostname, "ai-node-01");
        let ns = container.get_namespace(NamespaceType::Uts).unwrap();
        assert_eq!(ns.hostname, "ai-node-01");
    }

    #[test]
    fn test_manager_uid_mapping() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.set_uid_map(id, vec![(0, 1000, 1)]).unwrap();
        let result = mgr.map_uid(id, 0).unwrap();
        assert_eq!(result, Some(1000));
    }

    #[test]
    fn test_container_has_all_namespaces() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let container = mgr.get_container(id).unwrap();
        // Should have all 7 namespace types
        for ns_type in NamespaceType::all() {
            assert!(container.get_namespace(ns_type).is_some(),
                "Missing namespace: {:?}", ns_type);
        }
    }

    // ── Container Manager: Query Tests ──────────────────────

    #[test]
    fn test_manager_list_containers() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        mgr.create("c1", image.clone()).unwrap();
        mgr.create("c2", image.clone()).unwrap();
        mgr.create("c3", image).unwrap();
        let list = mgr.list_containers();
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_manager_list_by_state() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id1 = mgr.create("c1", image.clone()).unwrap();
        let id2 = mgr.create("c2", image.clone()).unwrap();
        let id3 = mgr.create("c3", image).unwrap();
        mgr.start(id1, 100).unwrap();
        mgr.start(id2, 200).unwrap();
        // c3 is still Created
        let running = mgr.list_running();
        assert_eq!(running.len(), 2);
        assert!(running.contains(&id1));
        assert!(running.contains(&id2));
    }

    #[test]
    fn test_manager_find_by_name() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("findme", image).unwrap();
        assert_eq!(mgr.find_by_name("findme"), Some(id));
        assert_eq!(mgr.find_by_name("nonexistent"), None);
    }

    #[test]
    fn test_manager_stats() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id1 = mgr.create("c1", image.clone()).unwrap();
        let _id2 = mgr.create("c2", image).unwrap();
        mgr.start(id1, 100).unwrap();
        let stats = mgr.stats();
        assert_eq!(stats.total_containers, 2);
        assert_eq!(stats.running, 1);
        assert_eq!(stats.stopped, 0);
        assert_eq!(stats.total_created, 2);
        assert_eq!(stats.total_started, 1);
    }

    // ── Container Manager: Restart Tests ────────────────────

    #[test]
    fn test_manager_restart() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        mgr.start(id, 100).unwrap();
        mgr.stop(id, 0).unwrap();
        mgr.restart(id, 200).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.state, ContainerState::Running);
        assert_eq!(container.restart_count, 1);
        assert_eq!(container.init_pid, Some(200));
    }

    #[test]
    fn test_manager_max_restarts() {
        let mut mgr = ContainerManager::new();
        let image = ContainerImage::new("test", "/root", "/bin/test");
        let id = mgr.create("test", image).unwrap();
        let mut container = mgr.get_container_mut(id).unwrap();
        container.max_restarts = 2;
        drop(container);
        mgr.start(id, 100).unwrap();
        mgr.stop(id, 0).unwrap();
        mgr.restart(id, 200).unwrap();
        mgr.stop(id, 0).unwrap();
        mgr.restart(id, 300).unwrap();
        mgr.stop(id, 0).unwrap();
        let result = mgr.restart(id, 400);
        assert_eq!(result, Err(ContainerError::MaxRestartsReached));
    }

    // ── Error Name Tests ────────────────────────────────────

    #[test]
    fn test_resource_error_names() {
        assert_eq!(ResourceError::MemoryExceeded.name(), "memory_limit_exceeded");
        assert_eq!(ResourceError::CpuQuotaExceeded.name(), "cpu_quota_exceeded");
        assert_eq!(ResourceError::PidLimitExceeded.name(), "pid_limit_exceeded");
        assert_eq!(ResourceError::FdLimitExceeded.name(), "fd_limit_exceeded");
        assert_eq!(ResourceError::IoLimitExceeded.name(), "io_limit_exceeded");
        assert_eq!(ResourceError::NetLimitExceeded.name(), "net_limit_exceeded");
    }

    #[test]
    fn test_container_error_names() {
        assert_eq!(ContainerError::NotFound.name(), "not_found");
        assert_eq!(ContainerError::AlreadyExists.name(), "already_exists");
        assert_eq!(ContainerError::NotRunning.name(), "not_running");
        assert_eq!(ContainerError::AlreadyRunning.name(), "already_running");
        assert_eq!(ContainerError::MaxContainersReached.name(), "max_containers_reached");
        assert_eq!(ContainerError::SyscallBlocked.name(), "syscall_blocked");
        assert_eq!(ContainerError::AgentNotRegistered.name(), "agent_not_registered");
    }

    // ── Full Integration Test ────────────────────────────────

    #[test]
    fn test_integration_full_agent_lifecycle() {
        let mut mgr = ContainerManager::new();

        // 1. Create agent container
        let image = ContainerImage::new("aurora-agent", "/var/agents/aurora", "/bin/aurora");
        let id = mgr.create_agent_container("aurora-main", "aurora-prod", image,
            ResourceLimits::high_perf()).unwrap();

        // 2. Verify sandbox
        let container = mgr.get_container(id).unwrap();
        assert!(container.is_agent_container());
        assert!(container.auto_restart);
        assert!(!container.check_syscall("reboot"));

        // 3. Start agent
        mgr.start(id, 1000).unwrap();
        assert_eq!(mgr.running_count(), 1);

        // 4. Add child processes
        mgr.add_process(id, 1001).unwrap();
        mgr.add_process(id, 1002).unwrap();
        let container = mgr.get_container(id).unwrap();
        assert_eq!(container.process_count(), 3);

        // 5. Health check
        let status = mgr.run_health_check(id, true).unwrap();
        assert_eq!(status, HealthStatus::Healthy);

        // 6. Resource usage
        mgr.update_usage(id, 50000, 1_000_000_000, 10_000_000, 5_000_000, 50_000_000, 30_000_000).unwrap();

        // 7. Grant capability
        mgr.grant_capability(id, "net.access").unwrap();
        assert!(mgr.check_capability(id, "net.access").unwrap());

        // 8. Port mapping
        mgr.add_port_mapping(id, 8080, 80, PortProtocol::Tcp).unwrap();

        // 9. Snapshot
        let snap = container.snapshot();
        assert_eq!(snap.agent_id, Some("aurora-main".to_string()));
        assert_eq!(snap.health, HealthStatus::Healthy);

        // 10. Stop agent
        mgr.stop_agent("aurora-main").unwrap();
        assert_eq!(mgr.get_container(id).unwrap().state, ContainerState::Stopped);

        // 11. Destroy
        mgr.destroy(id).unwrap();
        assert_eq!(mgr.container_count(), 0);
        assert_eq!(mgr.get_agent_container("aurora-main"), None);

        // 12. Check stats
        let stats = mgr.stats();
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.total_started, 1);
        assert_eq!(stats.total_stopped, 1);
    }
}
