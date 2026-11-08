// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 11 — Block-Device-Layer
// Kernel Layer | Chain-ID 658467
// Abstraktion für Block-Storage (virtio-blk, NVMe, RAM-Disk).
// BlockBuffer-Cache, Partition-Table (MBR), VFS-Mount-Integration.
// ─────────────────────────────────────────────────────────────────────────

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

// ─── Konstanten ─────────────────────────────────────────────────────────────

pub const BLOCK_SIZE: usize = 512;
pub const SECTOR_SIZE: usize = BLOCK_SIZE;

// ─── BlockDevice Trait ─────────────────────────────────────────────────────

/// Abstraktion für ein Block-Storage-Gerät.
pub trait BlockDevice: Send + Sync {
    /// Liest einen Block (512 Bytes) an der gegebenen LBA (Logical Block Address).
    fn read_block(&self, lba: u64, buf: &mut [u8; BLOCK_SIZE]) -> Result<(), BlockError>;

    /// Schreibt einen Block an die gegebene LBA.
    fn write_block(&self, lba: u64, buf: &[u8; BLOCK_SIZE]) -> Result<(), BlockError>;

    /// Anzahl der Blöcke auf dem Gerät.
    fn block_count(&self) -> u64;

    /// Block-Grösse in Bytes (meist 512).
    fn block_size(&self) -> usize { BLOCK_SIZE }

    /// Gesamtgrösse in Bytes.
    fn capacity(&self) -> u64 {
        self.block_count() * self.block_size() as u64
    }

    /// Ob das Gerät schreibbar ist.
    fn is_read_only(&self) -> bool { false }

    /// Gerät-Name (z.B. "virtio-blk0", "ramdisk0").
    fn name(&self) -> &str { "block-device" }
}

// ─── BlockError ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// LBA ausserhalb des gültigen Bereichs
    OutOfRange,
    /// Schreibversuch auf Read-Only-Gerät
    ReadOnly,
    /// I/O-Fehler (Hardware)
    IoError(String),
    /// Block nicht im Cache
    NotCached,
    /// Ungültige Partition
    InvalidPartition,
    /// Nicht initialisiert
    NotInitialized,
}

// ─── SimulatedBlockDevice (RAM-backed, für Tests) ───────────────────────────

pub struct SimulatedBlockDevice {
    blocks: Mutex<Vec<[u8; BLOCK_SIZE]>>,
    read_only: bool,
    dev_name: String,
}

impl SimulatedBlockDevice {
    pub fn new(block_count: u64, name: &str) -> Self {
        let mut blocks = Vec::with_capacity(block_count as usize);
        for _ in 0..block_count {
            blocks.push([0u8; BLOCK_SIZE]);
        }
        SimulatedBlockDevice {
            blocks: Mutex::new(blocks),
            read_only: false,
            dev_name: name.to_string(),
        }
    }

    pub fn new_read_only(block_count: u64, name: &str) -> Self {
        let mut dev = Self::new(block_count, name);
        dev.read_only = true;
        dev
    }

    /// Füllt einen Block mit Daten (für Test-Setup).
    pub fn fill_block(&self, lba: u64, data: &[u8]) {
        let mut blocks = self.blocks.lock();
        if (lba as usize) < blocks.len() {
            let block = &mut blocks[lba as usize];
            let len = data.len().min(BLOCK_SIZE);
            block[..len].copy_from_slice(&data[..len]);
        }
    }
}

impl BlockDevice for SimulatedBlockDevice {
    fn read_block(&self, lba: u64, buf: &mut [u8; BLOCK_SIZE]) -> Result<(), BlockError> {
        let blocks = self.blocks.lock();
        let idx = lba as usize;
        if idx >= blocks.len() {
            return Err(BlockError::OutOfRange);
        }
        buf.copy_from_slice(&blocks[idx]);
        Ok(())
    }

    fn write_block(&self, lba: u64, buf: &[u8; BLOCK_SIZE]) -> Result<(), BlockError> {
        if self.read_only {
            return Err(BlockError::ReadOnly);
        }
        let mut blocks = self.blocks.lock();
        let idx = lba as usize;
        if idx >= blocks.len() {
            return Err(BlockError::OutOfRange);
        }
        blocks[idx].copy_from_slice(buf);
        Ok(())
    }

    fn block_count(&self) -> u64 {
        self.blocks.lock().len() as u64
    }

    fn is_read_only(&self) -> bool { self.read_only }
    fn name(&self) -> &str { &self.dev_name }
}

// ─── BlockBuffer (einfacher Block-Cache) ────────────────────────────────────

