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

fn parse_input<T: AsRef<str>>(input: T) -> (Coord, HashSet<Coord>) {
    let mut start = (-1, -1);
    let mut map = HashSet::new();
    for (i, l) in input.as_ref().split_whitespace().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '.' || c == 'S' {
                map.insert((i as isize, j as isize));
            }
            if c == 'S' {
                start = (i as isize, j as isize);
            }
        }
    }
    (start, map)
}

fn bfs(start: Coord, mut step: usize, map: &HashSet<Coord>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() && step > 0 {
        step -= 1;
        let l = queue.len();
        let mut visited = HashSet::new();
        for _ in 0..l {
            let curr = queue.pop_front().unwrap();
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (curr.0 + dx, curr.1 + dy);
                if map.contains(&(nx, ny)) && visited.insert((nx, ny)) {
                    queue.push_back((nx, ny))
                }
            }
        }
    }
    queue.len()
}

fn part1(start: Coord, map: &HashSet<Coord>) -> Result<usize> {
    let _start = Instant::now();

    let result = bfs(start, 64, map);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (start, map) = parse_input(input);
    part1(start, &map)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let (start, map) = parse_input(input);
    assert_eq!(bfs(start, 6, &map), 16);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (start, map) = parse_input(input);
    assert_eq!(part1(start, &map).unwrap(), 3600);
}
