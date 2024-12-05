use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Page = i32;
struct RuleSet(HashSet<(Page, Page)>);

fn main() {
    let mut lines = BufReader::new(File::open("input/day05.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    let rules = RuleSet(
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|str| {
                let (a, b) = str.split_once('|').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect(),
    );
    let updates: Vec<Vec<Page>> = lines
        .map(|str| str.split(',').map(|p| p.parse().unwrap()).collect())
        .collect();

    let total: i32 = updates
        .iter()
        .filter(|update| rules.is_sorted(update))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Day 5 part one: {total}");

    let total: i32 = updates
        .iter()
        .filter(|update| !rules.is_sorted(update))
        .map(|update| rules.sorted(update)[update.len() / 2])
        .sum();
    println!("Day 5 part two: {total}");
}

impl RuleSet {
    fn is_sorted(&self, update: &[Page]) -> bool {
        for (i, &p) in update.iter().enumerate() {
            for &q in &update[i + 1..] {
                if self.0.contains(&(q, p)) {
                    return false;
                }
            }
        }
        true
    }

    fn sorted(&self, update: &[Page]) -> Vec<Page> {
        let mut update = update.to_vec();
        for i in 0..update.len() {
            'repeat: loop {
                for j in i + 1..update.len() {
                    if self.0.contains(&(update[j], update[i])) {
                        update.swap(i, j);
                        continue 'repeat;
                    }
                }
                break;
            }
        }
        update
    }
}
