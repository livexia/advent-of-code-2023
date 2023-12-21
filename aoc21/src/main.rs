use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);

fn parse_input<T: AsRef<str>>(input: T) -> (Coord, Coord, HashSet<Coord>) {
    let mut start = (-1, -1);
    let mut bound = (-1, -1);
    let mut map = HashSet::new();
    for (i, l) in input.as_ref().split_whitespace().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '.' || c == 'S' {
                map.insert((i as isize, j as isize));
            }
            if c == 'S' {
                start = (i as isize, j as isize);
            }
            bound.1 = bound.1.max(j as isize + 1);
        }
        bound.0 = bound.0.max(i as isize + 1);
    }
    (start, bound, map)
}

fn normalize_coord(pos: Coord, bound: Coord) -> (Coord, Coord) {
    let origin = (pos.0.rem_euclid(bound.0), pos.1.rem_euclid(bound.1));
    let dis = (pos.0 - origin.0, pos.1 - origin.1);
    (origin, dis)
}

fn bfs(start: Coord, mut step: usize, bound: Coord, map: &HashSet<Coord>) -> usize {
    let mut cache: HashMap<Coord, Vec<Coord>> = HashMap::new();

    for i in 0..bound.0 {
        for j in 0..bound.1 {
            if !map.contains(&(i, j)) {
                continue;
            }
            let e = cache.entry((i, j)).or_default();
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (i + dx, j + dy);
                let (origin, _) = normalize_coord((nx, ny), bound);
                if map.contains(&origin) {
                    e.push((nx, ny));
                }
            }
        }
    }

    let mut queue = HashSet::new();
    queue.insert(start);
    while step > 0 {
        step -= 1;

        queue = queue
            .iter()
            .flat_map(|&curr| {
                let (origin, dis) = normalize_coord(curr, bound);
                cache
                    .get(&origin)
                    .unwrap()
                    .iter()
                    .map(move |(x, y)| (x + dis.0, y + dis.1))
            })
            .collect();
    }
    queue.len()
}

fn part1(start: Coord, bound: Coord, map: &HashSet<Coord>) -> Result<usize> {
    let _start = Instant::now();

    let result = bfs(start, 64, bound, map);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(start: Coord, bound: Coord, map: &HashSet<Coord>) -> Result<usize> {
    let _start = Instant::now();

    let result = bfs(start, 26501365, bound, map);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (start, bound, map) = parse_input(input);
    part1(start, bound, &map)?;
    part2(start, bound, &map)?;
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
    let (start, bound, map) = parse_input(input);
    assert_eq!(bfs(start, 6, bound, &map), 16);
    assert_eq!(bfs(start, 10, bound, &map), 50);
    assert_eq!(bfs(start, 50, bound, &map), 1594);
    assert_eq!(bfs(start, 100, bound, &map), 6536);
    assert_eq!(bfs(start, 500, bound, &map), 167004);
    assert_eq!(bfs(start, 1000, bound, &map), 668697);
    assert_eq!(bfs(start, 5000, bound, &map), 16733044);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (start, bound, map) = parse_input(input);
    assert_eq!(part1(start, bound, &map).unwrap(), 3600);
    assert_eq!(part2(start, bound, &map).unwrap(), 3600);
}