/// Einfacher LRU-ähnlicher Block-Cache.
pub struct BlockBuffer {
    device: Arc<dyn BlockDevice>,
    cache: Mutex<BTreeMap<u64, [u8; BLOCK_SIZE]>>,
    dirty: Mutex<BTreeMap<u64, bool>>,
    max_cached: usize,
    hits: Mutex<u64>,
    misses: Mutex<u64>,
}

impl BlockBuffer {
    pub fn new(device: Arc<dyn BlockDevice>, max_cached: usize) -> Self {
        BlockBuffer {
            device,
            cache: Mutex::new(BTreeMap::new()),
            dirty: Mutex::new(BTreeMap::new()),
            max_cached,
            hits: Mutex::new(0),
            misses: Mutex::new(0),
        }
    }

    /// Liest einen Block (aus Cache oder Gerät).
    pub fn read(&self, lba: u64) -> Result<[u8; BLOCK_SIZE], BlockError> {
        {
            let cache = self.cache.lock();
            if let Some(block) = cache.get(&lba) {
                *self.hits.lock() += 1;
                return Ok(*block);
            }
        }

        *self.misses.lock() += 1;
        let mut buf = [0u8; BLOCK_SIZE];
        self.device.read_block(lba, &mut buf)?;

        // In Cache eintragen (mit Eviction wenn voll)
        {
            let mut cache = self.cache.lock();
            if cache.len() >= self.max_cached {
                // Evict ältesten Eintrag (BTreeMap ist sortiert, evict ersten)
                if let Some(&first_key) = cache.keys().next() {
                    // Flush wenn dirty
                    let mut dirty = self.dirty.lock();
                    if dirty.remove(&first_key).unwrap_or(false) {
                        let block = cache.get(&first_key).copied();
                        if let Some(b) = block {
                            let _ = self.device.write_block(first_key, &b);
                        }
                    }
                    cache.remove(&first_key);
                }
            }
            cache.insert(lba, buf);
        }

        Ok(buf)
    }

    /// Schreibt einen Block (in Cache, markiert als dirty).
    pub fn write(&self, lba: u64, data: &[u8; BLOCK_SIZE]) -> Result<(), BlockError> {
        {
            let mut cache = self.cache.lock();
            if cache.len() >= self.max_cached && !cache.contains_key(&lba) {
                if let Some(&first_key) = cache.keys().next() {
                    let mut dirty = self.dirty.lock();
                    if dirty.remove(&first_key).unwrap_or(false) {
                        let block = cache.get(&first_key).copied();
                        if let Some(b) = block {
                            let _ = self.device.write_block(first_key, &b);
                        }
                    }
                    cache.remove(&first_key);
                }
            }
            cache.insert(lba, *data);
        }
        self.dirty.lock().insert(lba, true);
        Ok(())
    }

    /// Flush: schreibt alle dirty Blocks auf das Gerät.
    pub fn flush(&self) -> Result<usize, BlockError> {
        let mut flushed = 0;
        let cache = self.cache.lock();
        let mut dirty = self.dirty.lock();
        let dirty_keys: Vec<u64> = dirty.keys().filter(|k| *dirty.get(k).unwrap_or(&false)).copied().collect();
        for key in dirty_keys {
            if let Some(block) = cache.get(&key) {
                self.device.write_block(key, block)?;
                dirty.insert(key, false);
                flushed += 1;
            }
        }
        Ok(flushed)
    }

    /// Cache-Statistiken.
    pub fn stats(&self) -> (u64, u64) {
        (*self.hits.lock(), *self.misses.lock())
    }

    /// Anzahl der gecachten Blocks.
    pub fn cached_count(&self) -> usize {
        self.cache.lock().len()
    }

    /// Leert den Cache (mit Flush).
    pub fn clear(&self) -> Result<(), BlockError> {
        self.flush()?;
        self.cache.lock().clear();
        self.dirty.lock().clear();
        Ok(())
    }
}

// ─── Partition-Table (MBR) ──────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionEntry {
    pub bootable: bool,
    pub partition_type: u8,
    pub start_lba: u64,
    pub block_count: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MbrPartitionTable {
    pub partitions: [Option<PartitionEntry>; 4],
}

impl MbrPartitionTable {
    /// Parst eine MBR-Partitionstabelle aus einem 512-Byte-Block.
    pub fn parse(block: &[u8; BLOCK_SIZE]) -> Result<Self, BlockError> {
        // MBR-Signatur: 0x55 0xAA an Offset 510-511
        if block[510] != 0x55 || block[511] != 0xAA {
            return Err(BlockError::InvalidPartition);
        }

        let mut partitions: [Option<PartitionEntry>; 4] = [None, None, None, None];

        for i in 0..4 {
            let offset = 0x1BE + i * 16;
            let entry = &block[offset..offset + 16];

            let bootable = entry[0] == 0x80;
            let partition_type = entry[4];

            // Start LBA (Little-Endian, 32-bit at offset 8)
            let start_lba = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]) as u64;

