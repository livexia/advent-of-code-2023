use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (isize, isize);

#[derive(Clone)]
struct Image {
    galaxies: Vec<Coord>,
    bound: Coord,
}

impl Image {
    fn expansion_rows(&mut self, expansion_rate: isize) {
        let mut empty_row = 0;
        let mut expanded_galaxies = Vec::new();
        for x in 0..self.bound.0 {
            let mut flag = true;
            self.galaxies
                .iter()
                .filter(|(i, _)| i == &x)
                .for_each(|(_, j)| {
                    flag = false;
                    expanded_galaxies.push((x + empty_row * (expansion_rate - 1), *j));
                });
            if flag {
                empty_row += 1;
            }
        }
        self.galaxies = expanded_galaxies;
    }

    fn expansion_columns(&mut self, expansion_rate: isize) {
        let mut empty_column = 0;
        let mut expanded_galaxies = Vec::new();
        for y in 0..self.bound.1 {
            let mut flag = true;
            self.galaxies
                .iter()
                .filter(|(_, j)| j == &y)
                .for_each(|(i, _)| {
                    flag = false;
                    expanded_galaxies.push((*i, y + empty_column * (expansion_rate - 1)));
                });
            if flag {
                empty_column += 1;
            }
        }
        self.galaxies = expanded_galaxies;
    }

    fn update_bound(&mut self) {
        self.bound = (
            (self.galaxies.iter().max_by_key(|c| c.0).unwrap()).0 + 1,
            (self.galaxies.iter().max_by_key(|c| c.1).unwrap()).1 + 1,
        );
    }

    fn expansion(&mut self, expansion_rate: isize) {
        self.expansion_rows(expansion_rate);
        self.expansion_columns(expansion_rate);
        self.update_bound();
    }

    fn shortest_path_sum(&self) -> isize {
        fn dis(p1: &Coord, p2: &Coord) -> isize {
            (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
        }
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                sum += dis(&self.galaxies[i], &self.galaxies[j])
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
                if self.galaxies.contains(&(x, y)) {
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
    let galaxies: Vec<_> = input
        .as_ref()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .flat_map(|(x, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(y, _)| (x as isize, y as isize))
        })
        .collect();

    let bound = (
        (galaxies.iter().max_by_key(|c| c.0).unwrap()).0 + 1,
        (galaxies.iter().max_by_key(|c| c.1).unwrap()).1 + 1,
    );

    Image { galaxies, bound }
}

fn shortest_path_sum_with_expansion(image: &Image, expansion_rate: isize) -> isize {
    let mut image = image.clone();
    image.expansion(expansion_rate);
    image.shortest_path_sum()
}

fn part1(image: &Image) -> Result<isize> {
    let _start = Instant::now();

    let sum = shortest_path_sum_with_expansion(image, 2);

    writeln!(io::stdout(), "Part 1: {sum}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(sum)
}

fn part2(image: &Image) -> Result<isize> {
    let _start = Instant::now();

    let sum = shortest_path_sum_with_expansion(image, 1000000);

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
    assert_eq!(shortest_path_sum_with_expansion(&image, 10), 1030);
    assert_eq!(shortest_path_sum_with_expansion(&image, 100), 8410);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let image = parse_input(input);
    assert_eq!(part1(&image).unwrap(), 9214785);
    assert_eq!(part2(&image).unwrap(), 613686987427);
}
