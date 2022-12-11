use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day11.txt").unwrap());
    let monkeys: Vec<Monkey> = monkeys(reader).collect();

    let mut items = starting_items(&monkeys);
    let mut inspected_count = vec![0; monkeys.len()];
    for _ in 0..20 {
        play_round(&monkeys, &mut items, &mut inspected_count, |n| n / 3);
    }
    println!("The level of business is {}", business_level(&inspected_count));

    let mut items = starting_items(&monkeys);
    let mut inspected_count = vec![0; monkeys.len()];
    let modulo: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 0..10000 {
        play_round(&monkeys, &mut items, &mut inspected_count, |n| n % modulo);
    }
    println!("The level of business is {}", business_level(&inspected_count));
}

struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    destination: (usize, usize),
}

#[derive(Copy, Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    pub fn apply(&self, input: u64) -> u64 {
        match *self {
            Operation::Add(n) => input + n,
            Operation::Mul(n) => input * n,
            Operation::Square => input * input,
        }
    }
}

fn starting_items(monkeys: &[Monkey]) -> Vec<Vec<u64>> {
    monkeys.iter().map(|m| m.starting_items.clone()).collect()
}

fn play_round(monkeys: &[Monkey], items: &mut [Vec<u64>], inspected_count: &mut[usize], worry: impl Fn(u64) -> u64) {
    for (i, monkey) in monkeys.iter().enumerate() {
        let v = std::mem::take(&mut items[i]);
        inspected_count[i] += v.len();
        for item in v {
            let item = monkey.operation.apply(item);
            let item = worry(item);
            let dest = if item % monkey.divisor == 0 { monkey.destination.0 } else { monkey.destination.1 };
            items[dest].push(item);
        }
    }
}

fn business_level(counts: &[usize]) -> usize {
    let (i,j) = counts.iter().fold((0,0), |(i,j), &n| {
        if n > i { (n, i) }
        else if n > j { (i, n) }
        else { (i, j) }
    });
    i * j
}

fn monkeys(reader: impl BufRead) -> impl Iterator<Item = Monkey> {
    #[derive(Default)]
    struct MonkeyBuilder {
        starting_items: Option<Vec<u64>>,
        operation: Option<Operation>,
        divisor: Option<u64>,
        destination_if_true: Option<usize>,
        destination_if_false: Option<usize>,
    }

    impl MonkeyBuilder {
        pub fn try_build(&mut self) -> Option<Monkey> {
            match self {
                &mut MonkeyBuilder {
                    starting_items: Some(ref mut v),
                    operation: Some(op),
                    divisor: Some(div),
                    destination_if_true: Some(i),
                    destination_if_false: Some(j),
                } => {
                    let monkey = Monkey {
                        starting_items: std::mem::take(v),
                        operation: op,
                        divisor: div,
                        destination: (i, j),
                    };
                    *self = Default::default();
                    Some(monkey)
                }
                _ => None,
            }
        }
    }

    reader.lines().scan(MonkeyBuilder::default(), |builder, line| {
        let line = line.unwrap();
        if let Some((_, items)) = line.split_once("Starting items: ") {
            assert!(builder.starting_items.is_none());
            let starting_items = items.split(", ").map(|n| n.parse::<u64>().unwrap()).collect();
            builder.starting_items = Some(starting_items);
        } else if let Some((_, expr)) = line.split_once("Operation: new = ") {
            assert!(builder.operation.is_none());
            let op = if expr == "old * old" {
                Operation::Square
            } else if expr.starts_with("old + ") {
                Operation::Add(expr.rsplit(' ').next().unwrap().parse().unwrap())
            } else if expr.starts_with("old * ") {
                Operation::Mul(expr.rsplit(' ').next().unwrap().parse().unwrap())
            } else {
                panic!("Unrecognized expression: {expr:?}")
            };
            builder.operation = Some(op);
        } else if let Some((_, n)) = line.split_once("Test: divisible by ") {
            assert!(builder.divisor.is_none());
            builder.divisor = Some(n.parse().unwrap());
        } else if let Some((_, id)) = line.split_once("If true: throw to monkey ") {
            assert!(builder.destination_if_true.is_none());
            builder.destination_if_true = Some(id.parse().unwrap());
        } else if let Some((_, id)) = line.split_once("If false: throw to monkey ") {
            assert!(builder.destination_if_false.is_none());
            builder.destination_if_false = Some(id.parse().unwrap());
        } else if !(line.is_empty() || line.starts_with("Monkey")) {
            panic!("Unexpected input {line:?}")
        };
        Some(builder.try_build())
    }).flatten()
}
