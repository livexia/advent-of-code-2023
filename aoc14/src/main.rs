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

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn next_pos(pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::North => pos.0.checked_sub(1).map(|i| (i, pos.1)),
        Direction::South => Some((pos.0 + 1, pos.1)),
        Direction::West => pos.1.checked_sub(1).map(|j| (pos.0, j)),
        Direction::East => Some((pos.0, pos.1 + 1)),
    }
}

fn roll(platform: &mut [Vec<char>], dir: Direction) {
    let (height, width) = (platform.len(), platform[0].len());
    let (rows, columns): (Vec<_>, Vec<_>) = match dir {
        Direction::North => ((1..height).collect(), (0..width).collect()),
        Direction::South => ((0..height - 1).rev().collect(), (0..width).collect()),
        Direction::West => ((0..height).collect(), (1..width).collect()),
        Direction::East => ((0..height).collect(), (0..width - 1).rev().collect()),
    };
    for &i in &rows {
        for &j in &columns {
            if platform[i][j] != 'O' {
                continue;
            }

            let mut prev = (i, j);
            let mut curr = next_pos((i, j), dir).unwrap();

            while platform[curr.0][curr.1] == '.' {
                platform[prev.0][prev.1] = '.';
                platform[curr.0][curr.1] = 'O';
                if let Some(temp) = next_pos(curr, dir) {
                    if temp.0 < height && temp.1 < width {
                        prev = curr;
                        curr = temp;
                        continue;
                    }
                }
                break;
            }
        }
    }
}

fn spin(platform: &mut [Vec<char>], cycle: usize) {
    use Direction::*;
    for _ in 0..cycle {
        roll(platform, North);
        roll(platform, West);
        roll(platform, South);
        roll(platform, East);
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

    roll(&mut platform, Direction::North);

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

    let dest = (1_000_000_000 - cycle_start) % cycle_length + cycle_start;

    let mut super_fast = platform.clone();
    spin(&mut super_fast, dest);

    let result = calc(&super_fast);
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
