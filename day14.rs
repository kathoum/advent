#[derive(Debug, Clone)]
struct Amount(i64, String);

#[derive(Debug)]
struct Rule {
    input: Vec<Amount>,
    output: Amount
}

fn parse_amount(s: &str) -> Amount {
    let mut words = s.split_whitespace();
    let count = words.next().unwrap().parse().unwrap();
    let name = words.next().unwrap();
    assert!(words.next().is_none());
    assert!(count > 0);
    Amount(count, name.to_string())
}

fn parse_rule(line: &str) -> Rule {
    let mut s = line.split("=>");
    let input = s.next().unwrap().split(",").map(|s| parse_amount(s)).collect();
    let output = parse_amount(s.next().unwrap());
    assert!(s.next().is_none());
    Rule { input, output }
}

fn div_ceil(a: i64, b: i64) -> i64 {
    assert!(b > 0 && a >= 0);
    (a + b - 1) / b
}

fn backtrack(rules: &Vec<Rule>, from: &str, to: &Amount) -> i64 {
    let mut stock = std::collections::HashMap::new();
    stock.insert(to.1.to_string(), to.0);
    loop {
        match stock.iter().find(|(name, &amount)| *name != from && amount > 0) {
            None => {
                return *stock.entry(from.to_string()).or_default();
            }
            Some((name, &required_count)) => {
                let rule = rules.iter().find(|rule| rule.output.1 == *name).unwrap();
                let Amount(produced_count, name) = rule.output.clone();
                let multiplier = div_ceil(required_count, produced_count);
                *stock.entry(name.to_string()).or_default() -= multiplier * produced_count;
                for Amount(count, material) in rule.input.iter() {
                    *stock.entry(material.to_string()).or_default() += multiplier * count;
                }
            }
        }
    }
}

fn trackforward(rules: &Vec<Rule>, from: &Amount, to: &str) -> i64 {
    // maximum material produced is >=lower and <upper
    let mut upper = from.0 + 1;
    let mut lower = from.0 / backtrack(rules, &from.1, &Amount(1, to.to_string()));
    // bisection
    while lower < upper - 1 {
        let target = Amount((lower + upper) / 2, to.to_string());
        let source = backtrack(rules, &from.1, &target);
        // source is necessary to produce target.0
        if source <= from.0 {
            lower = target.0
        } else {
            upper = target.0
        }
    }
    lower
}

fn parse_rules(reader: impl std::io::BufRead) -> impl Iterator<Item = Rule> {
    reader.lines().map(|l| parse_rule(&l.unwrap()))
}

fn run(reader: impl std::io::BufRead) {
    let rules: Vec<_> = parse_rules(reader).collect();

    let target = Amount(1, "FUEL".to_string());
    let source = backtrack(&rules, "ORE", &target);
    println!("{} ore required for 1 fuel", source);

    let source = Amount(1_000_000_000_000, "ORE".to_string());
    let target = trackforward(&rules, &source, "FUEL");
    println!("{} fuel produced with 1 trillion ore", target);
}

fn main() {
    run(std::io::Cursor::new(
        "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL"));

    run(std::io::Cursor::new(
        "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL"));

    run(std::io::Cursor::new(
        "157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"));

    run(std::io::Cursor::new(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF"));
    
    run(std::io::Cursor::new(
        "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX"));

    let input = "input14.txt";
    let reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    run(reader);
}
