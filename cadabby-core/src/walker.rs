use crate::{FileTree, File, Directory, Symlink};
use camino::{Utf8Path, Utf8DirEntry};
use std::cmp::Ordering;
use std::io;

// TODO: this is a very un-rust way to handle errors
// I probably want to make a new type to hold errors
// Like:
// struct WalkerResult {
//    tree: FileTree,
//    errors: Vec<io::Error>,
// }
// And then also need to do something like
// enum WalkerError {
//    Fatal(io::Error),    // couldn't read root dir, abort
//    Skipped(PathBuf, io::Error),  // couldn't read this entry, logged and skipped
// }
// Which should be better overall. This design is a little weird as-is.

pub fn walker(
    root: &Utf8Path,
    mut on_error: &mut dyn FnMut(io::Error),
    compare: &dyn Fn(&Utf8DirEntry, &Utf8DirEntry) -> Ordering,
) -> io::Result<Directory> {
    let name = match root.file_name() {
	Some(n) => n.to_string(),
	None => {
	    return Err(io::Error::new(
		io::ErrorKind::InvalidData,
		format!("Invalid path: {:?}", root)
	    ));
	}
    };
    let mut entries: Vec<Utf8DirEntry> = root.read_dir_utf8()?
	// .map_err() accepts FnOnce trait which FnMut satisfies
	.filter_map(|r| r.map_err(&mut on_error).ok())
        .collect();
    
    entries.sort_by(&compare);
    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());
    for e in entries {
	let path = e.path();
	let name = e.file_name();
	let metadata = {
	    match fs::metadata(&path) {
		Ok(data) => data,
		Err(error) => { on_error(error); continue; }
	    }
	};
	let node = match path {
	    path if path.is_dir() => FileTree::DirNode(
		// TODO: potential oneshot error?
		walker(&root.join(name), &mut on_error, &compare)?
	    ),
	    path if path.is_symlink() => FileTree::LinkNode(Symlink {
		name: name.into(),
		target: {
		    match path.read_link_utf8() {
			Ok(link) => link.to_string(),
			Err(error) => { on_error(error); continue; }
		    }
		},
		metadata: metadata,
	    }),
	    path if path.is_file() => FileTree::FileNode(File {
		name: name.into(),
		metadata: metadata,
	    }),
	    _ => unreachable!(),
	};
	directory.push(node);
    }
    Ok(Directory {
	name: name,
	entries: directory,
    })
}
