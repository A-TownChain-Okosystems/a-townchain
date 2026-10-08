// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Retrieval-Interface & Storage Service Implementierung.

use crate::cas::ContentAddress;
use crate::chunking::{Chunk, ChunkError, Chunker};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    NotFound,
    ChunkError(ChunkError),
    CorruptedStorage,
    InvalidChunkSize,
}

pub trait StorageRetrieval {
    fn put_blob(&mut self, blob: &[u8]) -> Result<ContentAddress, StorageError>;
    fn get_blob(&self, address: &ContentAddress) -> Result<Vec<u8>, StorageError>;
    fn delete_blob(&mut self, address: &ContentAddress) -> Result<bool, StorageError>;
    fn has_blob(&self, address: &ContentAddress) -> bool;
    fn get_chunk(&self, address: &ContentAddress, index: usize) -> Result<Chunk, StorageError>;
    fn verify_integrity(&self, address: &ContentAddress) -> Result<bool, StorageError>;
}

pub struct StorageService {
    chunk_size: usize,
    blob_manifests: HashMap<ContentAddress, Vec<ContentAddress>>,
    chunk_store: HashMap<ContentAddress, Chunk>,
}

impl StorageService {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size: if chunk_size == 0 { 1024 } else { chunk_size },
            blob_manifests: HashMap::new(),
            chunk_store: HashMap::new(),
        }
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl StorageRetrieval for StorageService {
    fn put_blob(&mut self, blob: &[u8]) -> Result<ContentAddress, StorageError> {
        let root_addr = ContentAddress::compute(blob);
        let chunks = Chunker::chunk_data(blob, self.chunk_size)
            .map_err(StorageError::ChunkError)?;

        let mut manifest = Vec::new();
        for chunk in chunks {
            manifest.push(chunk.address.clone());
            self.chunk_store.insert(chunk.address.clone(), chunk);
        }

        self.blob_manifests.insert(root_addr.clone(), manifest);
        Ok(root_addr)
    }

    fn get_blob(&self, address: &ContentAddress) -> Result<Vec<u8>, StorageError> {
        let manifest = self.blob_manifests.get(address).ok_or(StorageError::NotFound)?;
        let mut chunks = Vec::new();

        for (idx, chunk_addr) in manifest.iter().enumerate() {
            let chunk = self.chunk_store.get(chunk_addr).ok_or(StorageError::ChunkError(ChunkError::MissingChunk { index: idx }))?;
            chunks.push(chunk.clone());
        }

        Chunker::reassemble(chunks).map_err(StorageError::ChunkError)
    }

    fn delete_blob(&mut self, address: &ContentAddress) -> Result<bool, StorageError> {
        if let Some(manifest) = self.blob_manifests.remove(address) {
            for chunk_addr in manifest {
                self.chunk_store.remove(&chunk_addr);
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn has_blob(&self, address: &ContentAddress) -> bool {
        self.blob_manifests.contains_key(address)
    }

    fn get_chunk(&self, address: &ContentAddress, index: usize) -> Result<Chunk, StorageError> {
        let manifest = self.blob_manifests.get(address).ok_or(StorageError::NotFound)?;
        let chunk_addr = manifest.get(index).ok_or(StorageError::ChunkError(ChunkError::MissingChunk { index }))?;
        self.chunk_store.get(chunk_addr).cloned().ok_or(StorageError::NotFound)
    }

    fn verify_integrity(&self, address: &ContentAddress) -> Result<bool, StorageError> {
        let blob = self.get_blob(address)?;
        let recomputed_addr = ContentAddress::compute(&blob);
        Ok(recomputed_addr == *address)
    }
}
