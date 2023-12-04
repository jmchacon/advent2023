//! day4 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::{HashMap, HashSet};
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

    // Input has the form:
    //
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // ..
    //
    // The card number isn't important (it's the line number and the input is in order).
    // The numbers to the left are the winning numbers and the numbers after the |
    // are the choices.
    //
    // Part1 - Find how many choices are winners and then compute the score here
    //         as 2^(matches -1). i.e. it starts at 1 and just doubles for each
    //         additional match. Find the sum of this.
    //
    // Part2 - For the match count generate N more cards. 1 more each for the next N
    //         cards (up to the end of the list). Keep adding them on as you
    //         progress through each card. Each card processed also counts for 1.
    //         At the end sum up the total number of cards generated.
    let mut sum = 0;
    let mut card_count = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let mut winners = HashSet::new();
        let mut found = 0;
        let mut parse_done = false;
        for p in &parts[2..] {
            if *p == "|" {
                parse_done = true;
                continue;
            }
            let number = p.parse::<usize>().unwrap();
            if parse_done {
                if winners.contains(&number) {
                    found += 1;
                }
            } else {
                winners.insert(number);
            }
        }
        // The current line always gets an entry but do this *after* we pull
        // out the repeat count below so we don't add any extras.
        let cur = line_num + 1;
        let repeat = *card_count.get(&cur).unwrap_or(&0);
        card_count.entry(cur).and_modify(|f| *f += 1).or_insert(1);

        // Now if this card found something add to the sum for part1 then loop
        // and add more entries in the map for the following games that get
        // additional copies.
        if found > 0 {
            sum += 2_usize.pow(found - 1);
            for _ in 0..=repeat {
                for i in (line_num + 2)..(line_num + 2 + found as usize) {
                    if i <= lines.len() {
                        card_count.entry(i).and_modify(|f| *f += 1).or_insert(1);
                    }
                }
            }
        }
    }
    println!("part1 - {sum}");
    println!("part2 - {}", card_count.values().sum::<i32>());
    Ok(())
}
