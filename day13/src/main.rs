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

    // Run through the patterns and find the one without any changes.
    // Then do it again requiring a smudge fix.
    let mut sum = 0;
    let mut sum_part2 = 0;
    for p in &patterns {
        find_reflect(p, args.debug, false, &mut sum);
        find_reflect(p, args.debug, true, &mut sum_part2);
    }
    println!("part1: {sum}");
    println!("part2: {sum_part2}");
    Ok(())
}

// For a given grid try and find a mirror horizontally first, then flip it
// into a column rotation and try again. The input says we're guarenteed to find
// a solution in this.
fn find_reflect(p: &[Vec<i32>], debug: bool, find_smudge: bool, sum: &mut usize) {
    if debug {
        print_grid(p);
    }
    if let Some(r) = find_mirror(p, debug, find_smudge) {
        *sum += 100 * r;
        if debug {
            println!("Found mirror and {r} rows above.");
        }
    } else {
        let c = column_vec(p);
        if debug {
            print_grid(&c);
        }
        // Input claims we're guarenteed to find a reflection so if the
        // above didn't this must or something is broken..
        let r = find_mirror(&c, debug, find_smudge).unwrap();
        *sum += r;
        if debug {
            println!("Found mirror and {r} rows to the left.");
        }
    }
}

// For each row in a given grid walk from the 2nd row onwards comparing
// (via rows_equal so we can account for smudge correction). Each time
// we find a match loop expanding the rows out to match until we hit an edge.
// If we matched all the way back to the edge this is a valid mirror. Otherwise
// one didn't exist.
fn find_mirror(p: &[Vec<i32>], debug: bool, find_smudge: bool) -> Option<usize> {
    for pp in 1..p.len() {
        #[allow(clippy::cast_possible_wrap)]
        let mut right = pp as isize;
        let mut gap = 1_isize;
        let mut ret_find_smudge = find_smudge;
        loop {
            #[allow(clippy::cast_sign_loss)]
            let ret = rows_equal(
                &p[right as usize],
                &p[(right - gap) as usize],
                debug,
                ret_find_smudge,
            );
            if ret.0 {
                if debug {
                    println!(
                        "Found at {right} and {} with smudge: {find_smudge}",
                        right - gap
                    );
                }
                if ret.1 {
                    // Once we've corrected one place and gotten a further match we quit correcting for
                    // this iteration.
                    ret_find_smudge = false;
                }
                if right + 1 < p.len().try_into().unwrap() && ((right + 1) - (gap + 2)) >= 0 {
                    right += 1;
                    gap += 2;
                    if debug {
                        println!("Trying {right} and {}", right - gap);
                    }
                    continue;
                }

                // If we must find a smudge and we've matched completely but
                // never corrected (i.e. ret_find_smudge is still true) then
                // this isn't a match for part2.
                if find_smudge {
                    if ret_find_smudge {
                        break;
                    }
                    return Some(pp);
                }
                return Some(pp);
            }
            break;
        }
    }
    None
}

// For 2 rows check if they're equal. If not and we require finding a smudge
// correction start trying to change each entry for one row until this either
// does match or we can't find a match for this pair of rows.
//
// Returns (found, corrected) where found indicates if the rows matched and
// corrected indicates a smudge correction was used to perform this.
//
// NOTE: In real code this would be a struct as bool, bool is confusing to keep straight otherwise.
fn rows_equal(p1: &Vec<i32>, p2: &Vec<i32>, debug: bool, find_smudge: bool) -> (bool, bool) {
    if *p1 == *p2 {
        return (true, false);
    }
    if !find_smudge {
        return (false, false);
    }

    let mut smudge_p2 = p2.clone();
    for i in 0..smudge_p2.len() {
        let orig = smudge_p2[i];
        if orig == 0 {
            smudge_p2[i] = 1;
        } else {
            smudge_p2[i] = 0;
        }
        if smudge_p2 == *p1 {
            if debug {
                println!("Found smudge at position {i}");
            }
            return (true, true);
        }
        smudge_p2[i] = orig;
    }
    (false, false)
}

// Take a grid and rotate it clockwise 90 degrees and return a new grid.
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

// Print out the given grid for debugging.
fn print_grid(pattern: &[Vec<i32>]) {
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
