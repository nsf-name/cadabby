#[derive(Debug)]
pub enum EntryKind {
    File(FileEntry),
    Dir(DirEntry),
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64
}

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
}

// IDEA:
// We should use arena allocation, this is wildly inefficient.
// A much better idea might be to do the following with arenas:
//
// struct FileEntry {
//     name: (u32, u16),    // offset+len into name arena
//     parent: u32,         // index into flat Vec<FileEntry>
//     kind: EntryKind,
//     size: u64,
// }
//
// Then have one flat arena for names which we index into.
// We also get directory dedup for free (same entries in parent)
// We should have a bloom filter or cuckoo filter for chunk hashes...
//
// So we could use some type of stack to build up dirs as we recurse,
// instead of a naive function call? Just an idea.
//
// Might be better to handle file compression first one at a time,
// only later adding the ability to serialize out to a whole archive,
// since writing to many files is expensive and complex.
