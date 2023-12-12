use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<(Vec<char>, Vec<usize>)> {
    let mut result = vec![];
    for line in input.as_ref().lines().filter(|l| !l.trim().is_empty()) {
        if let Some((springs, counters)) = line.trim().split_once(' ') {
            let springs = springs.trim().chars().collect();
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
    curr_counter: usize,
    counters: &[usize],
) -> usize {
    // println!(
    //     "{} {:?} {} {:?}",
    //     curr_spring, springs, curr_counter, counters
    // );
    match curr_spring {
        '#' => {
            if springs.is_empty() {
                if counters.len() == 1 && curr_counter == 1 {
                    return 1;
                }
                return 0;
            }
            if curr_counter == 0 {
                // contiguous group is over, but cuurent is damage, this arrangement is impossible
                0
            } else {
                count_arrangement(springs[0], &springs[1..], curr_counter - 1, counters)
            }
        }
        '.' => {
            if curr_counter != 0 {
                // contiguous group is not over, but cuurent is operational
                // this arrangement is impossible or contiguous is not yet start
                if curr_counter != counters[0] || springs.is_empty() {
                    return 0;
                }
                count_arrangement(springs[0], &springs[1..], curr_counter, counters)
            } else if springs.is_empty() {
                if counters.len() == 1 {
                    1
                } else {
                    0
                }
            } else if counters.len() == 1 {
                count_arrangement(springs[0], &springs[1..], 0, counters)
            } else {
                count_arrangement(springs[0], &springs[1..], counters[1], &counters[1..])
            }
        }
        '?' => {
            count_arrangement('#', springs, curr_counter, counters)
                + count_arrangement('.', springs, curr_counter, counters)
        }
        _ => unreachable!("Wrong spring record: {:?}", springs),
    }
}

fn part1(records: &[(Vec<char>, Vec<usize>)]) -> Result<usize> {
    let _start = Instant::now();

    let result = records
        .iter()
        .map(|(s, c)| count_arrangement(s[0], &s[1..], c[0], c))
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let records = parse_input(input);
    part1(&records)?;
    // part2()?;
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
    assert_eq!(count_arrangement(s0[0], &s0[1..], c0[0], &c0), 1);

    let (s1, c1) = &records[5];
    assert_eq!(count_arrangement(s1[0], &s1[1..], c1[0], c1), 10);
    assert_eq!(part1(&records).unwrap(), 21);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let records = parse_input(input);
    assert_eq!(part1(&records).unwrap(), 7694);
}
