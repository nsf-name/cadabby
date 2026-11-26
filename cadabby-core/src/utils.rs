use std::io::Read;
use fastcdc::{v2020::StreamCDC, v2020};

pub struct Chunk {
    pub offset: u64,
    pub length: usize,
}

// Do all our real work in here.
pub fn chunk<R: Read>(reader: R) -> Result<Vec<Chunk>, v2020::Error> {
    StreamCDC::new(reader, 4096, 16384, 65534)
        .map(|result| result.map(|c| Chunk { offset: c.offset, length: c.length }))
        .collect()
}

pub fn hash<R: Read>(reader: R) {
    // TODO: basically use
    // let mmap = unsafe {
    // MmapOptions::new()
    //     .offset(chunk.offset)
    //     .len(chunk.length)
    //     .map(&file)?
    // };
    // blake3::hash(&mmap)
    // but we need to do this for EVERY chunk we were given.
}

pub fn compress<R: Read>(reader: R) {
    // TODO: basically do the same thing as hash()
    // with mmap(), but zstd-time.
    
}
