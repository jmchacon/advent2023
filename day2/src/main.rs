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
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts[0] == "Game", "Invalid line {}: {line}", line_num + 1);
        let game = parts[1].trim_end_matches(':').parse::<usize>().unwrap();
        assert!(parts.len() % 2 == 0, "Uneven line {}: {line}", line_num + 1);
        let rem = parts[2..].join(" ");
        let balls = rem.split(';').collect::<Vec<_>>();

        let mut good = true;
        for b in &balls {
            let parts = b.split_whitespace().collect::<Vec<_>>();
            for i in (0..parts.len()).step_by(2) {
                let num = parts[i].parse::<usize>().unwrap();
                match parts[i + 1].trim_end_matches(',') {
                    "red" => {
                        if num > args.red {
                            good = false;
                        }
                    }
                    "blue" => {
                        if num > args.blue {
                            good = false;
                        }
                    }
                    "green" => {
                        if num > args.green {
                            good = false;
                        }
                    }
                    _ => panic!(),
                }
                if !good {
                    break;
                }
            }
            if !good {
                break;
            }
        }
        if good {
            total += game;
        }
    }
    println!("part1 - {total}");
    Ok(())
}
