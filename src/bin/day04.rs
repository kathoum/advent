use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<Vec<u8>> = BufReader::new(File::open("input/day04.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if (dx, dy) != (0, 0) {
                for (y, line) in (0..).zip(&input) {
                    for (x, _) in (0..).zip(line) {
                        if match_at_position(&input, (x, y), (dx, dy), b"XMAS").is_some() {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Day 4 part one: {count}");

    let mut count = 0;
    for w1 in [b"MAS", b"SAM"] {
        for w2 in [b"MAS", b"SAM"] {
            for (y, line) in (0..).zip(&input) {
                for (x, _) in (0..).zip(line) {
                    if match_at_position(&input, (x, y), (1, 1), w1).is_some()
                        && match_at_position(&input, (x + 2, y), (-1, 1), w2).is_some()
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Day 4 part two: {count}");
}

fn match_at_position(
    input: &[Vec<u8>],
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    word: &[u8],
) -> Option<()> {
    for (i, c) in (0..).zip(word) {
        let u = usize::try_from(x + i * dx).ok()?;
        let v = usize::try_from(y + i * dy).ok()?;
        if input.get(v)?.get(u)? != c {
            return None;
        }
    }
    Some(())
}
