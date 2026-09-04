use fastcdc::v2020::FastCDC;
use blake3::Hash;
use std::io::Result;
use zstd::bulk::{compress, decompress};

/// Splits with FastCDC & hashes with BLAKE3.
pub fn chunkfile(file: &[u8]) -> Vec<(usize, usize, Hash)> {
    FastCDC::new(file, 262144, 1048576, 4194304)
        .map(|chunk| {
            let part = &file[chunk.offset..chunk.offset + chunk.length];
            let hash = blake3::hash(part);
            (chunk.offset, chunk.length, hash)
        })
        .collect()
}

#[allow(dead_code)]
pub struct Chunk {
    compressed: bool,
    data: Vec<u8>,
    hash: Option<Hash>,
}

/// Attempts to compress a chunk to ZSTD.
pub fn chunkshrink(chunk: &[u8]) -> Result<Vec<u8>> {
    compress(chunk, 0)
}

/// Attempts to decompress a chunk from ZSTD.
pub fn chunkexpand(chunk: &[u8]) -> Result<Vec<u8>> {
    decompress(chunk, 0)
}

