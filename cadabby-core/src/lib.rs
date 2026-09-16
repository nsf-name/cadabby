use fastcdc::v2020::FastCDC;
use blake3::Hash;
use zstd::bulk::{Compressor, Decompressor};
use rayon::prelude::*;
use std::cell::RefCell;
use std::fmt;

// Chunk boundary sizes.
const CHUNK_MAXSIZE: usize = 4194304;
const CHUNK_AVGSIZE: usize = 1048576;
const CHUNK_MINSIZE: usize = 262144;

#[derive(Debug, Clone, Hash)]
pub struct Chunk {
    pub compressed: bool,
    data: Vec<u8>,
    hash: Hash,
    number: usize,
    offset: usize,
    length: usize,
    pub savings: usize,
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
	// Equal iff the hashes are the same.
	// Make sure to check if compression is the same, too!
	self.hash == other.hash
    }
}

impl Eq for Chunk {}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	if self.compressed { 
            write!(f, "Chunk([{}] zstd: y, orig: {:.2}MB, size: {:.2}MB)",
		   self.number,
		   self.length as f32 / 1_000_000.0,
		   (self.length as f32 / 1_000_000.0) - (self.savings as f32 / 1_000_000.0),
            )
	} else {
	    write!(f, "Chunk([{}] zstd: n, size: {:.2}MB)",
		   self.number,
		   self.length as f32 / 1_000_000.0,
            )
	}
    }
}

/// Splits with FastCDC, then hashes with BLAKE3.
pub fn chunkfile(file: &[u8]) -> Vec<Chunk> {
    FastCDC::new(file, CHUNK_MINSIZE, CHUNK_AVGSIZE, CHUNK_MAXSIZE)
        .enumerate()
        .map(|(count, chunk)| {
            let data = file[chunk.offset..chunk.offset + chunk.length].to_vec();
            let hash = blake3::hash(&data);
            Chunk {
                compressed: false,
                data,
                hash,
		number: count,
                offset: chunk.offset,
                length: chunk.length,
                savings: 0,
            }
        })
        .collect()
}

/// Attempts to compress a Vec<Chunk> to ZSTD in parallel.
pub fn chunkcompress(mut chunks: Vec<Chunk>) -> Vec<Chunk> {
    thread_local! {
        // lifetime for an optional dict; we aren't using it
        static COMP: RefCell<Compressor<'static>> = RefCell::new(
            Compressor::new(0).expect("failed to load zstd")
        );
    }

    chunks.par_iter_mut().for_each(|chunk| {
        COMP.with(|c| {
            if !chunk.compressed {
                let mut buf = Vec::with_capacity(CHUNK_MAXSIZE);
		// TODO: handle errors.
                let Ok(size) = c
                    .borrow_mut()
                    .compress_to_buffer(&chunk.data, &mut buf) else { return };
            
                if size < chunk.data.len() {                
                    chunk.compressed = true;
                    chunk.savings = chunk.data.len() - size;
                    chunk.data = buf;
                }
            }
        });
    });

    chunks
}

/// Attempts to decompress a Vec<Chunk> from ZSTD in parallel.
pub fn chunkexpand(mut chunks: Vec<Chunk>) -> Vec<Chunk> {
    thread_local! {
        // lifetime for an optional dict; we aren't using it
        static DECOMP: RefCell<Decompressor<'static>> = RefCell::new(
            Decompressor::new().expect("failed to load zstd")
        );
    }

    chunks.par_iter_mut().for_each(|chunk| {
        DECOMP.with(|d| {
            if chunk.compressed {
                let mut buf = Vec::with_capacity(CHUNK_MAXSIZE);
		// TODO: handle errors.
                let Ok(_) = d
                    .borrow_mut()
                    .decompress_to_buffer(&chunk.data, &mut buf) else { return };
                
                chunk.compressed = false;
                chunk.data = buf;
            }
        });
    });

    chunks
}

