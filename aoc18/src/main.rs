use std::collections::HashMap;
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
type Rgb = String;

#[derive(Debug)]
struct Plan {
    dir: char,
    step: isize,
    rgb: Rgb,
}

impl FromStr for Plan {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        if let Some(dir) = parts.next() {
            if dir.len() == 1 {
                let dir = dir.chars().next().unwrap();
                if let Some(steps) = parts.next() {
                    let steps = steps.parse::<isize>()?;
                    if let Some(rgb) = parts.next() {
                        return Ok(Plan {
                            dir,
                            step: steps,
                            rgb: rgb.to_string(),
                        });
                    }
                }
            }
        }
        err!("unable to parse plan with: {s:?}")
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
fn display_grid(grid: &HashMap<Coord, Rgb>) -> String {
    let mut s = String::new();
    let min_x = grid.keys().min().unwrap().0;
    let max_x = grid.keys().max().unwrap().0;
    let min_y = grid.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = grid.keys().max_by_key(|k| k.1).unwrap().1;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if grid.contains_key(&(x, y)) {
                s.push('#')
            } else {
                s.push('.')
            }
        }
        s.push('\n')
    }
    s
}

fn dig_edge(pos: Coord, plan: &Plan, grid: &mut HashMap<Coord, Rgb>) -> Coord {
    let (x, y) = pos;
    let step = plan.step;
    match plan.dir {
        'U' => {
            for i in x - step..x {
                grid.insert((i, y), plan.rgb.clone());
            }
            (x - step, y)
        }
        'D' => {
            for i in x + 1..=(x + step) {
                grid.insert((i, y), plan.rgb.clone());
            }
            (x + step, y)
        }
        'L' => {
            for j in y - step..y {
                grid.insert((x, j), plan.rgb.clone());
            }
            (x, y - step)
        }
        'R' => {
            for j in y + 1..=(y + step) {
                grid.insert((x, j), plan.rgb.clone());
            }
            (x, y + step)
        }
        _ => unreachable!("Wrong direction: {plan:?}"),
    }
}

fn dig_trench(plans: &[Plan]) -> HashMap<Coord, Rgb> {
    let mut grid = HashMap::new();

    let mut curr = (0, 0);
    for plan in plans {
        curr = dig_edge(curr, plan, &mut grid);
    }

    grid
}

fn ray_cast(grid: &mut HashMap<Coord, Rgb>) {
    let min_x = grid.keys().min().unwrap().0;
    let max_x = grid.keys().max().unwrap().0;
    let min_y = grid.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = grid.keys().max_by_key(|k| k.1).unwrap().1;

    for x in min_x..=max_x {
        let mut count = 0;
        for y in min_y..=max_y {
            if let std::collections::hash_map::Entry::Vacant(e) = grid.entry((x, y)) {
                if count % 2 == 1 {
                    e.insert("".to_string());
                }
            } else if (grid.contains_key(&(x + 1, y)) && grid.contains_key(&(x, y + 1)))
                || (grid.contains_key(&(x, y - 1)) && grid.contains_key(&(x + 1, y)))
                || (grid.contains_key(&(x - 1, y)) && grid.contains_key(&(x + 1, y)))
            {
                count += 1
            }
        }
    }
}

fn part1(plans: &[Plan]) -> Result<usize> {
    let _start = Instant::now();

    let mut grid = dig_trench(plans);
    ray_cast(&mut grid);

    let result = grid.len();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let plans = parse_input(input)?;
    part1(&plans)?;
    // part2()?;
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
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let plans = parse_input(input).unwrap();
    assert_eq!(part1(&plans).unwrap(), 49897);
}
