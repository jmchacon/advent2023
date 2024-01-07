//! day12 advent 20XX
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
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut patterns = vec![];
    let mut pattern = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        if line.is_empty() {
            patterns.push(pattern);
            pattern = vec![];
            continue;
        }
        let mut row = vec![];
        for b in line.as_bytes() {
            row.push(match b {
                b'#' => 1,
                b'.' => 0,
                _ => panic!("bad line {line} at {}", line_num + 1),
            });
        }
        pattern.push(row);
    }
    patterns.push(pattern);

    let mut sum = 0;
    for p in &patterns {
        if args.debug {
            print_grid(p);
        }
        if let Some(r) = find_mirror(p, args.debug) {
            sum += 100 * r;
            if args.debug {
                println!("Found mirror and {r} rows above.");
            }
        } else {
            let c = column_vec(p);
            if args.debug {
                print_grid(&c);
            }
            let r = find_mirror(&c, args.debug).unwrap();
            sum += r;
            if args.debug {
                println!("Found mirror and {r} rows to the left.");
            }
        }
    }
    println!("part1: {sum}");
    Ok(())
}

fn find_mirror(p: &[Vec<i32>], debug: bool) -> Option<usize> {
    for pp in 1..p.len() {
        #[allow(clippy::cast_possible_wrap)]
        let mut right = pp as isize;
        let mut gap = 1_isize;
        loop {
            #[allow(clippy::cast_sign_loss)]
            if p[right as usize] == p[(right - gap) as usize] {
                if debug {
                    println!("Found at {right} and {}", right - gap);
                }
                if right + 1 < p.len().try_into().unwrap() && ((right + 1) - (gap + 2)) >= 0 {
                    right += 1;
                    gap += 2;
                    if debug {
                        println!("Trying {right} and {}", right - gap);
                    }
                    continue;
                }
                return Some(pp);
            }
            break;
        }
    }
    None
}

fn column_vec(pattern: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // Pre-reserve capacity in each so we can just iterate and place.
    let mut ret = vec![vec![0; pattern.len()]; pattern[0].len()];

    for y in 0..pattern.len() {
        let retx = ret[0].len() - 1 - y;
        #[allow(clippy::needless_range_loop)]
        for x in 0..pattern[0].len() {
            ret[x][retx] = pattern[y][x];
        }
    }
    ret
}

fn print_grid(pattern: &Vec<Vec<i32>>) {
    for y in pattern {
        for x in y {
            if *x == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}
