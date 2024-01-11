//! day14 advent 20XX
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

#[derive(Clone, Debug, Default, Display, PartialEq)]
enum Entry {
    #[default]
    Empty,
    Round,
    Cube,
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
        for (x, b) in line.as_bytes().iter().enumerate() {
            let x = isize::try_from(x).unwrap();
            let y = isize::try_from(line_num).unwrap();
            grid.add(
                &Location(x, y),
                match b {
                    b'.' => Entry::Empty,
                    b'#' => Entry::Cube,
                    b'O' => Entry::Round,
                    _ => panic!("Invalid line {line} on {}", line_num + 1),
                },
            );
        }
    }

    if args.debug {
        print_grid(&grid);
    }

    // Move all the Round entries as far forward (north) as we can. Just record
    // how many are in a line before they either stop at a cube or the bottom.
    // Then copy that many over to the new grid and copy the cubes as well
    // (technically we don't need the cubes for the problem but it makes it easier
    //  to visualize).
    let mut north_grid = Grid::<Entry>::new(grid.width(), grid.height());

    let mut start = Location(0, 0);
    let mut cur = Location(0, 0);
    let mut count = 0;
    loop {
        let mut col_done = false;
        match grid.get(&cur) {
            Entry::Empty => {
                if cur.1 + 1 == grid.height().try_into().unwrap() {
                    col_done = true;
                } else {
                    cur.1 += 1;
                }
            }
            Entry::Round => {
                count += 1;
                if cur.1 + 1 == grid.height().try_into().unwrap() {
                    col_done = true;
                } else {
                    cur.1 += 1;
                }
            }
            Entry::Cube => {
                for _ in 0..count {
                    north_grid.add(&start, Entry::Round);
                    start.1 += 1;
                }
                count = 0;
                north_grid.add(&cur, Entry::Cube);
                if cur.1 + 1 == grid.height().try_into().unwrap() {
                    // We're at the bottom so finish.
                    if cur.0 + 1 == grid.width().try_into().unwrap() {
                        break;
                    }
                    cur.0 += 1;
                    cur.1 = 0;
                    start = cur.clone();
                } else {
                    cur.1 += 1;
                    start = cur.clone();
                }
            }
        }
        if col_done {
            for _ in 0..count {
                north_grid.add(&start, Entry::Round);
                start.1 += 1;
            }
            // We're at the bottom so finish.
            if cur.0 + 1 == grid.width().try_into().unwrap() {
                break;
            }
            cur.0 += 1;
            cur.1 = 0;
            start = cur.clone();
            count = 0;
        }
    }

    if args.debug {
        print_grid(&north_grid);
    }

    // Compute the load.
    let mut sum = 0;
    let h = north_grid.height();
    for e in &north_grid {
        if *e.1 == Entry::Round {
            #[allow(clippy::cast_sign_loss)]
            let y = e.0 .1 as usize;
            sum += h - y;
        }
    }
    println!("part1: {sum}");
    Ok(())
}

fn print_grid(grid: &Grid<Entry>) {
    for g in grid {
        match g.1 {
            Entry::Empty => print!("."),
            Entry::Cube => print!("#"),
            Entry::Round => print!("O"),
        }
        if usize::try_from(g.0 .0).unwrap() == grid.width() - 1 {
            println!();
        }
    }
}
