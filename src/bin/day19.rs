use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("input/day19.txt").unwrap())
        .lines()
        .map(|line| line.unwrap());
    let towels = lines.next().unwrap();
    let towels: Vec<&str> = towels.split(", ").collect();
    let designs: Vec<String> = lines.filter(|line| !line.is_empty()).collect();

    let n = designs
        .iter()
        .filter(|design| is_possible(&towels, design))
        .count();
    println!("Day 19 part one: {n}");

    let mut cache = HashMap::new();
    let n = designs
        .iter()
        .map(|design| count_possible_ways(&towels, design, &mut cache))
        .sum::<usize>();
    println!("Day 19 part two: {n}");
}

fn is_possible(towels: &[&str], design: &str) -> bool {
    design.is_empty()
        || towels.iter().any(|towel| {
            design
                .strip_prefix(towel)
                .is_some_and(|rest| is_possible(towels, rest))
        })
}

fn count_possible_ways<'a>(
    towels: &[&str],
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&n) = cache.get(design) {
        return n;
    }
    let n = towels
        .iter()
        .map(|towel| {
            design
                .strip_prefix(towel)
                .map(|rest| count_possible_ways(towels, rest, cache))
                .unwrap_or_default()
        })
        .sum();
    cache.insert(design, n);
    n
}
