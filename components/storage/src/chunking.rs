// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Chunking & Reassembly Layer.

use crate::cas::ContentAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub index: usize,
    pub address: ContentAddress,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkError {
    EmptyBlob,
    InvalidChunkSize,
    MissingChunk { index: usize },
    IntegrityMismatch { index: usize },
    ReassemblyFailed,
}

pub struct Chunker;

impl Chunker {
    pub fn chunk_data(blob: &[u8], chunk_size: usize) -> Result<Vec<Chunk>, ChunkError> {
        if blob.is_empty() {
            return Err(ChunkError::EmptyBlob);
        }
        if chunk_size == 0 {
            return Err(ChunkError::InvalidChunkSize);
        }

        let mut chunks = Vec::new();
        for (i, slice) in blob.chunks(chunk_size).enumerate() {
            let addr = ContentAddress::compute(slice);
            chunks.push(Chunk {
                index: i,
                address: addr,
                data: slice.to_vec(),
            });
        }
        Ok(chunks)
    }

    pub fn reassemble(mut chunks: Vec<Chunk>) -> Result<Vec<u8>, ChunkError> {
        if chunks.is_empty() {
            return Err(ChunkError::EmptyBlob);
        }

        chunks.sort_by_key(|c| c.index);

        let mut blob = Vec::new();
        for (expected_idx, chunk) in chunks.iter().enumerate() {
            if chunk.index != expected_idx {
                return Err(ChunkError::MissingChunk { index: expected_idx });
            }
            let actual_addr = ContentAddress::compute(&chunk.data);
            if actual_addr != chunk.address {
                return Err(ChunkError::IntegrityMismatch { index: chunk.index });
            }
            blob.extend_from_slice(&chunk.data);
        }

        Ok(blob)
    }
}
