use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize, isize);

#[derive(Debug, Clone)]
struct Brick {
    start: Coord,
    end: Coord,
}

fn parse_input<T: AsRef<str>>(input: T) -> Vec<Brick> {
    input
        .as_ref()
        .split_whitespace()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            let mut start = start.split(',').map(|n| n.parse::<isize>());
            let mut end = end.split(',').map(|n| n.parse::<isize>());
            let start = (
                start.next().unwrap().unwrap(),
                start.next().unwrap().unwrap(),
                start.next().unwrap().unwrap(),
            );
            let end = (
                end.next().unwrap().unwrap(),
                end.next().unwrap().unwrap(),
                end.next().unwrap().unwrap(),
            );
            assert!(start.2 <= end.2);
            Brick::new(start, end)
        })
        .collect()
}

impl Brick {
    fn new(start: Coord, end: Coord) -> Self {
        Brick { start, end }
    }
    fn cubes(&self) -> Vec<Coord> {
        let mut cubes = Vec::new();
        let (start, end) = (self.start, self.end);
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                for z in start.2..=end.2 {
                    cubes.push((x, y, z));
                }
            }
        }

        cubes
    }
    fn falling(&self) -> Option<Self> {
        if self.start.2 == 1 {
            None
        } else {
            let mut start = self.start;
            let mut end = self.end;
            start.2 -= 1;
            end.2 -= 1;
            Some(Self::new(start, end))
        }
    }

    #[allow(dead_code)]
    fn is_intersect_hashset(&self, grid: &HashSet<Coord>) -> bool {
        self.cubes().iter().any(|p| grid.contains(p))
    }

    // https://stackoverflow.com/questions/55220355/how-to-detect-whether-two-segments-in-3d-space-intersect
    // https://stackoverflow.com/a/10288710
    #[allow(dead_code)]
    fn is_intersect_algebra(&self, other: &Brick) -> bool {
        fn dot(p1: Coord, p2: Coord) -> f64 {
            (p1.0 * p2.0 + p1.1 * p2.1 + p1.2 * p2.2) as f64
        }

        fn norm2(p: Coord) -> f64 {
            (p.0 * p.0 + p.1 * p.1 + p.2 * p.2) as f64
        }

        #[allow(dead_code)]
        fn norm(p: Coord) -> f64 {
            norm2(p).sqrt()
        }

        fn cross(p1: Coord, p2: Coord) -> Coord {
            (
                p1.1 * p2.2 - p2.1 * p1.2,
                p1.2 * p2.0 - p2.2 * p1.0,
                p1.0 * p2.1 - p2.0 * p1.1,
            )
        }

        if self.end.2 < other.start.2 || self.start.2 > other.end.2 {
            return false;
        }

        let da = (
            self.end.0 - self.start.0,
            self.end.1 - self.start.1,
            self.end.2 - self.start.2,
        );

        let db = (
            other.end.0 - other.start.0,
            other.end.1 - other.start.1,
            other.end.2 - other.start.2,
        );

        let dc = (
            other.start.0 - self.start.0,
            other.start.1 - self.start.1,
            other.start.2 - self.start.2,
        );
        if dot(dc, cross(da, db)) != 0.0 {
            return false;
        }

        if cross(da, db) == (0, 0, 0) {
            return is_in_line(self.start, other.start, other.end)
                || is_in_line(self.end, other.start, other.end)
                || is_in_line(other.start, self.start, self.end)
                || is_in_line(other.end, self.start, self.end);
        }

        let s = dot(cross(dc, db), cross(da, db)) / norm2(cross(da, db));
        // https://stackoverflow.com/a/33825948
        let t = dot(cross(dc, da), cross(da, db)) / norm2(cross(da, db));

        (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t)
    }
}

fn dis(p1: Coord, p2: Coord) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + p1.2.abs_diff(p2.2)
}

