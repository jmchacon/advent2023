//! day4 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashSet;
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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut sum = 0;
    for line in &lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mut winners = HashSet::new();
        let mut found = 0;
        let mut parse_done = false;
        for p in &parts[2..] {
            if *p == "|" {
                parse_done = true;
                continue;
            }
            let number = p.parse::<usize>().unwrap();
            if parse_done {
                if winners.contains(&number) {
                    found += 1;
                }
            } else {
                winners.insert(number);
            }
        }
        if found > 0 {
            sum += 2_usize.pow(found - 1);
        }
    }
    println!("part1 - {sum}");
    Ok(())
}
