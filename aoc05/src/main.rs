use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Number = i64;
type SingleMap = (Number, Number, Number); // dest, src, length

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Number>,
    maps: Vec<Vec<SingleMap>>,
}

impl Almanac {
    fn new() -> Self {
        Almanac {
            seeds: vec![],
            maps: vec![],
        }
    }

    fn append_maps(&mut self, maps: Vec<SingleMap>) {
        self.maps.push(maps);
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut almanac = Almanac::new();
    let mut maps = vec![];
    for line in input.lines() {
        if line.trim().is_empty() && !maps.is_empty() {
            almanac.append_maps(maps.clone());
            maps.clear();
        } else if let Some(seeds) = line.trim().strip_prefix("seeds: ") {
            almanac.seeds = seeds
                .split_whitespace()
                .map(|n| n.parse::<Number>().unwrap())
                .collect();
        } else if line.contains("map") {
            continue;
        } else {
            let mut parts = line.split_whitespace();
            if let (Some(dest), Some(src), Some(length), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            {
                maps.push((
                    dest.parse::<Number>().unwrap(),
                    src.parse::<Number>().unwrap(),
                    length.parse::<Number>().unwrap(),
                ))
            }
        }
    }
    if !maps.is_empty() {
        almanac.append_maps(maps);
    }
    almanac
}

fn convert(input: Number, dest: Number, src: Number, length: Number) -> Option<Number> {
    if input < src || input >= src + length {
        None
    } else {
        Some(input - src + dest)
    }
}

fn part1(almanac: &Almanac) -> Result<Number> {
    let start = Instant::now();

    let result = almanac
        .seeds
        .iter()
        .map(|seed| {
            let mut start = *seed;
            for maps in &almanac.maps {
                if let Some(next) = maps
                    .iter()
                    .find_map(|&(dest, src, length)| convert(start, dest, src, length))
                {
                    start = next
                }
            }
            start
        })
        .min()
        .unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let almanac = parse_input(&input);

    part1(&almanac)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    assert_eq!(convert(98, 50, 98, 2).unwrap(), 50);
    assert_eq!(convert(99, 50, 98, 2).unwrap(), 51);
    assert_eq!(convert(100, 50, 98, 2), None);
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let almanac = parse_input(input);
    assert_eq!(part1(&almanac).unwrap(), 35);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let almanac = parse_input(&input);
    assert_eq!(part1(&almanac).unwrap(), 424490994);
}
