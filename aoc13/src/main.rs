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

fn search_reflection(note: &[Vec<char>], i: usize, smudge: bool) -> bool {
    let smudge_cnt = smudge as usize;
    let mut diff_cnt = 0;
    let mut left = i - 1;
    let mut right = i;
    while right < note[0].len() {
        for line in note {
            if line[left] != line[right] {
                diff_cnt += 1;
            }
        }
        if left == 0 || right == note[0].len() - 1 || diff_cnt > smudge_cnt {
            break;
        }
        left -= 1;
        right += 1;
    }

    // eq instead of le because: "you discover that every mirror has exactly one smudge"
    diff_cnt == smudge_cnt
}

fn search_mirror(note: &[Vec<char>], smudge: bool) -> Option<usize> {
    let transpose_note = transpose(note);
    (1..note[0].len())
        .find(|&i| search_reflection(note, i, smudge))
        .or((1..transpose_note[0].len())
            .find(|&i| search_reflection(&transpose_note, i, smudge))
            .map(|i| i * 100))
}

fn part1(notes: &[Vec<Vec<char>>]) -> Result<usize> {
    let _start = Instant::now();

    let result = notes.iter().map(|n| search_mirror(n, false).unwrap()).sum();

    writeln!(io::stdout(), "Part 1:{result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}
fn part2(notes: &[Vec<Vec<char>>]) -> Result<usize> {
    let _start = Instant::now();

    let result = notes.iter().map(|n| search_mirror(n, true).unwrap()).sum();

    writeln!(io::stdout(), "Part 2:{result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let notes = parse_input(input);
    part1(&notes)?;
    part2(&notes)?;
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
    assert_eq!(part2(&notes).unwrap(), 400);
    assert_eq!(1, 1);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let notes = parse_input(input);
    assert_eq!(part1(&notes).unwrap(), 36448);
    assert_eq!(part2(&notes).unwrap(), 35799);
}
