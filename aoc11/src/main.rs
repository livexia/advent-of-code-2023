use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (usize, usize);

#[derive(Clone)]
struct Image {
    raw: Vec<Vec<bool>>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
    bound: Coord,
}

impl Image {
    fn new(raw: Vec<Vec<bool>>) -> Self {
        let bound = (raw.len(), raw[0].len());
        Image {
            empty_rows: Image::empty_rows(&raw),
            empty_columns: Image::empty_columns(&raw),
            raw,
            bound,
        }
    }

    fn empty_rows(raw: &[Vec<bool>]) -> Vec<usize> {
        let mut count = 0;
        raw.iter()
            .map(|row| {
                if row.iter().all(|b| !b) {
                    count += 1;
                }
                count
            })
            .collect()
    }

    fn empty_columns(raw: &[Vec<bool>]) -> Vec<usize> {
        let mut count = 0;
        (0..raw[0].len())
            .map(|y| {
                if (0..raw.len()).all(|x| !raw[x][y]) {
                    count += 1
                }
                count
            })
            .collect()
    }

    fn expansion(&self, pos: &Coord, expansion_rate: usize) -> Coord {
        let empty_row = self.empty_rows[pos.0];
        let empty_column = self.empty_columns[pos.1];
        (
            pos.0 + empty_row * (expansion_rate - 1),
            pos.1 + empty_column * (expansion_rate - 1),
        )
    }

    fn shortest_path_sum(&self, expansion_rate: usize) -> usize {
        fn dis(p1: &Coord, p2: &Coord) -> usize {
            p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
        }
        let mut sum = 0;
        let galaxies: Vec<_> = (0..self.raw.len())
            .flat_map(|x| (0..self.bound.1).map(move |y| (x, y)))
            .filter(|(x, y)| self.raw[*x][*y])
            .map(|p| self.expansion(&p, expansion_rate))
            .collect();
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                sum += dis(&galaxies[i], &galaxies[j])
            }
        }
        sum
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for x in 0..self.bound.0 {
            for y in 0..self.bound.1 {
                if self.raw[x][y] {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        f.write_str(&s)
    }
}

fn parse_input<T: AsRef<str>>(input: T) -> Image {
    let image: Vec<_> = input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(_, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(move |(_, c)| c == '#')
                .collect::<Vec<_>>()
        })
        .collect();

    Image::new(image)
}

fn part1(image: &Image) -> Result<usize> {
    let _start = Instant::now();

    let sum = image.shortest_path_sum(2);

    writeln!(io::stdout(), "Part 1: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(sum)
}

fn part2(image: &Image) -> Result<usize> {
    let _start = Instant::now();

    let sum = image.shortest_path_sum(1000000);

    writeln!(io::stdout(), "Part 2: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(sum)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let image = parse_input(input);

    part1(&image)?;
    part2(&image)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    let image = parse_input(input);
    println!("{:?}", image);
    assert_eq!(part1(&image).unwrap(), 374);
    assert_eq!(image.shortest_path_sum(10), 1030);
    assert_eq!(image.shortest_path_sum(100), 8410);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let image = parse_input(input);
    assert_eq!(part1(&image).unwrap(), 9214785);
    assert_eq!(part2(&image).unwrap(), 613686987427);
}
