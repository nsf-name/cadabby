use fastcdc::{v2020::StreamCDC, v2020};
use memmap2::MmapOptions;
use rayon::prelude::*;
use blake3::Hash;
use std::{fs::File, io};

pub struct Chunk {
    pub offset: u64,
    pub length: usize,
}

pub struct HashChunk {
    pub offset: u64,
    pub length: usize,
    pub hash: Hash, 
}

// Get chunks for regions of the file.
pub fn chunker(file: File) -> Result<Vec<Chunk>, v2020::Error> {
    StreamCDC::new(file, 4096, 16384, 65534)
        .map(|result| result.map(|c| Chunk { offset: c.offset, length: c.length }))
        .collect()
}

// Get chunks and hash them, using mmap() to minimize reads.
pub fn hasher(file: File, chunks: Vec<Chunk>) -> Result<Vec<HashChunk>, io::Error> {
    chunks
	.par_iter()
        .map(|chunk| -> io::Result<HashChunk> {
            let mmap = unsafe {
		MmapOptions::new()
                    .offset(chunk.offset)
                    .len(chunk.length)
                    .map(&file)?
            };
            Ok(HashChunk {
		offset: chunk.offset,
		length: chunk.length,
		hash: blake3::hash(&mmap),
            })
	})
        .collect()
}

pub fn compress(file: File, chunks: Vec<Chunk>) {
    
    // TODO: basically do the same thing as hash()
    // with mmap(), but zstd-time.
    
}
