use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut seeds = Vec::<u64>::new();
    let mut tables = Vec::<Table>::new();
    for line in BufReader::new(File::open("input/day05.txt").unwrap()).lines() {
        let line = line.unwrap();
        if let Some(str) = line.strip_prefix("seeds:") {
            seeds = str.split_whitespace().map(|s| s.parse().unwrap()).collect();
        } else if line.ends_with("map:") {
            tables.push(Table(Vec::new()));
        } else if !line.is_empty() {
            let table = tables.last_mut().unwrap();
            let mut nums = line.split_whitespace().map(|s| s.parse().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            let c = nums.next().unwrap();
            table.0.push((a, b, c));
        }
    }

    let answer = seeds
        .iter()
        .map(|seed| tables.iter().fold(*seed, |seed, table| table.map(seed)))
        .min()
        .unwrap();
    println!("Day 5 Part One: {answer}");

    let answer = seeds
        .chunks(2)
        .map(|s| {
            let &[mut start, mut len] = s else { panic!() };
            let mut m = None;
            loop {
                let (end, l) = tables.iter().fold((start, len), |(start, len), table| {
                    table.map_range(start, len)
                });
                let min = m.unwrap_or(end).min(end);
                if l == len {
                    break min;
                }
                m = Some(min);
                start += l;
                len -= l;
            }
        })
        .min()
        .unwrap();
    println!("Day 5 Part Two: {answer}");
}

struct Table(Vec<(u64, u64, u64)>);

impl Table {
    fn map(&self, x: u64) -> u64 {
        for &(dest, start, length) in &self.0 {
            if x >= start && x - start < length {
                return x - start + dest;
            }
        }
        x
    }

    fn map_range(&self, x: u64, len: u64) -> (u64, u64) {
        let mut y = len;
        for &(dest, start, length) in &self.0 {
            if x >= start {
                let d = x - start;
                if d < length {
                    return (dest + d, len.min(length - d));
                }
            } else {
                y = y.min(start - x);
            }
        }
        (x, y)
    }
}
