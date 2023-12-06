//! day6 advent 20XX
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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    assert!(lines[0].starts_with("Time:"), "Invalid first line");
    assert!(lines[1].starts_with("Distance:"), "Invalid second line");

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut ways = vec![];
    for (pos, t) in times.iter().enumerate() {
        let mut way = 0;
        // Skip first and last as their distance is always 0
        for i in 1..*t {
            let dist = (*t - i) * i;
            if dist > distances[pos] {
                way += 1;
            }
        }
        ways.push(way);
    }
    println!("part1 - {}", ways.iter().product::<usize>());
    let time = lines[0].split_whitespace().collect::<Vec<_>>()[1..]
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = lines[1].split_whitespace().collect::<Vec<_>>()[1..]
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut way = 0;
    // Skip first and last as their distance is always 0
    for i in 1..time {
        let dist = (time - i) * i;
        if dist > distance {
            way += 1;
        }
    }
    println!("part2 - {way}");

    Ok(())
}
