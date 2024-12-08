use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rows = 0;
    let mut cols = 0;
    let mut antennas = HashMap::<u8, Vec<(i32, i32)>>::new();
    for (r, line) in (0..).zip(BufReader::new(File::open("input/day08.txt").unwrap()).lines()) {
        rows = rows.max(r + 1);
        for (c, char) in (0..).zip(line.unwrap().bytes()) {
            cols = cols.max(c + 1);
            if char != b'.' {
                antennas.entry(char).or_default().push((r, c));
            }
        }
    }

    let mut antinodes1 = HashSet::new();
    let mut antinodes2 = HashSet::new();
    for pos in antennas.values() {
        for (r1, c1) in pos {
            for (r2, c2) in pos {
                if (r1, c1) != (r2, c2) {
                    let r = 2 * r1 - r2;
                    let c = 2 * c1 - c2;
                    if (0..rows).contains(&r) && (0..cols).contains(&c) {
                        antinodes1.insert((r, c));
                    }
                    if (r1 - r2) % 3 == 0 && (c1 - c2) % 3 == 0 {
                        let r = r2 + (r1 - r2) / 3;
                        let c = c2 + (c1 - c2) / 3;
                        if (0..rows).contains(&r) && (0..cols).contains(&c) {
                            antinodes1.insert((r, c));
                        }
                    }

                    for k in 1.. {
                        let r = r1 + k * (r2 - r1);
                        let c = c1 + k * (c2 - c1);
                        if (0..rows).contains(&r) && (0..cols).contains(&c) {
                            antinodes2.insert((r, c));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("Day 8 part one: {}", antinodes1.len());
    println!("Day 8 part two: {}", antinodes2.len());
}
