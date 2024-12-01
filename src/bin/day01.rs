use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in BufReader::new(File::open("input/day01.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();
    let total = zip(&left, &right)
        .map(|(&x, &y)| x.abs_diff(y))
        .sum::<u32>();
    println!("Day 1 part one: {total}");

    let similarity = left
        .iter()
        .map(|&x| {
            let first = right.partition_point(|&y| y < x);
            let count = right[first..].iter().take_while(|&&y| y == x).count();
            x * count as i32
        })
        .sum::<i32>();
    println!("Day 1 part two: {similarity}");
}
