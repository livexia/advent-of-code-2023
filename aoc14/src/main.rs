use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<char>> {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect()
}

fn tilt_west(platform: &mut [Vec<char>]) {
    for i in 0..platform.len() {
        let mut last_possible = 0;
        for j in 0..platform[0].len() {
            if platform[i][j] == '#' {
                last_possible = j + 1;
            } else if platform[i][j] == 'O' {
                platform[i][j] = '.';
                platform[i][last_possible] = 'O';
                last_possible += 1;
            }
        }
    }
}

fn tilt_east(platform: &mut [Vec<char>]) {
    for i in 0..platform.len() {
        let mut last_possible = platform[0].len() - 1;
        for j in (0..platform[0].len()).rev() {
            if platform[i][j] == '#' {
                last_possible = j.saturating_sub(1);
            } else if platform[i][j] == 'O' {
                platform[i][j] = '.';
                platform[i][last_possible] = 'O';
                last_possible = last_possible.saturating_sub(1);
            }
        }
    }
}

fn tilt_north(platform: &mut [Vec<char>]) {
    for j in 0..platform[0].len() {
        let mut last_possible = 0;
        for i in 0..platform.len() {
            if platform[i][j] == '#' {
                last_possible = i + 1;
            } else if platform[i][j] == 'O' {
                platform[i][j] = '.';
                platform[last_possible][j] = 'O';
                last_possible += 1;
            }
        }
    }
}

fn tilt_south(platform: &mut [Vec<char>]) {
    for j in 0..platform[0].len() {
        let mut last_possible = platform.len() - 1;
        for i in (0..platform.len()).rev() {
            if platform[i][j] == '#' {
                last_possible = i.saturating_sub(1);
            } else if platform[i][j] == 'O' {
                platform[i][j] = '.';
                platform[last_possible][j] = 'O';
                last_possible = last_possible.saturating_sub(1);
            }
        }
    }
}

fn spin(platform: &mut [Vec<char>], cycle: usize) {
    for _ in 0..cycle {
        tilt_north(platform);
        tilt_west(platform);
        tilt_south(platform);
        tilt_east(platform);
    }
}

fn calc(platform: &[Vec<char>]) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.iter().filter(|rock| rock == &&'O').count())
        .sum()
}

fn part1(mut platform: Vec<Vec<char>>) -> Result<usize> {
    let _start = Instant::now();

    tilt_north(&mut platform);
    let result = calc(&platform);
    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(platform: Vec<Vec<char>>) -> Result<usize> {
    let _start = Instant::now();

    let mut slow = platform.clone();
    let mut fast = platform.clone();
    spin(&mut slow, 1);
    spin(&mut fast, 2);
    while slow != fast {
        spin(&mut slow, 1);
        spin(&mut fast, 2);
    }

    let mut slow = platform.clone();
    let mut cycle_start = 0;
    while slow != fast {
        spin(&mut slow, 1);
        spin(&mut fast, 1);
        cycle_start += 1;
    }

    let mut cycle_length = 1;
    let mut fast = slow.clone();
    spin(&mut fast, 1);
    while slow != fast {
        spin(&mut fast, 1);
        cycle_length += 1;
    }

    let remain_spin = (1_000_000_000 - cycle_start) % cycle_length;

    spin(&mut slow, remain_spin);

    let result = calc(&slow);
    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let platform = parse_input(input);

    part1(platform.clone())?;
    part2(platform)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let platform = parse_input(input);
    assert_eq!(part1(platform.clone()).unwrap(), 136);
    assert_eq!(part2(platform.clone()).unwrap(), 64);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    assert_eq!(2, 2);
    let platform = parse_input(input);

    assert_eq!(part1(platform.clone()).unwrap(), 109654);
    assert_eq!(part2(platform).unwrap(), 94876);
}
