use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<u8>> {
    input
        .as_ref()
        .trim()
        .split(',')
        .map(|p| p.trim().bytes().collect())
        .collect()
}

fn hash(step: &[u8]) -> usize {
    step.iter().fold(0, |h, &i| ((h + i as usize) * 17) % 256)
}

fn part1(steps: &[Vec<u8>]) -> Result<usize> {
    let _start = Instant::now();

    let result = steps.iter().map(|s| hash(s)).sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn step_to_instr(step: &[u8]) -> (usize, usize, Option<usize>) {
    let i = step.iter().position(|&c| c == b'=' || c == b'-').unwrap();
    let k = hash(&step[..i]);
    let v = step.get(i + 1).map(|n| (n - b'0') as usize);
    assert!(i <= 8); // usize is u64, max lable length is 8
    let label = step[..i].iter().fold(0, |l, &b| l * 256 + b as usize);
    (k, label, v)
}

// BTreeMap
fn part2(steps: &[Vec<u8>]) -> Result<usize> {
    let _start = Instant::now();

    let mut map = vec![vec![]; 256];

    for step in steps {
        let (k, l, v) = step_to_instr(step);
        let p = map[k].iter().position(|(i, _)| i == &l);
        match (v, p) {
            (Some(f), None) => map[k].push((l, f)),
            (Some(f), Some(i)) => {
                map[k][i] = (l, f);
            }
            (None, Some(i)) => {
                map[k].remove(i);
            }
            _ => (),
        }
    }

    let result = map
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, f))| (i + 1) * (j + 1) * f)
                .sum::<usize>()
        })
        .sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let steps = parse_input(input);
    part1(&steps)?;
    part2(&steps)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let steps = parse_input(input);
    assert_eq!(part1(&steps).unwrap(), 1320);
    assert_eq!(part2(&steps).unwrap(), 145);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let steps = parse_input(input);
    assert_eq!(part1(&steps).unwrap(), 262454);
    assert_eq!(part2(&steps).unwrap(), 145);
}
