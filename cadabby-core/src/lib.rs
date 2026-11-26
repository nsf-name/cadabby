pub mod structs;
pub use structs::{
    EntryKind,
    FileEntry,
    DirEntry,
    SkippedEntry,
};

pub mod walker;
pub use walker::{walker, abspath};

pub mod utils;
pub use utils::{
    chunker,
    hasher,
    chunkhash,
    compress,
};