fn is_in_line(p: Coord, start: Coord, end: Coord) -> bool {
    dis(p, start) + dis(p, end) == dis(start, end)
}

fn falling(bricks: &mut [Brick], start: usize) -> usize {
    bricks.sort_by_key(|b| b.start.2);
    let mut count = 0;

    for i in start..bricks.len() {
        let mut flag = false;
        while let Some(next) = bricks[i].falling() {
            if bricks[0..i]
                .iter()
                .rev()
                .all(|b| !next.is_intersect_algebra(b))
            {
                bricks[i] = next;
                flag = true;
            } else {
                break;
            }
        }
        count += flag as usize;
    }
    count
}

fn part1(bricks: &[Brick]) -> Result<usize> {
    let _start = Instant::now();

    let mut bricks = bricks.to_vec();

    let _ = falling(&mut bricks, 0);

    bricks.sort_by_key(|b| b.start.2);
    let mut count = bricks.len();
    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);

        for j in i..temp.len() {
            if let Some(next) = temp[j].falling() {
                if temp[0..j]
                    .iter()
                    .rev()
                    .all(|b| !next.is_intersect_algebra(b))
                {
                    count -= 1;
                    break;
                }
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn part2(bricks: &[Brick]) -> Result<usize> {
    let _start = Instant::now();

    let mut bricks = bricks.to_vec();

    let _ = falling(&mut bricks, 0);

    bricks.sort_by_key(|b| b.start.2);
    let mut count = 0;
    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);
        count += falling(&mut temp, i);
    }

    writeln!(io::stdout(), "Part 2: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn falling_hashset(bricks: &mut [Brick], start: usize, chain: bool) -> usize {
    bricks.sort_by_key(|b| b.start.2);
    let mut count = 0;

    let mut grid = HashSet::new();
    for b in &bricks[..start] {
        grid.extend(b.cubes().iter());
    }
    for b in &mut bricks[start..] {
        let mut flag = false;
        while let Some(next) = b.falling() {
            if !next.is_intersect_hashset(&grid) {
                if !chain {
                    return 1;
                }
                *b = next;
                flag = true;
            } else {
                break;
            }
        }
        grid.extend(b.cubes().iter());

        count += flag as usize;
    }
    count
}

fn part1_hashset(bricks: &[Brick]) -> Result<usize> {
    let _start = Instant::now();

    let mut bricks = bricks.to_vec();

    let _ = falling_hashset(&mut bricks, 0, true);

    bricks.sort_by_key(|b| b.start.2);
    let mut count = bricks.len();
    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);
        if falling_hashset(&mut temp, i, false) == 1 {
            count -= 1;
        }
    }

    writeln!(io::stdout(), "Part 1: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn part2_hashset(bricks: &[Brick]) -> Result<usize> {
    let _start = Instant::now();

    let mut bricks = bricks.to_vec();

    let _ = falling_hashset(&mut bricks, 0, true);

    bricks.sort_by_key(|b| b.start.2);
    let mut count = 0;
    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);
        count += falling_hashset(&mut temp, i, true);
    }

    writeln!(io::stdout(), "Part 2: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bricks = parse_input(input);
    part1(&bricks)?;
    part2(&bricks)?;

    part1_hashset(&bricks)?;
    part2_hashset(&bricks)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    assert!(
        Brick::new((5, 0, 4), (8, 0, 4)).is_intersect_algebra(&Brick::new((5, 0, 4), (15, 0, 4)))
    );
    let bricks = parse_input(input);
    assert_eq!(part1(&bricks).unwrap(), 5);
    assert_eq!(part2(&bricks).unwrap(), 7);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let bricks = parse_input(input);
    assert_eq!(part1(&bricks).unwrap(), 401);
    assert_eq!(part2(&bricks).unwrap(), 63491);
    assert_eq!(part1_hashset(&bricks).unwrap(), 401);
    assert_eq!(part2_hashset(&bricks).unwrap(), 63491);
}
