use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let floor: Vec<Vec<u8>> = BufReader::new(File::open("input/day04.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let mut n = 0;
    for x in 0..floor.len() {
        for y in 0..floor[0].len() {
            if accessible(&floor, (x as i32, y as i32)) {
                n += 1;
            }
        }
    }

    println!("Day 4 part one: {n}");

    let mut m = 0;
    let mut floor = floor;
    let (mut x, mut y) = (0, 0);
    while x < floor.len() as i32 {
        if accessible(&floor, (x, y)) {
            m += 1;
            floor[x as usize][y as usize] = b'x';
            x = x.max(1) - 1;
            y = y.max(1) - 1;
        } else {
            y += 1;
            if y >= floor[0].len() as i32 {
                y = 0;
                x += 1;
            }
        }
    }

    println!("Day 4 part two: {m}");
}

fn accessible(floor: &[Vec<u8>], (x, y): (i32, i32)) -> bool {
    if floor[x as usize][y as usize] != b'@' {
        return false;
    }
    let mut n = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if (dx, dy) != (0, 0) {
                let xx = x + dx;
                let yy = y + dy;
                if xx >= 0
                    && yy >= 0
                    && (xx as usize) < floor.len()
                    && (yy as usize) < floor[0].len()
                    && floor[xx as usize][yy as usize] == b'@'
                {
                    n += 1;
                }
            }
        }
    }
    n < 4
}
