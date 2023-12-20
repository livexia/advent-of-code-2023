use std::collections::{HashMap, HashSet, VecDeque};
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
enum Module {
    FlipFlop(String, bool),
    Conjunction(String, HashMap<String, bool>),
    Broadcaster,
    Button,
}

impl Module {
    fn name(&self) -> &str {
        match self {
            Module::FlipFlop(name, _) => name,
            Module::Conjunction(name, _) => name,
            Module::Broadcaster => "broadcaster",
            Module::Button => "button",
        }
    }
}

impl FromStr for Module {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim().trim_matches(',');
        if s == "broadcaster" {
            Ok(Module::Broadcaster)
        } else if let Some(name) = s.strip_prefix('%') {
            Ok(Module::FlipFlop(name.to_string(), false))
        } else if let Some(name) = s.strip_prefix('&') {
            Ok(Module::Conjunction(name.to_string(), HashMap::new()))
        } else {
            Ok(Module::Button)
        }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    modules: HashMap<String, Module>,
    cables: HashMap<String, Vec<String>>,
}

impl FromStr for Machine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut modules = HashMap::new();
        let mut cables = HashMap::new();

        let mut cons = HashSet::new();

        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            if let Some((src, _)) = line.split_once("->") {
                let module = src.parse::<Module>()?;
                let name = module.name().to_string();
                if let Module::Conjunction(_, _) = module {
                    cons.insert(name.clone());
                }
                modules.insert(name, module);
            }
        }
        for line in s.lines().filter(|l| !l.trim().is_empty()) {
            if let Some((src, dest)) = line.split_once("->") {
                let name = src.trim().trim_matches(['%', '&']).to_string();
                let dest = dest
                    .trim()
                    .split(", ")
                    .map(|s| {
                        let d = s.trim().to_string();
                        if cons.contains(&d) {
                            if let Some(Module::Conjunction(_, m)) = modules.get_mut(&d) {
                                m.insert(name.clone(), false);
                            }
                        }
                        d
                    })
                    .collect();
                cables.insert(name, dest);
            }
        }
        Ok(Self { modules, cables })
    }
}

impl Machine {
    fn push_button(&mut self, check_on: Option<&str>) -> (usize, usize, bool) {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));

        let mut low_cnt = 0;
        let mut high_cnt = 0;

        let mut flag = false;

        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            low_cnt += (!pulse) as usize;
            high_cnt += pulse as usize;
            // println!("{sender} - {pulse:?} -> {receiver}");

            let module = if let Some(module) = self.modules.get_mut(receiver) {
                module
            } else {
                continue;
            };
            let dest = self.cables.get(receiver).unwrap();
            match module {
                Module::FlipFlop(_, status) => {
                    if !pulse {
                        *status = !*status;
                        let next_pulse = *status;
                        for next_module in dest {
                            queue.push_back((receiver, next_module, next_pulse));
                        }
                    }
                }
                Module::Conjunction(_, status) => {
                    status.insert(sender.to_string(), pulse);
                    let next_pulse = !status.values().all(|b| *b);
                    if Some(receiver) == check_on {
                        for s in status.iter().filter(|(_, v)| **v) {
                            flag = true;
                            println!("vd: {s:?}");
                        }
                    }
                    for next_module in dest {
                        queue.push_back((receiver, next_module, next_pulse));
                    }
                }
                Module::Broadcaster => {
                    for next_module in dest {
                        queue.push_back((receiver, next_module, pulse));
                    }
                }
                Module::Button => queue.push_back(("button", "broadcaster", false)),
            }
        }

        (low_cnt, high_cnt, flag)
    }
}

fn part1(machine: &Machine) -> Result<usize> {
    let _start = Instant::now();

    let mut machine = machine.clone();

    let (mut low_cnt, mut high_cnt) = (0, 0);
    for _ in 0..1000 {
        let (c1, c2, _) = machine.push_button(None);
        low_cnt += c1;
        high_cnt += c2;
    }

    let result = low_cnt * high_cnt;

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn part2(machine: &Machine) -> Result<usize> {
    let _start = Instant::now();

    let mut machine = machine.clone();

    let rx_rely: Vec<_> = machine
        .cables
        .iter()
        .filter(|(_, v)| v.contains(&"rx".to_string()))
        .map(|(k, _)| k)
        .collect();
    if rx_rely.len() != 1 {
        unimplemented!("Unknown input for part 2");
    }
    let rx_rely = rx_rely[0].clone();

    for i in 1..10000 {
        let (_, _, b) = machine.push_button(Some(&rx_rely));
        if b {
            println!("{i}");
        }
    }

    let result = 0;

    writeln!(io::stdout(), "Part 2: Manual Calculation Part Two")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let machine = input.parse::<Machine>()?;

    part1(&machine)?;
    part2(&machine)?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    let machine = input.parse::<Machine>().unwrap();
    assert_eq!(part1(&machine).unwrap(), 32000000);

    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    let machine = input.parse::<Machine>().unwrap();
    assert_eq!(part1(&machine).unwrap(), 11687500);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let machine = input.parse::<Machine>().unwrap();
    assert_eq!(part1(&machine).unwrap(), 788848550);
}
