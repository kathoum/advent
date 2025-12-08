use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for range in BufReader::new(File::open("input/day02.txt").unwrap()).split(b',') {
        let range = String::from_utf8(range.unwrap()).unwrap();
        let (a, b) = range.trim_ascii().split_once('-').unwrap();
        let (a, b) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());
        sum1 += invalid_in_range_1(a, b).sum::<u64>();
        sum2 += invalid_in_range_2(a, b).sum::<u64>();
    }
    println!("Day 2 part one: {sum1}");
    println!("Day 2 part two: {sum2}");
}

fn invalid_in_range_1(a: u64, b: u64) -> impl Iterator<Item = u64> {
    repeated_in_range(a, b, 2)
}

fn invalid_in_range_2(a: u64, b: u64) -> impl Iterator<Item = u64> {
    let mut all: Vec<u64> = (2..=20).flat_map(|n| repeated_in_range(a, b, n)).collect();
    all.sort();
    all.dedup();
    all.into_iter()
}

fn repeated_in_range(a: u64, b: u64, n: u32) -> impl Iterator<Item = u64> {
    assert!(a > 0);
    assert!(a <= b);
    let a_digits = a.ilog10() + 1;
    let b_digits = b.ilog10() + 1;
    (a_digits..=b_digits)
        .filter(move |digits| digits % n == 0)
        .flat_map(move |digits| {
            let k = (0..n).map(|m| 10u64.pow(digits / n * m)).sum::<u64>();
            let a1 = 10u64.pow(digits - 1).max(a).div_ceil(k);
            let b1 = (10u64.saturating_pow(digits) - 1).min(b) / k;
            (a1..=b1).map(move |i| i * k)
        })
}
