use std::collections::HashMap;

fn main() {
    let input = include_str!("input16.txt");
    let data =
        "children: 3
        cats: 7
        samoyeds: 2
        pomeranians: 3
        akitas: 0
        vizslas: 0
        goldfish: 5
        trees: 3
        cars: 2
        perfumes: 1";
    let parsed_data: HashMap<String, i32> = data.lines().map(parse_item).collect();

    let aunts = input.lines().map(parse_aunt);
    for aunt in aunts.filter(|aunt| aunt.matches(&parsed_data)) {
        println!("Part one: Aunt {} matches", aunt.name);
    }

    let condition_data: HashMap<String, Condition> = parsed_data.iter().map(|(k,v)| {
        let c = match k.as_str() {
            "cats" | "trees" => Condition::Greater(*v),
            "pomeranians" | "goldfish" => Condition::Less(*v),
            _ => Condition::Equal(*v),
        };
        (k.clone(), c)
    }).collect();
    let aunts = input.lines().map(parse_aunt);
    for aunt in aunts.filter(|aunt| aunt.matches_condition(&condition_data)) {
        println!("Part two: Aunt {} matches", aunt.name);
    }
}

struct Aunt {
    name: i32,
    stuff: HashMap<String, i32>,
}

enum Condition {
    Equal(i32),
    Less(i32),
    Greater(i32),
}

impl Aunt {
    fn matches(&self, stuff: &HashMap<String, i32>) -> bool {
        for (name, amount) in stuff {
            if let Some(n) = self.stuff.get(name) {
                if n != amount {
                    return false;
                }
            }
        }
        true
    }

    fn matches_condition(&self, stuff: &HashMap<String, Condition>) -> bool {
        for (name, filter) in stuff {
            if let Some(n) = self.stuff.get(name) {
                if !match filter {
                    Condition::Equal(m) => n == m,
                    Condition::Less(m) => n < m,
                    Condition::Greater(m) => n > m,
                } {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_aunt(input: &str) -> Aunt {
    let mut iter = input.splitn(2, ": ");

    let name = iter.next().unwrap();
    let name = name.split_whitespace().nth(1).unwrap();
    let name = name.parse().unwrap();

    let rest = iter.next().unwrap();
    let stuff = rest.split(", ").map(parse_item).collect();
    Aunt { name, stuff }
}

fn parse_item(input: &str) -> (String, i32) {
    let mut it = input.splitn(2, ": ");
    let word = it.next().unwrap().trim();
    let number = it.next().unwrap();
    (word.into(), number.parse().unwrap())
}
