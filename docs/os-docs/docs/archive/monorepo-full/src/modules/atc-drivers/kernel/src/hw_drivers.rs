// ShivaCore — K-Sprint 35: Hardware Driver Framework
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// PCI-Bus-Enumeration, HPET High-Precision Timer, virtio-blk Block Device,
// virtio-net Network Device. Trait-basiert mit simulierten Backends für cargo test.

use crate::ats1000::Pid;
use core::sync::atomic::{AtomicU64, Ordering};

// ═══════════════════════════════════════════════════════════════════════════════
// PCI Configuration Space
// ═══════════════════════════════════════════════════════════════════════════════

/// PCI Device identification
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PciDeviceId {
    pub vendor_id:   u16,
    pub device_id:   u16,
    pub class_code:  u8,
    pub subclass:    u8,
    pub revision:    u8,
}

impl PciDeviceId {
    pub fn new(vendor: u16, device: u16, class: u8, subclass: u8) -> Self {
        Self { vendor_id: vendor, device_id: device, class_code: class, subclass, revision: 0 }
    }
}

/// PCI Base Address (BAR)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PciBar {
    pub index:    u8,
    pub address:  u64,
    pub size:     u64,
    pub is_mmio:  bool,
    pub is_io:    bool,
    pub is_64bit: bool,
}

/// A discovered PCI device
#[derive(Clone, Debug)]
pub struct PciDevice {
    pub bus:       u8,
    pub slot:      u8,
    pub function:  u8,
    pub id:        PciDeviceId,
    pub bars:      Vec<PciBar>,
    pub irq_pin:   u8,
    pub irq_line:  u8,
}

impl PciDevice {
    pub fn new(bus: u8, slot: u8, func: u8, id: PciDeviceId) -> Self {
        Self { bus, slot, function: func, id, bars: Vec::new(), irq_pin: 0, irq_line: 0 }
    }

    pub fn bdf(&self) -> u32 {
        ((self.bus as u32) << 16) | ((self.slot as u32) << 11) | ((self.function as u32) << 8)
    }

    pub fn add_bar(&mut self, bar: PciBar) {
        self.bars.push(bar);
    }

    pub fn find_mmio_bar(&self) -> Option<&PciBar> {
        self.bars.iter().find(|b| b.is_mmio)
    }

    pub fn is_virtio(&self) -> bool {
        self.id.vendor_id == 0x1AF4  // Red Hat / Qumranet (virtio)
    }
}

/// PCI class codes
pub const PCI_CLASS_STORAGE:  u8 = 0x01;
pub const PCI_CLASS_NETWORK:  u8 = 0x02;
pub const PCI_CLASS_DISPLAY:  u8 = 0x03;
pub const PCI_CLASS_BRIDGE:   u8 = 0x06;
pub const PCI_CLASS_SERIAL:    u8 = 0x0C;  // Serial bus (USB, etc.)

/// PCI Bus Scanner (simulated for testing)
pub struct PciBus {
    devices: Vec<PciDevice>,
}

impl Default for PciBus {
    fn default() -> Self { Self::new() }
}

impl PciBus {
    pub fn new() -> Self {
        Self { devices: Vec::new() }
    }

    /// Register a device on the bus (simulated hot-plug)
    pub fn register(&mut self, device: PciDevice) {
        self.devices.push(device);
    }

    /// Scan the bus for all devices
    pub fn scan(&self) -> &[PciDevice] {
        &self.devices
    }

    /// Find devices by class code
    pub fn find_by_class(&self, class: u8) -> Vec<&PciDevice> {
        self.devices.iter().filter(|d| d.id.class_code == class).collect()
    }

    /// Find all virtio devices
    pub fn find_virtio(&self) -> Vec<&PciDevice> {
        self.devices.iter().filter(|d| d.is_virtio()).collect()
    }

    /// Find by vendor:device ID
    pub fn find_by_id(&self, vendor: u16, device: u16) -> Option<&PciDevice> {
        self.devices.iter().find(|d| d.id.vendor_id == vendor && d.id.device_id == device)
    }

    /// Get device count
    pub fn device_count(&self) -> usize { self.devices.len() }

    /// Create a simulated bus with common virtio devices
    pub fn simulated() -> Self {
        let mut bus = PciBus::new();

        // virtio-blk (block device)
        let mut blk = PciDevice::new(0, 4, 0, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0));
        blk.add_bar(PciBar { index: 0, address: 0xFE000000, size: 0x1000, is_mmio: true, is_io: false, is_64bit: false });
        blk.irq_pin = 1;
        blk.irq_line = 16;
        bus.register(blk);

        // virtio-net (network device)
        let mut net = PciDevice::new(0, 5, 0, PciDeviceId::new(0x1AF4, 0x1000, PCI_CLASS_NETWORK, 0));
        net.add_bar(PciBar { index: 0, address: 0xFE001000, size: 0x1000, is_mmio: true, is_io: false, is_64bit: false });
        net.irq_pin = 1;
        net.irq_line = 17;
        bus.register(net);

