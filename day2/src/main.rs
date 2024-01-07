//! day2 advent 20XX
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

    #[arg(long, default_value_t = 12)]
    red: usize,

    #[arg(long, default_value_t = 13)]
    green: usize,

    #[arg(long, default_value_t = 14)]
    blue: usize,
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
    // Game X: A blue, B red; A red, B green, C blue; A green
    //
    // i.e. a game with a number then a variable number of ball groups where
    // each group is 1-3 colors (green, blue, red) and a number. Groups separated
    // by ; and possibly colors have trailing , after them.
    //
    // Part1 - Parse line and record game numbers based on max balls in bag
    //         as to whether that line is valid or not. Add up game numbers for
    //         valid games.
    // Part2 - Reverse things. Find the minimum number of balls needed in the bag
    //         to make that game valid. Multiply those 3 numbers together and make
    //         a sum of them.
    let mut total = 0;
    let mut total2 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        // Basic line validation.
        assert!(parts[0] == "Game", "Invalid line {}: {line}", line_num + 1);
        assert!(parts.len() % 2 == 0, "Uneven line {}: {line}", line_num + 1);

        // Find the game number.
        let game = parts[1].trim_end_matches(':').parse::<usize>().unwrap();

        // Put the line back together and then split it back on ; to get ball chunks.
        let rem = parts[2..].join(" ");
        let balls = rem.split(';').collect::<Vec<_>>();

        let mut good = true;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for b in &balls {
            // For each section split again on whitespace to get tokens.
            let ball_parts = b.split_whitespace().collect::<Vec<_>>();

            // Each token is 2 parts. A number and a color (with optional trailing ,)
            // So walk in steps to make this easier. We already validated above this
            // was even so it's ok.
            for i in (0..ball_parts.len()).step_by(2) {
                // The number is easy. Just parse it.
                let num = ball_parts[i].parse::<usize>().unwrap();

                // For the color have to strip off a possible trailing , and
                // then check the color.
                match ball_parts[i + 1].trim_end_matches(',') {
                    // Only documenting here as all colors are the same.
                    // If the number for this color is greater than the max
                    // in the bag (from args) then it's a bad row for part1.
                    // If we found a new max size record that for this color.
                    "red" => {
                        if num > args.red {
                            good = false;
                        }
                        if num > max_red {
                            max_red = num;
                        }
                    }
                    "blue" => {
                        if num > args.blue {
                            good = false;
                        }
                        if num > max_blue {
                            max_blue = num;
                        }
                    }
                    "green" => {
                        if num > args.green {
                            good = false;
                        }
                        if num > max_green {
                            max_green = num;
                        }
                    }
                    _ => panic!(),
                }
            }
        }
        // If the line was good part1 gets added up.
        if good {
            total += game;
        }
        // Part2 always happens. If there's a line that never put out a color
        // this will just reduce to 0 since max's didn't grow.
        total2 += max_red * max_blue * max_green;
    }
    println!("part1 - {total}");
    println!("part2 - {total2}");
    Ok(())
}
