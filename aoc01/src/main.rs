use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn match_digit_with_letters(chars: &[char]) -> Option<(u32, usize)> {
    if chars.is_empty() {
        return None;
    }
    let l = chars.len();
    match chars[0] {
        'o' => {
            if 3 <= l && chars[..3] == ['o', 'n', 'e'] {
                return Some((1, 1));
            }
        }
        't' => {
            if 3 <= l && chars[..3] == ['t', 'w', 'o'] {
                return Some((2, 2));
            }
            if 5 <= l && chars[..5] == ['t', 'h', 'r', 'e', 'e'] {
                return Some((3, 3));
            }
        }
        'f' => {
            if 4 <= l && chars[..4] == ['f', 'o', 'u', 'r'] {
                return Some((4, 1));
            }
            if 4 <= l && chars[..4] == ['f', 'i', 'v', 'e'] {
                return Some((5, 3));
            }
        }
        's' => {
            if 3 <= l && chars[..3] == ['s', 'i', 'x'] {
                return Some((6, 3));
            }
            if 5 <= l && chars[..5] == ['s', 'e', 'v', 'e', 'n'] {
                return Some((7, 1));
            }
        }
        'e' => {
            if 5 <= l && chars[..5] == ['e', 'i', 'g', 'h', 't'] {
                return Some((8, 4));
            }
        }
        'n' => {
            if 4 <= l && chars[..4] == ['n', 'i', 'n', 'e'] {
                return Some((9, 2));
            }
        }
        c => {
            if let Some(n) = c.to_digit(10) {
                return Some((n, 1));
            }
        }
    }
    None
}

fn part1(input: &str) -> Result<u32> {
    let start = Instant::now();

    let mut sum = 0;
    for line in input.lines() {
        let first_digit = match line.chars().find_map(|c| c.to_digit(10)) {
            None => return err!("unable to find the first digit for the calibration value"),
            Some(c) => c,
        };
        let last_digit = match line.chars().rev().find_map(|c| c.to_digit(10)) {
            None => return err!("unable to find the first digit for the calibration value"),
            Some(c) => c,
        };
        sum += first_digit * 10 + last_digit;
    }

    writeln!(io::stdout(), "Part 1: {}", sum)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn part2(input: &str) -> Result<u32> {
    let start = Instant::now();

    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i <= chars.len() {
            if let Some((digit, offset)) = match_digit_with_letters(&chars[i..]) {
                if first_digit == 0 {
                    first_digit = digit
                }
                last_digit = digit;
                i += offset;
            } else {
                i += 1;
            }
        }

        sum += first_digit * 10 + last_digit;
    }
    writeln!(io::stdout(), "Part 2: {}", sum)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

#[test]
fn example_input() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input).unwrap(), 142);

    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(input).unwrap(), 281);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();

    assert_eq!(part1(&input).unwrap(), 54390);
    assert_eq!(part2(&input).unwrap(), 54277);
}
