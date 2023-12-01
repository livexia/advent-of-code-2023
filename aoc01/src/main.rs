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

    let mut sum = 0;
    for line in input.lines() {
        let mut digits = vec![];
        let raw_chars: Vec<char> = line.chars().collect();

        let mut chars = &raw_chars[..];
        while let (Some(remain_chars), digit_opt) = find_digit_with_letters(&chars) {
            if let Some(digit) = digit_opt {
                digits.push(digit);
            }
            chars = remain_chars;
        }
        sum += digits[0] * 10 + digits.last().unwrap();
    }

    writeln!(io::stdout(), "Part 2: {}", sum)?;

    part1(&input)?;
    // part2()?;
    Ok(())
}

fn find_digit_with_letters(chars: &[char]) -> (Option<&[char]>, Option<u32>) {
    if chars.is_empty() {
        return (None, None);
    }
    let l = chars.len();
    match chars[0] {
        'o' => {
            if 3 <= l && chars[1] == 'n' && chars[2] == 'e' {
                return (Some(&chars[1..]), Some(1));
            }
        }
        't' => {
            if 3 <= l && chars[1] == 'w' && chars[2] == 'o' {
                return (Some(&chars[1..]), Some(2));
            }
            if 5 <= l && chars[0..5] == ['t', 'h', 'r', 'e', 'e'] {
                return (Some(&chars[1..]), Some(3));
            }
        }
        'f' => {
            if 4 <= l && chars[0..4] == ['f', 'o', 'u', 'r'] {
                return (Some(&chars[1..]), Some(4));
            }
            if 4 <= l && chars[0..4] == ['f', 'i', 'v', 'e'] {
                return (Some(&chars[1..]), Some(5));
            }
        }
        's' => {
            if 3 <= l && chars[0..3] == ['s', 'i', 'x'] {
                return (Some(&chars[1..]), Some(6));
            }
            if 5 <= l && chars[..5] == ['s', 'e', 'v', 'e', 'n'] {
                return (Some(&chars[1..]), Some(7));
            }
        }
        'e' => {
            if 5 <= l && chars[..5] == ['e', 'i', 'g', 'h', 't'] {
                return (Some(&chars[1..]), Some(8));
            }
        }
        'n' => {
            if 4 <= l && chars[..4] == ['n', 'i', 'n', 'e'] {
                return (Some(&chars[1..]), Some(9));
            }
        }
        c => {
            if let Some(n) = c.to_digit(10) {
                return (Some(&chars[1..]), Some(n));
            }
        }
    }
    return (Some(&chars[1..]), None);
}

fn part1(input: &str) -> Result<()> {
    let start = Instant::now();

    let mut sum = 0;
    for line in input.lines() {
        let first_digit = match line.chars().find(|c| c.is_numeric()) {
            None => return err!("nuable to find the first digit for the calibration value"),
            Some(c) => c.to_digit(10),
        }
        .unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += first_digit * 10 + last_digit;
    }

    writeln!(io::stdout(), "Part 1: {}", sum)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(())
}
