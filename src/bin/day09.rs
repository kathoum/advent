use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let answers = BufReader::new(File::open("input/day09.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|v| predict(&v))
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("Day 9 Part One: {}", answers.1);
    println!("Day 9 Part One: {}", answers.0);
}

fn predict(v: &[i32]) -> (i32, i32) {
    if v.iter().all(|n| *n == 0) {
        (0, 0)
    } else {
        let d: Vec<i32> = v.windows(2).map(|w| w[1] - w[0]).collect();
        let (a, b) = predict(&d);
        (v.first().unwrap() - a, v.last().unwrap() + b)
    }
}
