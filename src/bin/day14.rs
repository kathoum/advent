use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut platform: Vec<Vec<u8>> = BufReader::new(File::open("input/day14.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    tilt_up(&mut platform);
    println!("Day 14 Part One: {}", load(&platform));

    let mut met = HashMap::new();
    let target = 1_000_000_000;
    for i in 1..=target {
        cycle(&mut platform);
        if let Some(prev) = met.insert(platform.clone(), i) {
            let period = i - prev;
            let j = (target - i) % period;
            for _ in 0..j {
                cycle(&mut platform);
            }
            break;
        }
    }
    println!("Day 14 Part Two: {}", load(&platform));
}

fn load(platform: &[Vec<u8>]) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|c| **c == b'O').count() * (i + 1))
        .sum()
}

fn tilt_up(platform: &mut [Vec<u8>]) {
    for r in 0..platform.len() {
        for c in 0..platform[r].len() {
            if platform[r][c] == b'O' {
                platform[r][c] = b'.';
                let mut r1 = r;
                while r1 > 0 && platform[r1 - 1][c] == b'.' {
                    r1 -= 1;
                }
                platform[r1][c] = b'O';
            }
        }
    }
}

fn tilt_left(platform: &mut [Vec<u8>]) {
    for row in platform {
        for c in 0..row.len() {
            if row[c] == b'O' {
                row[c] = b'.';
                let mut c1 = c;
                while c1 > 0 && row[c1 - 1] == b'.' {
                    c1 -= 1;
                }
                row[c1] = b'O';
            }
        }
    }
}

fn tilt_down(platform: &mut [Vec<u8>]) {
    for r in (0..platform.len()).rev() {
        for c in 0..platform[r].len() {
            if platform[r][c] == b'O' {
                platform[r][c] = b'.';
                let mut r1 = r;
                while r1 + 1 < platform.len() && platform[r1 + 1][c] == b'.' {
                    r1 += 1;
                }
                platform[r1][c] = b'O';
            }
        }
    }
}

fn tilt_right(platform: &mut [Vec<u8>]) {
    for row in platform {
        for c in (0..row.len()).rev() {
            if row[c] == b'O' {
                row[c] = b'.';
                let mut c1 = c;
                while c1 + 1 < row.len() && row[c1 + 1] == b'.' {
                    c1 += 1;
                }
                row[c1] = b'O';
            }
        }
    }
}

fn cycle(platform: &mut [Vec<u8>]) {
    tilt_up(platform);
    tilt_left(platform);
    tilt_down(platform);
    tilt_right(platform);
}
