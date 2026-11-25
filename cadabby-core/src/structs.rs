use std::{path::PathBuf, io};

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

#[derive(Debug)]
pub struct SkippedEntry {
    pub path: Option<PathBuf>,
    pub error: io::Error,
}