        // HPET timer (class 0x08 = Generic system peripheral, subclass 0x03 = HPET)
        let mut hpet = PciDevice::new(0, 6, 0, PciDeviceId::new(0x8086, 0x7010, 0x08, 0x03));
        hpet.add_bar(PciBar { index: 0, address: 0xFED00000, size: 0x400, is_mmio: true, is_io: false, is_64bit: false });
        bus.register(hpet);

        bus
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MMIO Register Access (abstraction for memory-mapped I/O)
// ═══════════════════════════════════════════════════════════════════════════════

/// Simulated MMIO region (for testing without real hardware)
pub struct MmioRegion {
    base:   u64,
    size:   u64,
    data:   Vec<AtomicU64>,
}

impl MmioRegion {
    pub fn new(base: u64, size: u64) -> Self {
        let words = (size / 8) as usize;
        Self {
            base,
            size,
            data: (0..words).map(|_| AtomicU64::new(0)).collect(),
        }
    }

    pub fn read(&self, offset: u64) -> u64 {
        let idx = (offset / 8) as usize;
        if idx < self.data.len() {
            self.data[idx].load(Ordering::SeqCst)
        } else {
            0
        }
    }

    pub fn write(&self, offset: u64, value: u64) {
        let idx = (offset / 8) as usize;
        if idx < self.data.len() {
            self.data[idx].store(value, Ordering::SeqCst);
        }
    }

    pub fn read_reg(&self, reg: u32) -> u64 {
        self.read(reg as u64 * 8)
    }

    pub fn write_reg(&self, reg: u32, value: u64) {
        self.write(reg as u64 * 8, value);
    }

    pub fn base_address(&self) -> u64 { self.base }
    pub fn size(&self) -> u64 { self.size }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HPET Timer Driver (implements TimerSource trait)
// ═══════════════════════════════════════════════════════════════════════════════

/// HPET Register offsets (in bytes)
const HPET_GENERAL_CAPS:    u32 = 0x00;
const HPET_GENERAL_CONFIG:   u32 = 0x10;
const HPET_GENERAL_INT_STATUS: u32 = 0x20;
const HPET_MAIN_COUNTER:     u32 = 0xF0;
const HPET_TIMER0_CONFIG:    u32 = 0x100;
const HPET_TIMER0_COMPARE:   u32 = 0x108;
const HPET_TIMER0_FSB:       u32 = 0x110;

/// HPET capability flags
const HPET_CAP_LEGACY_ROUTE: u64 = 1 << 15;
const HPET_CAP_COUNT_SIZE:   u64 = 1 << 13;
const HPET_CAP_TIMER_SIZE:   u64 = 1 << 8;

/// HPET configuration flags
const HPET_CFG_ENABLE:       u64 = 1 << 0;
const HPET_CFG_LEGACY:       u64 = 1 << 1;

/// HPET Timer configuration
const HPET_TN_ENABLE:        u64 = 1 << 2;
const HPET_TN_PERIODIC:       u64 = 1 << 3;
const HPET_TN_32BIT:         u64 = 1 << 5;
const HPET_TN_FSB_ENABLE:    u64 = 1 << 14;

/// HPET frequency (in femtoseconds per tick)
pub struct HpetTimer {
    mmio:       MmioRegion,
    freq_fs:   u64,   // Femtoseconds per counter tick
    counter:   AtomicU64,
    enabled:   AtomicU64,
    tick_count: AtomicU64,
}

impl HpetTimer {
    pub fn new(base_addr: u64) -> Self {
        let mmio = MmioRegion::new(base_addr, 0x400);

        // Simulated: 10 MHz = 100 ns per tick = 100_000_000 fs per tick
        let freq_fs = 100_000_000;

        Self {
            mmio,
            freq_fs,
            counter: AtomicU64::new(0),
            enabled: AtomicU64::new(0),
            tick_count: AtomicU64::new(0),
        }
    }

    /// Initialize the HPET timer
    pub fn init(&self) {
        // Read capabilities
        let caps = self.mmio.read_reg(HPET_GENERAL_CAPS >> 3);
        let _ = caps; // In real hardware, this tells us the frequency

        // Disable HPET during config
        self.mmio.write_reg(HPET_GENERAL_CONFIG >> 3, 0);

        // Set main counter to 0
        self.counter.store(0, Ordering::SeqCst);
        self.mmio.write_reg(HPET_MAIN_COUNTER >> 3, 0);

        // Configure Timer 0 for periodic mode
        let timer0_cfg = HPET_TN_ENABLE | HPET_TN_PERIODIC;
        self.mmio.write_reg(HPET_TIMER0_CONFIG >> 3, timer0_cfg);

        // Set compare value (10000 ticks = 1ms at 10MHz)
        self.mmio.write_reg(HPET_TIMER0_COMPARE >> 3, 10000);

        // Enable HPET
        self.mmio.write_reg(HPET_GENERAL_CONFIG >> 3, HPET_CFG_ENABLE);
        self.enabled.store(1, Ordering::SeqCst);
    }

