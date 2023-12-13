use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::once;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut result = vec![];
    for line in input.as_ref().lines().filter(|l| !l.trim().is_empty()) {
        if let Some((raw_springs, counters)) = line.trim().split_once(' ') {
            let springs = raw_springs.trim().chars().collect();
            let counters = counters
                .trim()
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();
            result.push((springs, counters));
        }
    }
    result
}

fn count_arrangement(
    curr_spring: char,
    springs: &[char],
    counters: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    match curr_spring {
        '#' => {
            let remain = counters[0] - 1;
            if remain <= springs.len()
                && springs[..remain].iter().all(|c| c != &'.')
                && springs.get(remain) != Some(&'#')
            {
                // skip ahead
                if remain == springs.len() {
                    (counters.len() == 1) as usize
                } else if counters.len() == 1 {
                    springs[remain + 1..].iter().all(|c| c != &'#') as usize
                } else {
                    count_arrangement('.', &springs[remain + 1..], &counters[1..], cache)
                }
            } else {
                0
            }
        }
        '.' => {
            if let Some(i) = (0..springs.len()).find(|&i| springs[i] != '.') {
                count_arrangement(springs[i], &springs[i + 1..], counters, cache)
            } else {
                0
            }
        }
        '?' => {
            if let Some(&c) = cache.get(&(springs.len(), counters.len())) {
                c
            } else {
                let c = count_arrangement('#', springs, counters, cache)
                    + count_arrangement('.', springs, counters, cache);
                cache.insert((springs.len(), counters.len()), c);
                c
            }
        }
        _ => unreachable!("Wrong spring record: {:?}", springs),
    }
}

fn count_arrangement_with_unfold(springs: &[char], counters: &[usize], rate: usize) -> usize {
    let s: Vec<_> = springs
        .iter()
        .cloned()
        .chain(once('?'))
        .cycle()
        .take(springs.len() * rate + rate - 1)
        .collect();
    let c: Vec<_> = counters
        .iter()
        .cloned()
        .cycle()
        .take(counters.len() * rate)
        .collect();
    count_arrangement(s[0], &s[1..], &c, &mut HashMap::new())
}

fn part1(records: &[(Vec<char>, Vec<usize>)]) -> Result<usize> {
    let _start = Instant::now();

    let result = records
        .iter()
        .map(|(s, c)| count_arrangement_with_unfold(s, c, 1))
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(records: &[(Vec<char>, Vec<usize>)]) -> Result<usize> {
    let _start = Instant::now();

    let result = records
        .iter()
        .map(|(s, c)| count_arrangement_with_unfold(s, c, 5))
        .sum();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let records = parse_input(input);
    part1(&records)?;
    part2(&records)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    let records = parse_input(input);
    let (s0, c0) = (vec!['#', '.', '#', '.', '#', '#', '#'], vec![1, 1, 3]);
    assert_eq!(count_arrangement_with_unfold(&s0, &c0, 1), 1);

    let (s1, c1) = &records[5];
    assert_eq!(count_arrangement_with_unfold(s1, c1, 1), 10);
    assert_eq!(part1(&records).unwrap(), 21);
    assert_eq!(part2(&records).unwrap(), 525152);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let records = parse_input(input);
    assert_eq!(part1(&records).unwrap(), 7694);
    assert_eq!(part2(&records).unwrap(), 5071883216318);
}
