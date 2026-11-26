use cadabby_core::{SkippedEntry, abspath, chunkhash, walker, EntryKind};
use clap::{crate_version, Parser, ValueHint};
use anyhow::{Context, Result};
use indextree::NodeId;
use std::{
    path::PathBuf,
    collections::HashMap,
    fs::File,
};

#[derive(Parser)]
#[command(name = "cadabby")]
#[command(bin_name = "cadabby")]
#[command(version = crate_version!())]
#[command(about = "Deduplication and compression tool.")]
#[command(author = "Nathaniel S. Flores (2026)")]
#[command(before_help = "!! Demo program for CS333 final project !!")]
struct Cli {
    #[arg(value_hint = ValueHint::DirPath)]
    path: String,
    #[arg(short, long, help = "Increase verbosity")]
    verbose: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let path = PathBuf::from(cli.path);

    // TODO: make skips, pass it in as a borrow
    // TODO: use anyhow to stop all this annoying branching

    let mut skips: Vec<SkippedEntry> = Vec::new();
    let (root, tree) = walker(&path, &mut skips).context("failed to create tree")?; 

    println!("Successfully scanned: {} paths ({} files)",
	     tree.len(), root.leaves(&tree).count());

    let mut hashes = HashMap::new();
    let files: Vec<NodeId> = root.leaves(&tree)
	.filter(|e| {
	    let thing = tree.get_data(*e);
	    match thing.unwrap() {
		EntryKind::File(_) => true,
		EntryKind::Dir(_) => false,
	    }
	})
	.collect();
    
    for leaf in files {
	let path = abspath(leaf, &tree).into_string().unwrap();
	let f = File::open(&path).context("opening file failed")?;
	let ch = chunkhash(f).context("chunkhashing failed")?;
	for hash in ch {
	    let entry = hashes.entry(hash.hash).or_insert(0);
	    *entry += 1;
	}
    }

    let values: Vec<f64> = hashes.values().map(|&v| v as f64).collect();
    let n = values.len() as f64;
    
    let mean = values.iter().sum::<f64>() / n;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    println!("Hash count: {}", hashes.len());
    println!("mean: {}, var: {}, std. dev: {}, min: {}, max: {}", mean, variance, std_dev, min, max);
    println!("Success!");
    
    Ok(())
}
