use cadabby_core::{walker, abspath, chunkhash};
use clap::{crate_version, Parser, ValueHint};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use std::fs::File;

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

fn main() {
    let cli = Cli::parse();
    let path = PathBuf::from(cli.path);

    // TODO: make skips, pass it in as a borrow
    // TODO: use anyhow to stop all this annoying branching

    let (root, tree, skips) = match walker(&path) {
	Ok(e) => e,
	Err(e) => {
	    eprintln!("Error: {}", e);
	    process::exit(1);
	}
    };

    skips.into_iter().for_each(|e| eprintln!("Warning: {}", e));

    println!("Successfully scanned: {} paths ({} files)", tree.len(), root.leaves(&tree).count());

    let mut hashes = HashMap::new();
    
    // TODO: better error handling
    
    'leaves: for leaf in root.leaves(&tree).into_iter() {
	let path = abspath(leaf, &tree).into_string().unwrap();
	let f = match File::open(&path) {
	    Ok(f) => f,
	    Err(f) => {
		//eprintln!("Error: {}, {}", f, &path);
		continue 'leaves;
	    }
	};
	let ch = match chunkhash(f) {
	    Ok(f) => f,
	    Err(f) => {
		//eprintln!("Error: {}, {}", f, &path);
		continue 'leaves;
	    }
	};
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
}
