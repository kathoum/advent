use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let answer = BufReader::new(File::open("input/day01.txt").unwrap())
        .lines()
        .map(|line| key(&line.unwrap()))
        .sum::<u32>();
    println!("Part One: {answer}");

    let answer = BufReader::new(File::open("input/day01.txt").unwrap())
        .lines()
        .map(|line| key2(&line.unwrap()))
        .sum::<u32>();
    println!("Part Two: {answer}");
}

fn key(line: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    for char in line.chars() {
        if let Some(n) = char.to_digit(10) {
            if first.is_none() {
                first = Some(n);
            }
            last = Some(n);
        }
    }
    first.unwrap() * 10 + last.unwrap()
}

fn key2(line: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first = None;
    let mut last = None;
    for (pos, char) in line.char_indices() {
        if let Some(n) = if let Some(digit) = char.to_digit(10) {
            Some(digit)
        } else if let Some((number, _)) = numbers
            .iter()
            .enumerate()
            .find(|(_, name)| line[pos..].starts_with(**name))
        {
            Some(number as u32 + 1)
        } else {
            None
        } {
            if first.is_none() {
                first = Some(n);
            }
            last = Some(n);
        }
    }
    first.unwrap() * 10 + last.unwrap()
}
