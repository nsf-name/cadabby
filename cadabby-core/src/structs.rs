use std::{path::PathBuf, io, fmt};
use indextree::{Arena, NodeId};
use crate::abspath;

// Arena traits
#[derive(Debug)]
pub enum EntryKind {
    File(FileEntry),
    Dir(DirEntry),
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
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

impl fmt::Display for SkippedEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match &self.path {
	    Some(p) => write!(f, "({}, {})", p.display(), self.error),
	    None => write!(f, "((none), {})", self.error),
	}
    }   
}

// File traits
#[derive(Debug)]
pub struct ResolvedFile {
    pub path: PathBuf,
}

impl ResolvedFile {
    pub fn from_node(node: NodeId, arena: &Arena<EntryKind>) -> Self {
	ResolvedFile {
	    path: abspath(node, arena),
	}
    }
}

impl fmt::Display for ResolvedFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "({})", self.path.display())
    }   
}