    /// Disable the HPET
    pub fn disable(&self) {
        self.mmio.write_reg(HPET_GENERAL_CONFIG >> 3, 0);
        self.enabled.store(0, Ordering::SeqCst);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    /// Read the main counter value
    pub fn counter_value(&self) -> u64 {
        self.counter.load(Ordering::SeqCst)
    }

    /// Advance the simulated counter by N ticks
    pub fn advance_ticks(&self, ticks: u64) {
        self.counter.fetch_add(ticks, Ordering::SeqCst);
        self.tick_count.fetch_add(1, Ordering::SeqCst);
        self.mmio.write_reg(HPET_MAIN_COUNTER >> 3, self.counter.load(Ordering::SeqCst));
    }

    /// Get timer frequency in Hz
    pub fn frequency_hz(&self) -> u64 {
        1_000_000_000_000_000 / self.freq_fs
    }

    /// Get femtoseconds per tick
    pub fn femtoseconds_per_tick(&self) -> u64 { self.freq_fs }

    /// Convert ticks to nanoseconds
    pub fn ticks_to_ns(&self, ticks: u64) -> u64 {
        ticks * (self.freq_fs / 1_000_000)
    }

    /// Convert nanoseconds to ticks
    pub fn ns_to_ticks(&self, ns: u64) -> u64 {
        ns / (self.freq_fs / 1_000_000)
    }

    /// Get total tick count (for stats)
    pub fn tick_count(&self) -> u64 {
        self.tick_count.load(Ordering::SeqCst)
    }

    /// Read a register (for debugging)
    pub fn read_reg(&self, offset: u32) -> u64 {
        self.mmio.read(offset as u64)
    }

    /// Write a register (for debugging)
    pub fn write_reg(&self, offset: u32, value: u64) {
        self.mmio.write(offset as u64, value);
    }
}

impl crate::timer::TimerSource for HpetTimer {
    fn frequency_ns(&self) -> u64 {
        self.freq_fs / 1_000_000  // Convert fs to ns
    }

    fn nanoseconds(&self) -> u64 {
        let ticks = self.counter.load(Ordering::SeqCst);
        self.ticks_to_ns(ticks)
    }

    fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
        self.mmio.write_reg(HPET_MAIN_COUNTER >> 3, 0);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// virtio Block Device Driver (implements BlockDevice trait)
// ═══════════════════════════════════════════════════════════════════════════════

/// virtio-blk request types
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VirtioBlkReqType {
    Read  = 0,
    Write = 1,
    Flush = 4,
}

/// virtio-blk request header
#[derive(Clone, Copy, Debug)]
pub struct VirtioBlkHeader {
    pub req_type:  u32,
    pub reserved:  u32,
    pub sector:    u64,
}

impl VirtioBlkHeader {
    pub fn read(sector: u64) -> Self {
        Self { req_type: VirtioBlkReqType::Read as u32, reserved: 0, sector }
    }
    pub fn write(sector: u64) -> Self {
        Self { req_type: VirtioBlkReqType::Write as u32, reserved: 0, sector }
    }
    pub fn flush() -> Self {
        Self { req_type: VirtioBlkReqType::Flush as u32, reserved: 0, sector: 0 }
    }
}

/// virtio-blk status codes
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VirtioBlkStatus {
    Ok       = 0,
    IoError  = 1,
    Unsupp   = 2,
}

/// virtio-blk configuration
#[derive(Clone, Debug)]
pub struct VirtioBlkConfig {
    pub capacity:     u64,   // Number of 512-byte sectors
    pub size_max:      u32,   // Max segment size
    pub seg_max:       u32,   // Max segments per request
    pub block_size:    u32,   // Block size (usually 512)
    pub read_only:     bool,
    pub write_cache:   bool,
}

impl Default for VirtioBlkConfig {
    fn default() -> Self {
        Self {
            capacity: 1024,  // 1024 sectors = 512 KiB
            size_max: 4096,
            seg_max: 64,
            block_size: 512,
            read_only: false,
            write_cache: true,
        }
    }
}

/// virtio-blk device driver
pub struct VirtioBlkDevice {
    config:      VirtioBlkConfig,
    mmio:        MmioRegion,
    storage:     Vec<Vec<u8>>,  // Simulated backing storage
    read_count:  AtomicU64,
    write_count: AtomicU64,
    flush_count: AtomicU64,
    initialized: AtomicU64,
}

impl VirtioBlkDevice {
    pub fn new(base_addr: u64, config: VirtioBlkConfig) -> Self {
        let block_size = config.block_size as usize;
        let storage = (0..config.capacity as usize)
            .map(|_| vec![0u8; block_size])
            .collect();

        Self {
            config,
            mmio: MmioRegion::new(base_addr, 0x1000),
            storage,
            read_count: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            flush_count: AtomicU64::new(0),
            initialized: AtomicU64::new(0),
        }
    }

    /// Initialize the device (virtio negotiation)
    pub fn init(&self) {
        // Simulated: set status to ACK | DRIVER | FEATURES_OK
        self.mmio.write_reg(0, 0x07); // VIRTIO_CONFIG_S_ACKNOWLEDGE | DRIVER | FEATURES_OK
        self.initialized.store(1, Ordering::SeqCst);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst) == 1
    }

    pub fn config(&self) -> &VirtioBlkConfig { &self.config }

    pub fn read_count(&self) -> u64 { self.read_count.load(Ordering::SeqCst) }
    pub fn write_count(&self) -> u64 { self.write_count.load(Ordering::SeqCst) }
    pub fn flush_count(&self) -> u64 { self.flush_count.load(Ordering::SeqCst) }

