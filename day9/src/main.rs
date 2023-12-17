//! day9 advent 20XX
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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut sum = 0;
    let mut sum_part2 = 0;
    for line in &lines {
        // Seed the initial line.
        let mut parts = line
            .split_whitespace()
            .map(|f| f.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        // Keep a record of the last entry for each line.
        let mut diffs = vec![*parts.last().unwrap()];
        let mut fronts = vec![*parts.first().unwrap()];
        loop {
            if args.debug {
                println!("parts: {parts:?}");
            }
            let mut next = vec![];
            for i in 0..parts.len() - 1 {
                next.push(parts[i + 1] - parts[i]);
            }
            diffs.push(*next.last().unwrap());
            fronts.push(*next.first().unwrap());
            parts = next;
            if parts.iter().any(|f| *f != 0) {
                continue;
            }
            break;
        }
        if args.debug {
            println!("diffs: {diffs:?}");
        }
        sum += diffs.iter().sum::<isize>();
        fronts.reverse();
        let mut last = 0;
        for i in fronts.iter().skip(1) {
            let new = *i - last;
            last = new;
        }
        sum_part2 += last;
    }
    println!("part1: {sum}");
    println!("part2: {sum_part2}");
    Ok(())
}
