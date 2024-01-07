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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    let mut hands = vec![];
    let mut hands_part2 = vec![];
    // Input has the form:
    //
    // XXXXX nnn
    //
    // Where X are various combinations of cards forming 5 card poker hands
    // of 5 of a kind, 4 of a kind, full hourse, 2 pair, pair and high card.
    // i.e. no straights/flushes here.
    //
    // The cards don't change order so you have to find the best hand and then
    // sort them by hand type. Then inside each hand they sort based on Card
    // which is ordered.
    //
    // Once sorted make a sum of <pos> * nnn
    //
    // Part2 is the same except there can now be jokers (represented as J) which
    // make hands as expected (wild card) but for sorting are the worst card.
    // Compute the same sum once you get a proper sort.
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
        hands.push((make_hand(&counts, cards), bid));
        hands_part2.push((make_hand(&counts_part2, cards_part2), bid));
    }

    hands.sort();
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

fn make_hand(counts: &HashMap<Card, usize>, cards: Vec<Card>) -> Hand {
    // part2
    // Check for jokers on each part first. Otherwise as above.
    match counts.len() {
        1 => Hand::FiveOfAKind(cards),
        2 => {
            if counts.contains_key(&Card::Joker) {
                // Any jokers means 1 or 4 which means 5 of a kind.
                Hand::FiveOfAKind(cards)
            } else if *counts.values().max().unwrap() == 4 {
                // Either AAAAx or AAAKK so if one count is 4 we know 4 of a kind.
                Hand::FourOfAKind(cards)
            } else {
                // Otherwise it has to be XXXYY so a full house.
                Hand::FullHouse(cards)
            }
        }
        3 => {
            if counts.contains_key(&Card::Joker) {
                match counts[&Card::Joker] {
                    2 | 3 => {
                        // 3 jokers means 4 of a kind
                        // 2 jokers means 4 of a kind also (JJxxy)
                        // 1 joker means 4 of a kind also (Jxxxy)
                        Hand::FourOfAKind(cards)
                    }
                    1 => {
                        // 1 Joker means either xxxJy or xxJyy. The former is 4 of a kind
                        // and the latter is a full house.
                        if *counts.values().max().unwrap() == 3 {
                            Hand::FourOfAKind(cards)
                        } else {
                            Hand::FullHouse(cards)
                        }
                    }
                    _ => panic!(),
                }
            } else {
                // Either AAAxy or AAKKy so if one count is 3 we know 3 of a kind.
                if *counts.values().max().unwrap() == 3 {
                    Hand::ThreeOfAKind(cards)
                } else {
                    Hand::TwoPair(cards)
                }
            }
        }
        4 => {
            if counts.contains_key(&Card::Joker) {
                // any joker means xyzaa == 3 of a kind for a=J or x|y|z=J
                Hand::ThreeOfAKind(cards)
            } else {
                Hand::OnePair(cards)
            }
        }
        5 => {
            if counts.contains_key(&Card::Joker) {
                // any joker means xyzab == pair
                Hand::OnePair(cards)
            } else {
                Hand::HighCard(cards)
            }
        }
        _ => panic!(),
    }
}
