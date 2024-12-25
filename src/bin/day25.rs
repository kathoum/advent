use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("input/day25.txt").unwrap())
        .lines()
        .map(Result::unwrap);
    let keys_and_locks: Vec<u32> = std::iter::from_fn(|| parse(&mut lines)).collect();
    let count = keys_and_locks
        .iter()
        .map(|&key| {
            keys_and_locks
                .iter()
                .filter(|&&lock| key < lock && (key + lock) & 0x88888a == 0)
                .count()
        })
        .sum::<usize>();
    println!("Day 25 part one: {count}");
}

fn parse(iter: &mut impl Iterator<Item = String>) -> Option<u32> {
    let mut n = 0;
    loop {
        let line = iter.next().unwrap_or_default();
        if line.is_empty() {
            return (n != 0).then_some(n);
        }
        assert_eq!(line.len(), 5);
        for (i, c) in (1..).zip(line.bytes()) {
            if c == b'#' {
                n += 1 << (4 * i);
            } else {
                assert_eq!(c, b'.');
            }
        }
        if n & 5 == 0 {
            n += if n == 0 { 1 } else { 4 };
        }
    }
}
