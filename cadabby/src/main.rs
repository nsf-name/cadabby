use cadabby_core::walker;
use clap::{crate_version, Parser, ValueHint};
use std::path::PathBuf;
use std::process;

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

    let (tree, skips) = match walker(&path) {
	Ok(e) => e,
	Err(e) => {
	    eprintln!("Error: {}", e);
	    process::exit(1);
	}
    };

    // TODO: implement Display for SkippedEntry
    skips.into_iter().for_each(|e| eprintln!("Warning with file: {:?}", e));

    println!("Successfully scanned: {} files", tree.len());
    println!("Success!");
}
