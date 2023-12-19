use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type WorkflowIdMap = HashMap<usize, Workflow>;
type WorkflowNameMap = HashMap<String, usize>;

#[derive(Debug, Clone, Copy)]
enum ProcessingResult {
    Accepted,
    Rejected,
    Workflow(usize),
}

impl ProcessingResult {
    fn from_str(s: &str, last_id: &mut usize, workflow_map: &mut WorkflowNameMap) -> Self {
        if s == "A" {
            Self::Accepted
        } else if s == "R" {
            Self::Rejected
        } else {
            Self::Workflow(gen_workflow_map(s, last_id, workflow_map))
        }
    }
}

#[derive(Debug)]
struct Rule {
    rating: Option<char>,
    op: Option<char>,
    op1: Option<usize>,
    result: ProcessingResult,
}

impl Rule {
    fn from_str(s: &str, last_id: &mut usize, workflow_map: &mut WorkflowNameMap) -> Result<Self> {
        if let Some((left, right)) = s.split_once(':') {
            let (rating, op, op1) = if let Some((rating, op1)) = left.split_once('<') {
                (rating.chars().next().unwrap(), '<', op1.parse()?)
            } else if let Some((rating, op1)) = left.split_once('>') {
                (rating.chars().next().unwrap(), '>', op1.parse()?)
            } else {
                return err!("unable to parse rule: {s:?}");
            };

            let name = right.trim().trim_matches('}');
            Ok(Rule {
                rating: Some(rating),
                op: Some(op),
                op1: Some(op1),
                result: ProcessingResult::from_str(name, last_id, workflow_map),
            })
        } else {
            let name = s.trim().trim_matches(['}']);
            Ok(Rule {
                rating: None,
                op: None,
                op1: None,
                result: ProcessingResult::from_str(name, last_id, workflow_map),
            })
        }
    }

    fn process(&self, other: char, op2: usize) -> Option<ProcessingResult> {
        if let (Some(name), Some(op), Some(op1)) = (self.rating, self.op, self.op1) {
            if name != other {
                None
            } else {
                match op {
                    '>' => {
                        if op2.cmp(&op1) == std::cmp::Ordering::Greater {
                            Some(self.result)
                        } else {
                            None
                        }
                    }
                    '<' => {
                        if op2.cmp(&op1) == std::cmp::Ordering::Less {
                            Some(self.result)
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            Some(self.result)
        }
    }
}

#[derive(Debug)]
struct Workflow {
    id: usize,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_str(
        s: &str,
        last_id: &mut usize,
        workflow_map: &mut HashMap<String, usize>,
    ) -> Result<Self> {
        if let Some((w, r)) = s.split_once('{') {
            let name = w.trim();
            let id = gen_workflow_map(name, last_id, workflow_map);
            let rules = r
                .split(',')
                .map(|p| Rule::from_str(p.trim().trim_matches(['}']), last_id, workflow_map))
                .collect::<Result<Vec<_>>>()?;
            return Ok(Self { id, rules });
        }
        err!("unable to parse workflow: {s:?}")
    }

    fn process(&self, rating: &[(char, usize)]) -> ProcessingResult {
        let l = self.rules.len();
        for rule in &self.rules[..l - 1] {
            for &(name, op2) in rating {
                if let Some(r) = rule.process(name, op2) {
                    return r;
                }
            }
        }
        self.rules[l - 1].result
    }
}

fn gen_workflow_map(
    name: &str,
    last_id: &mut usize,
    workflow_map: &mut HashMap<String, usize>,
) -> usize {
    if let Some(id) = workflow_map.get(name) {
        *id
    } else {
        *last_id += 1;
        workflow_map.insert(name.to_string(), *last_id);
        *last_id
    }
}

#[allow(clippy::type_complexity)]
fn parse_input<T: AsRef<str>>(
    input: T,
) -> Result<(Vec<Vec<(char, usize)>>, WorkflowIdMap, WorkflowNameMap)> {
    if let Some((raw_ws, rs)) = input.as_ref().split_once("\n\n") {
        let mut last_id = 0;
        let mut workflow_map = HashMap::new();

        let mut ws = HashMap::new();
        for l in raw_ws.split_whitespace() {
            let wf = Workflow::from_str(l, &mut last_id, &mut workflow_map)?;
            ws.insert(wf.id, wf);
        }

        return Ok((
            rs.split_whitespace()
                .map(|l| {
                    l.trim_matches(['}', '{'])
                        .split(',')
                        .map(|p| {
                            if let Some((r, op2)) = p.split_once('=') {
                                Ok((r.chars().next().unwrap(), op2.parse::<usize>().unwrap()))
                            } else {
                                err!("unabel to parse rating")
                            }
                        })
                        .collect::<Result<Vec<_>>>()
                })
                .collect::<Result<Vec<_>>>()?,
            ws,
            workflow_map,
        ));
    }
    err!("unable to parse input")
}

fn process(rating: &[(char, usize)], ws: &WorkflowIdMap, map: &WorkflowNameMap) -> bool {
    let mut curr_wf = ws.get(map.get("in").unwrap()).unwrap();

    loop {
        match curr_wf.process(rating) {
            ProcessingResult::Accepted => return true,
            ProcessingResult::Rejected => return false,
            ProcessingResult::Workflow(id) => {
                curr_wf = ws.get(&id).unwrap();
            }
        }
    }
}

fn part1(
    rs: &[Vec<(char, usize)>],
    ws: &HashMap<usize, Workflow>,
    map: &HashMap<String, usize>,
) -> Result<usize> {
    let _start = Instant::now();

    let result = rs
        .iter()
        .filter(|r| process(r, ws, map))
        .map(|r| r.iter().map(|(_, v)| v).sum::<usize>())
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", _start.elapsed())?;
    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (rs, ws, map) = parse_input(input)?;
    part1(&rs, &ws, &map)?;
    // part2()?;
    Ok(())
}

#[test]
fn example_input() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    let (rs, ws, map) = parse_input(input).unwrap();
    assert_eq!(part1(&rs, &ws, &map).unwrap(), 19114);
}

#[test]
fn real_input() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let (rs, ws, map) = parse_input(input).unwrap();
    assert_eq!(part1(&rs, &ws, &map).unwrap(), 391132);
}
