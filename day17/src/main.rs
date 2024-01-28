//! day17 advent 20XX
use crate::Direction::{East, North, South, West};
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{print_grid, Grid, Location};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut grid = Grid::<u32>::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, b) in line.split("").enumerate() {
            if b.is_empty() {
                continue;
            }
            // X is off by one since the first entry is the blank from split().
            let x = x - 1;
            grid.add(
                &Location(x.try_into().unwrap(), y.try_into().unwrap()),
                b.parse::<u32>().unwrap(),
            );
        }
    }

    if args.debug {
        print_grid(&grid);
    }

    println!("part1: {}", run_grid(&grid, 0, 3, false));
    println!("part2: {}", run_grid(&grid, 4, 10, false));
    Ok(())
}

fn run_grid(grid: &Grid<u32>, min: u32, max: u32, debug: bool) -> u32 {
    let mut q = BinaryHeap::new();
    let begin = Location(0, 0);

    #[allow(clippy::cast_possible_wrap)]
    let max_x = (grid.width() - 1) as isize;
    #[allow(clippy::cast_possible_wrap)]
    let max_y = (grid.height() - 1) as isize;
    let end = Location(max_x, max_y);
    q.push(Reverse((0, (begin.clone(), &East), 0)));
    q.push(Reverse((0, (begin.clone(), &South), 0)));

    let mut seen = HashSet::new();

    while let Some(e) = q.pop() {
        if debug {
            println!("Testing {:?}", e.0);
        }
        let loc = &e.0 .1 .0;

        let dir = e.0 .1 .1;
        let cost = e.0 .0;
        let steps = e.0 .2;

        if *loc == end && steps >= min {
            return cost;
        }

        if seen.contains(&(loc.clone(), dir, steps)) {
            continue;
        }
        seen.insert((loc.clone(), dir, steps));

        for n in grid.neighbors(loc) {
            let mut count = e.0 .2;
            let newloc = &n.0;

            let mut newdir = &North;
            if newloc.0 == loc.0 + 1 {
                newdir = &East;
            }
            if newloc.0 == loc.0 - 1 {
                newdir = &West;
            }
            if newloc.1 == loc.1 + 1 {
                newdir = &South;
            }
            if newloc.1 == loc.1 - 1 {
                newdir = &North;
            }

            // Can't backup.
            match (dir, newdir) {
                (North, South) | (South, North) | (East, West) | (West, East) => continue,
                _ => {}
            };

            if dir == newdir {
                count += 1;
                if count > max {
                    // Can't go the same direction more than max times in a row.
                    continue;
                }
            } else {
                if count < min {
                    // Must go min steps before changing directions.
                    continue;
                }
                count = 1;
            }

            let new = (cost + grid.get(newloc), (newloc.clone(), newdir), count);
            if debug {
                println!("Pushing {new:?}");
            }
            q.push(Reverse(new));
        }
    }
    u32::MAX
}
