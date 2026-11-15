use cadabby_core::walker;
use clap::{crate_version, Parser, ValueHint};
use camino::{Utf8PathBuf};
use std::io::ErrorKind;
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
    let path = Utf8PathBuf::from(cli.path);

    let mut errors = vec![];
    let entries_result = walker(&path,
				&mut |e| errors.push(e),
				&|a, b| a.file_name().cmp(&b.file_name()));

    // Fatal errors go here.
    let tree = match entries_result {
        Ok(entries) => entries,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprintln!("Error: the specified path does not exist.");
                process::exit(1);
            }
            ErrorKind::NotADirectory => {
                eprintln!("Error: the specified path is not a directory.");
                process::exit(1);
            }
            _ => {
                eprintln!("Error: {error:?}");
                process::exit(1);
            }
        }
    };

    // Soft errors go here.
    // TODO: these are not helpful.
    for e in &errors {
        eprintln!("Warning on read: {e}");
    }

    println!("Successfully scanned: {}", tree.name);
    println!("Children: {}", tree.entries.len());

    println!("Success!");
    // let data = fs::read(path).expect("file should exist");
    // let splits = chunkfile(&data);

    // println!("total chunk count: {}", &splits.len());

    // let compress = chunkcompress(splits);
    // for chunk in &compress {
    // 	println!("{}", chunk);
    // }

    // let compnum: Vec<_> = compress
    //     .into_iter()
    //     .filter(|x| x.compressed)
    //     .collect();

    // println!("total compressed chunks: {}", &compnum.len());

    // let mut compsave = 0;
    // for chunk in compnum {
    //     compsave += chunk.savings;
    // }

    // println!("compression saved {}MB", compsave as f32 / 1_000_000.0);
    // println!("done");
}
