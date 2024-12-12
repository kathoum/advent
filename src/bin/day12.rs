use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let plot: Vec<Vec<u8>> = BufReader::new(File::open("input/day12.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let rows = plot.len() as i32;
    let cols = plot[0].len() as i32;

    let mut total1 = 0;
    let mut total2 = 0;
    let mut region = vec![vec![0; cols as _]; rows as _];
    for r in 0..rows {
        for c in 0..cols {
            if get(&region, r, c) == Some(0) {
                let (area, perimeter, num_sides) = fill_region(&plot, &mut region, r, c);
                total1 += area * perimeter;
                total2 += area * num_sides;
            }
        }
    }
    println!("Day 12 part one: {total1}");
    println!("Day 12 part two: {total2}");
}

fn fill_region(plot: &[Vec<u8>], region: &mut [Vec<i32>], r0: i32, c0: i32) -> (u32, u32, u32) {
    let k = get(plot, r0, c0).unwrap();
    let mut area = 0;
    let mut perimeter = 0;
    let mut sides = vec![];
    let mut boundary = vec![(r0, c0)];
    region[r0 as usize][c0 as usize] = 1;
    while let Some((r, c)) = boundary.pop() {
        area += 1;
        for (r1, c1, d1) in [
            (r - 1, c, b'u'),
            (r + 1, c, b'd'),
            (r, c - 1, b'l'),
            (r, c + 1, b'r'),
        ] {
            if get(plot, r1, c1) == Some(k) {
                if get(region, r1, c1) == Some(0) {
                    boundary.push((r1, c1));
                    region[r1 as usize][c1 as usize] = 1;
                }
            } else {
                perimeter += 1;
                sides.push(if d1 == b'u' || d1 == b'd' {
                    (d1, r1, c1)
                } else {
                    (d1, c1, r1)
                });
            }
        }
    }
    sides.sort();
    let mut num_sides = 1;
    for w in sides.windows(2) {
        let [(d1, r1, c1), (d2, r2, c2)] = *w else {
            panic!()
        };
        if !(d1 == d2 && r1 == r2 && c1 + 1 == c2) {
            num_sides += 1;
        }
    }
    (area, perimeter, num_sides)
}

fn get<T: Copy>(plot: &[Vec<T>], r: i32, c: i32) -> Option<T> {
    plot.get(r as usize)
        .and_then(|row| row.get(c as usize).copied())
}
