# Cadabby
*Mirroring done so fast, it's magic*

`cadabby` is a tool for mirroring files, or at least it will be in the future. For now, the goal is to build a reasonably-performant way of compressing and restoring a file tree at a content-aware level, not just a mere compression level.

The final deliverable will work like this:
-  Walk the file hierarchy and identify all regular files (no symlinks, hardlinks, special files) + directories into a Merkle trie or radix tree (file path is the key, but we also have a hash associated with each element)
- For each file, use FastCDC to determine which chunks exist
- Find duplicate chunks across files and compress chunks with low entropy
- Create a recipe for reconstruction and then compress that into a final, serial form at for compression and decompression

After this class, I would like to work on this design further with the goal of making the delivery of highly-redundant data and deltas as efficient as possible, since mirroring is one of the most-embarrassingly slow parts of the modern internet. However, for the purposes of this class, I'll work just on making this compression portion as fast and efficient as possible. Reaching the point where a directory can be compressed and decompressed safely is the goal; maybe I'll reach deltas, but that depends on time.

## Usage
Install Rust with `rustup`, then run `cargo build --release` (`zstd` and `fastcdc` are substantially slower in the default mode). The current code doesn't do very much. I'm working on it.

## Bugs?
Yes. I will work on them over the next several weeks. 
