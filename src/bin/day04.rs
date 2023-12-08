use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut num_cards = 0;
    let s = BufReader::new(File::open("input/day04.txt").unwrap())
        .lines()
        .map(|line| {
            num_cards += 1;
            1 << matches(&line.unwrap()) >> 1
        })
        .sum::<u32>();
    println!("Day 4 Part One: {s}");

    let mut copies = vec![1; num_cards];
    for (i, m) in BufReader::new(File::open("input/day04.txt").unwrap())
        .lines()
        .map(|line| matches(&line.unwrap()))
        .enumerate()
    {
        let n = copies[i];
        for c in &mut copies[(i + 1)..(i + 1 + m).min(num_cards)] {
            *c += n;
        }
    }
    println!("Day 4 Part Two: {}", copies.iter().sum::<u32>());
}

fn matches(card: &str) -> usize {
    let (_, s) = card.split_once(':').unwrap();
    let (w, n) = s.split_once('|').unwrap();
    let winners: HashSet<u32> = w.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let numbers: HashSet<u32> = n.split_whitespace().map(|s| s.parse().unwrap()).collect();
    winners.intersection(&numbers).count()
}
