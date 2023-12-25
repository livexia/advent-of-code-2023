use std::collections::{HashMap, HashSet, VecDeque};
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

fn parse_input<T: AsRef<str>>(input: T) -> HashMap<usize, HashSet<usize>> {
    let mut edges: HashMap<_, HashSet<_>> = HashMap::new();
    input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .for_each(|l| {
            if let Some((left, right)) = l.split_once(':') {
                let left = name_to_num(left);
                let right: Vec<_> = right.split_whitespace().map(name_to_num).collect();
                edges.entry(left).or_default().extend(right.iter());
                for n in right {
                    edges.entry(n).or_default().insert(left);
                }
            } else {
                panic!("Unable to parse input.")
            }
        });

    edges
}

fn dfs(
    v: usize,
    edges: &HashMap<usize, HashSet<usize>>,
    visited: &mut HashSet<usize>,
    exclued: &[(usize, usize)],
) -> usize {
    if visited.insert(v) {
        let mut count = 1;
        for &next in &edges[&v] {
            if exclued.contains(&(v, next)) || exclued.contains(&(next, v)) {
                continue;
            }
            count += dfs(next, edges, visited, exclued)
        }
        count
    } else {
        0
    }
}

fn edges_freq(
    start: usize,
    edges: &HashMap<usize, HashSet<usize>>,
    freq: &mut HashMap<(usize, usize), usize>,
) {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut visited = HashSet::new();
    visited.insert(start);
    while let Some(v) = queue.pop_front() {
        for &n in &edges[&v] {
            if visited.insert(n) {
                let e = freq.entry((v.min(n), n.max(v))).or_default();
                *e += 1;
                queue.push_back(n);
            }
        }
    }
}

fn part1(edges: &HashMap<usize, HashSet<usize>>) -> Result<usize> {
    let _start = Instant::now();

    let mut freq = HashMap::new();
    for &v in edges.keys() {
        edges_freq(v, edges, &mut freq);
    }

    let mut edges_sort_by_freq: Vec<_> = freq.iter().collect();
    edges_sort_by_freq.sort_by_key(|(_, v)| *v);
    edges_sort_by_freq.reverse();

    let mut result = 0;

    'searching: for (i, (&e1, _)) in edges_sort_by_freq.iter().enumerate() {
        for (j, (&e2, _)) in edges_sort_by_freq.iter().enumerate().skip(i + 1) {
            for (&e3, _) in edges_sort_by_freq.iter().skip(j + 1) {
                let size = dfs(e1.0, edges, &mut HashSet::new(), &[e1, e2, e3]);
                if size != edges.len() {
                    result = (edges.len() - size) * size;
                    break 'searching;
                }
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

    let edges = parse_input(input);

    part1(&edges)?;
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
    let edges = parse_input(input);

    assert_eq!(part1(&edges).unwrap(), 54);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let edges = parse_input(input);

    assert_eq!(part1(&edges).unwrap(), 506202);
}
