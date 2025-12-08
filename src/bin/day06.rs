use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<Vec<String>> = BufReader::new(File::open("input/day06.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(String::from)
                .collect()
        })
        .collect();

    let operands = input.len() - 1;
    let total = input
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let iter = input[..operands]
                .iter()
                .map(|l| l[i].parse::<i64>().unwrap());
            match op.as_str() {
                "+" => iter.sum::<i64>(),
                "*" => iter.product::<i64>(),
                _ => panic!(),
            }
        })
        .sum::<i64>();

    println!("Day 6 part one: {total}");

    let input: Vec<Vec<u8>> = BufReader::new(File::open("input/day06.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let transposed: Vec<String> = (0..input[0].len())
        .map(|c| input.iter().map(|l| l[c] as char).collect())
        .collect();

    let mut total = 0;
    let mut partial = 0;
    let mut operation = '?';
    for line in transposed.into_iter().chain(Some(String::new())) {
        if let Some(n) = line.strip_suffix('+') {
            assert!(operation == '?');
            operation = '+';
            assert!(partial == 0);
            partial = n.trim_ascii().parse().unwrap();
        } else if let Some(n) = line.strip_suffix('*') {
            assert!(operation == '?');
            operation = '*';
            assert!(partial == 0);
            partial = n.trim_ascii().parse().unwrap();
        } else if let Ok(n) = line.trim_ascii().parse::<i64>() {
            match operation {
                '+' => partial += n,
                '*' => partial *= n,
                _ => panic!(),
            }
        } else {
            assert!(operation != '?');
            operation = '?';
            assert!(partial != 0);
            total += partial;
            partial = 0;
        }
    }

    println!("Day 6 part two: {total}");
}
