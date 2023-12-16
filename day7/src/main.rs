//! day7 advent 20XX
use clap::Parser;
use color_eyre::eyre::Result;
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

#[derive(Clone, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Display, Eq, Ord, PartialEq, PartialOrd)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hands = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(
            parts.len() == 2 && parts[0].len() == 5,
            "Invalid line {} - {line}",
            line_num + 1
        );
        let mut cards = vec![];
        let mut counts = HashMap::new();
        let bid = parts[1].parse::<usize>().unwrap();
        for c in parts[0].as_bytes() {
            let card = match c {
                b'2' => Card::Number(2),
                b'3' => Card::Number(3),
                b'4' => Card::Number(4),
                b'5' => Card::Number(5),
                b'6' => Card::Number(6),
                b'7' => Card::Number(7),
                b'8' => Card::Number(8),
                b'9' => Card::Number(9),
                b'T' => Card::Number(10),
                b'J' => Card::Jack,
                b'Q' => Card::Queen,
                b'K' => Card::King,
                b'A' => Card::Ace,
                _ => panic!(),
            };
            counts
                .entry(card.clone())
                .and_modify(|f| *f += 1)
                .or_insert(1);
            cards.push(card);
        }
        // We don't move cards around so use the counts to determine type of hand.
        match counts.len() {
            1 => hands.push((Hand::FiveOfAKind(cards), bid)),
            2 => {
                // Either AAAAx or AAAKK so if one count is 4 we know 4 of a kind.
                if *counts.values().max().unwrap() == 4 {
                    hands.push((Hand::FourOfAKind(cards), bid));
                } else {
                    hands.push((Hand::FullHouse(cards), bid));
                }
            }
            3 => {
                // Either AAAxy or AAKKy so if one count is 3 we know 3 of a kind.
                if *counts.values().max().unwrap() == 3 {
                    hands.push((Hand::ThreeOfAKind(cards), bid));
                } else {
                    hands.push((Hand::TwoPair(cards), bid));
                }
            }
            4 => hands.push((Hand::OnePair(cards), bid)), // 4 distinct cards == 1 pair
            5 => hands.push((Hand::HighCard(cards), bid)),
            _ => panic!(),
        };
    }
    hands.sort();

    let mut score = 0;
    for (i, hand) in hands.iter().enumerate() {
        score += (i + 1) * hand.1;
    }
    println!("part1: {score}");
    Ok(())
}
