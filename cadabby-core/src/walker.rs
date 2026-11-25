use crate::{EntryKind, DirEntry, FileEntry};
use indextree::{Arena, NodeId};
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::io;

#[derive(Debug)]
pub struct SkippedEntry {
    pub path: Option<PathBuf>,
    pub error: io::Error,
}

pub fn walker(root: &Path) -> Result<(Arena<EntryKind>, Vec<SkippedEntry>), io::Error> {
    let root_name = root.to_string_lossy().into_owned();
    // Let's use tree based arena with a decent amount of space.
    let mut arena = Arena::with_capacity(4096);
    // Now add the root directory.
    let root_node = arena.new_node(EntryKind::Dir(crate::DirEntry {
	name: String::from(root_name), 
    }));

    // Stacks are a convenient way to think about this construction.
    let mut dir_stack: Vec<NodeId> = vec![root_node];    
    let mut skips: Vec<SkippedEntry> = Vec::new();

    for entry in WalkDir::new(root).min_depth(1) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                skips.push(SkippedEntry {
                    path: e.path().map(Path::to_owned),
                    error: e.into(),
                });
                continue;
            }
        };

        let depth = entry.depth(); // min_depth(1) so this is >= 1
        let parent = dir_stack[depth - 1];

	// TODO: maybe don't worry about UTF-8 errors? 
        let kind = if entry.file_type().is_dir() {
	    EntryKind::Dir(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
            })
        } else {
            EntryKind::File(FileEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                size: 0,
            })
        };

        let node = parent.append_value(kind, &mut arena);

        if entry.file_type().is_dir() {
            dir_stack.truncate(depth);
            dir_stack.push(node);
        }
    }
    
    Ok((arena, skips))
}
