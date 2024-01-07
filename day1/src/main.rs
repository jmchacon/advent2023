//! day1 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::merge;
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

    // Input looks like:
    //
    // 4nineeightseven2
    // 1abc2
    //
    // For part1 just find the left most digit and right most digit and make
    // a 2 digit number out of them. For a single digit case like:
    //
    // two1nine
    //
    // It would compute to 11 then. Sum up all the numbers.
    //
    // For part2 spelled out digits are now possible and again we want the
    // left most and right most ones. So the example above instead of 11 actually
    // becomes 29. Again sum these up.

    let mut sum = 0;
    let mut sum2 = 0;
    // Make 2 different matches arrays since we have to do digits by themselves
    // before combining with alpha style.
    let matches = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    // A quick map of the string -> numeric so we don't have to add
    // X.parse::<usize>().unwrap() everywhere.
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
        // Skip empty lines in case the file got extra somehow.
        if line.is_empty() {
            continue;
        }

        // Run over the digits and map to all the places each one matches in the string.
        // Flatten it all down as the digit matches comes back in the 2nd piece
        // of the match_indices tuple and the first piece of the tuple is the index
        // which we can use to min/max once flattened.
        let digits_m = digits
            .iter()
            .flat_map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // Do the same with the alpha matches. Could inline below but pulled into
        // it's own var for readability.
        let alpha_m = matches
            .iter()
            .flat_map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Create a combined vec of the digits/alpha where they are paired up correctly.
        // Flatten it back again so we can min/max on it easily.
        let combined = merge(digits_m.clone(), alpha_m).collect::<Vec<_>>();

        if digits_m.iter().min().is_some() {
            sum += to_digits[digits_m.iter().min().unwrap().1] * 10
                + to_digits[digits_m.iter().max().unwrap().1];
        }
        sum2 += to_digits[combined.iter().min().unwrap().1] * 10
            + to_digits[combined.iter().max().unwrap().1];
    }
    println!("part1 - {sum}");
    println!("part2 - {sum2}");
    Ok(())
}
