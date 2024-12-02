use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<Vec<i32>> = BufReader::new(File::open("input/day02.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect();

    let count = input
        .iter()
        .filter(|report| is_safe(report.iter().copied()))
        .count();
    println!("Day 2 part one: {count}");

    let count = input.iter().filter(|report| is_almost_safe(report)).count();
    println!("Day 2 part two: {count}");
}

fn is_safe(mut iter: impl Iterator<Item = i32>) -> bool {
    let Some(mut x) = iter.next() else {
        return true;
    };
    let Some(y) = iter.next() else { return true };
    if x == y || (x - y).abs() > 3 {
        return false;
    }
    let incr = y > x;
    x = y;
    for y in iter {
        if x == y || (x - y).abs() > 3 || (y > x) != incr {
            return false;
        }
        x = y;
    }
    true
}

fn is_almost_safe(data: &[i32]) -> bool {
    for i in 0..data.len() {
        let (left, right) = data.split_at(i);
        if is_safe(left.iter().chain(&right[1..]).copied()) {
            return true;
        }
    }
    false
}
