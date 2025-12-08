use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let sum = BufReader::new(File::open("input/day03.txt").unwrap())
        .lines()
        .map(|line| largest_jolt(line.unwrap().as_bytes()))
        .sum::<u32>();
    println!("Day 3 part one: {sum}");

    let sum = BufReader::new(File::open("input/day03.txt").unwrap())
        .lines()
        .map(|line| largest_jolt_n(line.unwrap().as_bytes(), 12))
        .sum::<u64>();
    println!("Day 3 part two: {sum}");
}

fn largest_jolt(bank: &[u8]) -> u32 {
    let i1 = argmax(&bank[..bank.len() - 1]);
    let i2 = i1 + 1 + argmax(&bank[i1 + 1..]);
    let n1 = (bank[i1] as char).to_digit(10).unwrap();
    let n2 = (bank[i2] as char).to_digit(10).unwrap();
    n1 * 10 + n2
}

fn largest_jolt_n(bank: &[u8], n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let i = argmax(&bank[..bank.len() - (n - 1) as usize]);
    let a = (bank[i] as char).to_digit(10).unwrap();
    let a = a as u64 * 10u64.pow(n - 1);
    a + largest_jolt_n(&bank[i + 1..], n - 1)
}

fn argmax<Iter>(iter: Iter) -> usize
where
    Iter: IntoIterator,
    Iter::Item: Ord,
{
    let f = iter
        .into_iter()
        .enumerate()
        .fold(None, |f, (i, y)| match f {
            None => Some((i, y)),
            Some((_, x)) if x < y => Some((i, y)),
            Some(f) => Some(f),
        });
    f.unwrap().0
}
