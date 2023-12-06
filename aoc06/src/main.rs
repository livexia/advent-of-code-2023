use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut time = vec![];
    let mut distance = vec![];
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        if let Some(time_line) = line.strip_prefix("Time:") {
            time = time_line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        }
        if let Some(distance_line) = line.strip_prefix("Distance:") {
            distance = distance_line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
        }
    }
    (time, distance)
}

fn get_distance(time: usize, pressed_time: usize) -> usize {
    time.saturating_sub(pressed_time) * pressed_time
}

fn part1(time: &[usize], distance: &[usize]) -> Result<usize> {
    let start = Instant::now();

    let result = time
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| (1..*t).filter(|&i| get_distance(*t, i) > *d).count())
        .product();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(time: &[usize], distance: &[usize]) -> Result<usize> {
    let start = Instant::now();
    let time = time
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = distance
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let result = (1..time)
        .filter(|t| get_distance(time, *t) > distance)
        .count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (time, distance) = parse_input(&input);

    part1(&time, &distance)?;
    part2(&time, &distance)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    let (time, distance) = parse_input(input);

    assert_eq!(time[0], 7);
    assert_eq!(get_distance(7, 8), 0);
    assert_eq!(get_distance(7, 2), 10);
    assert_eq!(part1(&time, &distance).unwrap(), 288);
    assert_eq!(part2(&time, &distance).unwrap(), 71503);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (time, distance) = parse_input(&input);
    assert_eq!(part1(&time, &distance).unwrap(), 211904);
}
