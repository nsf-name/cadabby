# Cadabby
Mirroring done so fast, it's magic

`cadabby` is a tool for mirroring files, or at least it will be in the future. For now, the goal is simply learning Rust and filesystems stuff, one step at a time.

TODO: 
- Allocate a bunch of chunks from a file really quickly (use slab to do this fast since we know number of elements when chunkfile completes)
- Use Rayon to accelerate both hashing and compressing steps
- Then run our ZSTD compress check: either ZSTD is smaller, or it isn't
- Figure out how we're going to use flatbuffer for this
