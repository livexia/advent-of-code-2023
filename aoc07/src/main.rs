use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandKind {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

impl HandKind {
    fn to_u8(self) -> u8 {
        match self {
            HandKind::Five => 6,
            HandKind::Four => 5,
            HandKind::Full => 4,
            HandKind::Three => 3,
            HandKind::Two => 2,
            HandKind::One => 1,
            HandKind::High => 0,
        }
    }
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_u8().cmp(&other.to_u8())
    }
}

#[derive(Debug, Clone)]
struct Hand {
    kind: HandKind,
    raw: [u8; 5],
    bid: usize,
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut raw: [u8; 5] = [0; 5];
        if let Some((hand_str, bid_str)) = s.split_once(' ') {
            if hand_str.len() != 5 {
                return err!("Wrong hand length: {:?}", hand_str);
            }
            let mut count = HashMap::new();
            for (i, c) in hand_str.chars().enumerate() {
                *count.entry(c).or_insert(0) += 1;
                raw[i] = match c {
                    '2' => 0,
                    '3' => 1,
                    '4' => 2,
                    '5' => 3,
                    '6' => 4,
                    '7' => 5,
                    '8' => 6,
                    '9' => 7,
                    'T' => 8,
                    'J' => 9,
                    'Q' => 10,
                    'K' => 11,
                    'A' => 12,
                    _ => return err!("Wrong hand char: {:?}", c),
                };
            }
            let mut values: Vec<_> = count.values().collect();
            values.sort();
            let count_number = values.iter().fold(0, |sum, i| sum * 10 + *i);
            let kind = match count_number {
                5 => HandKind::Five,
                14 => HandKind::Four,
                23 => HandKind::Full,
                113 => HandKind::Three,
                122 => HandKind::Two,
                1112 => HandKind::One,
                11111 => HandKind::High,
                _ => return err!("Wrong hand: {:?}", hand_str),
            };
            let bid = bid_str.parse()?;
            return Ok(Self { kind, raw, bid });
        }
        err!("Wrong input for a hand: {:?}", s)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.raw == other.raw && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            Equal => {
                for (a, b) in self.raw.iter().zip(other.raw.iter()) {
                    match a.cmp(b) {
                        Less => return Less,
                        Equal => continue,
                        Greater => return Greater,
                    }
                }
                Equal
            }
            ord => ord,
        }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

fn part1(hands: &[Hand]) -> Result<usize> {
    let start = Instant::now();

    let mut hands = hands.to_vec();

    hands.sort();

    let result = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let hands = parse_input(&input);
    dbg!(hands.len());
    part1(&hands)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    let hands = parse_input(input);
    assert_eq!(hands[0].bid, 765);
    assert_eq!(part1(&hands).unwrap(), 6440);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let hands = parse_input(&input);
    assert_eq!(part1(&hands).unwrap(), 248559379);
}
