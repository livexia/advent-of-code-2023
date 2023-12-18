use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);
type Direction = char;

#[derive(Debug)]
struct Plan {
    dir: Direction,
    step: isize,
    rgb: Option<String>,
}

impl FromStr for Plan {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        if let Some(dir) = parts.next() {
            if dir.len() == 1 {
                let dir = dir.chars().next().unwrap();
                if let Some(step) = parts.next() {
                    let step = step.parse::<isize>()?;
                    if let Some(rgb) = parts.next() {
                        return Ok(Plan {
                            dir,
                            step,
                            rgb: Some(rgb.to_string()),
                        });
                    }
                }
            }
        }
        err!("unable to parse plan with: {s:?}")
    }
}

impl Plan {
    fn from_rgb(s: &str) -> Result<Self> {
        let s = s.replace(['(', ')'], "");
        if let Some(raw) = s.strip_prefix('#') {
            let mut raw = raw.chars().rev();
            if let Some(dir) = raw.next() {
                let dir = match dir {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => return err!("unable to parse plan with rgb: {s:?}"),
                };
                let step = raw.fold(0, |sum, d| sum * 16 + d.to_digit(16).unwrap()) as isize;
                return Ok(Self {
                    dir,
                    step,
                    rgb: None,
                });
            }
        }
        err!("unable to parse plan with rgb: {s:?}")
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Plan>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_>>()
}

#[allow(dead_code)]
fn display_grid(grid: &HashSet<Coord>) -> String {
    let mut s = String::new();
    let min_x = grid.iter().min().unwrap().0;
    let max_x = grid.iter().max().unwrap().0;
    let min_y = grid.iter().min_by_key(|k| k.1).unwrap().1;
    let max_y = grid.iter().max_by_key(|k| k.1).unwrap().1;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if grid.contains(&(x, y)) {
                s.push('#')
            } else {
                s.push('.')
            }
        }
        s.push('\n')
    }
    s
}

fn dig_edge(pos: Coord, plan: &Plan, grid: &mut HashSet<Coord>) -> Coord {
    let (x, y) = pos;
    let step = plan.step;
    match plan.dir {
        'U' => {
            for i in x - step..x {
                grid.insert((i, y));
            }
            (x - step, y)
        }
        'D' => {
            for i in x + 1..=(x + step) {
                grid.insert((i, y));
            }
            (x + step, y)
        }
        'L' => {
            for j in y - step..y {
                grid.insert((x, j));
            }
            (x, y - step)
        }
        'R' => {
            for j in y + 1..=(y + step) {
                grid.insert((x, j));
            }
            (x, y + step)
        }
        _ => unreachable!("Wrong direction: {plan:?}"),
    }
}

fn dig_trench(plans: &[Plan]) -> HashSet<Coord> {
    let mut grid = HashSet::new();

    let mut curr = (0, 0);
    for plan in plans {
        curr = dig_edge(curr, plan, &mut grid);
    }

    grid
}

fn ray_cast(grid: &mut HashSet<Coord>) -> usize {
    let min_x = grid.iter().min().unwrap().0;
    let max_x = grid.iter().max().unwrap().0;
    let min_y = grid.iter().min_by_key(|k| k.1).unwrap().1;
    let max_y = grid.iter().max_by_key(|k| k.1).unwrap().1;

    let mut total_count = 0;
    for x in min_x..=max_x {
        let mut count = 0;
        for y in min_y..=max_y {
            if !grid.contains(&(x, y)) {
                if count % 2 == 1 {
                    total_count += 1;
                }
            } else if (grid.contains(&(x + 1, y)) && grid.contains(&(x, y + 1)))
                || (grid.contains(&(x, y - 1)) && grid.contains(&(x + 1, y)))
                || (grid.contains(&(x - 1, y)) && grid.contains(&(x + 1, y)))
            {
                count += 1
            }
        }
    }
    total_count + grid.len()
}

fn part1(plans: &[Plan]) -> Result<usize> {
    let _start = Instant::now();

    let mut grid = dig_trench(plans);

    let result = ray_cast(&mut grid);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn plans_to_edges(plans: &[Plan]) -> (Vec<(Coord, Coord, Direction)>, Coord, Coord) {
    let mut top_left = (0, 0);
    let mut bottom_right = (0, 0);
    let mut edges = Vec::with_capacity(plans.len());
    let mut curr = (0, 0);
    for plan in plans {
        let (x, y) = curr;
        let step = plan.step;
        let next = match plan.dir {
            'U' => (x - step, y),
            'D' => (x + step, y),
            'L' => (x, y - step),
            'R' => (x, y + step),
            _ => unreachable!("Wrong direction: {plan:?}"),
        };
        top_left.0 = top_left.0.min(x.min(next.0));
        top_left.1 = top_left.1.min(y.min(next.1));
        bottom_right.0 = top_left.0.max(x.max(next.0));
        bottom_right.1 = top_left.1.max(y.max(next.1));

        edges.push((curr, next, plan.dir));
        curr = next;
    }
    (edges, top_left, bottom_right)
}

fn on_edge(pos: Coord, edge: (Coord, Coord)) -> bool {
    (pos.0.abs_diff(edge.0 .0)
        + pos.1.abs_diff(edge.0 .1)
        + pos.0.abs_diff(edge.1 .0)
        + pos.1.abs_diff(edge.1 .1))
        == edge.0 .0.abs_diff(edge.1 .0) + edge.0 .1.abs_diff(edge.1 .1)
}

fn on_edges(pos: Coord, edges: &[(Coord, Coord, Direction)]) -> Vec<Direction> {
    let mut result = vec![];
    for &(start, end, dir) in edges {
        if on_edge(pos, (start, end)) {
            result.push(dir);
        }
    }
    result.sort();
    result
}

fn ray_cast_with_edges(edges: &[(Coord, Coord, Direction)], top_left: Coord, bottom_right: Coord) {}

fn part2(plans: &[Plan]) -> Result<usize> {
    let _start = Instant::now();

    let plans = plans
        .iter()
        .map(|p| Plan::from_rgb(p.rgb.as_ref().unwrap()))
        .collect::<Result<Vec<_>>>()?;

    let mut grid = dig_trench(&plans);
    let result = ray_cast(&mut grid);

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let plans = parse_input(input)?;
    part1(&plans)?;
    part2(&plans)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    let plans = parse_input(input).unwrap();
    assert_eq!(part1(&plans).unwrap(), 62);
    assert_eq!(part2(&plans).unwrap(), 952408144115);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let plans = parse_input(input).unwrap();
    assert_eq!(part1(&plans).unwrap(), 49897);
}
