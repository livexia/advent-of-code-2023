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
    cubes: HashSet<Coord>,
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
        let mut b = Brick {
            start,
            end,
            cubes: HashSet::new(),
        };
        let mut set = HashSet::new();
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                for z in start.2..=end.2 {
                    set.insert((x, y, z));
                }
            }
        }

        b.cubes = set;
        b
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

    fn is_intersect_hashset(&self, other: &Brick) -> bool {
        self.cubes.iter().any(|p| other.cubes.contains(p))
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

        let s = dot(cross(dc, db), cross(da, db)) / norm2(cross(da, db));
        // https://stackoverflow.com/a/33825948
        let t = dot(cross(dc, da), cross(da, db)) / norm2(cross(da, db));

        (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t)
    }
}

fn part1(bricks: &[Brick]) -> Result<usize> {
    let _start = Instant::now();

    let mut bricks = bricks.to_vec();
    bricks.sort_by_key(|b| b.start.2);

    for i in 0..bricks.len() {
        while let Some(next) = bricks[i].falling() {
            if bricks[0..i]
                .iter()
                .rev()
                .all(|b| !next.is_intersect_hashset(b))
                && bricks[i + 1..]
                    .iter()
                    .all(|b| !next.is_intersect_hashset(b))
            {
                bricks[i] = next;
            } else {
                break;
            }
        }
    }

    bricks.sort_by_key(|b| b.start.2);
    let mut count = bricks.len();
    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);

        for j in 0..temp.len() {
            if let Some(next) = temp[j].falling() {
                if temp[0..j]
                    .iter()
                    .rev()
                    .all(|b| !next.is_intersect_hashset(b))
                    && temp[j + 1..].iter().all(|b| !next.is_intersect_hashset(b))
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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bricks = parse_input(input);
    part1(&bricks)?;
    // part2()?;
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
        Brick::new((0, 1, 4), (2, 1, 4)).is_intersect_algebra(&Brick::new((1, 1, 4), (1, 1, 5)))
    );
    let bricks = parse_input(input);
    assert_eq!(part1(&bricks).unwrap(), 5);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let bricks = parse_input(input);
    assert_eq!(part1(&bricks).unwrap(), 401);
}
