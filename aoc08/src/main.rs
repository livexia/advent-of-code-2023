use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type NodeId = usize;
type NextNode = (usize, usize);

fn set_node_id(name: &str, map: &mut HashMap<String, NodeId>, last_id: &mut usize) -> NodeId {
    if let Some(id) = map.get(name) {
        *id
    } else {
        *last_id += 1;
        map.insert(name.to_string(), *last_id);
        *last_id
    }
}

fn parse_input(
    input: &str,
) -> Result<(
    Vec<char>,
    HashMap<NodeId, NextNode>,
    HashMap<String, NodeId>,
)> {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty());
    // parse instructions as chars;
    let instrs: Vec<_> = if let Some(line) = lines.next() {
        line.trim().chars().collect()
    } else {
        return err!("Unable to parse instructions");
    };
    let mut last_id = 0;
    let mut map = HashMap::new();
    let mut network = HashMap::new();
    for line in lines {
        if let Some((node_name, node_next)) = line.split_once('=') {
            let id = set_node_id(node_name.trim(), &mut map, &mut last_id);
            if let Some((left, right)) = node_next.trim_matches(['(', ')', ' ']).split_once(',') {
                let left_id =
                    set_node_id(left.trim_matches(['(', ')', ' ']), &mut map, &mut last_id);
                let right_id =
                    set_node_id(right.trim_matches(['(', ')', ' ']), &mut map, &mut last_id);
                network.insert(id, (left_id, right_id));
            } else {
                return err!("Unable to parse nodes left and right: {:?}", line);
            }
        } else {
            return err!("Unable to parse node: {:?}", line);
        }
    }
    Ok((instrs, network, map))
}

fn move_node(instr: char, id: NodeId, network: &HashMap<NodeId, NextNode>) -> Result<NodeId> {
    match (instr, network.get(&id)) {
        ('R', Some((_, r))) => Ok(*r),
        ('L', Some((l, _))) => Ok(*l),
        _ => err!("Unable to move from {id}"),
    }
}

fn part1(
    instrs: &[char],
    network: &HashMap<NodeId, NextNode>,
    map: &HashMap<String, NodeId>,
) -> Result<usize> {
    let start = Instant::now();

    let &start_id = map
        .get("AAA")
        .expect("Unable to find the AAA node in network");
    let &end_id = map
        .get("ZZZ")
        .expect("Unable to find the ZZZ node in network");

    let mut cur = start_id;
    let mut cur_instr = 0;
    let mut steps = 0;
    while cur != end_id {
        cur = move_node(instrs[cur_instr], cur, network)?;
        steps += 1;
        cur_instr = (cur_instr + 1) % instrs.len();
    }

    writeln!(io::stdout(), "Part 1: {steps}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(steps)
}

fn part2(
    instrs: &[char],
    network: &HashMap<NodeId, NextNode>,
    map: &HashMap<String, NodeId>,
) -> Result<usize> {
    let start = Instant::now();

    let mut cur_ids: HashSet<_> = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| *v)
        .collect();
    let end_ids: HashSet<_> = map
        .iter()
        .filter(|(k, _)| k.ends_with('Z'))
        .map(|(_, v)| *v)
        .collect();

    let mut cur_instr = 0;
    let mut steps = 0;
    while cur_ids != end_ids {
        let instr = instrs[cur_instr];
        cur_ids = cur_ids
            .iter()
            .map(|id| move_node(instr, *id, network))
            .collect::<Result<HashSet<_>>>()?;
        steps += 1;
        cur_instr = (cur_instr + 1) % instrs.len();
    }

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
    assert_eq!(part2(&instrs, &network, &map).unwrap(), 17287);
}
