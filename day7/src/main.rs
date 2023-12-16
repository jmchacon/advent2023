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

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
enum Hand {
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FiveOfAKind(Vec<Card>),
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hands = vec![];
    let mut hands_part2 = vec![];
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(
            parts.len() == 2 && parts[0].len() == 5,
            "Invalid line {} - {line}",
            line_num + 1
        );
        let mut cards = vec![];
        let mut counts = HashMap::new();
        let mut cards_part2 = vec![];
        let mut counts_part2 = HashMap::new();
        let bid = parts[1].parse::<usize>().unwrap();
        for c in parts[0].as_bytes() {
            let mut card = match c {
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
            cards.push(card.clone());

            // part2
            if *c == b'J' {
                card = Card::Joker;
            }
            counts_part2
                .entry(card.clone())
                .and_modify(|f| *f += 1)
                .or_insert(1);
            cards_part2.push(card);
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

        // part2
        // Check for jokers on each part first. Otherwise as above.
        match counts_part2.len() {
            1 => hands_part2.push((Hand::FiveOfAKind(cards_part2), bid)),
            2 => {
                if counts_part2.contains_key(&Card::Joker) {
                    // Any jokers means 1 or 4 which means 5 of a kind.
                    hands_part2.push((Hand::FiveOfAKind(cards_part2), bid));
                } else {
                    if *counts_part2.values().max().unwrap() == 4 {
                        hands_part2.push((Hand::FourOfAKind(cards_part2), bid));
                    } else {
                        hands_part2.push((Hand::FullHouse(cards_part2), bid));
                    }
                }
            }
            3 => {
                if counts_part2.contains_key(&Card::Joker) {
                    match counts_part2[&Card::Joker] {
                        2 | 3 => {
                            // 3 jokers means 4 of a kind
                            // 2 jokers means 4 of a kind also (JJxxy)
                            // 1 joker means 4 of a kind also (Jxxxy)
                            hands_part2.push((Hand::FourOfAKind(cards_part2), bid));
                        }
                        1 => {
                            // 1 Joker means either xxxJy or xxJyy. The former is 4 of a kind
                            // and the latter is a full house.
                            if *counts_part2.values().max().unwrap() == 3 {
                                hands_part2.push((Hand::FourOfAKind(cards_part2), bid));
                            } else {
                                hands_part2.push((Hand::FullHouse(cards_part2), bid));
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    // Either AAAxy or AAKKy so if one count is 3 we know 3 of a kind.
                    if *counts_part2.values().max().unwrap() == 3 {
                        hands_part2.push((Hand::ThreeOfAKind(cards_part2), bid));
                    } else {
                        hands_part2.push((Hand::TwoPair(cards_part2), bid));
                    }
                }
            }
            4 => {
                if counts_part2.contains_key(&Card::Joker) {
                    // any joker means xyzaa == 3 of a kind for a=J or x|y|z=J
                    hands_part2.push((Hand::ThreeOfAKind(cards_part2), bid));
                } else {
                    hands_part2.push((Hand::OnePair(cards_part2), bid));
                }
            }
            5 => {
                if counts_part2.contains_key(&Card::Joker) {
                    // any joker means xyzab == pair
                    hands_part2.push((Hand::OnePair(cards_part2), bid));
                } else {
                    hands_part2.push((Hand::HighCard(cards_part2), bid));
                }
            }
            _ => panic!(),
        }
    }

    hands.sort();

    /*hands_part2 = hands_part2
    .iter()
    .map(|f| {
        (
            match &f.0 {
                Hand::HighCard(v) => Hand::HighCard(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::OnePair(v) => Hand::OnePair(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::TwoPair(v) => Hand::TwoPair(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::ThreeOfAKind(v) => Hand::ThreeOfAKind(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::FullHouse(v) => Hand::FullHouse(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::FourOfAKind(v) => Hand::FourOfAKind(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                Hand::FiveOfAKind(v) => Hand::FiveOfAKind(
                    v.iter()
                        .map(|f| {
                            if *f == Card::Joker {
                                Card::Jack
                            } else {
                                f.clone()
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            f.1,
        )
    })
    .collect::<Vec<_>>();*/
    hands_part2.sort();

    if args.debug {
        for h in &hands_part2 {
            println!("{h:?}");
        }
    }
    for (part, h) in [(1, &hands), (2, &hands_part2)] {
        let score = h
            .iter()
            .enumerate()
            .fold(0, |acc, f| acc + (f.0 + 1) * f.1 .1);
        println!("part{part}: {score}");
    }
    Ok(())
}
