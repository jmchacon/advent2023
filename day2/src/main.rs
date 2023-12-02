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
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut total = 0;
    let mut total2 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts[0] == "Game", "Invalid line {}: {line}", line_num + 1);
        let game = parts[1].trim_end_matches(':').parse::<usize>().unwrap();
        assert!(parts.len() % 2 == 0, "Uneven line {}: {line}", line_num + 1);
        let rem = parts[2..].join(" ");
        let balls = rem.split(';').collect::<Vec<_>>();

        let mut good = true;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for b in &balls {
            let ball_parts = b.split_whitespace().collect::<Vec<_>>();
            for i in (0..ball_parts.len()).step_by(2) {
                let num = ball_parts[i].parse::<usize>().unwrap();
                match ball_parts[i + 1].trim_end_matches(',') {
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
        if good {
            total += game;
        }
        total2 += max_red * max_blue * max_green;
    }
    println!("part1 - {total}");
    println!("part2 - {total2}");
    Ok(())
}
