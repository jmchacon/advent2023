//! day1 advent 20XX
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

    let mut sum = 0;
    let mut sum2 = 0;
    let matches = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let to_digits = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for line in &lines {
        if line.is_empty() {
            continue;
        }
        let digits_m = digits[1..]
            .iter()
            .map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let digits_min = digits_m
            .iter()
            .filter_map(|f| f.iter().min())
            .min()
            .unwrap_or(&(usize::MAX, ""));
        let digits_max = digits_m
            .iter()
            .filter_map(|f| f.iter().max())
            .max()
            .unwrap_or(&(usize::MIN, ""));
        let matches_m = matches[1..]
            .iter()
            .map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let matches_min = matches_m
            .iter()
            .filter_map(|f| f.iter().min())
            .min()
            .unwrap_or(&(usize::MAX, ""));
        let matches_max = matches_m
            .iter()
            .filter_map(|f| f.iter().max())
            .max()
            .unwrap_or(&(usize::MIN, ""));

        // If we didn't find any digits just skip sum since it only involved them.
        // If we didn't find alpha matches that's ok since the unwrap_or above
        // will handle that with sentinel values.
        if digits_min.0 != usize::MAX {
            sum += to_digits[digits_min.1] * 10 + to_digits[digits_max.1];
        }
        sum2 += if digits_min.0 < matches_min.0 {
            to_digits[digits_min.1] * 10
        } else {
            to_digits[matches_min.1] * 10
        } + if digits_max.0 >= matches_max.0 {
            to_digits[digits_max.1]
        } else {
            to_digits[matches_max.1]
        };
    }
    println!("part1 - {sum}");
    println!("part2 - {sum2}");
    Ok(())
}
