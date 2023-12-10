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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next_pos(&self, cur_pos: &Coord) -> Coord {
        let &(x, y) = cur_pos;
        match self {
            Direction::North => (x - 1, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y - 1),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Tile {
    pos: Coord,
    connect: (Direction, Direction),
}

impl Tile {
    fn new(connect: (Direction, Direction), pos: Coord) -> Self {
        Tile { pos, connect }
    }

    fn from_char(c: char, pos: Coord) -> Option<Self> {
        // default pipe direction
        use Direction::*;
        Some(match c {
            '|' => Self::new((North, South), pos),
            '-' => Self::new((West, East), pos),
            'L' => Self::new((North, East), pos),
            'J' => Self::new((North, West), pos),
            '7' => Self::new((West, South), pos),
            'F' => Self::new((East, South), pos),
            'S' => return None,
            _ => return None,
        })
    }

    fn new_direction(&self, dir: &Direction) -> Option<Direction> {
        let dir = dir.reverse();
        if self.connect.0 == dir {
            Some(self.connect.1)
        } else if self.connect.1 == dir {
            Some(self.connect.0)
        } else {
            None
        }
    }
}

struct Grid {
    map: HashMap<Coord, Tile>,
    start: Coord,
    bound: Coord,
}

impl Grid {
    fn get(&self, pos: &Coord) -> Option<&Tile> {
        self.map.get(pos)
    }

    fn move_inside_grid<'a>(
        &self,
        pos: &Coord,
        move_dir: &Direction,
    ) -> Result<Option<(Coord, Direction)>> {
        let next_pos = move_dir.next_pos(pos);
        if next_pos == self.start {
            return Ok(None);
        }
        if let Some(next_tile) = self.get(&next_pos) {
            if let Some(new_dir) = next_tile.new_direction(&move_dir) {
                return Ok(Some((next_pos, new_dir)));
            }
        }
        err!("There is no loop")
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Grid {
    let mut map = HashMap::new();
    let mut bound = (0, 0);
    let mut start = (0, 0);
    for (x, line) in input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
    {
        bound.0 = bound.0.max(x as i32 + 1);
        for (y, c) in line.trim().chars().enumerate() {
            bound.1 = bound.1.max(x as i32 + 1);
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if let Some(pipe) = Tile::from_char(c, (x as i32, y as i32)) {
                map.insert((x as i32, y as i32), pipe);
            }
        }
    }
    Grid { map, start, bound }
}

fn scurry(
    pos: &Coord,
    move_dir: &Direction,
    grid: &Grid,
    start_pos: &Coord,
    path: &mut Vec<Coord>,
) -> bool {
    match grid.move_inside_grid(pos, move_dir) {
        Ok(next) => {
            path.push(pos.clone());
            match next {
                None => true,
                Some((np, nd)) => scurry(&np, &nd, grid, start_pos, path),
            }
        }
        _ => false,
    }
}

fn get_loop(grid: &Grid) -> Option<Vec<Coord>> {
    let start_pos = grid.start;

    for move_dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        if let Ok(Some((np, nd))) = grid.move_inside_grid(&start_pos, &move_dir) {
            let mut path = vec![grid.start];
            if scurry(&np, &nd, grid, &start_pos, &mut path) {
                return Some(path);
            }
        }
    }
    None
}

fn part1(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let result = get_loop(grid).unwrap().len() / 2;
    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(grid: &Grid) -> Result<usize> {
    let _start = Instant::now();

    let main_loop = get_loop(grid).unwrap();

    let mut connected_pipe = HashSet::new();
    for (p1, p2) in main_loop
        .iter()
        .cloned()
        .zip(main_loop.iter().cloned().cycle().skip(1))
        .take(main_loop.len())
    {
        connected_pipe.insert((p1, p2));
        connected_pipe.insert((p2, p1));
    }

    let bound = grid.bound;
    let mut expand_map = vec![vec![0; bound.1 as usize * 2 - 1]; bound.0 as usize * 2 - 1];

    // init expanded map
    for (x, y) in main_loop {
        expand_map[x as usize * 2][y as usize * 2] = 1;
    }
    for x in 0..expand_map.len() {
        for y in 0..expand_map[0].len() {
            if adjacent_pipe_connected(&(x as i32, y as i32), &connected_pipe) {
                expand_map[x][y] = 1;
            } else if x == 0 || y == 0 && expand_map[x][y] == 0 {
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

fn adjacent_pos(pos: &Coord) -> [Coord; 4] {
    let &(x, y) = pos;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
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

fn adjacent_pipe_connected(pos: &(i32, i32), connected_pipe: &HashSet<(Coord, Coord)>) -> bool {
    if pos.0 % 2 != 0 && pos.1 % 2 != 0 {
        let (x, y) = pos;
        let up = ((x - 1) / 2, y / 2);
        let down = ((x + 1) / 2, y / 2);
        let left = (x / 2, (y - 1) / 2);
        let right = (x / 2, (y + 1) / 2);
        return connected_pipe.contains(&(up, down)) || connected_pipe.contains(&(left, right));
    }
    false
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
    let grid = parse_input(input);

    assert_eq!(part2(&grid).unwrap(), 8);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(input);
    assert_eq!(part1(&grid).unwrap(), 6725);
    // assert_eq!(part2(&grid).unwrap(), 6725);
}
