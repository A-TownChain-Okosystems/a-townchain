// Copyright (c) 2026 Michael Wroblewski — Apache-2.0

use atc_storage::cas::ContentAddress;
use atc_storage::chunking::{ChunkError, Chunker};
use atc_storage::retrieval::{StorageError, StorageRetrieval, StorageService};

#[test]
fn test_01_cas_determinism() {
    let data = b"Hello A-TownChain Storage!";
    let addr1 = ContentAddress::compute(data);
    let addr2 = ContentAddress::compute(data);
    assert_eq!(addr1, addr2);
    assert_eq!(addr1.to_hex(), addr2.to_hex());
}

#[test]
fn test_02_cas_hex_roundtrip() {
    let data = b"Sample payload for hex serialization";
    let addr = ContentAddress::compute(data);
    let hex = addr.to_hex();
    let parsed = ContentAddress::from_hex(&hex).expect("Failed to parse hex");
    assert_eq!(addr, parsed);
}

#[test]
fn test_03_chunking_single_chunk() {
    let data = b"Small payload";
    let chunks = Chunker::chunk_data(data, 100).expect("Chunking failed");
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].index, 0);
    assert_eq!(chunks[0].data, data);
}

#[test]
fn test_04_chunking_multi_chunk() {
    let data = vec![0xAB; 25]; // 25 bytes
    let chunks = Chunker::chunk_data(&data, 10).expect("Chunking failed");
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].data.len(), 10);
    assert_eq!(chunks[1].data.len(), 10);
    assert_eq!(chunks[2].data.len(), 5);
}

#[test]
fn test_05_chunking_empty_data() {
    let res = Chunker::chunk_data(&[], 10);
    assert_eq!(res, Err(ChunkError::EmptyBlob));
}

#[test]
fn test_06_chunking_reassemble_success() {
    let data = b"Reassembly test data split over multiple chunks";
    let chunks = Chunker::chunk_data(data, 8).unwrap();
    let reassembled = Chunker::reassemble(chunks).expect("Reassembly failed");
    assert_eq!(reassembled, data);
}

#[test]
fn test_07_chunking_reassemble_corrupted_chunk() {
    let data = b"Data to corrupt";
    let mut chunks = Chunker::chunk_data(data, 5).unwrap();
    // Tamper with chunk data without updating hash
    chunks[0].data[0] ^= 0xFF;
    let res = Chunker::reassemble(chunks);
    assert!(matches!(res, Err(ChunkError::IntegrityMismatch { index: 0 })));
}

#[test]
fn test_08_storage_service_put_get_blob() {
    let mut store = StorageService::new(16);
    let data = b"Large blob stored across multiple 16-byte chunks in StorageService";
    let addr = store.put_blob(data).expect("put_blob failed");

    assert!(store.has_blob(&addr));
    let retrieved = store.get_blob(&addr).expect("get_blob failed");
    assert_eq!(retrieved, data);
}

#[test]
fn test_09_storage_service_has_and_delete_blob() {
    let mut store = StorageService::new(32);
    let data = b"Temporary data for deletion test";
    let addr = store.put_blob(data).unwrap();

    assert!(store.has_blob(&addr));
    let deleted = store.delete_blob(&addr).expect("delete_blob failed");
    assert!(deleted);
    assert!(!store.has_blob(&addr));
    assert_eq!(store.get_blob(&addr), Err(StorageError::NotFound));
}

#[test]
fn test_10_storage_service_get_chunk() {
    let mut store = StorageService::new(10);
    let data = b"0123456789ABCDEFGHIJ"; // 20 bytes -> 2 chunks
    let addr = store.put_blob(data).unwrap();

    let chunk0 = store.get_chunk(&addr, 0).expect("get_chunk 0 failed");
    assert_eq!(chunk0.index, 0);
    assert_eq!(chunk0.data, b"0123456789");

    let chunk1 = store.get_chunk(&addr, 1).expect("get_chunk 1 failed");
    assert_eq!(chunk1.index, 1);
    assert_eq!(chunk1.data, b"ABCDEFGHIJ");
}

#[test]
fn test_11_storage_service_verify_integrity() {
    let mut store = StorageService::new(16);
    let data = b"Verifiable content address integrity";
    let addr = store.put_blob(data).unwrap();

    let is_valid = store.verify_integrity(&addr).expect("verify_integrity failed");
    assert!(is_valid);
}

#[test]
fn test_12_storage_service_not_found_error() {
    let store = StorageService::default();
    let dummy_addr = ContentAddress::compute(b"Non existent blob");
    assert_eq!(store.get_blob(&dummy_addr), Err(StorageError::NotFound));
}
