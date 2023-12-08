use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let games: Vec<Game> = BufReader::new(File::open("input/day02.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().as_str().into())
        .collect();

    let possible = games.iter().filter(|game| {
        game.bags
            .iter()
            .all(|bag| bag.red <= 12 && bag.green <= 13 && bag.blue <= 14)
    });
    println!("Day 2 Part One: {}", possible.map(|g| g.n).sum::<u32>());

    let power = games.iter().map(|game| {
        let b = game.bags.iter().copied().reduce(bagmax).unwrap();
        b.red * b.green * b.blue
    });
    println!("Day 2 Part Two: {}", power.sum::<u32>());
}

struct Game {
    n: u32,
    bags: Vec<Bag>,
}

#[derive(Default, Clone, Copy)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let (s1, s) = s.split_once(':').unwrap();
        let n = s1.split_once(' ').unwrap().1.parse().unwrap();
        let bags = s.split(';').map(Bag::from).collect();
        Game { n, bags }
    }
}

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        let mut bag = Bag::default();
        for s in s.split(',') {
            let n = s.split_whitespace().next().unwrap().parse().unwrap();
            if s.ends_with("red") {
                bag.red = n;
            } else if s.ends_with("green") {
                bag.green = n;
            } else if s.ends_with("blue") {
                bag.blue = n;
            } else {
                panic!("{}", s);
            }
        }
        bag
    }
}

fn bagmax(a: Bag, b: Bag) -> Bag {
    Bag {
        red: a.red.max(b.red),
        green: a.green.max(b.green),
        blue: a.blue.max(b.blue),
    }
}
