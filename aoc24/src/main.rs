use nalgebra::{Vector2, Vector3};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl FromStr for Hailstone {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((p, v)) = s.split_once(" @ ") {
            let p = p
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<f64>>();
            let v = v
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<f64>>();
            Ok(Hailstone {
                position: Vector3::new(p[0], p[1], p[2]),
                velocity: Vector3::new(v[0], v[1], v[2]),
            })
        } else {
            err!("Unable to parse the hailstone: {s:?}")
        }
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Result<Vec<Hailstone>> {
    input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse())
        .collect()
}

impl Hailstone {
    fn interstion_xy(&self, other: &Hailstone) -> Option<Vector2<f64>> {
        let (x1, x2) = (self.position[0], other.position[0]);
        let (y1, y2) = (self.position[1], other.position[1]);
        let (vx1, vx2) = (self.velocity[0], other.velocity[0]);
        let (vy1, vy2) = (self.velocity[1], other.velocity[1]);
        // x1 + vx1 * t = x2 + vx2 * s
        //     (x1 + vx1 * t - x2) / vx2 = s
        // y1 + vy1 * t = y2 + vy2 * s
        //     (y1 - y2 + vy1 * t) / vy2 = s
        // (x1 + vx1 * t - x2) * vy2 = (y1 - y2 + vy1 * t ) * vx2
        // (x1 -x2) * vy2 - (y1- y2) * vx2    = (vx2 * vy1 - vx1 * vy2) * t
        let t = ((x1 - x2) * vy2 - (y1 - y2) * vx2) / (vx2 * vy1 - vx1 * vy2);
        let s = (x1 + vx1 * t - x2) / vx2;

        if t < 0.0 || s < 0.0 || t.is_infinite() || s.is_infinite() {
            return None;
        }
        Some(Vector2::new(x1 + vx1 * t, y1 + vy1 * t))
    }
}

fn part1(stones: &[Hailstone], min: isize, max: isize) -> Result<usize> {
    let _start = Instant::now();

    let mut count = 0;
    for (i, s1) in stones.iter().enumerate() {
        for s2 in stones.iter().skip(i + 1) {
            if let Some(cross) = s1.interstion_xy(s2) {
                if cross.iter().all(|&c| c >= min as f64 && c <= max as f64) {
                    count += 1;
                }
            }
        }
    }

    writeln!(io::stdout(), "Part 1: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn part2(stones: &[Hailstone]) -> Result<usize> {
    let _start = Instant::now();

    let mut count = 0;

    // rock (x, y, z) (vx, vy, vz)
    //      6 unknown
    // all stones have a t that:
    //      stone: (x1, y1, z1) (vx1, vy1, vz1)
    //      every time stone and rock smashed introudce a time t1
    //          6 + 1 unkonwn
    //      x + vx * t1 = x1 + vx1 * t1
    //      y + vy * t1 = y1 + vy1 * t1
    //      z + vz * t1 = z1 + vz1 * t1
    // 2 formula to solve 1 unknown
    // the number of unknown should be same as the number of equation
    // 6 + x = 3 * x
    // x = 3
    for (i, s) in stones[..3].iter().enumerate() {
        let p = s.position;
        let v = s.velocity;
        let (x, y, z) = (p[0], p[1], p[2]);
        let (vx, vy, vz) = (v[0], v[1], v[2]);
        println!("x+vx*t{i}={}+{}*t{i},", x, vx);
        println!("y+vy*t{i}={}+{}*t{i},", y, vy);
        println!("z+vz*t{i}={}+{}*t{i},", z, vz);
    }

    writeln!(io::stdout(), "Part 2: {count}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(count)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let stones = parse_input(input)?;
    part1(&stones, 200000000000000, 400000000000000)?;
    part2(&stones)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    let stones = parse_input(input).unwrap();
    assert_eq!(part1(&stones, 7, 27).unwrap(), 2);
    assert_eq!(part2(&stones).unwrap(), 47);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let stones = parse_input(input).unwrap();
    assert_eq!(
        part1(&stones, 200000000000000, 400000000000000).unwrap(),
        21785
    );
    assert_eq!(part2(&stones).unwrap(), 554668916217145);
}
