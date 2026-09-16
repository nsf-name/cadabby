use cadabby_core::{chunkfile, chunkcompress};
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
    
    let compress = chunkcompress(splits);
    let compnum: Vec<_> = compress
        .into_iter()
        .filter(|x| x.compressed)
        .collect();
        
    println!("total compressed chunks: {}", &compnum.len());

    let mut compsave = 0;
    for chunk in compnum {
	println!("{}", chunk);
        compsave += chunk.savings; 
    }

    println!("compression saved {}MB", compsave as f32 / 1_000_000.0);
    println!("done");
}
