# CS333 Final Project Proposal

## Team (list all members)
Nathaniel Flores

## High-level Idea (a few sentences, at most)
Create a content-aware compression tool that can convert a file tree (some directory and all its sub-directories, recursively) into a compressed file format and back again, taking advantage of the many lessons we have learned in this class about how to do so efficiently.

## Project Goals
- Learn about FastCDC and other variable-length chunking methods
- Determine empirically when our new compression scheme succeeds, and to what extent content-aware compression could be viable
- Write efficient, parallel code that takes advantage of the Rust language
- Make a tool that's useful in the real world

## Detailed Description of Project Deliverables
This project will produce a single Rust binary called `cadabby`. It will take an argument of exactly one directory, like so:
```
cadabby ./thing
```
It will then generate some compressed archive from these (format to be determined). Running something like
```
cadabby ./thing.cby #.cadabby maybe?
```
will generate a file. This file can then be unpacked, creating the same directory as before from the compressed archive. 

Internally the compression software will create a Merkle Patricia trie from the contents of the directory (will ignore symlinks). It will then go over each file, applying the FastCDC chunking method. The first N (to be empirically determined) chunks of each file will be Shannon entropy-tested to determine if the file is worth being compressed. If it is, every chunk will be compressed with `zstd` if and only if the compressed version turns out to be smaller than the uncompressed one. Finally, the chunks will be deduplicated, creating our final on-disk data structure. 

## List of Milestones
- Compress a single file in parallel for every FastCDC chunk (currently done)
- Walk a whole directory and save it in-memory efficiently
- Apply single-file chunking and hashing across many files
- Deduplicate at the chunk level in a hash table
- Some form of on-disk persistence of our recipe and minimal set of chunks
- Benchmarks of this tool on real-world datasets

## Required Technology/Resources
Just `rustc`, Emacs, hope, and a bunch of climate datasets. Also the kernel sources at specific git checkouts.

## AI Usage
I will not use AI on this assignment.

## Criteria for Success
- Able to actually compress a directory (meaning we can save out to disk)
- This could fail in terms of being less space-efficient; if it does, that's okay as long as it isn't because of a bug in the code. Such a failure is actually very informative, as it suggests this approach won't work
- Should be able to compile and use this on other people's machines (not just my laptop)
- Relatively efficient (the disk should be our bottleneck, not any other part of the system)
