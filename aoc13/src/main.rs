use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::once;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<Vec<char>>> {
    input
        .as_ref()
        .split("\n\n")
        .map(|note| note.lines().map(|l| l.trim().chars().collect()).collect())
        .collect()
}

#[allow(dead_code)]
fn display_note(note: &[Vec<char>]) -> String {
    note.iter()
        .cloned()
        .map(|r| r.iter().chain(once(&'\n')).collect::<String>())
        .collect()
}

fn transpose(note: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_note = vec![];
    for y in 0..note[0].len() {
        let mut temp = vec![];
        for row in note {
            temp.push(row[y]);
        }
        new_note.push(temp);
    }
    new_note
}

fn search_reflection(line: &[char], possible: &mut [usize]) {
    for (i, cnt) in possible.iter_mut().enumerate().filter(|(_, c)| c != &&0) {
        if i == 0 {
            continue;
        }
        let mut left = i - 1;
        let mut right = i;
        while line[left] == line[right] {
            if left == 0 || right == line.len() - 1 {
                break;
            }
            left -= 1;
            right += 1;
        }
        if (right != line.len() - 1 && left != 0) || line[left] != line[right] {
            *cnt = 0;
        }
    }
}

fn search_mirror(note: &[Vec<char>]) -> usize {
    let mut possible: Vec<usize> = (0..note[0].len()).collect();
    for row in note {
        search_reflection(row, &mut possible);
    }
    let mut result = *possible.iter().max().unwrap();
    if result != 0 {
        return result;
    }

    let note = transpose(note);
    let mut possible: Vec<usize> = (0..note[0].len()).collect();
    for row in &note {
        search_reflection(row, &mut possible);
    }
    result += *possible.iter().max().unwrap() * 100;
    result
}

fn part1(notes: &[Vec<Vec<char>>]) -> Result<usize> {
    let _start = Instant::now();

    let result = notes.iter().map(|n| search_mirror(n)).sum();

    writeln!(io::stdout(), "Part 1:{result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let notes = parse_input(input);
    part1(&notes)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let notes = parse_input(input);
    assert_eq!(part1(&notes).unwrap(), 405);
    assert_eq!(1, 1);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let notes = parse_input(input);
    assert_eq!(part1(&notes).unwrap(), 36448);
}
