use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (i32, i32);
type Grid = HashMap<Coord, Tile>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next_pos(&self, cur_pos: Coord) -> Coord {
        let (x, y) = cur_pos;
        match self {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        }
    }
}

#[derive(Debug)]
struct Tile {
    is_start: bool,
    pos: Coord,
    pipe_connect: Vec<Direction>,
}

impl Tile {
    fn new(connect: &[Direction], pos: Coord) -> Self {
        Tile {
            is_start: false,
            pos,
            pipe_connect: connect.to_vec(),
        }
    }

    fn start(pos: Coord) -> Self {
        use Direction::*;
        Tile {
            is_start: true,
            pos,
            pipe_connect: vec![North, South, West, East],
        }
    }

    fn from_char(c: char, pos: Coord) -> Option<Self> {
        use Direction::*;
        Some(match c {
            '|' => Self::new(&[North, South], pos),
            '-' => Self::new(&[East, West], pos),
            'L' => Self::new(&[North, East], pos),
            'J' => Self::new(&[North, West], pos),
            '7' => Self::new(&[West, South], pos),
            'F' => Self::new(&[East, South], pos),
            'S' => Self::start(pos),
            _ => return None,
        })
    }

    fn connect_pos(&self) -> Vec<Coord> {
        self.pipe_connect
            .iter()
            .map(|d| d.next_pos(self.pos))
            .collect()
    }

    fn is_adjacent_connected(&self, other: &Self) -> bool {
        self.connect_pos().contains(&other.pos) && other.connect_pos().contains(&self.pos)
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Grid {
    let mut grid = HashMap::new();
    for (x, line) in input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        for (y, c) in line.trim().chars().enumerate() {
            if let Some(pipe) = Tile::from_char(c, (x as i32, y as i32)) {
                grid.insert((x as i32, y as i32), pipe);
            }
        }
    }
    grid
}

fn adjacent_pos(pos: &Coord) -> [Coord; 4] {
    let &(x, y) = pos;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn part1(grid: &Grid) -> Result<i32> {
    let _start = Instant::now();

    let start_tile = grid.iter().find(|(_, tile)| tile.is_start).unwrap().1;

    let mut queue = VecDeque::new();
    queue.push_back(start_tile);

    let mut steps = 0;

    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let layer_count = queue.len();
        for _ in 0..layer_count {
            steps += 1;
            let cur_tile = queue.pop_front().unwrap();
            for next_pos in adjacent_pos(&cur_tile.pos) {
                if let Some(next_tile) = grid.get(&next_pos) {
                    if cur_tile.is_adjacent_connected(next_tile) {
                        if visited.insert(next_pos) {
                            queue.push_back(next_tile);
                        }
                    }
                }
            }
        }
    }

    let result = steps / 2;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let grid = parse_input(input);

    part1(&grid)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 4);

    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 8);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 6725);
}
