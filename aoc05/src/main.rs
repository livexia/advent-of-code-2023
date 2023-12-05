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
type Range = (Number, Number); // [start, end)

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

fn convert_range(
    input: Range,
    dest: Number,
    src: Number,
    length: Number,
) -> (Vec<Range>, Option<Range>) {
    // src range: src..src+length
    // src and dest offset is: dest - src
    // then: dest = src + offset
    let offset = dest - src;

    let src_end = src + length;
    let (start, end) = input;
    if end <= src || src_end <= start {
        return (vec![input], None);
    } else {
        let overlaps = Some((start.max(src) + offset, end.min(src_end) + offset));
        // Number::MIN..src, src_end..Number::MAX
        let remain_range: Vec<Range> = [(start, end.min(src)), (start.max(src_end), end)]
            .into_iter()
            .filter(|(a, b)| a < b)
            .collect();
        println!(
            "input: {:?}\nover: {:?}\nremain: {:?}",
            input, overlaps, remain_range
        );
        (remain_range, overlaps)
    }
}

// this funtion only works when a.0 <= b.0
fn merge_range(a: Range, b: Range) -> (Range, Option<Range>) {
    let a_end = a.1;
    let b_start = b.0;
    if a_end >= b_start {
        ((a.0, b.1), None)
    } else {
        (a, Some(b))
    }
}

fn convert_range_with_maps(range: Range, maps: &[SingleMap], converted: &mut Vec<Range>) {
    if maps.is_empty() {
        converted.push(range);
    }
    let (dest, src, length) = maps[0];
    let (r_ranges, overlaps) = convert_range(range, dest, src, length);
    if let Some(overlaps) = overlaps {
        converted.push(overlaps);
    }
    for r in r_ranges {
        convert_range_with_maps(r, &maps[1..], converted);
    }
}

fn part2(almanac: &Almanac) -> Result<Number> {
    let start = Instant::now();

    let mut ranges: Vec<Range> = almanac
        .seeds
        .chunks(2)
        .map(|chunks| (chunks[0], chunks[0] + chunks[1]))
        .collect();
    ranges.sort();
    for maps in &almanac.maps {
        let mut next_ranges = vec![];
        for &range in &ranges {
            let mut remain_ranges = vec![];
            let mut flag = false;
            for map in maps {
                let &(dest, src, length) = map;
                let (r_ranges, overlaps) = convert_range(range, dest, src, length);
                if let Some(overlaps) = overlaps {
                    next_ranges.push(overlaps);
                    remain_ranges.extend_from_slice(&r_ranges);
                    flag = true;
                };
            }
            if !flag {
                next_ranges.push(range);
                assert_eq!(remain_ranges.is_empty(), true);
            }
            next_ranges.extend_from_slice(&remain_ranges);
        }
        next_ranges.sort();

        std::mem::swap(&mut ranges, &mut next_ranges);
    }

    let result = ranges.into_iter().map(|(s, _)| s).min().unwrap();

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let almanac = parse_input(&input);

    part1(&almanac)?;
    part2(&almanac)?;
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
    assert_eq!(part2(&almanac).unwrap(), 46);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let almanac = parse_input(&input);
    assert_eq!(part1(&almanac).unwrap(), 424490994);
    assert_eq!(part2(&almanac).unwrap(), 424490994);
}
