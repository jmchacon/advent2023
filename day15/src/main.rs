//! day15 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
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

#[derive(Clone, Debug, Display, PartialEq)]
enum Op {
    Dash,
    Equals(usize),
}

#[derive(Clone, Debug)]
struct Label<'a>(&'a str, usize);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    // For this we can assume there's one line.
    let parts = lines[0].split(',').collect::<Vec<_>>();

    // For part1 compute a hash of the whole string. Then sum them up.
    let mut sum = 0;
    for p in parts.iter().copied() {
        sum += hash(p);
    }
    println!("part1: {sum}");

    // Part2 is a bit more complicated.
    // We only want to hash until we find a '-' or '=' sign.
    // We're going to track 256 buckets where each one contains N length
    // array (in order of insertion/update) which contains the string we hashed
    // along with whatever number is after the equals.
    // The bucket is the hash value of this new hash.
    //
    // For dash if the entry exists simply remove it from the list and shift over.
    // For equal we either add it (if it doesn't exist) or replace the value for
    // the existing entry in place.
    //
    // NOTE: This assumes small enough vectors that linear search here doesn't
    //       matter and we'll just user iter().find() or retain() to do updates.
    let mut boxes: Vec<Vec<Label>> = vec![vec![]; 256];
    for p in parts.iter().copied() {
        let op = hash2(p);
        if args.debug {
            println!("{p} -> {:?}", hash2(p));
        }
        match op.2 {
            Op::Dash => {
                // Keep anything which doesn't match the label we hashed.
                // NOTE: This is potentially expensive if we do a lot of these
                //       as it shifts the vector everytime.
                boxes[op.0].retain(|b| b.0 != op.1);
            }
            Op::Equals(v) => {
                // If we already have an entry update it via a mutable find.
                // Otherwise just add it onto the end.
                if let Some(f) = boxes[op.0].iter_mut().find(|p| p.0 == op.1) {
                    f.1 = v;
                } else {
                    boxes[op.0].push(Label(op.1, v));
                }
            }
        }
        if args.debug {
            print_boxes(&boxes);
            println!();
        }
    }

    let mut part2_sum = 0;
    for (pos, b) in boxes.iter().enumerate() {
        for (lenspos, l) in b.iter().enumerate() {
            part2_sum += (pos + 1) * (lenspos + 1) * l.1;
        }
    }
    println!("part2: {part2_sum}");
    Ok(())
}

fn print_boxes(boxes: &[Vec<Label>]) {
    for (pos, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            println!("Box {pos}: {b:?}");
        }
    }
}

fn hash(p: &str) -> usize {
    let mut val = 0;
    for b in p.bytes() {
        val = hash_logic(val, b);
    }
    val
}

fn hash_logic(val: usize, b: u8) -> usize {
    ((val + usize::from(b)) * 17) % 256
}

fn hash2(p: &str) -> (usize, &str, Op) {
    let mut val = 0;
    for (pos, b) in p.bytes().enumerate() {
        match b {
            b'-' => return (val, &p[0..pos], Op::Dash),
            b'=' => {
                let num = p[pos + 1..].parse::<usize>().unwrap();
                return (val, &p[0..pos], Op::Equals(num));
            }
            _ => {
                val = hash_logic(val, b);
            }
        }
    }
    panic!("Invalid string {p}");
}
