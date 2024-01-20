//! day14 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use core::fmt;
use grid::{print_grid, Grid, Location};
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

#[derive(Clone, Debug, Default, PartialEq)]
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

    let sum = compute_load(&north_grid(&grid));
    println!("part1: {sum}");

    // For part2 we actually need to cycle a billion times.
    // On the 10x10 this is actually doable and can get an answer in about
    // 10m with just brute force. But the real input is 100x100 which makes
    // simply processing one much larger.
    //
    // Instead we'll use a few assumptions:
    //
    // 1. The operations repeat and due to the closed system implies we
    //    eventually reach a state of repetition.
    // 2. There is always eventually a cycle but may take some iterations
    //    to begin.
    //
    // Let's try 1000 iterations to find the cycle starting at entry 200.
    // If it can't find one after 100 entries it can shift over by 1 and keep
    // looking until it runs out. After 1000 we give up since it really should
    // be repeating by now.
    let mut grid1 = grid.clone();
    let mut loads = vec![];
    for _ in 0..1000 {
        grid1 = do_cycle(&grid1, false);
        let load = compute_load(&grid1);

        loads.push(load);
    }

    let mut start = 200;
    let mut size = 2;
    loop {
        assert!(!start > 900, "Can't find a loop after checking 200-900");

        // Check the current run against the next N to see if they all match.
        // Then do it again with the next one and the sequence after. This way
        // we can assume it's actually repeating at this point.
        // This avoids a short sequence of
        // 1 2 1 2 3 4
        // possibly tripping it up. Yes it would fail to 1 2 1 2 1 2 for instance
        // but in general the sequences don't go up and down. A new repetition will
        // generally increase at the start but then decrease until it repeats.
        if loads[start..start + size] == loads[start + size..start + size * 2]
            && loads[start + size..start + size * 2] == loads[start + size * 2..start + size * 3]
        {
            break;
        }
        size += 1;
        if size == 100 {
            start += 1;
            size = 2;
        }
    }
    let idx = start + (1_000_000_000 - start - 1) % size;
    if args.debug {
        println!("found a loop starting at {start} of size {size}");
        println!("using index {idx}");
    }
    println!("part2: {}", loads[idx]);
    Ok(())
}

fn do_cycle(grid: &Grid<Entry>, debug: bool) -> Grid<Entry> {
    let north_grid = north_grid(grid);
    if debug {
        print_grid(&north_grid);
        println!("north: {}\n", compute_load(&north_grid));
    }

    let west_grid = west_grid(&north_grid);
    if debug {
        print_grid(&west_grid);
        println!("west: {}\n", compute_load(&west_grid));
    }

    let south_grid = south_grid(&west_grid);
    if debug {
        print_grid(&south_grid);
        println!("south: {}\n", compute_load(&south_grid));
    }

    let east_grid = east_grid(&south_grid);
    if debug {
        print_grid(&east_grid);
        println!("east: {}\n", compute_load(&east_grid));
    }

    east_grid
}

fn compute_load(grid: &Grid<Entry>) -> usize {
    // Compute the load.
    let mut sum = 0;
    let h = grid.height();
    for e in grid {
        if *e.1 == Entry::Round {
            #[allow(clippy::cast_sign_loss)]
            let y = e.0 .1 as usize;
            sum += h - y;
        }
    }
    sum
}

