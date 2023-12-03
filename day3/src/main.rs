//! day3 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq)]
enum Space {
    Digit(usize),
    Symbol,
    #[default]
    None,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut grid = Grid::<Space>::new(lines[0].len(), lines.len());
    for (line_num, line) in lines.iter().enumerate() {
        for (pos, b) in line.as_bytes().iter().enumerate() {
            let l = Location(pos.try_into().unwrap(), line_num.try_into().unwrap());
            if *b == b'.' {
                continue;
            }
            if b.is_ascii_digit() {
                grid.add(&l, Space::Digit(line[pos..=pos].parse::<usize>().unwrap()));
                continue;
            }
            grid.add(&l, Space::Symbol);
        }
    }

    let mut num_start = false;
    let mut num_digits = vec![];
    let mut has_symbol = false;
    let mut sum = 0;
    for g in &grid {
        if *g.1 == Space::None || *g.1 == Space::Symbol {
            if num_start && has_symbol {
                sum += compute_num(&num_digits);
            }
            num_start = false;
            continue;
        }
        if let Space::Digit(d) = g.1 {
            if !num_start {
                has_symbol = false;
                num_digits.clear();
            }
            num_start = true;
            num_digits.push(*d);
            if !has_symbol {
                has_symbol = grid
                    .neighbors_all(&g.0)
                    .iter()
                    .any(|f| *f.1 == Space::Symbol);
            }
        }
    }
    println!("part1: {sum}");
    Ok(())
}

fn compute_num(num_digits: &Vec<usize>) -> usize {
    let l = num_digits.len() - 1;
    num_digits.iter().enumerate().fold(0, |acc, f| {
        acc + *f.1 * 10_usize.pow((l - f.0).try_into().unwrap())
    })
}
