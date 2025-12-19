use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let k = [7, 6, 7, 5, 7, 7];
    let n = BufReader::new(File::open("input/day12.txt").unwrap())
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let (x, line) = line.split_once('x')?;
            let x = x.parse::<i32>().ok()?;
            let (y, line) = line.split_once(": ")?;
            let y = y.parse::<i32>().ok()?;
            let b = line
                .split_whitespace()
                .zip(k)
                .map(|(n, k)| n.parse::<i32>().unwrap() * k)
                .sum::<i32>();
            (x * y > b).then_some(())
        })
        .count();

    println!("Day 12 part one: {n}");
}