    pub fn total_bytes(&self) -> u64 {
        self.config.capacity * self.config.block_size as u64
    }

    pub fn is_read_only(&self) -> bool { self.config.read_only }

    /// Fill a block with test data (for testing)
    pub fn fill_block(&self, lba: u64, data: &[u8]) {
        let block_size = self.config.block_size as usize;
        if let Some(block) = self.storage.get(lba as usize) {
            let block_ref = unsafe { &*(block as *const Vec<u8> as *mut Vec<u8>) };
            let n = data.len().min(block_size);
            block_ref[..n].copy_from_slice(&data[..n]);
        }
    }
}

impl crate::block::BlockDevice for VirtioBlkDevice {
    fn block_size(&self) -> usize { self.config.block_size as usize }

    fn block_count(&self) -> u64 { self.config.capacity }

    fn read_block(&self, lba: u64) -> Result<[u8; 512], crate::block::BlockError> {
        self.read_count.fetch_add(1, Ordering::SeqCst);

        if lba >= self.config.capacity {
            return Err(crate::block::BlockError::OutOfRange);
        }

        let mut buf = [0u8; 512];
        if let Some(block) = self.storage.get(lba as usize) {
            let n = block.len().min(512);
            buf[..n].copy_from_slice(&block[..n]);
        }
        Ok(buf)
    }

    fn write_block(&self, lba: u64, data: &[u8; 512]) -> Result<(), crate::block::BlockError> {
        self.write_count.fetch_add(1, Ordering::SeqCst);

        if self.config.read_only {
            return Err(crate::block::BlockError::ReadOnly);
        }
        if lba >= self.config.capacity {
            return Err(crate::block::BlockError::OutOfRange);
        }

        if let Some(block) = self.storage.get(lba as usize) {
            let block_ref = unsafe { &mut *(block as *const Vec<u8> as *mut Vec<u8>) };
            block_ref[..512].copy_from_slice(data);
        }
        Ok(())
    }

    fn flush(&self) -> Result<(), crate::block::BlockError> {
        self.flush_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn name(&self) -> &str { "virtio-blk" }
}

// ═══════════════════════════════════════════════════════════════════════════════
// virtio-net Network Device Driver (implements NetworkDevice trait)
// ═══════════════════════════════════════════════════════════════════════════════

/// virtio-net configuration
#[derive(Clone, Debug)]
pub struct VirtioNetConfig {
    pub mac:        [u8; 6],
    pub max_vqs:    u16,   // Max virtqueues
    pub mtu:        u16,   // Maximum transmission unit
    pub status:     u8,    // Link status
    pub speed:      u32,   // Link speed in Mbps
}

impl Default for VirtioNetConfig {
    fn default() -> Self {
        Self {
            mac: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56], // QEMU default MAC
            max_vqs: 3,
            mtu: 1500,
            status: 1,  // Link up
            speed: 1000, // 1 Gbps
        }
    }
}

/// virtio-net device driver
pub struct VirtioNetDevice {
    config:      VirtioNetConfig,
    mmio:        MmioRegion,
    rx_queue:    Vec<Vec<u8>>,   // Receive queue (simulated)
    tx_queue:    Vec<Vec<u8>>,   // Transmit queue (simulated)
    rx_count:    AtomicU64,
    tx_count:    AtomicU64,
    link_up:     AtomicU64,
    initialized: AtomicU64,
}

impl VirtioNetDevice {
    pub fn new(base_addr: u64, config: VirtioNetConfig) -> Self {
        Self {
            config,
            mmio: MmioRegion::new(base_addr, 0x1000),
            rx_queue: Vec::new(),
            tx_queue: Vec::new(),
            rx_count: AtomicU64::new(0),
            tx_count: AtomicU64::new(0),
            link_up: AtomicU64::new(1),
            initialized: AtomicU64::new(0),
        }
    }

    /// Initialize the device
    pub fn init(&self) {
        self.mmio.write_reg(0, 0x07); // ACK | DRIVER | FEATURES_OK
        self.initialized.store(1, Ordering::SeqCst);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst) == 1
    }

    pub fn link_is_up(&self) -> bool {
        self.link_up.load(Ordering::SeqCst) == 1
    }

    pub fn set_link_down(&self) {
        self.link_up.store(0, Ordering::SeqCst);
    }

    pub fn set_link_up(&self) {
        self.link_up.store(1, Ordering::SeqCst);
    }

    pub fn mtu(&self) -> u16 { self.config.mtu }
    pub fn speed_mbps(&self) -> u32 { self.config.speed }

    pub fn rx_count(&self) -> u64 { self.rx_count.load(Ordering::SeqCst) }
    pub fn tx_count(&self) -> u64 { self.tx_count.load(Ordering::SeqCst) }

    /// Inject a received packet (simulated — hardware would put it in RX queue)
    pub fn inject_rx_packet(&mut self, data: Vec<u8>) {
        self.rx_queue.push(data);
    }

    /// Get TX queue contents (simulated — for testing what was sent)
    pub fn tx_queue_contents(&self) -> &[Vec<u8>] {
        &self.tx_queue
    }

