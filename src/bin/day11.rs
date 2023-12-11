use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;

fn main() {
    let mut galaxies = vec![];
    for (y, line) in BufReader::new(File::open("input/day11.txt").unwrap())
        .lines()
        .enumerate()
    {
        for (x, c) in line.unwrap().bytes().enumerate() {
            if c == b'#' {
                galaxies.push((x, y))
            }
        }
    }
    let ex = empty(galaxies.iter().map(|g| g.0));
    let ey = empty(galaxies.iter().map(|g| g.1));

    let expand = |e: bool| if e { 2 } else { 1 };
    let dx = cum_dist(ex.iter().copied().map(expand));
    let dy = cum_dist(ey.iter().copied().map(expand));

    let mut answer = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            answer += dx[g1.0].abs_diff(dx[g2.0]) + dy[g1.1].abs_diff(dy[g2.1]);
        }
    }
    println!("Day 11 Part One: {answer}");

    let expand = |e: bool| if e { 1_000_000 } else { 1 };
    let dx = cum_dist(ex.iter().copied().map(expand));
    let dy = cum_dist(ey.iter().copied().map(expand));

    let mut answer = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1..] {
            answer += dx[g1.0].abs_diff(dx[g2.0]) + dy[g1.1].abs_diff(dy[g2.1]);
        }
    }
    println!("Day 11 Part Two: {answer}");
}

fn empty(i: impl IntoIterator<Item = usize>) -> Vec<bool> {
    i.into_iter().fold(vec![], |mut v, n| {
        if n >= v.len() {
            v.resize(n + 1, true);
        }
        v[n] = false;
        v
    })
}

fn cum_dist(v: impl IntoIterator<Item = u64>) -> Vec<u64> {
    once(0)
        .chain(v.into_iter().scan(0, |d, n| {
            *d += n;
            Some(*d)
        }))
        .collect()
}
