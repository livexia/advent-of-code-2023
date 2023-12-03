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

type Grid = HashMap<Coord, Engine>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Engine {
    Number(u32),
    Symbol(char),
}

impl Engine {
    fn from_char(c: char) -> Option<Engine> {
        if let Some(n) = c.to_digit(10) {
            Some(Engine::Number(n))
        } else if c != '.' {
            Some(Engine::Symbol(c))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Coord, Grid) {
    let mut grid = Grid::new();
    let mut bound = (0, 0);
    for (i, row) in input
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        bound.0 = bound.0.max(i as i32);
        for (j, c) in row.trim().chars().enumerate() {
            if let Some(e) = Engine::from_char(c) {
                grid.insert((i as i32, j as i32), e);
            }
            bound.1 = bound.1.max(j as i32);
        }
    }
    (bound, grid)
}

fn adjacent(c: &Coord) -> [Coord; 4] {
    [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
    ]
}

fn is_coord_valid(c: &Coord, bound: &Coord) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < bound.0 && c.1 < bound.1
}

fn dfs(coord: Coord, grid: &Grid, bound: &Coord, visited: &mut HashSet<Coord>) {
    visited.insert(coord);
    todo!()
}

fn part1(bound: &Coord, grid: &Grid) -> Result<u32> {
    let start = Instant::now();

    // bfs or dfs?
    let mut visited: HashSet<Coord> = HashSet::new();
    for (&coord, engine) in grid {
        if !visited.contains(&coord) {
            dfs(coord, grid, bound, &mut visited);
        }
    }

    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    todo!()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

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
    let (bound, grid) = parse_input(input);

    assert_eq!(grid.get(&(0, 0)).unwrap(), &Engine::Number(4));
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    assert_eq!(2, 2);
}
