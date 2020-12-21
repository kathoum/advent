use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type RuleID = i32;
type RcRule = Rc<RefCell<Rule>>;

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

    let rules: HashMap<RuleID, RcRule> = rule_lines.keys().map(|id|
        (*id, Rc::new(RefCell::new(Rule::Placeholder)))
    ).collect();

    for (id, line) in rule_lines.iter() {
        let rule = Rule::parse(line, &rules);
        rules[id].replace(Rule::Alias(rule));
    }

    let messages = input.lines().filter(|&s| !s.contains(':')).collect::<Vec<_>>();

    println!("Part One");
    let rule_zero = rules[&0].borrow();
    let count = messages.iter()
        .filter(|&s| rule_zero.matches(s))
        .count();
    println!("{} matching lines", count);

    println!("Part Two");
    rules[&8].replace(Rule::Alias(Rule::parse("42 | 42 8", &rules)));
    rules[&11].replace(Rule::Alias(Rule::parse("42 31 | 42 11 31", &rules)));
    let count = messages.iter()
        .filter(|&s| rule_zero.matches(s))
        .count();
    println!("{} matching lines", count);
}

enum Rule {
    Text(String),
    Or(RcRule, RcRule),
    Seq(RcRule, RcRule),
    Alias(RcRule),
    Placeholder
}

impl Rule {
    fn parse(rule: &str, map: &HashMap<RuleID, RcRule>) -> RcRule {
        let rule = rule.trim();
        if rule.starts_with('"') && rule.ends_with('"') {
            let text = rule.trim_matches('"').into();
            Rc::new(RefCell::new(Rule::Text(text)))
        } else if let Some(sep) = rule.find('|') {
            let first = Rule::parse(&rule[..sep], map);
            let second = Rule::parse(&rule[(sep + 1)..], map);
            Rc::new(RefCell::new(Rule::Or(first, second)))
        } else if let Some(sep) = rule.find(' ') {
            let first = Rule::parse(&rule[..sep], map);
            let second = Rule::parse(&rule[(sep + 1)..], map);
            Rc::new(RefCell::new(Rule::Seq(first, second)))
        } else {
            let id = rule.parse::<RuleID>().unwrap();
            map[&id].clone()
        }
    }

    fn matches(&self, text: &str) -> bool {
        match self {
            Rule::Text(t) =>
                t == text,
            Rule::Or(a, b) =>
                a.borrow().matches(text) || b.borrow().matches(text),
            Rule::Seq(a, b) =>
                a.borrow().matches_prefix(text).into_iter().any(
                    |suffix| b.borrow().matches(suffix)),
            Rule::Alias(a) =>
                a.borrow().matches(text),
            Rule::Placeholder =>
                panic!()
        }
    }

    fn matches_prefix<'a>(&self, text: &'a str) -> Vec<&'a str> {
        match self {
            Rule::Text(t) =>
                match text.strip_prefix(t) {
                    Some(suffix) => vec![suffix],
                    None => Vec::new(),
                }
            Rule::Or(a, b) => {
                let mut m = a.borrow().matches_prefix(text);
                m.append(&mut b.borrow().matches_prefix(text));
                m
            }
            Rule::Seq(a, b) =>
                a.borrow().matches_prefix(text).into_iter().flat_map(
                    |suffix| b.borrow().matches_prefix(suffix)
                ).collect(),
            Rule::Alias(a) =>
                a.borrow().matches_prefix(text),
            Rule::Placeholder =>
                panic!()
        }
    }
}
