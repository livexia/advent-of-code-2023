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
struct Pipe {
    connect: (Direction, Direction),
}

impl Pipe {
    fn new(connect: (Direction, Direction)) -> Self {
        Pipe { connect }
    }

    fn from_char(c: char) -> Option<Self> {
        // default pipe direction
        use Direction::*;
        Some(match c {
            '|' => Self::new((North, South)),
            '-' => Self::new((West, East)),
            'L' => Self::new((North, East)),
            'J' => Self::new((North, West)),
            '7' => Self::new((West, South)),
            'F' => Self::new((East, South)),
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
    map: HashMap<Coord, Pipe>,
    start: Coord,
    bound: Coord,
}

impl Grid {
    fn get(&self, pos: &Coord) -> Option<&Pipe> {
        self.map.get(pos)
    }

    fn move_inside_grid(
        &self,
        pos: &Coord,
        move_dir: &Direction,
    ) -> Result<Option<(Coord, Direction)>> {
        if pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.bound.0 && pos.1 < self.bound.1 {
            let next_pos = move_dir.next_pos(pos);
            if next_pos == self.start {
                return Ok(None);
            }
            if let Some(next_pipe) = self.get(&next_pos) {
                if let Some(new_dir) = next_pipe.new_direction(move_dir) {
                    return Ok(Some((next_pos, new_dir)));
                }
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
            bound.1 = bound.1.max(y as i32 + 1);
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if let Some(pipe) = Pipe::from_char(c) {
                map.insert((x as i32, y as i32), pipe);
            }
        }
    }
    Grid { map, start, bound }
}

fn get_loop(grid: &mut Grid) -> Option<Vec<Coord>> {
    let start_pos = grid.start;

    use Direction::*;
    for move_dir in [North, South, East, West] {
        if let Ok(Some((p, d))) = grid.move_inside_grid(&start_pos, &move_dir) {
            let mut path = vec![grid.start];
            let (mut np, mut nd) = (p, d);
            while let Ok(next) = grid.move_inside_grid(&np, &nd) {
                path.push(np);
                match next {
                    None => {
                        grid.map.insert(
                            grid.start,
                            match (move_dir, nd.reverse()) {
                                (North, South) | (South, North) => Pipe::new((North, South)),
                                (North, East) | (East, North) => Pipe::new((North, East)),
                                (North, West) | (West, North) => Pipe::new((North, West)),
                                (South, East) | (East, South) => Pipe::new((East, South)),
                                (South, West) | (West, South) => Pipe::new((West, South)),
                                (East, West) | (West, East) => Pipe::new((West, East)),
                                _ => panic!(),
                            },
                        );
                        return Some(path);
                    }
                    Some((p, d)) => (np, nd) = (p, d),
                }
            }
        }
    }
    None
}

fn part1(grid: &mut Grid) -> Result<usize> {
    let _start = Instant::now();

    let result = get_loop(grid).unwrap().len() / 2;
    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

#[allow(dead_code)]
fn print_map(map: &[Vec<u8>], step: usize) -> Result<()> {
    let mut s = String::new();
    for row in map.iter().step_by(step) {
        for &c in row.iter().step_by(step) {
            if c == 2 {
                s.push('O')
            } else if c == 1 {
                s.push('.')
            } else {
                s.push('I')
            }
        }
        s.push('\n')
    }
    writeln!(io::stdout(), "{s}")?;
    Ok(())
}

fn part2(grid: &mut Grid) -> Result<usize> {
    let _start = Instant::now();

    let loop_path = get_loop(grid).unwrap();

    let bound = (grid.bound.0 * 2 - 1, grid.bound.1 * 2 - 1);
    let mut expand_map = vec![vec![0; bound.1 as usize]; bound.0 as usize];

    for (p1, p2) in loop_path
        .iter()
        .cloned()
        .zip(loop_path.iter().cloned().cycle().skip(1))
        .take(loop_path.len())
    {
        let (x1, y1) = p1;
        expand_map[x1 as usize * 2][y1 as usize * 2] = 1;
        let (x2, y2) = p2;
        match ((x1 - x2).abs(), (y1 - y2).abs()) {
            (1, 0) => expand_map[x1.min(x2) as usize * 2 + 1][y1 as usize * 2] = 1,
            (0, 1) => expand_map[x1 as usize * 2][y1.min(y2) as usize * 2 + 1] = 1,
            _ => panic!("There is no way"),
        }
    }

    let mut queue = VecDeque::new();

    for x in 0..expand_map.len() {
        for y in [0, expand_map[0].len() - 1] {
            if expand_map[x][y] == 0 {
                expand_map[x][y] = 2;
                queue.push_back((x, y));
            }
        }
    }
    for x in [0, expand_map.len() - 1] {
        for y in 0..expand_map[0].len() {
            if expand_map[x][y] == 0 {
                expand_map[x][y] = 2;
                queue.push_back((x, y));
            }
        }
    }

    let mut visited = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        if visited.insert((x, y)) {
            for (nx, ny) in adjacent_pos(&(x as i32, y as i32)) {
                if is_in_bound(&(nx, ny), &bound) && expand_map[nx as usize][ny as usize] == 0 {
                    expand_map[nx as usize][ny as usize] = 2;
                    queue.push_back((nx as usize, ny as usize))
                }
            }
        }
    }

    let result = expand_map
        .iter()
        .step_by(2)
        .flat_map(|r| r.iter().step_by(2))
        .filter(|n| n == &&0)
        .count();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn is_in_bound(next_pos: &(i32, i32), bound: &(i32, i32)) -> bool {
    next_pos.0 >= 0 && next_pos.1 >= 0 && (next_pos.0) < bound.0 && (next_pos.1) < bound.1
}
fn adjacent_pos(pos: &Coord) -> [Coord; 4] {
    let &(x, y) = pos;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

#[allow(dead_code)]
fn dfs(pos: (usize, usize), map: &mut [Vec<u8>], visited: &mut HashSet<(usize, usize)>) {
    if visited.insert(pos) {
        let (x, y) = pos;
        let bound = (map.len() as i32, map[0].len() as i32);
        for (nx, ny) in adjacent_pos(&(x as i32, y as i32)) {
            if is_in_bound(&(nx, ny), &bound) && map[nx as usize][ny as usize] == 0 {
                map[nx as usize][ny as usize] = 2;
                dfs((nx as usize, ny as usize), map, visited);
            }
        }
    }
}

fn part2_raycast(grid: &mut Grid) -> Result<usize> {
    let _start = Instant::now();

    use Direction::*;

    let loop_path: HashSet<_> = get_loop(grid).unwrap().into_iter().collect();

    let mut result = 0;
    for x in 0..grid.bound.0 {
        let mut crossing_count = 0;
        for y in 0..grid.bound.1 {
            if loop_path.contains(&(x, y)) {
                // F---J count as one crossing and L---7 count as one crossing
                // FJ -> +1 L7 -> +1
                // L---J and F---7 does not count as one crossing
                // F7 -> +0 LJ -> +0
                match grid.get(&(x, y)).unwrap().connect {
                    (North, South) | (North, East) | (North, West) => {
                        // | and L and J increase counter
                        // or | and F and 7 increase counter
                        crossing_count += 1;
                    }
                    _ => (),
                }
            } else {
                if crossing_count % 2 == 1 {
                    result += 1;
                }
            }
        }
    }

    writeln!(io::stdout(), "Part 2 with Raycast: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut grid = parse_input(input);

    part1(&mut grid)?;
    part2(&mut grid)?;
    part2_raycast(&mut grid)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    let mut grid = parse_input(input);
    assert_eq!(part1(&mut grid).unwrap(), 4);

    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    let mut grid = parse_input(input);
    assert_eq!(part1(&mut grid).unwrap(), 8);
}

#[test]
fn part2_example_input() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    let mut grid = parse_input(input);
    assert_eq!(part2(&mut grid).unwrap(), 4);
    assert_eq!(part2_raycast(&mut grid).unwrap(), 4);

    let input = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";
    let mut grid = parse_input(input);
    assert_eq!(part2(&mut grid).unwrap(), 4);
    assert_eq!(part2_raycast(&mut grid).unwrap(), 4);

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
    let mut grid = parse_input(input);
    assert_eq!(part2(&mut grid).unwrap(), 8);
    assert_eq!(part2_raycast(&mut grid).unwrap(), 8);

    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    let mut grid = parse_input(input);
    assert_eq!(part2(&mut grid).unwrap(), 10);
    assert_eq!(part2_raycast(&mut grid).unwrap(), 10);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut grid = parse_input(input);
    assert_eq!(part1(&mut grid).unwrap(), 6725);
    assert_eq!(part2(&mut grid).unwrap(), 383);
    assert_eq!(part2_raycast(&mut grid).unwrap(), 383);
}
