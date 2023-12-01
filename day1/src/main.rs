//! day1 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
use itertools::Itertools;
use slab_tree::tree::TreeBuilder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

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
    for (line_num, line) in lines.iter().enumerate() {
        let mut val = 0;
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

        for b in line.as_bytes().iter().rev() {
            match b {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    let t = std::str::from_utf8(&[*b])
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    val += t;
                    break;
                }
                _ => {}
            }
        }
        sum += val;
    }
    println!("part1 - {sum}");
    Ok(())
}
