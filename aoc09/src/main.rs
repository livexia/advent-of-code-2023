use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn difference(values: &[i64]) -> Vec<i64> {
    values.windows(2).map(|w| w[1] - w[0]).collect()
}

fn all_zero(values: &[i64]) -> bool {
    values.iter().all(|n| n == &0)
}

fn predict_last(history: &[i64]) -> i64 {
    let mut last = 0;

    let mut values = history.to_vec();
    while !all_zero(&values) {
        last += *values.last().unwrap();
        values = difference(&values);
    }
    last
}

fn part1(histories: &[Vec<i64>]) -> Result<i64> {
    let start = Instant::now();

    let result = histories.iter().map(|h| predict_last(h)).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn predict_first(history: &[i64]) -> i64 {
    let mut first = 0;
    let mut sign = 1;

    let mut values = history.to_vec();
    while !all_zero(&values) {
        first += sign * values[0];
        sign *= -1;
        values = difference(&values);
    }
    first
}

fn part2(histories: &[Vec<i64>]) -> Result<i64> {
    let start = Instant::now();

    let result = histories.iter().map(|h| predict_first(h)).sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let histories = parse_input(&input);
    part1(&histories)?;
    part2(&histories)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let histories = parse_input(input);
    assert_eq!(part1(&histories).unwrap(), 114);
    assert_eq!(part2(&histories).unwrap(), 2);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let histories = parse_input(&input);
    assert_eq!(part1(&histories).unwrap(), 2175229206);
    assert_eq!(part2(&histories).unwrap(), 942);
}
