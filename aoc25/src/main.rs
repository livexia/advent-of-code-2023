use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn name_to_num(s: &str) -> usize {
    s.trim()
        .bytes()
        .rev()
        .fold(0, |sum, n| sum << 8 | (n as usize))
}

#[allow(dead_code)]
fn num_to_name(mut n: usize) -> String {
    let mut s = String::new();
    while n != 0 {
        s.push(((n & 0xff) as u8) as char);
        n >>= 8;
    }
    s
}

fn parse_input<T: AsRef<str>>(input: T) -> HashMap<usize, Vec<usize>> {
    input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            if let Some((left, right)) = l.split_once(':') {
                let left = name_to_num(left);
                let right = right.split_whitespace().map(name_to_num).collect();
                (left, right)
            } else {
                panic!("Unable to parse input.")
            }
        })
        .collect()
}

fn dfs(
    v: usize,
    edges: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
    exclued: &[(usize, usize)],
) -> usize {
    let mut count = 1;
    for &next in &edges[&v] {
        if exclued.contains(&(v, next)) {
            continue;
        }
        if visited.insert(next) {
            count += dfs(next, edges, visited, exclued)
        }
    }
    count
}

fn connected_even_exclued(
    v: usize,
    end: usize,
    edges: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
    exclued: &[(usize, usize)],
) -> bool {
    for &next in &edges[&v] {
        if exclued.contains(&(v, next)) {
            continue;
        }
        if next == end {
            return true;
        }
        if visited.insert(next) && connected_even_exclued(next, end, edges, visited, exclued) {
            return true;
        }
    }
    false
}

fn part1(connected: &HashMap<usize, Vec<usize>>) -> Result<usize> {
    let _start = Instant::now();

    let mut edges: HashMap<_, HashSet<_>> = HashMap::new();

    for (k, v) in connected {
        let e = edges.entry(*k).or_default();
        e.extend(v.iter());
        for &next in v {
            let e1 = edges.entry(next).or_default();
            e1.insert(*k);
        }
    }

    let total_edges: Vec<_> = edges
        .iter()
        .flat_map(|(k, v)| v.iter().map(|v0| (*k, *v0)))
        .collect();

    let l = total_edges.len();
    for (k, v) in &edges {
        println!("{} {k} -> {v:?}", num_to_name(*k));
    }

    println!("{}", l * (l - 1) * (l - 2));

    dbg!(&edges[&name_to_num("ffj")]);
    // dbg!(&edges[&name_to_num("xjb")]);
    dbg!(&edges[&name_to_num("xhg")]);

    let mut result = 0;
    let mut visited = HashSet::new();
    let mut group = vec![];
    for &v in edges.keys() {
        if visited.insert(v) {
            group.push(dfs(
                v,
                &edges,
                &mut visited,
                &[
                    (name_to_num("xjb"), name_to_num("vgs")),
                    (name_to_num("ffj"), name_to_num("lkm")),
                    (name_to_num("xhg"), name_to_num("ljl")),
                ],
            ));
            if group.len() == 2 {
                result = group[0] * group[1];
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let connected = parse_input(input);

    part1(&connected)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    let connected = parse_input(input);

    assert_eq!(part1(&connected).unwrap(), 54);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let connected = parse_input(input);

    assert_eq!(part1(&connected).unwrap(), 54);
}
