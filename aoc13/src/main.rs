use std::error::Error;
use std::io::{self, Read, Write};
use std::iter::once;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn parse_input<T: AsRef<str>>(input: T) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .as_ref()
        .split("\n\n")
        .map(|note| {
            let (mut rows, mut cols) = (vec![], vec![]);
            for line in note.lines() {
                cols.resize(line.trim().len(), 0);
                let mut row = 0;
                for (j, c) in line.trim().chars().enumerate() {
                    row = (row << 1) | ((c == '#') as u32);
                    cols[j] = (cols[j] << 1) | ((c == '#') as u32);
                }
                rows.push(row);
            }
            (rows, cols)
        })
        .collect()
}

#[allow(dead_code)]
fn display_note(note: &[Vec<char>]) -> String {
    note.iter()
        .cloned()
        .map(|r| r.iter().chain(once(&'\n')).collect::<String>())
        .collect()
}

#[allow(dead_code)]
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

fn search_reflection(note: &[u32], i: usize, smudge: bool) -> bool {
    // eq instead of le because: "you discover that every mirror has exactly one smudge"
    (0..i)
        .rev()
        .zip(i..note.len())
        .map(|(i, j)| (note[i] ^ note[j]).count_ones())
        .sum::<u32>()
        == smudge as u32
}

fn search_mirror(note: &(Vec<u32>, Vec<u32>), smudge: bool) -> Option<usize> {
    println!("{:0b}", note.0[0]);
    (1..note.1.len())
        .find(|&i| search_reflection(&note.1, i, smudge))
        .or((1..note.0.len())
            .find(|&i| search_reflection(&note.0, i, smudge))
            .map(|i| i * 100))
}

fn part1(notes: &[(Vec<u32>, Vec<u32>)]) -> Result<usize> {
    let _start = Instant::now();

    let result = notes.iter().map(|n| search_mirror(n, false).unwrap()).sum();

    writeln!(io::stdout(), "Part 1:{result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}
fn part2(notes: &[(Vec<u32>, Vec<u32>)]) -> Result<usize> {
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
