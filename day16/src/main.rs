//! day16 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::{collections::HashSet, fs::File};
use strum_macros::Display;

use Direction::{East, North, South, West};
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Default, Display, PartialEq)]
enum Entry {
    #[default]
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterUp,
    SplitterSide,
}

#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Default, Display, PartialEq)]
enum Energized {
    #[default]
    Empty,
    Entered(HashSet<Direction>),
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

    let mut grid = Grid::<Entry>::new(lines[0].len(), lines.len());
    for (line_num, line) in lines.iter().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            let e = match b {
                b'.' => Entry::Empty,
                b'|' => Entry::SplitterUp,
                b'-' => Entry::SplitterSide,
                b'/' => Entry::MirrorForward,
                b'\\' => Entry::MirrorBackward,
                _ => panic!("Invalid line {}: {line}", line_num + 1),
            };
            grid.add(
                &Location(x.try_into().unwrap(), line_num.try_into().unwrap()),
                e,
            );
        }
    }
    if args.debug {
        print_grid(&grid);
    }

    // For part1 we always start in the upper left facing east and then walk and count.
    let energized_grid_cnt = walk_grid(&grid, &Location(0, 0), &East, args.debug);
    println!("part1: {energized_grid_cnt}",);

    // For part2 we need to start on every outside location and each possible
    // direction that could use. i.e. more are one direction but each corner
    // has 2. Then find the max tiles energized after trying all of these.
    #[allow(clippy::cast_possible_wrap)]
    let max_x = grid.width() as isize - 1;
    #[allow(clippy::cast_possible_wrap)]
    let max_y = grid.height() as isize - 1;

    let mut choices = vec![
        (Location(0, 0), East),          // Upper left
        (Location(0, 0), South),         // Upper left
        (Location(0, max_y), East),      // Bottom left
        (Location(0, max_y), North),     // Bottom left
        (Location(max_x, 0), West),      // Upper right
        (Location(max_x, 0), South),     // Upper right
        (Location(max_x, max_y), West),  // Bottom right
        (Location(max_x, max_y), North), // Bottom right
    ];
    for x in 1..max_x {
        choices.push((Location(x, 0), South)); // Top row not corners
        choices.push((Location(x, max_y), North)); // Bottom row not corners
    }
    for y in 1..max_y {
        choices.push((Location(0, y), East)); // Left edge not corners.
        choices.push((Location(max_x, y), East)); // Right edge not corners.
    }

    let m = choices
        .iter()
        .map(|f| walk_grid(&grid, &f.0, &f.1, args.debug))
        .max()
        .unwrap();
    println!("part2: {m}");
    Ok(())
}

// Depending on initial facing more than one path may have to be initially
// evaluated or the resulting initial step turns before we start. Account for
// all those conditions here.
fn setup_initial_work(
    work: &mut Vec<(Location, Direction)>,
    grid: &Grid<Entry>,
    start: &Location,
    init_dir: &Direction,
) {
    // Even though we start facing one way the initial mirror may immediately repoint us so do that now
    // since the loop below will not do that.
    let dir = match init_dir {
        North => match grid.get(start) {
            Entry::Empty | Entry::SplitterUp => North,
            Entry::SplitterSide => {
                work.push((start.clone(), East));
                West
            }
            Entry::MirrorBackward => West,
            Entry::MirrorForward => East,
        },

        South => match grid.get(start) {
            Entry::Empty | Entry::SplitterUp => South,
            Entry::SplitterSide => {
                work.push((start.clone(), East));
                West
            }
            Entry::MirrorBackward => East,
            Entry::MirrorForward => West,
        },
        East => match grid.get(start) {
            Entry::Empty | Entry::SplitterSide => East,
            Entry::MirrorBackward => South,
            Entry::SplitterUp => {
                work.push((start.clone(), North));
                South
            }
            Entry::MirrorForward => North,
        },
        West => match grid.get(start) {
            Entry::Empty | Entry::SplitterSide => West,
            Entry::MirrorBackward => North,
            Entry::SplitterUp => {
                work.push((start.clone(), South));
                North
            }
            Entry::MirrorForward => South,
        },
    };

    work.push((start.clone(), dir));
}

