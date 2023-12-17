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
fn bfs(start: Coord, map: &[Vec<u8>], min_step: usize, max_step: usize) -> usize {
    let bound = (map.len() as isize, map[0].len() as isize);

    let mut queue = VecDeque::new();
    queue.push_back((start, RIGHT, 0));
    queue.push_back((start, DOWN, 0));

    let mut min_loss = HashMap::new();

    let mut result = usize::MAX;
    while let Some((pos, dir, loss)) = queue.pop_front() {
        if pos.0 == bound.0 - 1 && pos.1 == bound.1 - 1 {
            result = result.min(loss);
            continue;
        }
        for i in min_step..=max_step {
            if let Some((next, next_loss)) = next_nth(pos, dir, i, map) {
                for nd in [turn_left(dir), turn_right(dir)] {
                    if let Some(p_l) = min_loss.get(&(next, nd)) {
                        if *p_l <= loss + next_loss {
                            continue;
                        }
                    }
                    min_loss.insert((next, nd), loss + next_loss);
                    if loss + next_loss < result {
                        queue.push_back((next, nd, loss + next_loss))
                    }
                }
            } else {
                break;
            }
        }
    }
    result
}

#[allow(dead_code)]
fn dfs(
    start: Coord,
    dir: u8,
    map: &[Vec<u8>],
    visited: &mut HashSet<Coord>,
    min_step: usize,
    max_step: usize,
) -> Option<usize> {
    let bound = (map.len() as isize, map[0].len() as isize);
    if start == (bound.0 - 1, bound.1 - 1) {
        Some(0)
    } else {
        if visited.insert(start) {
            let mut result = usize::MAX;
            for i in min_step..=max_step {
                if let Some((next, next_loss)) = next_nth(start, dir, i, map) {
                    if next_loss >= result {
                        break;
                    }
                    for nd in [turn_left(dir), turn_right(dir)] {
                        if let Some(remain_loss) = dfs(next, nd, map, visited, min_step, max_step) {
                            result = result.min(next_loss + remain_loss);
                        }
                    }
                }
            }
            visited.remove(&start);
            if result != usize::MAX {
                return Some(result);
            }
        }
        None
    }
}

fn part1(map: &[Vec<u8>]) -> Result<usize> {
    let _start = Instant::now();

    // let result = dfs((0, 0), RIGHT, map, &mut HashSet::new()).unwrap_or(usize::MAX);
    // let result = result.min(dfs((0, 0), DOWN, map, &mut HashSet::new()).unwrap_or(usize::MAX));
    let result = bfs((0, 0), map, 1, 3);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;

    Ok(result)
}
fn part2(map: &[Vec<u8>]) -> Result<usize> {
    let _start = Instant::now();

    let result = bfs((0, 0), map, 4, usize::MAX);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;

    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = parse_input(input);
    part1(&map)?;
    part2(&map)?;
    Ok(())
}
#[test]
fn simple_input() {
    let input = "19999
11111";
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 13);

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
    assert_eq!(part2(&map).unwrap(), 94);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let map = parse_input(input);
    assert_eq!(part1(&map).unwrap(), 674);
    assert_eq!(part2(&map).unwrap(), 674);
}
