fn main() {
    let input = include_str!("input19.txt");

    let rules: std::collections::HashMap<i32, &str> = input.lines().filter_map(|s| {
        let sep = s.find(':');
        match sep {
            None => None,
            Some(sep) => {
                let (n, rule) = s.split_at(sep);
                Some((n.parse().unwrap(), &rule[1..]))
            }
        }
    }).collect();

    println!("Part One");
    let head_regex = format!("^{}$", to_regex("0", &rules));
    //println!("{}", head_regex);
    let re = regex::Regex::new(&head_regex).unwrap();
    let count = input.lines()
        .filter(|s| !s.contains(':'))
        .filter(|s| re.is_match(&s))
        //.inspect(|s| println!("{}", s))
        .count();
    println!("{} matching lines", count);

    println!("Part Two");
    let mut rules = rules;
    rules.insert(8, "42 +");
    rules.insert(11, "42 ++ 31");
    let modified_regex = format!("^{}$", to_regex("0", &rules));
    //println!("{}", head_regex);
    let re = regex::Regex::new(&modified_regex).unwrap();
    let count = input.lines()
        .filter(|s| !s.contains(':'))
        .filter(|s| re.is_match(&s))
        //.inspect(|s| println!("{}", s))
        .count();
    println!("{} matching lines", count);
}

fn to_regex(rule: &str, rules: &std::collections::HashMap<i32, &str>) -> String {
    let rule = rule.trim();
    if rule.starts_with('"') && rule.ends_with('"') {
        rule.trim_matches('"').into()
    } else if rule.contains('|') {
        let r = rule.split('|')
            .map(|part| to_regex(part, rules))
            .collect::<Vec<String>>()
            .join("|");
        "(".to_string() + &r + &")"
    } else if rule.ends_with('+') {
        "((".to_string() + &to_regex(rule.trim_end_matches('+'), rules) + &")+)"
    } else if rule.contains("++") {
        let mut it = rule.split("++");
        let a = &to_regex(it.next().unwrap(), rules);
        let b = &to_regex(it.next().unwrap(), rules);
        assert!(it.next().is_none());
        [
            "(",
            a, b,
            "|", a, "{2}", b, "{2}",
            "|", a, "{3}", b, "{3}",
            "|", a, "{4}", b, "{4}",
            "|", a, "{5}", b, "{5}",
            // stop here, the number of matches doesn't increase after 5
            // "|", a, "{6}", b, "{6}",
            // "|", a, "{7}", b, "{7}",
            // "|", a, "{8}", b, "{8}",
            // "|", a, "{9}", b, "{9}",
            ")"
        ].concat()
    } else {
        rule.split_whitespace()
            .map(|n| to_regex(rules[&n.parse().unwrap()], rules))
            .fold(String::new(), |x, y| x + &y)
    }
}
