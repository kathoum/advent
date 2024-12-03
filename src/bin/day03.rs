use regex::Regex;
use std::sync::LazyLock;

fn main() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();

    let total: u32 = mul_instructions(&input).map(|(x, y)| x * y).sum();
    println!("Day 3 part one: {total}");

    let total: u32 = input
        .split("do()")
        .flat_map(|str| str.split("don't()").take(1).flat_map(mul_instructions))
        .map(|(x, y)| x * y)
        .sum();
    println!("Day 3 part two: {total}");
}

fn mul_instructions(input: &str) -> impl Iterator<Item = (u32, u32)> + use<'_> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap());
    REGEX.captures_iter(input).map(|capture| {
        let x = capture.get(1).unwrap().as_str().parse().unwrap();
        let y = capture.get(2).unwrap().as_str().parse().unwrap();
        (x, y)
    })
}
