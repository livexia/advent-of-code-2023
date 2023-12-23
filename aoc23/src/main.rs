use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (usize, usize);

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Vec<char>> {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| l.chars().collect())
        .collect()
}

fn neighbors(pos: Coord, trails: &[Vec<char>], part2: bool) -> Vec<Coord> {
    let (x, y) = pos;
    let possible: &[_] = if !part2 {
        match trails[x][y] {
            '.' => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
            '>' => &[(0, 1)],
            '<' => &[(0, -1)],
            '^' => &[(-1, 0)],
            'v' => &[(1, 0)],
            _ => return vec![],
        }
    } else if trails[x][y] != '#' {
        &[(-1, 0), (1, 0), (0, -1), (0, 1)]
    } else {
        return vec![];
    };
    let mut result = vec![];
    for (dx, dy) in possible {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx as usize >= trails.len() || ny as usize >= trails[0].len() {
            continue;
        }
        let nx = nx as usize;
        let ny = ny as usize;
        if trails[nx][ny] != '#' {
            result.push((nx, ny));
        }
    }
    result
}

fn dfs_grid(
    pos: Coord,
    trails: &[Vec<char>],
    visited: &mut [Vec<bool>],
    part2: bool,
) -> Option<usize> {
    if pos.0 == trails.len() - 1 {
        if trails[pos.0][pos.1] == '.' {
            Some(0)
        } else {
            None
        }
    } else {
        let mut count = 0;
        for next in neighbors(pos, trails, part2) {
            if !visited[next.0][next.1] {
                visited[next.0][next.1] = true;
                if let Some(remain) = dfs_grid(next, trails, visited, part2) {
                    count = count.max(1 + remain);
                }
                visited[next.0][next.1] = false;
            }
        }
        if count == 0 {
            None
        } else {
            Some(count)
        }
    }
}

fn gen_graph(trails: &[Vec<char>], part2: bool) -> HashMap<Coord, Vec<(Coord, usize)>> {
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();
    for i in 0..trails.len() {
        for j in 0..trails[0].len() {
            if trails[i][j] == '#' {
                continue;
            }

            let e = graph.entry((i, j)).or_default();
            e.extend(neighbors((i, j), trails, part2).into_iter().map(|p| (p, 1)));
        }
    }
    graph
}

fn prune_graph(graph: &mut HashMap<Coord, Vec<(Coord, usize)>>) {
    while let Some((&pos, _)) = graph.iter().find(|(_, v)| v.len() == 2) {
        let neighbors = graph.remove(&pos).unwrap();
        let (a, ad) = neighbors[0];
        let (b, bd) = neighbors[1];
        let ea = graph.entry(a).or_default();
        if let Some(i) = ea.iter().position(|(p, _)| p == &pos) {
            ea[i] = (b, ad + bd);
        }
        let eb = graph.entry(b).or_default();
        if let Some(i) = eb.iter().position(|(p, _)| p == &pos) {
            eb[i] = (a, ad + bd);
        }
    }
}

fn dfs_graph(
    pos: Coord,
    end: Coord,
    graph: &HashMap<Coord, Vec<(Coord, usize)>>,
    visited: &mut [Vec<bool>],
) -> Option<usize> {
    if pos == end {
        return Some(0);
    }
    graph[&pos]
        .iter()
        .filter_map(|&(next, d)| {
            if !visited[next.0][next.1] {
                visited[next.0][next.1] = true;
                let d = dfs_graph(next, end, graph, visited).map(|r| r + d);
                visited[next.0][next.1] = false;
                d
            } else {
                None
            }
        })
        .max()
}

fn part1(trails: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let start = (
        0,
        (0..trails[0].len()).find(|&y| trails[0][y] == '.').unwrap(),
    );
    let result = dfs_grid(
        start,
        trails,
        &mut vec![vec![false; trails[0].len()]; trails.len()],
        false,
    )
    .unwrap();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

#[allow(dead_code)]
fn part2_grid(trails: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let start = (
        0,
        (0..trails[0].len()).find(|&y| trails[0][y] == '.').unwrap(),
    );
    let result = dfs_grid(
        start,
        trails,
        &mut vec![vec![false; trails[0].len()]; trails.len()],
        true,
    )
    .unwrap();

    writeln!(io::stdout(), "Part 2 with grid: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2_graph(trails: &[Vec<char>]) -> Result<usize> {
    let _start = Instant::now();

    let start = (
        0,
        (0..trails[0].len()).find(|&y| trails[0][y] == '.').unwrap(),
    );
    let end = (
        trails.len() - 1,
        (0..trails[0].len())
            .find(|&y| trails[trails.len() - 1][y] == '.')
            .unwrap(),
    );
    let mut graph = gen_graph(trails, true);
    prune_graph(&mut graph);
    let result = dfs_graph(
        start,
        end,
        &graph,
        &mut vec![vec![false; trails[0].len()]; trails.len()],
    )
    .unwrap();

    writeln!(io::stdout(), "Part 2 with graph: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let trails = parse_input(input);
    part1(&trails)?;
    part2_graph(&trails)?;
    // part2_grid(&trails)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    let trails = parse_input(input);
    assert_eq!(part1(&trails).unwrap(), 94);
    assert_eq!(part2_graph(&trails).unwrap(), 154);
    assert_eq!(part2_grid(&trails).unwrap(), 154);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let trails = parse_input(input);
    assert_eq!(part1(&trails).unwrap(), 2394);
    assert_eq!(part2_graph(&trails).unwrap(), 6554);
    // assert_eq!(part2_grid(&trails).unwrap(), 6554);
}
