use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let mut lines = BufReader::new(File::open("input/day19.txt").unwrap()).lines();
    let mut workflows = HashMap::new();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (name, workflow) = parse_workflow(&line).unwrap();
        workflows.insert(name, workflow);
    }
    let ratings: Vec<Rating> = lines.map(|line| line.unwrap().parse().unwrap()).collect();

    let answer: i32 = ratings
        .iter()
        .filter(|rating| process(&workflows, rating))
        .map(|rating| rating.0.iter().sum::<i32>())
        .sum();
    println!("Day 19 Part One: {answer}");

    const FULL_RANGE: RangeInclusive<i32> = 1..=4000;
    let answer = count_accepted(&workflows, RatingRange([FULL_RANGE; 4]), "in");
    println!("Day 19 Part Two: {answer}");
}

fn process(workflows: &HashMap<String, Workflow>, rating: &Rating) -> bool {
    let mut state = "in";
    loop {
        state = transform(&workflows[state], rating);
        match state {
            "A" => return true,
            "R" => return false,
            _ => (),
        }
    }
}

fn transform<'a>(workflow: &'a Workflow, rating: &Rating) -> &'a str {
    for (rule, dest) in &workflow.0 {
        if match rule {
            &Rule::Less(c, v) => rating.0[c] < v,
            &Rule::More(c, v) => rating.0[c] > v,
            Rule::Always => true,
        } {
            return dest;
        }
    }
    panic!()
}

fn count_accepted(
    workflows: &HashMap<String, Workflow>,
    mut range: RatingRange,
    state: &str,
) -> usize {
    if state == "A" {
        range.count()
    } else if state == "R" || range.is_empty() {
        0
    } else {
        let mut count = 0;
        for (rule, dest) in &workflows[state].0 {
            let (matched, rest) = range.split(rule);
            count += count_accepted(workflows, matched, dest);
            range = rest;
        }
        count
    }
}

struct Workflow(Vec<(Rule, String)>);
struct Rating([i32; 4]);

#[derive(Clone)]
struct RatingRange([RangeInclusive<i32>; 4]);

impl RatingRange {
    fn empty() -> Self {
        const EMPTY: RangeInclusive<i32> = RangeInclusive::new(1, 0);
        RatingRange([EMPTY; 4])
    }

    fn is_empty(&self) -> bool {
        self.0.iter().any(RangeInclusive::is_empty)
    }

    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|r| (r.end() - r.start() + 1).max(0) as usize)
            .product()
    }

    fn split(self, rule: &Rule) -> (RatingRange, RatingRange) {
        match rule {
            &Rule::Less(c, v) => {
                let (a, b) = self.0[c].clone().into_inner();
                let mut matched = self.clone();
                matched.0[c] = a..=b.min(v - 1);
                let mut rest = self;
                rest.0[c] = a.max(v)..=b;
                (matched, rest)
            }
            &Rule::More(c, v) => {
                let (a, b) = self.split(&Rule::Less(c, v + 1));
                (b, a)
            }
            Rule::Always => (self, Self::empty()),
        }
    }
}

enum Rule {
    Less(Category, i32),
    More(Category, i32),
    Always,
}

type Category = usize;

fn parse_workflow(s: &str) -> Option<(String, Workflow)> {
    let i = s.find('{')?;
    let workflow = s[i..].parse().ok()?;
    Some((s[..i].to_owned(), workflow))
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').ok_or(())?;
        let s = s.strip_suffix('}').ok_or(())?;
        let mut workflow = Workflow(vec![]);
        for rule in s.split(',') {
            let (rule, dest) = if let Some((rule, dest)) = rule.split_once(':') {
                if let Some((c, v)) = rule.split_once('<') {
                    (Rule::Less(category(c)?, v.parse().map_err(|_| ())?), dest)
                } else if let Some((c, v)) = rule.split_once('>') {
                    (Rule::More(category(c)?, v.parse().map_err(|_| ())?), dest)
                } else {
                    return Err(());
                }
            } else {
                (Rule::Always, rule)
            };
            workflow.0.push((rule, dest.to_owned()));
        }
        Ok(workflow)
    }
}

impl FromStr for Rating {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').ok_or(())?;
        let s = s.strip_suffix('}').ok_or(())?;
        let mut rating = Rating([0; 4]);
        for s in s.split(',') {
            let (c, v) = s.split_once('=').ok_or(())?;
            rating.0[category(c)?] = v.parse().map_err(|_| ())?;
        }
        Ok(rating)
    }
}

fn category(s: &str) -> Result<Category, ()> {
    match s {
        "x" => Ok(0),
        "m" => Ok(1),
        "a" => Ok(2),
        "s" => Ok(3),
        _ => Err(()),
    }
}
