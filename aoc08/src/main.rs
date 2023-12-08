use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Network = Vec<(usize, usize)>;
type NodeMap = HashMap<String, usize>;

fn set_node_id(name: &str, map: &mut NodeMap, last_id: &mut usize) -> usize {
    if let Some(id) = map.get(name) {
        *id
    } else {
        *last_id += 1;
        map.insert(name.to_string(), *last_id);
        *last_id
    }
}

fn parse_input(input: &str) -> Result<(Vec<char>, Network, NodeMap)> {
    let input = input.replace([')', '('], "");
    let max_length = input.lines().count();
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());
    // parse instructions as chars;
    let instrs: Vec<_> = if let Some(line) = lines.next() {
        line.trim().chars().collect()
    } else {
        return err!("Unable to parse instructions");
    };
    let mut last_id = 0;
    let mut map = NodeMap::new();
    let mut network = vec![(0, 0); max_length];
    for line in lines {
        if let Some((node_name, node_next)) = line.split_once('=') {
            let id = set_node_id(node_name.trim(), &mut map, &mut last_id);
            if let Some((left, right)) = node_next.trim().split_once(',') {
                let left_id = set_node_id(left.trim(), &mut map, &mut last_id);
                let right_id = set_node_id(right.trim(), &mut map, &mut last_id);
                network[id] = (left_id, right_id);
            } else {
                return err!("Unable to parse nodes left and right: {:?}", line);
            }
        } else {
            return err!("Unable to parse node: {:?}", line);
        }
    }
    dbg!(network.len());
    Ok((instrs, network, map))
}

fn next_node(start: (usize, usize), instrs: &[char], network: &Network) -> Result<(usize, usize)> {
    Ok(match (instrs[start.1], network[start.0]) {
        ('R', (_, r)) => (r, (start.1 + 1) % instrs.len()),
        ('L', (l, _)) => (l, (start.1 + 1) % instrs.len()),
        _ => return err!("Unable to move with {start:?}"),
    })
}

// see: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
fn cycle_detect(start: usize, instrs: &[char], network: &Network) -> Result<usize> {
    let mut tortoise = next_node((start, 0), instrs, network)?;
    let mut hare = next_node(tortoise, instrs, network)?;
    while tortoise != hare {
        tortoise = next_node(tortoise, instrs, network)?;

        hare = next_node(hare, instrs, network)?;
        hare = next_node(hare, instrs, network)?;
    }

    let mut mu = 0;
    tortoise = (start, 0);
    while tortoise != hare {
        tortoise = next_node(tortoise, instrs, network)?;

        hare = next_node(hare, instrs, network)?;

        mu += 1;
    }

    let mut lam = 1;
    hare = next_node(tortoise, instrs, network)?;
    while tortoise != hare {
        hare = next_node(hare, instrs, network)?;

        lam += 1;
    }

    Ok(mu + lam)
}

// search until cycle
fn get_steps(
    start: usize,
    end_ids: &[usize],
    instrs: &[char],
    network: &Network,
) -> Result<Vec<(usize, usize)>> {
    let mut cur = start;
    let mut cur_instr = 0;

    let mut ends = vec![];

    let limit = cycle_detect(start, instrs, network)?;

    for steps in 0..limit {
        if end_ids.contains(&cur) {
            ends.push((cur, steps))
        }
        (cur, cur_instr) = next_node((cur, cur_instr), instrs, network)?;
    }
    Ok(ends)
}

fn part1(instrs: &[char], network: &Network, map: &NodeMap) -> Result<usize> {
    let start = Instant::now();

    let &start_id = map
        .get("AAA")
        .expect("Unable to find the AAA node in network");
    let &end_id = map
        .get("ZZZ")
        .expect("Unable to find the ZZZ node in network");

    let steps = get_steps(start_id, &[end_id], instrs, network)?[0].1;

    writeln!(io::stdout(), "Part 1: {steps}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(steps)
}

// https://zh.wikipedia.org/wiki/%E6%9C%80%E5%A4%A7%E5%85%AC%E5%9B%A0%E6%95%B8
fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = (a.max(b), b.min(a));
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// https://zh.wikipedia.org/zh-hans/%E6%9C%80%E5%B0%8F%E5%85%AC%E5%80%8D%E6%95%B8
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn dfs(
    start_to_end_steps: &HashMap<usize, Vec<(usize, usize)>>,
    start_count: usize,
    end_count: usize,
    visited_start: &mut HashSet<usize>,
    visited_end: &mut HashSet<usize>,
) -> Option<usize> {
    let mut min_steps = usize::MAX;
    for (&start, end_steps) in start_to_end_steps.iter() {
        if visited_start.insert(start) {
            for &(end, steps) in end_steps {
                if visited_end.insert(end) {
                    if let Some(remain_steps) = dfs(
                        start_to_end_steps,
                        start_count,
                        end_count,
                        visited_start,
                        visited_end,
                    ) {
                        min_steps = min_steps.min(lcm(steps, remain_steps));
                    }
                    visited_end.remove(&end);
                }
            }
            visited_start.remove(&start);
        }
    }
    if min_steps != usize::MAX {
        Some(min_steps)
    } else if visited_start.len() == start_count && visited_end.len() == end_count {
        Some(1)
    } else {
        None
    }
}

fn part2(instrs: &[char], network: &Network, map: &NodeMap) -> Result<usize> {
    let start = Instant::now();

    let start_ids: Vec<_> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| *v)
        .collect();
    let end_ids: Vec<_> = map
        .iter()
        .filter(|(k, _)| k.ends_with('Z'))
        .map(|(_, v)| *v)
        .collect();
    let mut start_to_end_steps = HashMap::new();

    for &start in &start_ids {
        start_to_end_steps.insert(start, get_steps(start, &end_ids, instrs, network)?);
    }

    let steps = dfs(
        &start_to_end_steps,
        start_ids.len(),
        end_ids.len(),
        &mut HashSet::new(),
        &mut HashSet::new(),
    )
    .unwrap();

    writeln!(io::stdout(), "Part 2: {steps}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(steps)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (instrs, network, map) = parse_input(&input)?;

    part1(&instrs, &network, &map)?;
    part2(&instrs, &network, &map)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    let (instrs, network, map) = parse_input(input).unwrap();
    assert_eq!(part1(&instrs, &network, &map).unwrap(), 2);

    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    let (instrs, network, map) = parse_input(input).unwrap();
    assert_eq!(part2(&instrs, &network, &map).unwrap(), 6);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (instrs, network, map) = parse_input(&input).unwrap();
    assert_eq!(part1(&instrs, &network, &map).unwrap(), 17287);
    assert_eq!(part2(&instrs, &network, &map).unwrap(), 18625484023687);
}
