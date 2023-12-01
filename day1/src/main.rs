//! day1 advent 20XX
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

    let mut sum = 0;
    let mut sum2 = 0;
    let matches = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in &lines {
        let mut val = 0;
        let mut val2 = 0;
        for b in line.as_bytes() {
            match b {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let t = std::str::from_utf8(&[*b])
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    val = 10 * t;
                    break;
                }
                _ => {}
            }
        }
        let mut loc = 0;
        for (l, b) in line.as_bytes().iter().enumerate() {
            match b {
                b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let t = std::str::from_utf8(&[*b])
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    val = 10 * t;
                    loc = l;
                    break;
                }
                _ => {}
            }
        }
        // We know the index for the first digit. Now run through matches and
        // find the first index for any of those.
        let mut mm = matches
            .iter()
            .map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if val != 0 {
            mm[(val / 10) - 1].push((loc, ""));
        }

        let mut min = usize::MAX;
        for (num, vals) in mm.iter().enumerate() {
            for v in vals {
                if v.0 < min {
                    min = v.0;
                    val2 = 10 * (num + 1);
                }
            }
        }
        let mut sec = 0;
        let mut found = false;
        for (l, b) in line.as_bytes().iter().rev().enumerate() {
            match b {
                b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let t = std::str::from_utf8(&[*b])
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    sec = t;
                    loc = line.as_bytes().len() - l - 1;
                    found = true;
                    break;
                }
                _ => {}
            }
        }
        val += sec;
        if found {
            mm[sec - 1].push((loc, ""));
        }
        min = 0;
        let mut t = 0;
        for (num, vals) in mm.iter().enumerate() {
            for v in vals {
                if v.0 >= min {
                    min = v.0;
                    t = num + 1;
                }
            }
        }
        val2 += t;

        sum += val;
        sum2 += val2;
    }
    println!("part1 - {sum}");
    println!("part2 - {sum2}");
    Ok(())
}
