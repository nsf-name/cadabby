use fastcdc::v2020::FastCDC;
use blake3::Hash;
use zstd::bulk::{Compressor, Decompressor};
use rayon::prelude::*;
use std::cell::RefCell;
use std::{fmt, hash};

const CHUNK_MAXSIZE: usize = 4194304;
const CHUNK_AVGSIZE: usize = 1048576;
const CHUNK_MINSIZE: usize = 262144;

#[allow(dead_code)]
/// TODO: implement Eq, remove PartialEq 
/// Also, implement Ord
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub compressed: bool,
    data: Vec<u8>,
    hash: Hash,
    offset: usize,
    length: usize,
    pub savings: usize,
}

/// TODO: this could be better
impl hash::Hash for Chunk {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk(con: {}, off: {}kb, len: {}kb)",
            self.compressed,
            self.offset as f32 / 1000.0,
            self.offset as f32 / 1000.0,
        )
    }
}

/// TODO: this could be better
impl fmt::Pointer for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = self as *const Self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

/// Splits with FastCDC, then hashes with BLAKE3.
pub fn chunkfile(file: &[u8]) -> Vec<Chunk> {
    FastCDC::new(file, CHUNK_MINSIZE, CHUNK_AVGSIZE, CHUNK_MAXSIZE)
        .map(|chunk| {
            let data = file[chunk.offset..chunk.offset + chunk.length].to_vec();
            let hash = blake3::hash(&data);
            Chunk {
                compressed: false,
                data,
                hash,
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

