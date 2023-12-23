use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (usize, usize);

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<char>> {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect()
}

fn dfs(pos: Coord, trails: &[Vec<char>], visited: &mut HashSet<Coord>) -> usize {
    if pos.1 == trails.len() - 1 {
        0
    } else {
        let (x, y) = pos;
        let possible = match trails[x][y] {
            '.' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            '>' => vec![(0, 1)],
            '<' => vec![(0, -1)],
            '^' => vec![(-1, 0)],
            'v' => vec![(1, 0)],
            _ => return 0,
        };
        let mut count = 0;
        for (dx, dy) in possible {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx as usize >= trails.len() || ny as usize >= trails[0].len() {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if trails[nx][ny] != '#' && visited.insert((nx, ny)) {
                count = count.max(1 + dfs((nx, ny), trails, visited));
                visited.remove(&(nx, ny));
            }
        }
        count
    }
}

fn part1(trails: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let result = (0..trails[0].len())
        .map(|y| dfs((0, y), trails, &mut HashSet::new()))
        .max()
        .unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let trails = parse_input(input);
    part1(&trails)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let trails = parse_input(input);
    assert_eq!(part1(&trails).unwrap(), 94);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let trails = parse_input(input);
    assert_eq!(part1(&trails).unwrap(), 2394);
    assert_eq!(2, 2);
}