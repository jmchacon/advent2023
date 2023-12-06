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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

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
    let mut cur = "seed";
    let mut id;
    for s in &seeds {
        id = *s;
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
                if id < min {
                    min = id;
                }
                cur = "seed";
                break;
            }
        }
    }
    println!("part1 - {min}");
    Ok(())
}