            // Block count (Little-Endian, 32-bit at offset 12)
            let block_count = u32::from_le_bytes([entry[12], entry[13], entry[14], entry[15]]) as u64;

            if partition_type != 0 && block_count > 0 {
                partitions[i] = Some(PartitionEntry {
                    bootable,
                    partition_type,
                    start_lba,
                    block_count,
                });
            }
        }

        Ok(MbrPartitionTable { partitions })
    }

    /// Anzahl der gültigen Partitionen.
    pub fn partition_count(&self) -> usize {
        self.partitions.iter().filter(|p| p.is_some()).count()
    }

    /// Liefert Partition by index (0-3).
    pub fn get(&self, index: usize) -> Option<&PartitionEntry> {
        self.partitions.get(index).and_then(|p| p.as_ref())
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── SimulatedBlockDevice ────────────────────────────────────────────────

    #[test]
    fn test_block_device_create() {
        let dev = SimulatedBlockDevice::new(100, "ramdisk0");
        assert_eq!(dev.block_count(), 100);
        assert_eq!(dev.block_size(), 512);
        assert_eq!(dev.capacity(), 100 * 512);
        assert!(!dev.is_read_only());
        assert_eq!(dev.name(), "ramdisk0");
    }

    #[test]
    fn test_block_device_read_write() {
        let dev = SimulatedBlockDevice::new(10, "test");
        let mut block = [0u8; BLOCK_SIZE];
        block[0] = 0xDE;
        block[511] = 0xAD;

        dev.write_block(5, &block).unwrap();

        let mut read = [0u8; BLOCK_SIZE];
        dev.read_block(5, &mut read).unwrap();
        assert_eq!(read[0], 0xDE);
        assert_eq!(read[511], 0xAD);
    }

    #[test]
    fn test_block_device_read_zero_block() {
        let dev = SimulatedBlockDevice::new(10, "test");
        let mut read = [0u8; BLOCK_SIZE];
        dev.read_block(0, &mut read).unwrap();
        // Frisches Gerät sollte Nullen haben
        assert_eq!(read, [0u8; BLOCK_SIZE]);
    }

    #[test]
    fn test_block_device_out_of_range() {
        let dev = SimulatedBlockDevice::new(10, "test");
        let block = [0xFFu8; BLOCK_SIZE];
        assert_eq!(dev.write_block(10, &block), Err(BlockError::OutOfRange));
        let mut buf = [0u8; BLOCK_SIZE];
        assert_eq!(dev.read_block(10, &mut buf), Err(BlockError::OutOfRange));
    }

    #[test]
    fn test_block_device_read_only() {
        let dev = SimulatedBlockDevice::new_read_only(10, "ro");
        assert!(dev.is_read_only());
        let block = [0xFFu8; BLOCK_SIZE];
        assert_eq!(dev.write_block(0, &block), Err(BlockError::ReadOnly));
    }

    #[test]
    fn test_block_device_fill_for_tests() {
        let dev = SimulatedBlockDevice::new(10, "test");
        dev.fill_block(0, b"Hello, Block World!");
        let mut read = [0u8; BLOCK_SIZE];
        dev.read_block(0, &mut read).unwrap();
        assert_eq!(&read[..19], b"Hello, Block World!");
    }

    // ── BlockBuffer ──────────────────────────────────────────────────────────

    #[test]
    fn test_buffer_write_read() {
        let dev = Arc::new(SimulatedBlockDevice::new(100, "test"));
        let buf = BlockBuffer::new(dev, 10);

        let block = [0x42u8; BLOCK_SIZE];
        buf.write(5, &block).unwrap();

        let read = buf.read(5).unwrap();
        assert_eq!(read, block);
    }

    #[test]
    fn test_buffer_cache_hit_miss() {
        let dev = Arc::new(SimulatedBlockDevice::new(100, "test"));
        let buf = BlockBuffer::new(dev, 10);

        // Fill device
        let block = [0xAAu8; BLOCK_SIZE];
        buf.write(0, &block).unwrap();

        // Read 1: cache hit (just written)
        buf.read(0).unwrap();
        let (hits, misses) = buf.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 0);

        // Read different block: cache miss
        buf.read(1).unwrap();
        let (hits, misses) = buf.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }

    #[test]
    fn test_buffer_flush_writes_dirty() {
        let dev = Arc::new(SimulatedBlockDevice::new(100, "test"));
        let buf = BlockBuffer::new(dev.clone(), 10);

        let block = [0x77u8; BLOCK_SIZE];
        buf.write(3, &block).unwrap();

        // Before flush: device still has zeros
        let mut dev_read = [0u8; BLOCK_SIZE];
        dev.read_block(3, &mut dev_read).unwrap();
        assert_eq!(dev_read, [0u8; BLOCK_SIZE]);

        // Flush
        let count = buf.flush().unwrap();
        assert_eq!(count, 1);

        // After flush: device has data
        dev.read_block(3, &mut dev_read).unwrap();
        assert_eq!(dev_read, block);
    }

    #[test]
    fn test_buffer_eviction() {
        let dev = Arc::new(SimulatedBlockDevice::new(100, "test"));
        let buf = BlockBuffer::new(dev, 3); // small cache

        // Write 5 blocks → should evict
        for i in 0..5 {
            let block = [i as u8; BLOCK_SIZE];
            buf.write(i, &block).unwrap();
        }

        assert!(buf.cached_count() <= 3);
    }

    #[test]
    fn test_buffer_clear() {
        let dev = Arc::new(SimulatedBlockDevice::new(100, "test"));
        let buf = BlockBuffer::new(dev, 10);

        buf.write(0, &[0xFFu8; BLOCK_SIZE]).unwrap();
        assert_eq!(buf.cached_count(), 1);

        buf.clear().unwrap();
        assert_eq!(buf.cached_count(), 0);
    }

    // ── MBR Partition Table ─────────────────────────────────────────────────

    #[test]
    fn test_mbr_parse_valid() {
        let mut block = [0u8; BLOCK_SIZE];
        // MBR-Signatur
        block[510] = 0x55;
        block[511] = 0xAA;

        // Partition 0: bootable, type=0x83 (Linux), start=2048, count=100000
        let offset = 0x1BE;
        block[offset] = 0x80; // bootable
        block[offset + 4] = 0x83; // type
        block[offset + 8..offset + 12].copy_from_slice(&2048u32.to_le_bytes());
        block[offset + 12..offset + 16].copy_from_slice(&100000u32.to_le_bytes());

        let mbr = MbrPartitionTable::parse(&block).unwrap();
        assert_eq!(mbr.partition_count(), 1);

        let p0 = mbr.get(0).unwrap();
        assert!(p0.bootable);
        assert_eq!(p0.partition_type, 0x83);
        assert_eq!(p0.start_lba, 2048);
        assert_eq!(p0.block_count, 100000);
    }

    #[test]
    fn test_mbr_parse_empty() {
        let mut block = [0u8; BLOCK_SIZE];
        block[510] = 0x55;
        block[511] = 0xAA;
        // No partitions

        let mbr = MbrPartitionTable::parse(&block).unwrap();
        assert_eq!(mbr.partition_count(), 0);
    }

    #[test]
    fn test_mbr_parse_invalid_signature() {
        let block = [0u8; BLOCK_SIZE];
        assert_eq!(MbrPartitionTable::parse(&block), Err(BlockError::InvalidPartition));
    }

    #[test]
    fn test_mbr_multiple_partitions() {
        let mut block = [0u8; BLOCK_SIZE];
        block[510] = 0x55;
        block[511] = 0xAA;

        for i in 0..3 {
            let offset = 0x1BE + i * 16;
            block[offset + 4] = 0x83;
            block[offset + 8..offset + 12].copy_from_slice(&(((i + 1) as u32 * 1000)).to_le_bytes());
            block[offset + 12..offset + 16].copy_from_slice(&(((i + 1) as u32 * 50000)).to_le_bytes());
        }

        let mbr = MbrPartitionTable::parse(&block).unwrap();
        assert_eq!(mbr.partition_count(), 3);
        assert_eq!(mbr.get(0).unwrap().start_lba, 1000);
        assert_eq!(mbr.get(1).unwrap().start_lba, 2000);
        assert_eq!(mbr.get(2).unwrap().start_lba, 3000);
    }

    // ── Integration: Device + Buffer ──────────────────────────────────────────

    #[test]
    fn test_write_through_buffer_to_device() {
        let dev = Arc::new(SimulatedBlockDevice::new(50, "test"));
        let buf = BlockBuffer::new(dev.clone(), 10);

        // Write via buffer
        let data = [0xCDu8; BLOCK_SIZE];
        buf.write(42, &data).unwrap();
        buf.flush().unwrap();

        // Read directly from device
        let mut read = [0u8; BLOCK_SIZE];
        dev.read_block(42, &mut read).unwrap();
        assert_eq!(read, data);
    }
}
