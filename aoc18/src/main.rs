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
            let hex = isize::from_str_radix(raw, 16)?;

            let dir = match hex & 0xf {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => return err!("unable to parse plan with rgb: {s:?}"),
            };
            let step = hex >> 4;
            return Ok(Self {
                dir,
                step,
                rgb: None,
            });
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
            grid.extend((x - step..x).map(|i| (i, y)));
            (x - step, y)
        }
        'D' => {
            grid.extend((x + 1..=x + step).map(|i| (i, y)));
            (x + step, y)
        }
        'L' => {
            grid.extend((y - step..y).map(|j| (x, j)));
            (x, y - step)
        }
        'R' => {
            grid.extend((y + 1..=y + step).map(|j| (x, j)));
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

fn ray_cast(grid: &HashSet<Coord>) -> usize {
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

    let grid = dig_trench(plans);

    let result = ray_cast(&grid);

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(plans: &[Plan]) -> Result<usize> {
    let _start = Instant::now();

    let plans = plans
        .iter()
        .map(|p| Plan::from_rgb(p.rgb.as_ref().unwrap()))
        .collect::<Result<Vec<_>>>()?;

    let mut interior = 0;
    let mut edge = 0;
    let mut curr = (0, 0);
    for plan in &plans {
        let (x, y) = curr;
        let step = plan.step;
        let next = match plan.dir {
            'U' => (x - step, y),
            'D' => (x + step, y),
            'L' => (x, y - step),
            'R' => (x, y + step),
            _ => unreachable!(),
        };
        edge += step;
        interior += x * next.1 - next.0 * y;
        curr = next;
    }
    let result = ((edge + interior.abs()) / 2 + 1).unsigned_abs();

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
    assert_eq!(part2(&plans).unwrap(), 194033958221830);
}
