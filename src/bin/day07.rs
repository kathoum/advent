use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(File::open("input/day07.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (result, rest) = line.split_once(':').unwrap();
            let result: u64 = result.parse().unwrap();
            let factors: Vec<u64> = rest
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (result, factors)
        })
        .collect::<Vec<_>>();

    let total = input
        .iter()
        .filter(|(x, y)| has_solution(*x, y, false))
        .map(|(x, _)| x)
        .sum::<u64>();
    println!("Day 7 part one: {total}");

    let total = input
        .iter()
        .filter(|(x, y)| has_solution(*x, y, true))
        .map(|(x, _)| x)
        .sum::<u64>();
    println!("Day 7 part two: {total}");
}

fn has_solution(result: u64, factors: &[u64], concat: bool) -> bool {
    match *factors {
        [] => false,
        [x] => result == x,
        [ref x @ .., y] => {
            result >= y && has_solution(result - y, x, concat)
                || result % y == 0 && has_solution(result / y, x, concat)
                || concat && {
                    let base = 10u64.pow(num_digits(y));
                    result % base == y && has_solution(result / base, x, concat)
                }
        }
    }
}

fn num_digits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
