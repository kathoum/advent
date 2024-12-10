use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map: Vec<Vec<u8>> = BufReader::new(File::open("input/day10.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    println!("Day 10 part one: {}", sum_for_all_trails(&map, score));
    println!("Day 10 part two: {}", sum_for_all_trails(&map, rating));
}

fn sum_for_all_trails(map: &[Vec<u8>], f: impl Fn(&[Vec<u8>], usize, usize) -> usize) -> usize {
    (0..)
        .zip(map)
        .map(|(r, row)| {
            (0..)
                .zip(row)
                .filter(|(_, tile)| **tile == b'0')
                .map(|(c, _)| f(map, r, c))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn score(map: &[Vec<u8>], r: usize, c: usize) -> usize {
    let mut frontier = vec![(r, c)];
    for i in b'1'..=b'9' {
        let mut new_frontier = vec![];
        for (r, c) in frontier {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let r = r.wrapping_add_signed(dr);
                let c = c.wrapping_add_signed(dc);
                if map.get(r).is_some_and(|row| row.get(c) == Some(&i)) {
                    new_frontier.push((r, c));
                }
            }
        }
        new_frontier.sort();
        new_frontier.dedup();
        frontier = new_frontier;
    }
    frontier.len()
}

fn rating(map: &[Vec<u8>], r: usize, c: usize) -> usize {
    let mut frontier = HashMap::from([((r, c), 1)]);
    for i in b'1'..=b'9' {
        let mut new_frontier = HashMap::new();
        for ((r, c), mult) in frontier {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let r = r.wrapping_add_signed(dr);
                let c = c.wrapping_add_signed(dc);
                if map.get(r).is_some_and(|row| row.get(c) == Some(&i)) {
                    *new_frontier.entry((r, c)).or_default() += mult;
                }
            }
        }
        frontier = new_frontier;
    }
    frontier.values().sum()
}
