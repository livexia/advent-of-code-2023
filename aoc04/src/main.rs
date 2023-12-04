use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> Result<Vec<(Vec<usize>, Vec<usize>)>> {
    let mut cards = vec![];

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        if let Some((_, nums)) = line.trim().split_once(':') {
            if let Some((win, have)) = nums.split_once('|') {
                cards.push((
                    win.trim()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                    have.trim()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                ));
            } else {
                return err!("Unable to parse card: {:?}", line);
            }
        } else {
            return err!("Unable to parse card: {:?}", line);
        }
    }
    Ok(cards)
}

fn part1(cards: &[(Vec<usize>, Vec<usize>)]) -> Result<usize> {
    let start = Instant::now();

    let result = cards
        .iter()
        .map(|card| {
            let mut count = 0;
            for n1 in &card.0 {
                for n2 in &card.1 {
                    if n1 == n2 {
                        count += 1;
                    }
                }
            }
            if count == 0 {
                0
            } else {
                2usize.pow(count - 1)
            }
        })
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let cards = parse_input(&input)?;

    part1(&cards)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards = parse_input(input).unwrap();
    assert_eq!(cards[0].0[0], 41);

    assert_eq!(part1(&cards).unwrap(), 13);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let cards = parse_input(&input).unwrap();
    assert_eq!(part1(&cards).unwrap(), 24733);
    assert_eq!(2, 2);
}
