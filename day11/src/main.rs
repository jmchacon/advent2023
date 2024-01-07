//! day11 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use grid::Location;
use itertools::Itertools;
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
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut locs = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            match b {
                b'.' => {}
                b'#' => {
                    locs.push(Location(
                        isize::try_from(x).unwrap(),
                        isize::try_from(line_num).unwrap(),
                    ));
                }
                _ => panic!("bad line: {line} on {}", line_num + 1),
            }
        }
    }
    let cols = locs.iter().map(|f| f.1).collect::<HashSet<_>>();
    let rows = locs.iter().map(|f| f.0).collect::<HashSet<_>>();
    let mut empty_cols = vec![];
    let mut empty_rows = vec![];
    for i in 0..lines[0].len() {
        let x = isize::try_from(i).unwrap();
        if !cols.contains(&x) {
            empty_rows.push(x);
        }
    }
    for i in 0..lines.len() {
        let y = isize::try_from(i).unwrap();
        if !rows.contains(&y) {
            empty_cols.push(y);
        }
    }
    if args.debug {
        println!("empty_cols: {empty_cols:?}");
        println!("empty_rows: {empty_rows:?}");
    }

    // Take each original loc and move it along if needed.
    let mut adjusted_locs = vec![];
    let mut adjusted_big_locs = vec![];
    for l in &locs {
        let mut new = l.clone();
        let mut new_big = l.clone();
        for c in &empty_cols {
            if l.0 > *c {
                new.0 += 1;
                new_big.0 += 999_999;
            }
        }
        for r in &empty_rows {
            if l.1 > *r {
                new.1 += 1;
                new_big.1 += 999_999;
            }
        }
        adjusted_locs.push(new);
        adjusted_big_locs.push(new_big);
    }

    if args.debug {
        print_grid(lines[0].len(), lines.len(), &locs);
        println!();
        print_grid(
            lines[0].len() + empty_cols.len(),
            lines.len() + empty_rows.len(),
            &adjusted_locs,
        );
        println!();
        print_grid(
            lines[0].len() + 10 * empty_cols.len(),
            lines.len() + 10 * empty_rows.len(),
            &adjusted_big_locs,
        );
    }

    let sum = adjusted_locs
        .iter()
        .combinations(2)
        .fold(0, |acc, f| acc + f[0].distance(f[1]));
    let sum_big: u64 = adjusted_big_locs
        .iter()
        .combinations(2)
        .fold(0, |acc, f| acc + u64::from(f[0].distance(f[1])));
    println!("part1: {sum}");
    println!("part2: {sum_big}");
    Ok(())
}

fn print_grid(width: usize, height: usize, locs: &[Location]) {
    let c = locs.iter().collect::<HashSet<_>>();
    for y in 0..height {
        for x in 0..width {
            let x = isize::try_from(x).unwrap();
            let y = isize::try_from(y).unwrap();
            if c.contains(&Location(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
