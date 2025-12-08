use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    let input = BufReader::new(File::open("input/day05.txt").unwrap());
    let mut lines = input.lines().map(Result::unwrap);
    let ranges: Vec<RangeInclusive<i64>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            a..=b
        })
        .collect();
    let ingredients: Vec<i64> = lines.filter_map(|line| line.parse().ok()).collect();

    let fresh_count = ingredients
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();
    println!("Day 5 part one: {fresh_count}");

    let all_fresh_ingredients = ranges.iter().fold(vec![], unite);
    let all_count = all_fresh_ingredients
        .iter()
        .map(|r| r.end() + 1 - r.start())
        .sum::<i64>();
    println!("Day 5 part two: {all_count}");
}

fn unite(
    mut ranges: Vec<RangeInclusive<i64>>,
    range: &RangeInclusive<i64>,
) -> Vec<RangeInclusive<i64>> {
    let i = ranges
        .binary_search_by_key(range.start(), |r| *r.start())
        .unwrap_or_else(identity);
    ranges.insert(i, range.clone());
    while overlaps_with_next(&ranges, i) {
        merge_overlapping(&mut ranges, i);
    }
    if i > 0 && overlaps_with_next(&ranges, i - 1) {
        merge_overlapping(&mut ranges, i - 1);
    }
    ranges
}

fn overlaps_with_next(ranges: &[RangeInclusive<i64>], i: usize) -> bool {
    i + 1 < ranges.len() && ranges[i].end() + 1 >= *ranges[i + 1].start()
}

fn merge_overlapping(ranges: &mut Vec<RangeInclusive<i64>>, i: usize) {
    let a = *ranges[i].start();
    let b = *ranges[i].end().max(ranges[i + 1].end());
    ranges[i] = a..=b;
    ranges.remove(i + 1);
}