fn north_grid(grid: &Grid<Entry>) -> Grid<Entry> {
    // Move all the Round entries as far forward (north) as we can. Just record
    // how many are in a line before they either stop at a cube or the bottom.
    // Then copy that many over to the new grid and copy the cubes as well
    // (technically we don't need the cubes for the problem but it makes it easier
    //  to visualize).
    let mut north_grid = Grid::<Entry>::new(grid.width(), grid.height());

    let x_max: isize = grid.width().try_into().unwrap();
    let y_max: isize = grid.height().try_into().unwrap();

    let mut start = Location(0, 0);
    let mut cur = start.clone();
    let mut count = 0;
    loop {
        let mut col_done = false;
        match grid.get(&cur) {
            Entry::Empty => {
                if cur.1 + 1 == y_max {
                    col_done = true;
                } else {
                    cur.1 += 1;
                }
            }
            Entry::Round => {
                count += 1;
                if cur.1 + 1 == y_max {
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
                if cur.1 + 1 == y_max {
                    // We're at the right bottom so finish.
                    if cur.0 + 1 == x_max {
                        break;
                    }
                    cur.0 += 1;
                    cur.1 = 0;
                } else {
                    cur.1 += 1;
                }
                start = cur.clone();
            }
        }
        if col_done {
            for _ in 0..count {
                north_grid.add(&start, Entry::Round);
                start.1 += 1;
            }
            // We're at the right bottom so finish.
            if cur.0 + 1 == x_max {
                break;
            }
            cur.0 += 1;
            cur.1 = 0;
            start = cur.clone();
            count = 0;
        }
    }
    north_grid
}

fn south_grid(grid: &Grid<Entry>) -> Grid<Entry> {
    // Move all the Round entries as far down (south) as we can. Just record
    // how many are in a line before they either stop at a cube or the bottom.
    // Then copy that many over to the new grid and copy the cubes as well
    // (technically we don't need the cubes for the problem but it makes it easier
    //  to visualize).
    let mut south_grid = Grid::<Entry>::new(grid.width(), grid.height());

    let start_y = (grid.height() - 1).try_into().unwrap();
    let x_max: isize = grid.width().try_into().unwrap();
    let mut start = Location(0, start_y);
    let mut cur = start.clone();
    let mut count = 0;
    loop {
        let mut col_done = false;
        match grid.get(&cur) {
            Entry::Empty => {
                if cur.1 - 1 < 0 {
                    col_done = true;
                } else {
                    cur.1 -= 1;
                }
            }
            Entry::Round => {
                count += 1;
                if cur.1 - 1 < 0 {
                    col_done = true;
                } else {
                    cur.1 -= 1;
                }
            }
            Entry::Cube => {
                for _ in 0..count {
                    south_grid.add(&start, Entry::Round);
                    start.1 -= 1;
                }
                count = 0;
                south_grid.add(&cur, Entry::Cube);
                if cur.1 - 1 < 0 {
                    // We're at the right top so finish.
                    if cur.0 + 1 == x_max {
                        break;
                    }
                    cur.0 += 1;
                    cur.1 = start_y;
                } else {
                    cur.1 -= 1;
                }
                start = cur.clone();
            }
        }
        if col_done {
            for _ in 0..count {
                south_grid.add(&start, Entry::Round);
                start.1 -= 1;
            }
            // We're at the right top so finish.
            if cur.0 + 1 == x_max {
                break;
            }
            cur.0 += 1;
            cur.1 = start_y;
            start = cur.clone();
            count = 0;
        }
    }
    south_grid
}

fn east_grid(grid: &Grid<Entry>) -> Grid<Entry> {
    // Move all the Round entries as far right (east) as we can. Just record
    // how many are in a line before they either stop at a cube or the bottom.
    // Then copy that many over to the new grid and copy the cubes as well
    // (technically we don't need the cubes for the problem but it makes it easier
    //  to visualize).
    let mut east_grid = Grid::<Entry>::new(grid.width(), grid.height());

    let start_x = (grid.width() - 1).try_into().unwrap();
    let y_max: isize = grid.height().try_into().unwrap();

    let mut start = Location(start_x, 0);
    let mut cur = start.clone();
    let mut count = 0;
    loop {
        let mut row_done = false;
        match grid.get(&cur) {
            Entry::Empty => {
                if cur.0 - 1 < 0 {
                    row_done = true;
                } else {
                    cur.0 -= 1;
                }
            }
            Entry::Round => {
                count += 1;
                if cur.0 - 1 < 0 {
                    row_done = true;
                } else {
                    cur.0 -= 1;
                }
            }
            Entry::Cube => {
                for _ in 0..count {
                    east_grid.add(&start, Entry::Round);
                    start.0 -= 1;
                }
                count = 0;
                east_grid.add(&cur, Entry::Cube);
                if cur.0 - 1 < 0 {
                    // We're at the left bottom so finish.
                    if cur.1 + 1 == y_max {
                        break;
                    }
                    cur.0 = start_x;
                    cur.1 += 1;
                } else {
                    cur.0 -= 1;
                }
                start = cur.clone();
            }
        }
        if row_done {
            for _ in 0..count {
                east_grid.add(&start, Entry::Round);
                start.0 -= 1;
            }
            // We're at the bottom so finish.
            if cur.1 + 1 == y_max {
                break;
            }
            cur.0 += start_x;
            cur.1 += 1;
            start = cur.clone();
            count = 0;
        }
    }
    east_grid
}

fn west_grid(grid: &Grid<Entry>) -> Grid<Entry> {
    // Move all the Round entries as far left (west) as we can. Just record
    // how many are in a line before they either stop at a cube or the bottom.
    // Then copy that many over to the new grid and copy the cubes as well
    // (technically we don't need the cubes for the problem but it makes it easier
    //  to visualize).
    let mut west_grid = Grid::<Entry>::new(grid.width(), grid.height());

    let x_max: isize = grid.width().try_into().unwrap();
    let y_max: isize = grid.height().try_into().unwrap();

    let mut start = Location(0, 0);
    let mut cur = start.clone();
    let mut count = 0;
    loop {
        let mut row_done = false;
        match grid.get(&cur) {
            Entry::Empty => {
                if cur.0 + 1 == x_max {
                    row_done = true;
                } else {
                    cur.0 += 1;
                }
            }
            Entry::Round => {
                count += 1;
                if cur.0 + 1 == x_max {
                    row_done = true;
                } else {
                    cur.0 += 1;
                }
            }
            Entry::Cube => {
                for _ in 0..count {
                    west_grid.add(&start, Entry::Round);
                    start.0 += 1;
                }
                count = 0;
                west_grid.add(&cur, Entry::Cube);
                if cur.0 + 1 == x_max {
                    // We're at the left bottom so finish.
                    if cur.1 + 1 == y_max {
                        break;
                    }
                    cur.0 = 0;
                    cur.1 += 1;
                } else {
                    cur.0 += 1;
                }
                start = cur.clone();
            }
        }
        if row_done {
            for _ in 0..count {
                west_grid.add(&start, Entry::Round);
                start.0 += 1;
            }
            // We're at the bottom so finish.
            if cur.1 + 1 == y_max {
                break;
            }
            cur.0 = 0;
            cur.1 += 1;
            start = cur.clone();
            count = 0;
        }
    }
    west_grid
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Empty => write!(f, "."),
            Entry::Cube => write!(f, "#"),
            Entry::Round => write!(f, "O"),
        }
    }
}
