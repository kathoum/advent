use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;

type Rule = (String, Vec<(u32, String)>);

fn parse_color(s: &str) -> Option<(u32, String)> {
    let words: Vec<_> = s.split_whitespace().collect();
    if words.is_empty() {
        return None;
    }
    let count: u32 = match words[0].parse() {
        Ok(n) => n,
        Err(_) => return None
    };
    Some((count, words[1].to_owned() + " " + words[2]))
}

fn parse_rule(s: &str) -> Rule {
    let mut i = s.splitn(2, "contain");
    let container = i.next().unwrap().split_whitespace().take(2).collect::<Vec<_>>().join(" ");
    let contained: Vec<&str> = i.next().unwrap().split(&[',', '.'][..]).collect();
    (container, contained.iter().filter_map(|s| parse_color(*s)).collect())
}

fn parents<'a>(ruleset: &'a [Rule], item: &str) -> Vec<&'a Rule> {
    ruleset.iter().filter(|r| r.1.iter().find(|(_, s)| s == item).is_some()).collect()
}

fn parents_recursive<'a>(ruleset: &'a [Rule], item: &str) -> HashSet<&'a str> {
    let mut set = HashSet::<&str>::new();
    for rule in parents(ruleset, item) {
        set.insert(&rule.0);
        set.extend(parents_recursive(ruleset, &rule.0));
    }
    set
}

fn contained_bags(ruleset: &HashMap<String, Vec<(u32, String)>>, item: &str) -> u32 {
    ruleset[item].iter().map(|(count, s)|
        count * (1 + contained_bags(ruleset, s))
    ).sum()
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input07.txt"));
    let rules: Vec<Rule> = reader.lines().map(|l| parse_rule(&l.unwrap())).collect();

    println!("Part One");
    let p = parents_recursive(&rules, "shiny gold");
    println!("{} possible colors", p.len());

    println!("Part Two");
    let rulesmap: HashMap<_,_> = rules.into_iter().collect();
    let total = contained_bags(&rulesmap, "shiny gold");
    println!("{} total contained bags", total);
}
