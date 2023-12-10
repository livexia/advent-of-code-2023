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

fn parse_input<T: AsRef<str>>(input: T) -> (Grid, Coord) {
    let mut grid = HashMap::new();
    let mut bound = (0, 0);
    for (x, line) in input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        bound.0 = bound.0.max(x as i32);
        for (y, c) in line.trim().chars().enumerate() {
            bound.1 = bound.1.max(x as i32);
            if let Some(pipe) = Tile::from_char(c, (x as i32, y as i32)) {
                grid.insert((x as i32, y as i32), pipe);
            }
        }
    }
    (grid, bound)
}

fn verify_start_pipe(grid: &Grid) -> Result<Vec<Tile>> {
    let (pos, tile) = grid.iter().find(|(_, tile)| tile.is_start).unwrap();

    // 0: up, 1: down, 2: left, 3: right
    let adjacent_pipe: Vec<_> = adjacent_pos(pos).iter().map(|np| grid.get(np)).collect();
    todo!()
}

fn adjacent_pos(pos: &Coord) -> [Coord; 4] {
    let &(x, y) = pos;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn part1(grid: &Grid) -> Result<usize> {
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
                    if cur_tile.is_adjacent_connected(next_tile) && visited.insert(next_pos) {
                        queue.push_back(next_tile);
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

fn surround_pos(pos: &Coord) -> [Coord; 8] {
    // order：
    // 0 -> 1 -> 2
    // 7         3
    // 6 <- 5 <- 4
    let &(x, y) = pos;
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
    ]
}
fn is_enclosed_by_the_loop(pos: &Coord, connected_pipe: &HashSet<(Coord, Coord)>) -> bool {
    let surround = surround_pos(pos);
    surround
        .iter()
        .cycle()
        .cloned()
        .zip(surround.iter().cycle().cloned().skip(1))
        .take(8)
        .any(|t| !connected_pipe.contains(&t))
}

fn part2(grid: &Grid, bound: Coord) -> Result<usize> {
    let _start = Instant::now();

    let mut connected_pipe = HashSet::new();
    for (&p1, t1) in grid.iter() {
        for (&p2, t2) in grid.iter() {
            if t1.is_adjacent_connected(t2) {
                connected_pipe.insert((p1, p2));
            }
        }
    }

    let mut expand_map = vec![vec![0; bound.1 as usize * 2 + 1]; bound.0 as usize * 2 + 1];

    // init expanded map
    for x in 0..expand_map.len() {
        for y in 0..expand_map[0].len() {
            if pipe_in_the_loop(&(x, y)) {
                expand_map[x][y] = 1;
            } else if adjacent_pipe_connected(&(x, y)) {
                expand_map[x][y] = 1;
            } else if x == 0 || y == 0 {
                expand_map[x][y] = 2;
            }
        }
    }

    let mut visited = HashSet::new();

    // search start from wall
    // left and right wall
    for x in 0..expand_map.len() {
        let (left, right) = (0, expand_map[0].len() - 1);
        if expand_map[x][left] == 2 {
            dfs((x, left), &mut expand_map, &mut visited);
        }
        if expand_map[x][right] == 2 {
            dfs((x, right), &mut expand_map, &mut visited);
        }
    }

    for y in 0..expand_map[0].len() {
        let (top, bottom) = (0, expand_map.len() - 1);
        if expand_map[top][y] == 2 {
            dfs((top, y), &mut expand_map, &mut visited);
        }
        if expand_map[bottom][y] == 2 {
            dfs((bottom, y), &mut expand_map, &mut visited);
        }
    }

    let result = expand_map.iter().flatten().filter(|&&n| n == 2).count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn dfs(pos: (usize, usize), map: &mut [Vec<u8>], visited: &mut HashSet<(usize, usize)>) {
    if visited.insert(pos) {
        let (x, y) = pos;
        let bound = (map.len(), map[0].len());
        for (nx, ny) in adjacent_pos(&(x as i32, y as i32)) {
            if is_in_bound(&(nx, ny), &bound) && map[nx as usize][ny as usize] == 0 {
                map[nx as usize][ny as usize] = 2;
                dfs((nx as usize, ny as usize), map, visited);
            }
        }
    }
}

fn is_in_bound(next_pos: &(i32, i32), bound: &(usize, usize)) -> bool {
    next_pos.0 >= 0
        && next_pos.1 >= 0
        && (next_pos.0 as usize) < bound.0
        && (next_pos.1 as usize) < bound.1
}

fn adjacent_pipe_connected(pos: &(i32, i32), loop_pipes: &HashSet<Coord>, grid: &Grid) -> bool {
    if pos.0 % 2 != 0 && pos.1 % 2 != 0 {
        let (x, y) = pos;
        let up = ((x - 1) / 2, y / 2);
        let down = ((x + 1) / 2, y / 2);
        let left = (x / 2, (y - 1) / 2);
        let right = (x / 2, (y + 1) / 2);
        return (loop_pipes.contains(&up)
            && loop_pipes.contains(&down)
            && grid
                .get(&up)
                .unwrap()
                .is_adjacent_connected(grid.get(&down).unwrap()))
            || (loop_pipes.contains(&left)
                && loop_pipes.contains(&right)
                && grid
                    .get(&left)
                    .unwrap()
                    .is_adjacent_connected(grid.get(&right).unwrap()));
    }
    false
}

fn pipe_in_the_loop(pos: &(i32, i32), loop_pipes: &HashSet<Coord>) -> bool {
    loop_pipes.contains(&(pos.0 / 2, pos.1 / 2))
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (grid, bound) = parse_input(input);

    part1(&grid)?;
    part2(&grid, bound)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let (grid, _) = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 4);

    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let (grid, _) = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 8);
}

#[test]
fn part2_example_input() {
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
    let (grid, bound) = parse_input(input);

    dbg!(part2(&grid, bound));
    dbg!(grid.len());
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (grid, bound) = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 6725);
    assert_eq!(part2(&grid, &bound).unwrap(), 6725);
}
