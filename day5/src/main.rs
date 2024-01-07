//! day5 advent 20XX
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

    let mut maps = HashMap::new();
    let mut map = vec![];
    let mut src = "";
    let mut dest = "";

    // Just parse seeds direct and then start parsing.
    assert!(
        lines[0].starts_with("seeds:"),
        "1st line invalid. Need 'seeds: '"
    );
    assert!(lines[1].is_empty(), "line 2 isn't blank");
    let seeds = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for (line_num, line) in lines[2..].iter().enumerate() {
        if line.is_empty() {
            maps.insert(src, (dest, map));
            map = vec![];
            src = "";
            continue;
        }

        let parts = line.split_whitespace().collect::<Vec<_>>();
        if src.is_empty() {
            assert!(parts[1] == "map:", "Not a proper map line {}", line_num + 3);
            let sub = parts[0].split('-').collect::<Vec<_>>();
            src = sub[0];
            dest = sub[2];
            continue;
        }

        // Anything is parsing the current map
        assert!(parts.len() == 3, "Invalid data line {}", line_num + 3);
        map.push(
            parts
                .iter()
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    if !src.is_empty() {
        maps.insert(src, (dest, map));
    }

    let mut min = usize::MAX;
    for s in &seeds {
        let new_min = find_min(*s, &maps);
        if new_min < min {
            min = new_min;
        }
    }
    println!("part1 - {min}");

    let mut part2_min = usize::MAX;
    for i in (0..seeds.len()).step_by(2) {
        let seed_start = seeds[i];
        let seed_rng = seeds[i + 1];
        for s in seed_start..seed_start + seed_rng {
            let new_min = find_min(s, &maps);
            if new_min < part2_min {
                part2_min = new_min;
            }
        }
    }
    println!("part2 - {part2_min}");
    Ok(())
}

fn find_min(seed: usize, maps: &HashMap<&str, (&str, Vec<Vec<usize>>)>) -> usize {
    let mut cur = "seed";

    let mut id = seed;
    loop {
        let m = &maps[cur];

        for r in &m.1 {
            let dest = r[0];
            let src = r[1];
            let rng = r[2];
            if id >= src && id < src + rng {
                id = id - src + dest;
                break;
            }
        }
        cur = m.0;
        if !maps.contains_key(cur) {
            return id;
        }
    }
}
