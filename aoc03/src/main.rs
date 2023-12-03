use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (i32, i32);

type Engine = HashMap<Coord, EnginePart>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnginePart {
    Number(u32),
    Symbol(char),
}

impl EnginePart {
    fn from_char(c: char) -> Option<EnginePart> {
        if let Some(n) = c.to_digit(10) {
            Some(EnginePart::Number(n))
        } else if c != '.' {
            Some(EnginePart::Symbol(c))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Coord, Engine) {
    let mut engine = Engine::new();
    let mut bound = (0, 0);
    for (i, row) in input
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        bound.0 = bound.0.max(i as i32 + 1);
        for (j, c) in row.trim().chars().enumerate() {
            if let Some(e) = EnginePart::from_char(c) {
                engine.insert((i as i32, j as i32), e);
            }
            bound.1 = bound.1.max(j as i32 + 1);
        }
    }
    (bound, engine)
}

fn adjacent(c: &Coord) -> [Coord; 8] {
    [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
        (c.0 - 1, c.1 - 1),
        (c.0 - 1, c.1 + 1),
        (c.0 + 1, c.1 - 1),
        (c.0 + 1, c.1 + 1),
    ]
}

fn is_coord_valid(c: &Coord, bound: &Coord) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < bound.0 && c.1 < bound.1
}

fn dfs(
    c: Coord,
    engine: &Engine,
    bound: &Coord,
    visited: &mut HashSet<Coord>,
) -> (bool, Option<HashSet<Coord>>, u32) {
    if visited.insert(c) {
        if let Some(EnginePart::Number(n)) = engine.get(&c) {
            let mut adjacent_gears = HashSet::new();
            let mut flag = false;
            for next_c in adjacent(&c) {
                if let Some(EnginePart::Symbol(c)) = engine.get(&next_c) {
                    if c == &'*' {
                        adjacent_gears.insert(next_c);
                    }
                    flag = true;
                }
            }
            if is_coord_valid(&(c.0, c.1 - 1), bound) {
                let (next_flag, next_gears, next_sum) = dfs((c.0, c.1 - 1), engine, bound, visited);
                let sum = n + 10 * next_sum;
                if let Some(next_gears) = next_gears {
                    adjacent_gears.extend(next_gears);
                }
                return (flag | next_flag, Some(adjacent_gears), sum);
            } else {
                return (flag, Some(adjacent_gears), *n);
            }
        }
    }
    (false, None, 0)
}

fn part1(bound: &Coord, engine: &Engine) -> Result<u32> {
    let start = Instant::now();

    // bfs or dfs?
    let mut sum = 0;
    let mut visited: HashSet<Coord> = HashSet::new();
    for x in (0..bound.0).rev() {
        for y in (0..bound.1).rev() {
            if engine.contains_key(&(x, y)) && !visited.contains(&(x, y)) {
                let (flag, _, whole_number) = dfs((x, y), engine, bound, &mut visited);
                if flag {
                    sum += whole_number;
                }
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn part2(bound: &Coord, engine: &Engine) -> Result<u32> {
    let start = Instant::now();

    // bfs or dfs?
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut gears = HashMap::new();
    for x in (0..bound.0).rev() {
        for y in (0..bound.1).rev() {
            if engine.contains_key(&(x, y)) && !visited.contains(&(x, y)) {
                let (_, next_gears, whole_number) = dfs((x, y), engine, bound, &mut visited);
                if let Some(next_gears) = next_gears {
                    for c in next_gears {
                        gears.entry(c).or_insert(vec![]).push(whole_number);
                    }
                }
            }
        }
    }

    let sum = gears
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.iter().product::<u32>())
        .sum();

    writeln!(io::stdout(), "Part 2: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (bound, engine) = parse_input(&input);

    part1(&bound, &engine)?;
    part2(&bound, &engine)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let (bound, engine) = parse_input(input);

    assert_eq!(engine.get(&(0, 0)).unwrap(), &EnginePart::Number(4));
    assert_eq!(part1(&bound, &engine).unwrap(), 4361);
    assert_eq!(part2(&bound, &engine).unwrap(), 467835);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (bound, engine) = parse_input(&input);

    assert_eq!(part1(&bound, &engine).unwrap(), 540131);
    assert_eq!(part2(&bound, &engine).unwrap(), 86879020);
}
