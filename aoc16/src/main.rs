use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);
type Grid = Vec<Vec<char>>;

fn parse_input<T: AsRef<str>>(input: T) -> Grid {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn next_pos(&self, pos: Coord) -> Coord {
        let (x, y) = pos;
        match self {
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
        }
    }

    fn rev(&self) -> Self {
        use Direction::*;
        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
        }
    }

    fn turn(&self, tile: char) -> Vec<Self> {
        use Direction::*;
        match self {
            Right => match tile {
                '/' => vec![Up],
                '\\' => vec![Down],
                '-' | '.' => vec![Right],
                '|' => vec![Up, Down],
                _ => unreachable!(),
            },
            Left => Right.turn(tile).iter().map(|d| d.rev()).collect(),
            Up => match tile {
                '/' => vec![Right],
                '\\' => vec![Left],
                '-' => vec![Left, Right],
                '|' | '.' => vec![Up],
                _ => unreachable!(),
            },
            Down => Up.turn(tile).iter().map(|d| d.rev()).collect(),
        }
    }
}

fn valid_pos(pos: &Coord, grid: &Grid) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < grid.len() && (pos.1 as usize) < grid[0].len()
}

fn bounce(pos: Coord, dir: Direction, grid: &Grid) -> Option<(Coord, Vec<Direction>)> {
    let next_pos = if pos.0 == -1 {
        (0, pos.1)
    } else if pos.1 == -1 {
        (pos.0, 0)
    } else if pos.0 == grid.len() as isize {
        (grid.len() as isize - 1, pos.1)
    } else if pos.1 == grid[0].len() as isize {
        (pos.0, grid[0].len() as isize - 1)
    } else {
        dir.next_pos(pos)
    };
    if !valid_pos(&next_pos, grid) {
        return None;
    }
    let tile = grid[next_pos.0 as usize][next_pos.1 as usize];

    Some((next_pos, dir.turn(tile)))
}

fn bfs(start_pos: Coord, dir: Direction, grid: &Grid) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, dir));

    let mut visited = HashSet::new();

    let mut energized = HashSet::new();

    while let Some((pos, dir)) = queue.pop_front() {
        energized.insert(pos);
        if visited.insert((pos, dir)) {
            if let Some((next, dirs)) = bounce(pos, dir, grid) {
                for n_d in dirs {
                    queue.push_back((next, n_d));
                }
            }
        }
    }
    energized.len() - 1
}

fn part1(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let result = bfs((-1, 0), Direction::Right, grid);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let mut result = 0;
    for y in 0..grid[0].len() {
        result = result.max(bfs((-1, y as isize), Direction::Down, grid));
    }

    for y in 0..grid[0].len() {
        result = result.max(bfs((grid.len() as isize, y as isize), Direction::Up, grid));
    }

    for x in 0..grid.len() {
        result = result.max(bfs((x as isize, -1), Direction::Right, grid));
    }

    for x in 0..grid.len() {
        result = result.max(bfs(
            (x as isize, grid[0].len() as isize),
            Direction::Left,
            grid,
        ));
    }

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(input);
    part1(&grid)?;
    part2(&grid)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 46);
    assert_eq!(part2(&grid).unwrap(), 51);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 7562);
    assert_eq!(part2(&grid).unwrap(), 7793);
}
