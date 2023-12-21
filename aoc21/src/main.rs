use std::collections::HashSet;
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

fn bfs(start: Coord, step: usize, bound: Coord, map: &HashSet<Coord>) -> usize {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut count = 0;
    let mut f = [0, 0, 0];

    for i in 1..=step {
        let mut temp = HashSet::with_capacity(visited.len() * 3);
        for curr in visited {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (curr.0 + dx, curr.1 + dy);
                let (origin, _) = normalize_coord((nx, ny), bound);
                if map.contains(&origin) {
                    temp.insert((nx, ny));
                }
            }
        }
        visited = temp;
        if (step as isize) % bound.0 == (i as isize) % bound.0 {
            f[count] = visited.len();
            // let x = i as isize / bound.0;
            // ax^2 + bx + c = visited.len()
            count += 1;
            if count == 3 {
                // solve a b c
                let c = f[0];
                let b = (4 * (f[1] - c) - (f[2] - c)) / 2;
                let a = f[1] - b - c;
                dbg!(a, b, c);
                let x = step / bound.0 as usize;
                return a * x * x + b * x + c;
            }
        }
    }

    visited.len()
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
    // assert_eq!(bfs(start, 65, bound, &map), 3720);
    // assert_eq!(bfs(start, 65 + 131, bound, &map), 33150);
    // assert_eq!(bfs(start, 65 + 131 * 2, bound, &map), 91890);
    assert_eq!(part2(start, bound, &map).unwrap(), 599763113936220);
}
