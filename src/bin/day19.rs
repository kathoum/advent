use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

type RuleID = i32;

fn main() {
    let input = include_str!("input19.txt");

    let rule_lines: HashMap<RuleID, &'static str> = input.lines().filter_map(|line| {
        let sep = line.find(':');
        match sep {
            None => None,
            Some(sep) =>
                Some((line[..sep].parse().unwrap(), &line[(sep + 1)..]))
        }
    }).collect();

    let rules: HashMap<RuleID, Rc<RefCell<Rule>>> = rule_lines.keys().map(|id|
        (*id, Rc::new(RefCell::new(Rule::Placeholder)))
    ).collect();

    for (id, line) in rule_lines.iter() {
        let rule = Rule::parse(line, &rules);
        let old = rules[id].replace(rule);
        assert!(matches!(old, Rule::Placeholder), "Multiple definition for rule {}", id);
    }

    let messages = input.lines().filter(|&s| !s.contains(':')).collect::<Vec<_>>();

    println!("Part One");
    let rule_zero = rules[&0].borrow();
    let count = messages.iter()
        .filter(|&s| rule_zero.matches(s))
        .count();
    println!("{} matching lines", count);

    println!("Part Two");
    rules[&8].replace(Rule::parse("42 | 42 8", &rules));
    rules[&11].replace(Rule::parse("42 31 | 42 11 31", &rules));
    let count = messages.iter()
        .filter(|&s| rule_zero.matches(s))
        .count();
    println!("{} matching lines", count);
}

enum Rule {
    Text(String),
    Or(Box<Rule>, Box<Rule>),
    Seq(Box<Rule>, Box<Rule>),
    Alias(Weak<RefCell<Rule>>),
    Placeholder
}

impl Rule {
    fn parse(rule: &str, map: &HashMap<RuleID, Rc<RefCell<Rule>>>) -> Rule {
        let rule = rule.trim();
        if rule.starts_with('"') && rule.ends_with('"') {
            let text = rule.trim_matches('"').into();
            Rule::Text(text)
        } else if let Some(sep) = rule.find('|') {
            let first = Rule::parse(&rule[..sep], map);
            let second = Rule::parse(&rule[(sep + 1)..], map);
            Rule::Or(Box::new(first), Box::new(second))
        } else if let Some(sep) = rule.find(' ') {
            let first = Rule::parse(&rule[..sep], map);
            let second = Rule::parse(&rule[(sep + 1)..], map);
            Rule::Seq(Box::new(first), Box::new(second))
        } else {
            let id = rule.parse::<RuleID>().unwrap();
            Rule::Alias(Rc::downgrade(&map[&id]))
        }
    }

    fn matches(&self, text: &str) -> bool {
        self.matches_prefix(text).iter()
            .any(|suffix| suffix.is_empty())
    }

    fn matches_prefix<'a>(&self, text: &'a str) -> Vec<&'a str> {
        match self {
            Rule::Text(t) =>
                match text.strip_prefix(t) {
                    Some(suffix) => vec![suffix],
                    None => Vec::new(),
                }
            Rule::Or(a, b) => {
                let mut m = a.matches_prefix(text);
                m.append(&mut b.matches_prefix(text));
                m
            }
            Rule::Seq(a, b) =>
                a.matches_prefix(text).into_iter().flat_map(
                    |suffix| b.matches_prefix(suffix)
                ).collect(),
            Rule::Alias(a) =>
                a.upgrade().unwrap().borrow().matches_prefix(text),
            Rule::Placeholder =>
                panic!("Trying to match undefined rule")
        }
    }
}
