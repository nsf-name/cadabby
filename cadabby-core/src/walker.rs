use crate::{EntryKind, DirEntry, FileEntry, SkippedEntry};
use indextree::{Arena, NodeId};
use walkdir::WalkDir;
use std::{path::Path, path::PathBuf, io};

// builds a path up relative to the root.
// TODO: should this be a method on Arena or EntryKind?
pub fn abspath(node: NodeId, arena: &Arena<EntryKind>) -> PathBuf{
    node.ancestors(&arena)
	.filter_map(|e| match arena.get_data(e)? {
            EntryKind::File(f) => Some(f.name.as_str()),
            EntryKind::Dir(d) => Some(d.name.as_str()),
	})
	.collect::<Vec<_>>()
	.into_iter() 
	.rev()  // ancestors goes leaf->root, paths go root->leaf
	.collect()
}

pub fn walker(root: &Path, skips: &mut Vec<SkippedEntry>) -> Result<(NodeId, Arena<EntryKind>), io::Error> {
    let root_name = root.to_string_lossy().into_owned();
    // Let's use tree based arena with a decent amount of space.
    let mut arena = Arena::with_capacity(4096);
    // Now add the root directory.
    let root_node = arena.new_node(EntryKind::Dir(
	DirEntry { name: String::from(root_name) }
    ));

    let mut stack: Vec<NodeId> = vec![root_node];

    for entry in WalkDir::new(root).min_depth(1).sort_by_file_name() {
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
        let parent = stack[depth - 1];
	let name = entry.file_name().to_string_lossy().into_owned();
 
        let kind = if entry.file_type().is_dir() {
	    EntryKind::Dir(DirEntry {
                name: name,
            })
        } else {
            EntryKind::File(FileEntry {
                name: name,
            })
        };

        let node = parent.append_value(kind, &mut arena);

        if entry.file_type().is_dir() {
            stack.truncate(depth);
            stack.push(node);
        }
    }
    
    Ok((root_node, arena))
}
