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

fn dfs(c: Coord, engine: &Engine, bound: &Coord, visited: &mut HashSet<Coord>) -> (bool, u32) {
    if visited.insert(c) {
        if let Some(EnginePart::Number(n)) = engine.get(&c) {
            let flag = adjacent(&c).iter().any(|next_c| {
                if let Some(EnginePart::Symbol(_)) = engine.get(&next_c) {
                    true
                } else {
                    false
                }
            });
            if is_coord_valid(&(c.0, c.1 - 1), bound) {
                let (next_flag, next_sum) = dfs((c.0, c.1 - 1), engine, bound, visited);
                return (flag | next_flag, n + 10 * next_sum);
            } else {
                return (flag, *n);
            }
        }
    }
    (false, 0)
}

fn part1(bound: &Coord, engine: &Engine) -> Result<u32> {
    let start = Instant::now();

    // bfs or dfs?
    let mut sum = 0;
    let mut visited: HashSet<Coord> = HashSet::new();
    for x in (0..bound.0).rev() {
        for y in (0..bound.1).rev() {
            if engine.contains_key(&(x, y)) && !visited.contains(&(x, y)) {
                let (flag, row_sum) = dfs((x, y), engine, bound, &mut visited);
                if flag {
                    sum += row_sum;
                }
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (bound, engine) = parse_input(&input);

    part1(&bound, &engine)?;
    // part1()?;
    // part2()?;
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
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (bound, engine) = parse_input(&input);

    assert_eq!(part1(&bound, &engine).unwrap(), 540131);
}
