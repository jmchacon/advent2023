//! day15 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    // For this we can assume there's one line.
    let parts = lines[0].split(',').collect::<Vec<_>>();

    let mut sum = 0;
    for p in parts.iter().copied() {
        sum += hash(p);
    }
    println!("part1: {sum}");
    Ok(())
}

fn hash(p: &str) -> usize {
    let mut val = 0;
    for b in p.bytes() {
        val += usize::from(b);
        val = (val * 17) % 256;
    }
    val
}
