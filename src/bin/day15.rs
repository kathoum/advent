use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let line = BufReader::new(File::open("input/day15.txt").unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .into_bytes();

    let answer: usize = line.split(|c| *c == b',').map(hash).sum();
    println!("Day 15 Part One: {answer}");

    const EMPTY: Vec<(&[u8], u8)> = Vec::new();
    let mut hashmap = [EMPTY; 256];
    for step in line.split(|c| *c == b',') {
        match *step {
            [ref label @ .., b'-'] => {
                let h = hash(label);
                hashmap[h].retain(|lens| lens.0 != label);
            }
            [ref label @ .., b'=', f @ b'1'..=b'9'] => {
                let h = hash(label);
                let focal = f - b'0';
                match hashmap[h].iter_mut().find(|lens| lens.0 == label) {
                    Some(lens) => lens.1 = focal,
                    None => hashmap[h].push((label, focal)),
                };
            }
            _ => panic!("Unexpected step: {:?}", std::str::from_utf8(step).unwrap()),
        }
    }

    let answer: usize = hashmap
        .iter()
        .enumerate()
        .flat_map(|(box_idx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(slot_idx, (_, focal))| {
                    (box_idx + 1) * (slot_idx + 1) * (*focal as usize)
                })
        })
        .sum();
    println!("Day 15 Part Two: {answer}");
}

fn hash(s: &[u8]) -> usize {
    s.iter()
        .fold(0, |h, c| c.wrapping_add(h).wrapping_mul(17))
        .into()
}
