use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("input/day13.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut total1 = 0;
    let mut total2 = 0;
    while let Some(linea) = lines.next() {
        let lineb = lines.next().unwrap();
        let prize = lines.next().unwrap();
        assert!(lines.next().unwrap_or_default().is_empty());

        let (ax, ay) = linea
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (bx, by) = lineb
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (x, y) = prize
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();

        let input = [ax, ay, bx, by, x, y];
        let mut input = input.map(|n| n.parse().unwrap());

        if let Some((a, b)) = solution(input) {
            total1 += 3 * a + b;
        }

        input[4] += 10_000_000_000_000;
        input[5] += 10_000_000_000_000;
        if let Some((a, b)) = solution(input) {
            total2 += 3 * a + b;
        }
    }

    println!("Day 13 part one: {total1}");
    println!("Day 13 part two: {total2}");
}

fn solution([ax, ay, bx, by, x, y]: [i64; 6]) -> Option<(i64, i64)> {
    let a = (y * bx - x * by) / (ay * bx - ax * by);
    let b = (y * ax - x * ay) / (by * ax - bx * ay);
    (x == a * ax + b * bx && y == a * ay + b * by).then_some((a, b))
}
