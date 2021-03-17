use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input07.txt");

    let rules: Result<Vec<Rule>, ParseRuleError> = input.lines().map(Rule::from_str).collect();
    let rules = rules.unwrap();
    let rules = rules.iter().map(|Rule(gate, wire)| (wire, gate)).collect();
    let mut results = HashMap::new();
    let signal = eval(&"a".to_string(), &rules, &mut results);
    println!("Part one: wire a has signal {}", signal);

    results.clear();
    results.insert("b".to_string(), signal);
    let signal = eval(&"a".to_string(), &rules, &mut results);
    println!("Part two: wire a has signal {}", signal);
}

fn eval(name: &Wire, rules: &HashMap<&Wire, &Gate>, results: &mut HashMap<Wire, u16>) -> u16 {
    if let Some(n) = results.get(name) {
        return *n;
    }
    if let Ok(n) = u16::from_str(name) {
        return n;
    }
    let n = match rules[name] {
        Gate::Copy(w) => eval(w, rules, results),
        Gate::Not(w) => ! eval(w, rules, results),
        Gate::And(w1, w2) => eval(w1, rules, results) & eval(w2, rules, results),
        Gate::Or(w1, w2) => eval(w1, rules, results) | eval(w2, rules, results),
        Gate::LShift(w, n) => eval(w, rules, results) << n,
        Gate::RShift(w, n) => eval(w, rules, results) >> n,
    };
    results.insert(name.clone(), n);
    n
}

type Wire = String;
enum Gate {
    Copy(Wire),
    Not(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    LShift(Wire, u8),
    RShift(Wire, u8),
}
struct Rule(Gate, Wire);

struct ParseGateError;
impl std::str::FromStr for Gate {
    type Err = ParseGateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("NOT ") {
            Ok(Gate::Not(rest.into()))
        } else if let Some((left, right)) = split_mid(s, " AND ") {
            Ok(Gate::And(left.into(), right.into()))
        } else if let Some((left, right)) = split_mid(s, " OR ") {
            Ok(Gate::Or(left.into(), right.into()))
        } else if let Some((left, right)) = split_mid(s, " LSHIFT ") {
            Ok(Gate::LShift(left.into(), right.parse().map_err(|_| ParseGateError)?))
        } else if let Some((left, right)) = split_mid(s, " RSHIFT ") {
            Ok(Gate::RShift(left.into(), right.parse().map_err(|_| ParseGateError)?))
        } else {
            Ok(Gate::Copy(s.into()))
        }
    }
}

#[derive(Debug)] struct ParseRuleError;
impl std::str::FromStr for Rule {
    type Err = ParseRuleError;
    fn from_str(s: &str) -> Result<Rule, ParseRuleError> {
        let (left, right) = split_mid(s, " -> ").ok_or(ParseRuleError)?;
        Ok(Rule(
            left.parse().map_err(|_| ParseRuleError)?,
            right.into()))
    }
}

fn split_mid<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    let mut split = s.splitn(2, pattern);
    let left = split.next();
    let right = split.next();
    match (left, right) {
        (Some(l), Some(r)) => Some((l, r)),
        _ => None,
    }
}