fn walk_grid(grid: &Grid<Entry>, start: &Location, init_dir: &Direction, debug: bool) -> usize {
    let mut energized_grid = Grid::<Energized>::new(grid.width(), grid.height());
    let mut work = vec![];
    setup_initial_work(&mut work, grid, start, init_dir);

    #[allow(clippy::cast_possible_wrap)]
    let max_x = grid.width() as isize;
    #[allow(clippy::cast_possible_wrap)]
    let max_y = grid.height() as isize;

    // DFS the space and make sure to ignore paths we have looped back around onto.
    // i.e. one you enter a given location in a direction you never need to eval
    // that again. That's the short circuit that makes this workable in O(4N) time.
    // (you might have to visit every of the N squares 4 times due to each direction).
    while let Some(c) = work.pop() {
        if debug {
            println!("Processing: {c:?}");
        }
        match energized_grid.get_mut(&c.0) {
            Energized::Empty => energized_grid.add(
                &c.0,
                Energized::Entered(HashSet::<Direction>::from([c.1.clone()])),
            ),
            Energized::Entered(hs) => {
                if hs.contains(&c.1) {
                    // If we've already been here in this direction no need to replay.
                    continue;
                }
                hs.insert(c.1.clone());
            }
        }
        let next = match c.1 {
            North => {
                // Can't go off the top so this path ends.
                if c.0 .1 - 1 < 0 {
                    continue;
                }
                Location(c.0 .0, c.0 .1 - 1)
            }
            South => {
                // Can't go off the bottom so this path ends.
                if c.0 .1 + 1 >= max_y {
                    continue;
                }
                Location(c.0 .0, c.0 .1 + 1)
            }
            East => {
                // Can't go off the right edge so this path ends.
                if c.0 .0 + 1 >= max_x {
                    continue;
                }
                Location(c.0 .0 + 1, c.0 .1)
            }
            West => {
                // Can't go off the left edge so this path ends.
                if c.0 .0 - 1 < 0 {
                    continue;
                }
                Location(c.0 .0 - 1, c.0 .1)
            }
        };
        if debug {
            println!("next -> {next:?}");
        }
        match grid.get(&next) {
            // Empty we just keep moving along.
            Entry::Empty => work.push((next, c.1.clone())),
            Entry::MirrorForward => match c.1 {
                North => work.push((next, East)),
                South => work.push((next, West)),
                East => work.push((next, North)),
                West => work.push((next, South)),
            },
            Entry::MirrorBackward => match c.1 {
                North => work.push((next, West)),
                South => work.push((next, East)),
                East => work.push((next, South)),
                West => work.push((next, North)),
            },
            Entry::SplitterUp => {
                if c.1 == East || c.1 == West {
                    work.push((next.clone(), North));
                    work.push((next, South));
                } else {
                    work.push((next, c.1.clone()));
                }
            }
            Entry::SplitterSide => {
                if c.1 == North || c.1 == South {
                    work.push((next.clone(), East));
                    work.push((next, West));
                } else {
                    work.push((next, c.1.clone()));
                }
            }
        }
    }
    if debug {
        print_energized_grid(&energized_grid);
    }

    energized_grid
        .iter()
        .filter(|p| p.1 != &Energized::Empty)
        .count()
}

fn print_grid(grid: &Grid<Entry>) {
    for g in grid {
        match g.1 {
            Entry::Empty => print!("."),
            Entry::MirrorForward => print!("/"),
            Entry::MirrorBackward => print!("\\"),
            Entry::SplitterUp => print!("|"),
            Entry::SplitterSide => print!("-"),
        }
        if usize::try_from(g.0 .0).unwrap() == grid.width() - 1 {
            println!();
        }
    }
    println!();
}

fn print_energized_grid(grid: &Grid<Energized>) {
    for g in grid {
        match g.1 {
            Energized::Empty => print!("."),
            Energized::Entered(_) => print!("#"),
        }
        if usize::try_from(g.0 .0).unwrap() == grid.width() - 1 {
            println!();
        }
    }
    println!();
}
