use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;

fn main() {
    let reader = BufReader::new(File::open("input/day03.txt").unwrap());
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();

    let n: u32 = lines.iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a,b)| common_item(a, b))
        .map(priority)
        .sum();

    let m: u32 = lines
        .chunks(3)
        .map(badge)
        .map(priority)
        .sum();

    println!("The sum of the priorities of misplaced items is {n}");
    println!("The sum of the priorities of badges is {m}");
}

fn chars_of(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn common_item(a: &str, b: &str) -> char {
    let x = chars_of(a);
    let y = chars_of(b);
    *x.intersection(&y).next().unwrap()
}

fn badge(elves: &[String]) -> char {
    let mut iter = elves.iter();
    let mut set = chars_of(iter.next().unwrap());
    for line in iter {
        set = &set & &chars_of(line);
    }
    *set.iter().next().unwrap()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
        _ => panic!("Unexpected item '{c}'")
    }
}
