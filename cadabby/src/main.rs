use cadabby_core::chunkfile;
use std::env::args;
use std::path::PathBuf;
use std::fs;

fn main() {
    let path = PathBuf::from(args().nth(1).expect("no pattern given"));

    println!("Cadabby v0.1.0");
    println!("path: {:?}", path);

    let data = fs::read(path).expect("file should exist");
    let splits = chunkfile(&data);
    println!("total chunk count: {}", &splits.len());
    println!("done");
}
