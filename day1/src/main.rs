//! day1 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::zip;
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
        let digits_m = digits
            .iter()
            .map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // Do the same with the alpha matches.
        let full_m = matches
            .iter()
            .map(|f| line.match_indices(f).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Compute the alpha min by zipping together the digits matches and the alpha matches.
        // Then as we will get each sub vector as pairs merge those together and take the min
        // of that but use filter_map to eliminate all the None's and unwrap implicitly. At this
        // point index doesn't matter since the match_indicies filled in the string value so we
        // can use the map to look that up later.
        // After we have the reduced set take another min() on that to get the final value and then
        // unwrap it. This can't be None if the string is proper (has at least 1 number or 1 alpha case).
        // If not unwrap panic is fine here.
        let full_min = zip(digits_m.clone(), full_m.clone())
            .filter_map(|f| itertools::merge(f.0, f.1).min())
            .min()
            .unwrap();
        // Do the same as above but get the max. Due to the way min/max work can't avoid 2 iterations like this.
        let full_max = zip(digits_m.clone(), full_m.clone())
            .filter_map(|f| itertools::merge(f.0, f.1).max())
            .max()
            .unwrap();

        // Digits are generally the same except we need unwrap_or() instead since we may have a string
        // of only alpha. In that case a sentinel just means the part1 case below gets skipped.
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

        // If we didn't find any digits just skip sum since it only involved them.
        if digits_min.0 != usize::MAX {
            sum += to_digits[digits_min.1] * 10 + to_digits[digits_max.1];
        }
        sum2 += to_digits[full_min.1] * 10 + to_digits[full_max.1];
    }
    println!("part1 - {sum}");
    println!("part2 - {sum2}");
    Ok(())
}