    pub fn config(&self) -> &VirtioNetConfig { &self.config }
}

impl crate::net::NetworkDevice for VirtioNetDevice {
    fn mac_address(&self) -> crate::net::MacAddress {
        crate::net::MacAddress::new(
            self.config.mac[0], self.config.mac[1], self.config.mac[2],
            self.config.mac[3], self.config.mac[4], self.config.mac[5],
        )
    }

    fn send(&self, data: &[u8]) -> Result<(), crate::net::NetworkError> {
        if !self.link_is_up() {
            return Err(crate::net::NetworkError::DeviceError);
        }
        self.tx_count.fetch_add(1, Ordering::SeqCst);

        let tx_ref = unsafe { &mut *(self as *const Self as *mut Self) };
        tx_ref.tx_queue.push(data.to_vec());
        Ok(())
    }

    fn receive(&self) -> Result<Vec<u8>, crate::net::NetworkError> {
        if !self.link_is_up() {
            return Err(crate::net::NetworkError::DeviceError);
        }

        let self_ref = unsafe { &mut *(self as *const Self as *mut Self) };
        if let Some(pkt) = self_ref.rx_queue.pop() {
            self.rx_count.fetch_add(1, Ordering::SeqCst);
            Ok(pkt)
        } else {
            Err(crate::net::NetworkError::NoPacket)
        }
    }

    fn has_packets(&self) -> bool {
        let self_ref = unsafe { &*(self as *const Self) };
        !self_ref.rx_queue.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Driver Manager (coordinates all hardware drivers)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct DriverManager {
    pub pci_bus:    PciBus,
    pub hpet:       Option<HpetTimer>,
    pub virtio_blk: Option<VirtioBlkDevice>,
    pub virtio_net: Option<VirtioNetDevice>,
    initialized:    bool,
}

impl Default for DriverManager {
    fn default() -> Self { Self::new() }
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            pci_bus: PciBus::new(),
            hpet: None,
            virtio_blk: None,
            virtio_net: None,
            initialized: false,
        }
    }

    /// Create a simulated driver manager with all common devices
    pub fn simulated() -> Self {
        let pci_bus = PciBus::simulated();

        // Create drivers from PCI BAR addresses
        let hpet = pci_bus.find_by_id(0x8086, 0x7010)
            .and_then(|d| d.find_mmio_bar())
            .map(|bar| HpetTimer::new(bar.address));

        let virtio_blk = pci_bus.find_virtio().iter()
            .find(|d| d.id.class_code == PCI_CLASS_STORAGE)
            .and_then(|d| d.find_mmio_bar())
            .map(|bar| VirtioBlkDevice::new(bar.address, VirtioBlkConfig::default()));

        let virtio_net = pci_bus.find_virtio().iter()
            .find(|d| d.id.class_code == PCI_CLASS_NETWORK)
            .and_then(|d| d.find_mmio_bar())
            .map(|bar| VirtioNetDevice::new(bar.address, VirtioNetConfig::default()));

        Self {
            pci_bus,
            hpet,
            virtio_blk,
            virtio_net,
            initialized: false,
        }
    }

    /// Initialize all discovered drivers
    pub fn init_all(&mut self) {
        if let Some(ref hpet) = self.hpet { hpet.init(); }
        if let Some(ref blk) = self.virtio_blk { blk.init(); }
        if let Some(ref net) = self.virtio_net { net.init(); }
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool { self.initialized }

    /// Summary of discovered devices
    pub fn device_summary(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("PCI devices: {}\n", self.pci_bus.device_count()));
        s.push_str(&format!("HPET: {}\n", if self.hpet.is_some() { "present" } else { "absent" }));
        s.push_str(&format!("virtio-blk: {}\n", if self.virtio_blk.is_some() { "present" } else { "absent" }));
        s.push_str(&format!("virtio-net: {}\n", if self.virtio_net.is_some() { "present" } else { "absent" }));
        s
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- PciDeviceId tests ---

    #[test]
    fn test_pci_device_id_new() {
        let id = PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0);
        assert_eq!(id.vendor_id, 0x1AF4);
        assert_eq!(id.device_id, 0x1001);
        assert_eq!(id.class_code, PCI_CLASS_STORAGE);
    }

    // --- PciDevice tests ---

    #[test]
    fn test_pci_device_new() {
        let dev = PciDevice::new(0, 4, 0, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0));
        assert_eq!(dev.bus, 0);
        assert_eq!(dev.slot, 4);
        assert_eq!(dev.function, 0);
    }

    #[test]
    fn test_pci_device_bdf() {
        let dev = PciDevice::new(0, 4, 1, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0));
        let bdf = dev.bdf();
        assert_eq!(bdf & 0xFF, 1); // function
        assert_eq!((bdf >> 8) & 0x1F, 4); // slot
        assert_eq!((bdf >> 16) & 0xFF, 0); // bus
    }

    #[test]
    fn test_pci_device_is_virtio() {
        let dev = PciDevice::new(0, 4, 0, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0));
        assert!(dev.is_virtio());

