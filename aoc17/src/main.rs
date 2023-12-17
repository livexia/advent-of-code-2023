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

const UP: u8 = 0;
const LEFT: u8 = 1;
const DOWN: u8 = 2;
const RIGHT: u8 = 3;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<u8>> {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn turn_left(dir: u8) -> u8 {
    (dir + 1) % 4
}

fn turn_right(dir: u8) -> u8 {
    (dir + 3) % 4
}

fn next_nth(curr: Coord, dir: u8, step: usize, map: &[Vec<u8>]) -> Option<(Coord, usize)> {
    let bound = (map.len() as isize, map[0].len() as isize);
    if step == 0 {
        Some((curr, 0))
    } else {
        let next = match dir {
            UP => (curr.0 - 1, curr.1),
            DOWN => (curr.0 + 1, curr.1),
            LEFT => (curr.0, curr.1 - 1),
            RIGHT => (curr.0, curr.1 + 1),
            _ => unreachable!("unknown direction: {dir}"),
        };
        if next.0 >= 0 && next.1 >= 0 && next.0 < bound.0 && next.1 < bound.1 {
            let loss = map[next.0 as usize][next.1 as usize] as usize;
            next_nth(next, dir, step - 1, map).map(|(c, l)| (c, loss + l))
        } else {
            None
        }
    }
}

#[allow(dead_code)]
// Wrong Code
fn bfs(start: Coord, map: &[Vec<u8>]) -> usize {
    let bound = (map.len() as isize, map[0].len() as isize);

    let mut queue = VecDeque::new();
    queue.push_back((start, RIGHT, 0));
    queue.push_back((start, DOWN, 0));

    let mut visited = HashSet::new();

    let mut result = usize::MAX;
    while let Some((pos, dir, loss)) = queue.pop_front() {
        if pos == (1, 3) || pos == (1, 5) {
            println!("{loss}");
        }
        if pos.0 == bound.0 - 1 && pos.1 == bound.1 - 1 {
            result = result.min(loss);
            continue;
        }
        // println!("{pos:?} {dir}-> ");
        for i in 1..=3 {
            if let Some((next, next_loss)) = next_nth(pos, dir, i, map) {
                for nd in [turn_left(dir), turn_right(dir)] {
                    // print!("{next:?}, {nd} ");
                    if visited.insert((next, nd)) {
                        queue.push_back((next, nd, loss + next_loss))
                    }
                }
            }
        }
        // println!()
    }
    result
}

fn dfs(
    start: Coord,
    dir: u8,
    map: &[Vec<u8>],
    visited: &mut HashSet<(Coord, u8)>,
) -> Option<usize> {
    let bound = (map.len() as isize, map[0].len() as isize);
    if start == (bound.0 - 1, bound.1 - 1) {
        Some(0)
    } else {
        if visited.insert((start, dir)) {
            let mut result = usize::MAX;
            for i in 1..=3 {
                if let Some((next, next_loss)) = next_nth(start, dir, i, map) {
                    for nd in [turn_left(dir), turn_right(dir)] {
                        if let Some(remain_loss) = dfs(next, nd, map, visited) {
                            result = result.min(next_loss + remain_loss);
                        }
                    }
                }
            }
            visited.remove(&(start, dir));
            if result != usize::MAX {
                return Some(result);
            }
        }
        None
    }
}

fn part1(map: &[Vec<u8>]) -> Result<usize> {
    let _start = Instant::now();

    let result = dfs((0, 0), RIGHT, map, &mut HashSet::new()).unwrap_or(usize::MAX);
    let result = result.min(dfs((0, 0), DOWN, map, &mut HashSet::new()).unwrap_or(usize::MAX));

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;

    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input);
    part1(&map)?;
    // part2()?;
    Ok(())
}
#[test]
fn simple_input() {
    let input = "1119
1111";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 4);

    let input = "11119999
99911199
99999111";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 9);
}

#[test]
fn example_input() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 102);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 102);
}
