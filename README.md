# Cadabby
Mirroring done so fast, it's magic

`cadabby` is a tool for mirroring files, or at least it will be in the future. For now, the goal is simply learning Rust and filesystems stuff, one step at a time.

TODO: 
- Allocate a bunch of chunks from a file really quickly (use slab to do this fast since we know number of elements when chunkfile completes)
- Serialize to-disk and back again, so we can benchmark how much it is on the wire
- Learn how to walk through directories
- Use clap to make a nice command-line parser interface
- Figure out how we're going to use flatbuffer for this
