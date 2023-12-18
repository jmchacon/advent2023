//! day10 advent 20XX
#[allow(clippy::enum_glob_use)]
use crate::Direction::*;
#[allow(clippy::enum_glob_use)]
use crate::Pipes::*;
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Clone, Debug, Default, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Pipes {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    #[default]
    Ground,
    Start,
}

#[derive(Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let allowed = HashMap::from([
        (Vertical, HashSet::from([North, South])),
        (Horizontal, HashSet::from([East, West])),
        (NEBend, HashSet::from([North, East])),
        (NWBend, HashSet::from([North, West])),
        (SWBend, HashSet::from([South, West])),
        (SEBend, HashSet::from([South, East])),
    ]);

    let mut grid = Grid::<Pipes>::new(lines[0].len(), lines.len());

    let mut start = Location(0, 0);
    for (line_num, line) in lines.iter().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            let x = isize::try_from(x).unwrap();
            let y = isize::try_from(line_num).unwrap();
            grid.add(
                &Location(x, y),
                match b {
                    b'.' => Ground,
                    b'-' => Horizontal,
                    b'|' => Vertical,
                    b'L' => NEBend,
                    b'J' => NWBend,
                    b'7' => SWBend,
                    b'F' => SEBend,
                    b'S' => {
                        start = Location(x, y);
                        Start
                    }
                    _ => panic!("Invalid line {line} on {}", line_num + 1),
                },
            );
        }
    }

    let mut start_valid = HashSet::new();
    for n in grid.neighbors(&start) {
        if n.1 == &Ground {
            continue;
        }
        // East
        if n.0 .0 - start.0 == 1 && allowed[n.1].contains(&West) {
            start_valid.insert(East);
        }
        // West
        if n.0 .0 - start.0 == -1 && allowed[n.1].contains(&East) {
            start_valid.insert(West);
        }
        // North
        if n.0 .1 - start.1 == -1 && allowed[n.1].contains(&South) {
            start_valid.insert(North);
        }
        // South
        if n.0 .1 - start.1 == 1 && allowed[n.1].contains(&North) {
            start_valid.insert(South);
        }
    }

    let start_pipe = allowed
        .iter()
        .find(|f| f.1 == &start_valid)
        .unwrap()
        .0
        .clone();
    if args.debug {
        println!("start: {start}");
        println!("grid:\n");
        print_grid(&grid);
        println!();
        println!("start_valid: {start_valid:?}");
        println!("start_pipe: {start_pipe}");
    }

    let mut cur = start.clone();
    let mut last = start.clone();
    let mut cur_symbol = start_pipe;
    let mut cnt = 1;
    let mut walk_grid = Grid::<Pipes>::new(lines[0].len(), lines.len());
    walk_grid.add(&cur, Start);
    loop {
        for c in &allowed[&cur_symbol] {
            let testloc = match c {
                North => Location(cur.0, cur.1 - 1),
                South => Location(cur.0, cur.1 + 1),
                East => Location(cur.0 + 1, cur.1),
                West => Location(cur.0 - 1, cur.1),
            };
            if testloc != last {
                cnt += 1;
                last = cur;
                cur_symbol = grid.get(&testloc).clone();
                cur = testloc;
                walk_grid.add(&cur, cur_symbol.clone());
                break;
            }
        }
        if cur == start {
            break;
        }
    }

    if args.debug {
        print_grid(&walk_grid);
    }
    println!("part1: {}", cnt / 2);

    Ok(())
}

fn print_grid(grid: &Grid<Pipes>) {
    for g in grid {
        match g.1 {
            Vertical => print!("|"),
            Horizontal => print!("-"),
            NEBend => print!("L"),
            NWBend => print!("J"),
            SWBend => print!("7"),
            SEBend => print!("F"),
            Ground => print!("."),
            Start => print!("S"),
        }
        if usize::try_from(g.0 .0).unwrap() == grid.width() - 1 {
            println!();
        }
    }
}
