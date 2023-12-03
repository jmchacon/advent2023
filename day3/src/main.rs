//! day3 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use std::collections::HashMap;
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
    Symbol(u8),
    #[default]
    None,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    // Input will looks like this:
    //
    // 467..114..
    // ...*......
    // ..35..633.
    // 33.....#...
    //
    // Part1 - Find numbers which have any symbols next (all directions) to them.
    //         Add them up. i.e. above that would exclude 114 and 33.
    // Part2 - The * symbol is special. It's a gear. So find all numbers adjacent
    //         to one of these. For gears with 2 numbers adjacent multiply them together
    //         for each gear and then sum these up.
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
            grid.add(&l, Space::Symbol(*b));
        }
    }

    // Checking for valid numbers. They can be any length so something
    // indicating we've started parsing and continue to see number vals.
    // Also a vec to accumulate digits into for later computing.
    // Finally a sentinel to indicate whether this number counts (no symbols nearby we don't care).
    let mut num_start = false;
    let mut num_digits = vec![];
    let mut has_symbol = false;
    let mut sum = 0;

    // Gear data. Have we seen a gear anytime during this number check?
    // If so temp_locs will have all gears that number saw and their
    // locations.
    let mut found_gear = false;
    let mut temp_locs: Vec<_> = vec![];

    // For each gear location keep a list of adjacent numbers. It'll get
    // filtered later for valid gears.
    let mut gear_locs = HashMap::new();

    for g in &grid {
        match g.1 {
            Space::None | Space::Symbol(_) => {
                if num_start && has_symbol {
                    sum += compute_num(&num_digits);
                }
                if num_start && found_gear {
                    let number = compute_num(&num_digits);
                    for f in &temp_locs {
                        let lookup = Location::clone(f);
                        gear_locs
                            .entry(lookup)
                            .and_modify(|m: &mut Vec<usize>| m.push(number))
                            .or_insert(vec![number]);
                    }
                }
                num_start = false;
            }
            Space::Digit(d) => {
                if !num_start {
                    has_symbol = false;
                    found_gear = false;
                    num_digits.clear();
                    temp_locs.clear();
                }
                num_start = true;
                num_digits.push(*d);
                if !has_symbol {
                    has_symbol = grid
                        .neighbors_all(&g.0)
                        .iter()
                        .any(|f| matches!(*f.1, Space::Symbol(_)));
                    for n in grid.neighbors_all(&g.0) {
                        if matches!(*n.1, Space::Symbol(b'*')) {
                            found_gear = true;
                            temp_locs.push(n.0.clone());
                        }
                    }
                }
            }
        };
    }

    // Take all the gear locations, filter down to ones with only 2 entries
    // and then for each of those accumulate multiplying both entries together.
    let sum2 = gear_locs
        .iter()
        .filter(|f| f.1.len() == 2)
        .fold(0, |acc, f| acc + f.1[0] * f.1[1]);
    println!("part1: {sum}");
    println!("part2: {sum2}");
    Ok(())
}

fn compute_num(num_digits: &Vec<usize>) -> usize {
    let l = num_digits.len() - 1;

    // The numbers look like this in the vec:
    // [4,5,6] == 456
    // So inverting against the location apply a power of 10 to that in order
    // to shift into the right position and accumulate each entry until we're done.
    num_digits.iter().enumerate().fold(0, |acc, f| {
        acc + *f.1 * 10_usize.pow((l - f.0).try_into().unwrap())
    })
}
