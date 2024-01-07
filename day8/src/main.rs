//! day8 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
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

    let steps = lines[0].as_str();
    let mut conditions = HashMap::new();
    for line in &lines[2..] {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let left = parts[2].trim_end_matches(',').trim_start_matches('(');
        let right = parts[3].trim_end_matches(')');
        conditions.insert(parts[0], (left, right));
    }

    if conditions.contains_key("AAA") {
        let tot = find_total("AAA", &conditions, steps);
        println!("part1: {tot}");
    }

    // For part2 we need to find everything ending with "A" as a start position
    // and compute their path. Then the way to get them all together is to get the
    // LCM of all 6 numbers which is the fold at the end.
    //
    // It's not guarenteed LCM works here except if you check each path you'll
    // see it takes N steps to get to a Z and then the same N to return to that
    // Z. At that point LCM is valid for "when do all 6 paths meet at the same time?".
    // If LCM didn't work likely Chinese Remainder Therom could be used.
    let curs = conditions
        .keys()
        .filter(|f| f.ends_with('A'))
        .copied()
        .map(|f| find_total(f, &conditions, steps))
        .fold(1, num::integer::lcm);
    println!("part2: {curs}");
    Ok(())
}

fn find_total(start: &str, conditions: &HashMap<&str, (&str, &str)>, steps: &str) -> usize {
    let mut tot = 0;
    let mut cur = start;
    let mut done = false;
    loop {
        for s in steps.as_bytes() {
            match s {
                b'R' => {
                    cur = conditions[cur].1;
                }
                b'L' => {
                    cur = conditions[cur].0;
                }
                _ => panic!(),
            }
            tot += 1;
            if cur.ends_with('Z') {
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    tot
}
