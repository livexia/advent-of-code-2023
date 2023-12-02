use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Vec<Cube>>,
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let id;
        let mut sets: Vec<Vec<Cube>> = vec![];
        if let Some((raw_id, record)) = s.trim().split_once(':') {
            if let Some((_, str_id)) = raw_id.trim().split_once(' ') {
                id = str_id.trim().parse::<usize>()?;
            } else {
                return err!("Unable to parse game id: {:?}", s);
            }
            for raw_set in record.trim().split(';') {
                let mut set = vec![];
                for cube in raw_set.trim().split(',') {
                    if let Some((n, c)) = cube.trim().split_once(' ') {
                        let cube_count = n.trim().parse::<usize>()?;
                        set.push(match c.trim() {
                            "red" => Cube::Red(cube_count),
                            "green" => Cube::Green(cube_count),
                            "blue" => Cube::Blue(cube_count),
                            _ => return err!("Unable to parse game recore: {:?}", s),
                        });
                    }
                }
                sets.push(set);
            }
        } else {
            return err!("Unable to parse game record");
        }
        Ok(Game { id, sets })
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<Game>())
        .collect()
}

fn part1(games: &[Game]) -> Result<usize> {
    let start = Instant::now();

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let sum: usize = games
        .iter()
        .map(|g| {
            for set in &g.sets {
                for c in set {
                    match c {
                        Cube::Red(n) => {
                            if n > &red_max {
                                return 0usize;
                            }
                        }
                        Cube::Green(n) => {
                            if n > &green_max {
                                return 0usize;
                            }
                        }
                        Cube::Blue(n) => {
                            if n > &blue_max {
                                return 0usize;
                            }
                        }
                    }
                }
            }
            g.id
        })
        .sum();

    writeln!(io::stdout(), "Part 1: {}", sum)?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(sum)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let games = parse_input(&input)?;
    dbg!(&games[0]);

    part1(&games)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let games = parse_input(&input).unwrap();

    assert_eq!(part1(&games).unwrap(), 8);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let games = parse_input(&input).unwrap();

    assert_eq!(part1(&games).unwrap(), 1734);
}