        let dev2 = PciDevice::new(0, 5, 0, PciDeviceId::new(0x8086, 0x7010, 0x08, 0x03));
        assert!(!dev2.is_virtio());
    }

    #[test]
    fn test_pci_device_add_bar() {
        let mut dev = PciDevice::new(0, 4, 0, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0));
        dev.add_bar(PciBar { index: 0, address: 0xFE000000, size: 0x1000, is_mmio: true, is_io: false, is_64bit: false });
        assert_eq!(dev.bars.len(), 1);
        assert!(dev.find_mmio_bar().is_some());
    }

    // --- PciBus tests ---

    #[test]
    fn test_pci_bus_new() {
        let bus = PciBus::new();
        assert_eq!(bus.device_count(), 0);
    }

    #[test]
    fn test_pci_bus_register() {
        let mut bus = PciBus::new();
        bus.register(PciDevice::new(0, 4, 0, PciDeviceId::new(0x1AF4, 0x1001, PCI_CLASS_STORAGE, 0)));
        assert_eq!(bus.device_count(), 1);
    }

    #[test]
    fn test_pci_bus_simulated() {
        let bus = PciBus::simulated();
        assert_eq!(bus.device_count(), 3); // virtio-blk + virtio-net + HPET
    }

    #[test]
    fn test_pci_bus_find_by_class() {
        let bus = PciBus::simulated();
        let storage = bus.find_by_class(PCI_CLASS_STORAGE);
        assert_eq!(storage.len(), 1);
        let net = bus.find_by_class(PCI_CLASS_NETWORK);
        assert_eq!(net.len(), 1);
    }

    #[test]
    fn test_pci_bus_find_virtio() {
        let bus = PciBus::simulated();
        let virtio = bus.find_virtio();
        assert_eq!(virtio.len(), 2); // virtio-blk + virtio-net
    }

    #[test]
    fn test_pci_bus_find_by_id() {
        let bus = PciBus::simulated();
        let dev = bus.find_by_id(0x8086, 0x7010);
        assert!(dev.is_some());
        assert_eq!(dev.unwrap().id.class_code, 0x08);
    }

    #[test]
    fn test_pci_bus_scan() {
        let bus = PciBus::simulated();
        let devices = bus.scan();
        assert_eq!(devices.len(), 3);
    }

    // --- MmioRegion tests ---

    #[test]
    fn test_mmio_new() {
        let mmio = MmioRegion::new(0xFE000000, 0x1000);
        assert_eq!(mmio.base_address(), 0xFE000000);
        assert_eq!(mmio.size(), 0x1000);
    }

    #[test]
    fn test_mmio_read_write() {
        let mmio = MmioRegion::new(0xFE000000, 0x1000);
        mmio.write(0x100, 0xDEADBEEF);
        assert_eq!(mmio.read(0x100), 0xDEADBEEF);
    }

    #[test]
    fn test_mmio_read_write_reg() {
        let mmio = MmioRegion::new(0xFE000000, 0x1000);
        mmio.write_reg(4, 0x12345678);
        assert_eq!(mmio.read_reg(4), 0x12345678);
    }

    #[test]
    fn test_mmio_out_of_bounds() {
        let mmio = MmioRegion::new(0xFE000000, 0x100);
        mmio.write(0x200, 0xABCD);
        assert_eq!(mmio.read(0x200), 0); // Out of bounds → 0
    }

    #[test]
    fn test_mmio_default_zero() {
        let mmio = MmioRegion::new(0xFE000000, 0x1000);
        assert_eq!(mmio.read(0), 0);
    }

    // --- HpetTimer tests ---

    #[test]
    fn test_hpet_new() {
        let timer = HpetTimer::new(0xFED00000);
        assert!(!timer.is_enabled());
        assert_eq!(timer.counter_value(), 0);
    }

    #[test]
    fn test_hpet_init() {
        let timer = HpetTimer::new(0xFED00000);
        timer.init();
        assert!(timer.is_enabled());
    }

    #[test]
    fn test_hpet_disable() {
        let timer = HpetTimer::new(0xFED00000);
        timer.init();
        assert!(timer.is_enabled());
        timer.disable();
        assert!(!timer.is_enabled());
    }

    #[test]
    fn test_hpet_advance_ticks() {
        let timer = HpetTimer::new(0xFED00000);
        timer.advance_ticks(100);
        assert_eq!(timer.counter_value(), 100);
        timer.advance_ticks(50);
        assert_eq!(timer.counter_value(), 150);
        assert_eq!(timer.tick_count(), 2);
    }

    #[test]
    fn test_hpet_frequency() {
        let timer = HpetTimer::new(0xFED00000);
        assert_eq!(timer.frequency_hz(), 10_000_000); // 10 MHz
    }

    #[test]
    fn test_hpet_ticks_to_ns() {
        let timer = HpetTimer::new(0xFED00000);
        assert_eq!(timer.ticks_to_ns(10), 100); // 10 ticks = 100 ns at 10MHz
        assert_eq!(timer.ticks_to_ns(10000), 100000); // 10000 ticks = 1ms
    }

    #[test]
    fn test_hpet_ns_to_ticks() {
        let timer = HpetTimer::new(0xFED00000);
        assert_eq!(timer.ns_to_ticks(100), 10);
        assert_eq!(timer.ns_to_ticks(1_000_000), 10000); // 1ms = 10000 ticks
    }

    #[test]
    fn test_hpet_timer_source_impl() {
        let timer = HpetTimer::new(0xFED00000);
        timer.advance_ticks(10000); // 1ms
        assert_eq!(timer.nanoseconds(), 1_000_000);
        assert_eq!(timer.frequency_ns(), 100); // 100 ns per tick
    }

    #[test]
    fn test_hpet_reset() {
        let timer = HpetTimer::new(0xFED00000);
        timer.advance_ticks(500);
        timer.reset();
        assert_eq!(timer.counter_value(), 0);
        assert_eq!(timer.nanoseconds(), 0);
    }

    #[test]
    fn test_hpet_femtoseconds_per_tick() {
        let timer = HpetTimer::new(0xFED00000);
        assert_eq!(timer.femtoseconds_per_tick(), 100_000_000);
    }

    // --- VirtioBlkDevice tests ---

    #[test]
    fn test_virtio_blk_new() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        assert!(!dev.is_initialized());
        assert_eq!(dev.config().capacity, 1024);
        assert_eq!(dev.config().block_size, 512);
    }

    #[test]
    fn test_virtio_blk_init() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        dev.init();
        assert!(dev.is_initialized());
    }

    #[test]
    fn test_virtio_blk_total_bytes() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        assert_eq!(dev.total_bytes(), 1024 * 512); // 512 KiB
    }

    #[test]
    fn test_virtio_blk_read_block() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        dev.fill_block(0, &[0xAA; 512]);
        let block = dev.read_block(0).unwrap();
        assert_eq!(block[0], 0xAA);
        assert_eq!(dev.read_count(), 1);
    }

    #[test]
    fn test_virtio_blk_write_block() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        let data = [0x42u8; 512];
        dev.write_block(0, &data).unwrap();
        let block = dev.read_block(0).unwrap();
        assert_eq!(block[0], 0x42);
        assert_eq!(dev.write_count(), 1);
    }

    #[test]
    fn test_virtio_blk_read_out_of_range() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        assert!(dev.read_block(2000).is_err());
    }

    #[test]
    fn test_virtio_blk_write_out_of_range() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        assert!(dev.write_block(2000, &[0u8; 512]).is_err());
    }

    #[test]
    fn test_virtio_blk_read_only() {
        let mut config = VirtioBlkConfig::default();
        config.read_only = true;
        let dev = VirtioBlkDevice::new(0xFE000000, config);
        assert!(dev.is_read_only());
        assert!(dev.write_block(0, &[0u8; 512]).is_err());
    }

    #[test]
    fn test_virtio_blk_flush() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        dev.flush().unwrap();
        assert_eq!(dev.flush_count(), 1);
    }

    #[test]
    fn test_virtio_blk_block_device_impl() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        assert_eq!(dev.block_size(), 512);
        assert_eq!(dev.block_count(), 1024);
        assert_eq!(dev.name(), "virtio-blk");
    }

    #[test]
    fn test_virtio_blk_header() {
        let read_hdr = VirtioBlkHeader::read(42);
        assert_eq!(read_hdr.req_type, 0); // Read
        assert_eq!(read_hdr.sector, 42);

        let write_hdr = VirtioBlkHeader::write(10);
        assert_eq!(write_hdr.req_type, 1); // Write

        let flush_hdr = VirtioBlkHeader::flush();
        assert_eq!(flush_hdr.req_type, 4); // Flush
    }

    #[test]
    fn test_virtio_blk_custom_config() {
        let config = VirtioBlkConfig {
            capacity: 4096,
            size_max: 8192,
            seg_max: 128,
            block_size: 512,
            read_only: false,
            write_cache: false,
        };
        let dev = VirtioBlkDevice::new(0xFE000000, config);
        assert_eq!(dev.block_count(), 4096);
        assert_eq!(dev.total_bytes(), 4096 * 512);
    }

    // --- VirtioNetDevice tests ---

    #[test]
    fn test_virtio_net_new() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        assert!(!dev.is_initialized());
        assert!(dev.link_is_up());
    }

    #[test]
    fn test_virtio_net_init() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        dev.init();
        assert!(dev.is_initialized());
    }

    #[test]
    fn test_virtio_net_link_control() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        assert!(dev.link_is_up());
        dev.set_link_down();
        assert!(!dev.link_is_up());
        dev.set_link_up();
        assert!(dev.link_is_up());
    }

    #[test]
    fn test_virtio_net_mac() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        let mac = dev.mac_address();
        assert_eq!(mac.0, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
    }

    #[test]
    fn test_virtio_net_mtu() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        assert_eq!(dev.mtu(), 1500);
        assert_eq!(dev.speed_mbps(), 1000);
    }

    #[test]
    fn test_virtio_net_send() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        let data = vec![0xFF; 64];
        dev.send(&data).unwrap();
        assert_eq!(dev.tx_count(), 1);
    }

    #[test]
    fn test_virtio_net_send_link_down() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        dev.set_link_down();
        assert!(dev.send(&[0u8; 10]).is_err());
    }

    #[test]
    fn test_virtio_net_receive_empty() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        assert!(dev.receive().is_err()); // No packets
    }

    #[test]
    fn test_virtio_net_inject_and_receive() {
        let mut dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        dev.inject_rx_packet(vec![0xAA, 0xBB, 0xCC]);
        assert!(dev.has_packets());
        let pkt = dev.receive().unwrap();
        assert_eq!(pkt, vec![0xAA, 0xBB, 0xCC]);
        assert_eq!(dev.rx_count(), 1);
    }

    #[test]
    fn test_virtio_net_receive_link_down() {
        let mut dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        dev.inject_rx_packet(vec![0x01]);
        dev.set_link_down();
        assert!(dev.receive().is_err());
    }

    #[test]
    fn test_virtio_net_custom_config() {
        let config = VirtioNetConfig {
            mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            max_vqs: 4,
            mtu: 9000,  // Jumbo frames
            status: 1,
            speed: 10000, // 10 Gbps
        };
        let dev = VirtioNetDevice::new(0xFE001000, config);
        assert_eq!(dev.mac_address().0, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(dev.mtu(), 9000);
        assert_eq!(dev.speed_mbps(), 10000);
    }

    #[test]
    fn test_virtio_net_network_device_impl() {
        let dev = VirtioNetDevice::new(0xFE001000, VirtioNetConfig::default());
        let mac = dev.mac_address();
        assert_eq!(mac.0[0], 0x52);
        assert!(!dev.has_packets());
    }

    // --- DriverManager tests ---

    #[test]
    fn test_driver_manager_new() {
        let mgr = DriverManager::new();
        assert!(!mgr.is_initialized());
        assert_eq!(mgr.pci_bus.device_count(), 0);
    }

    #[test]
    fn test_driver_manager_simulated() {
        let mgr = DriverManager::simulated();
        assert_eq!(mgr.pci_bus.device_count(), 3);
        assert!(mgr.hpet.is_some());
        assert!(mgr.virtio_blk.is_some());
        assert!(mgr.virtio_net.is_some());
    }

    #[test]
    fn test_driver_manager_init_all() {
        let mut mgr = DriverManager::simulated();
        mgr.init_all();
        assert!(mgr.is_initialized());
        assert!(mgr.hpet.as_ref().unwrap().is_enabled());
        assert!(mgr.virtio_blk.as_ref().unwrap().is_initialized());
        assert!(mgr.virtio_net.as_ref().unwrap().is_initialized());
    }

    #[test]
    fn test_driver_manager_device_summary() {
        let mgr = DriverManager::simulated();
        let summary = mgr.device_summary();
        assert!(summary.contains("PCI devices: 3"));
        assert!(summary.contains("HPET: present"));
        assert!(summary.contains("virtio-blk: present"));
        assert!(summary.contains("virtio-net: present"));
    }

    #[test]
    fn test_driver_manager_empty() {
        let mgr = DriverManager::new();
        let summary = mgr.device_summary();
        assert!(summary.contains("PCI devices: 0"));
        assert!(summary.contains("HPET: absent"));
        assert!(summary.contains("virtio-blk: absent"));
        assert!(summary.contains("virtio-net: absent"));
    }

    #[test]
    fn test_pci_class_constants() {
        assert_eq!(PCI_CLASS_STORAGE, 0x01);
        assert_eq!(PCI_CLASS_NETWORK, 0x02);
        assert_eq!(PCI_CLASS_DISPLAY, 0x03);
        assert_eq!(PCI_CLASS_BRIDGE, 0x06);
    }

    #[test]
    fn test_virtio_blk_status() {
        assert_eq!(VirtioBlkStatus::Ok as u8, 0);
        assert_eq!(VirtioBlkStatus::IoError as u8, 1);
        assert_eq!(VirtioBlkStatus::Unsupp as u8, 2);
    }

    #[test]
    fn test_virtio_blk_multiple_writes() {
        let dev = VirtioBlkDevice::new(0xFE000000, VirtioBlkConfig::default());
        for i in 0..10 {
            dev.write_block(i, &[((i % 256) as u8); 512]).unwrap();
        }
        assert_eq!(dev.write_count(), 10);
        for i in 0..10 {
            let block = dev.read_block(i).unwrap();
            assert_eq!(block[0], (i % 256) as u8);
        }
        assert_eq!(dev.read_count(), 10);
    }

    #[test]
    fn test_hpet_reg_read_write() {
        let timer = HpetTimer::new(0xFED00000);
        timer.write_reg(0x10, 0xABCD);
        assert_eq!(timer.read_reg(0x10), 0xABCD);
    }

    #[test]
    fn test_full_driver_lifecycle() {
        let mut mgr = DriverManager::simulated();
        assert!(!mgr.is_initialized());

        // Initialize all drivers
        mgr.init_all();
        assert!(mgr.is_initialized());

        // Test HPET
        let hpet = mgr.hpet.as_ref().unwrap();
        hpet.advance_ticks(10000);
        assert_eq!(hpet.nanoseconds(), 1_000_000);

        // Test virtio-blk
        let blk = mgr.virtio_blk.as_ref().unwrap();
        blk.write_block(0, &[0x55; 512]).unwrap();
        let block = blk.read_block(0).unwrap();
        assert_eq!(block[0], 0x55);

        // Test virtio-net
        let net = mgr.virtio_net.as_ref().unwrap();
        net.send(&[0xFF; 64]).unwrap();
        assert_eq!(net.tx_count(), 1);
    }
}
