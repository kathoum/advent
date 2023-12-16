use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mirrors: Vec<Vec<u8>> = BufReader::new(File::open("input/day16.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let mut answer = energy_from_ray(&mirrors, (0, 0, Dir::R));
    println!("Day 16 Part One: {answer}");

    for r in 0..mirrors.len() {
        answer = answer.max(energy_from_ray(&mirrors, (r, 0, Dir::R)));
        answer = answer.max(energy_from_ray(&mirrors, (r, mirrors[r].len() - 1, Dir::L)));
    }
    for c in 0..mirrors[0].len() {
        answer = answer.max(energy_from_ray(&mirrors, (0, c, Dir::D)));
        answer = answer.max(energy_from_ray(&mirrors, (mirrors.len() - 1, 0, Dir::U)));
    }
    println!("Day 16 Part Two: {answer}");
}

#[derive(Copy, Clone)]
enum Dir {
    U = 1,
    D = 2,
    R = 4,
    L = 8,
}

fn energy_from_ray(mirrors: &[Vec<u8>], start: (usize, usize, Dir)) -> usize {
    let mut light = vec![vec![0u8; mirrors[0].len()]; mirrors.len()];
    add_ray(mirrors, &mut light, start.0, start.1, start.2);
    light
        .iter()
        .map(|row| row.iter().filter(|x| **x != 0).count())
        .sum()
}

fn add_ray(mirrors: &[Vec<u8>], light: &mut [Vec<u8>], r: usize, c: usize, dir: Dir) {
    if light[r][c] & (dir as u8) != 0 {
        return;
    }
    light[r][c] |= dir as u8;
    let (d1, d2) = split_ray(dir, mirrors[r][c]);
    if let Some(d2) = d2 {
        add_next(mirrors, light, r, c, d2);
    }
    add_next(mirrors, light, r, c, d1);
}

fn add_next(mirrors: &[Vec<u8>], light: &mut [Vec<u8>], r: usize, c: usize, dir: Dir) {
    if let Some(n) = next(dir, (r, c), mirrors.len(), mirrors[r].len()) {
        add_ray(mirrors, light, n.0, n.1, dir)
    }
}

fn next(dir: Dir, (r, c): (usize, usize), rows: usize, cols: usize) -> Option<(usize, usize)> {
    match dir {
        Dir::U if r > 0 => Some((r - 1, c)),
        Dir::D if r + 1 < rows => Some((r + 1, c)),
        Dir::R if c + 1 < cols => Some((r, c + 1)),
        Dir::L if c > 0 => Some((r, c - 1)),
        _ => None,
    }
}

fn split_ray(dir: Dir, mirror: u8) -> (Dir, Option<Dir>) {
    match mirror {
        b'.' => (dir, None),
        b'|' => match dir {
            Dir::U | Dir::D => (dir, None),
            Dir::R | Dir::L => (Dir::U, Some(Dir::D)),
        },
        b'-' => match dir {
            Dir::R | Dir::L => (dir, None),
            Dir::U | Dir::D => (Dir::R, Some(Dir::L)),
        },
        b'\\' => match dir {
            Dir::U => (Dir::L, None),
            Dir::D => (Dir::R, None),
            Dir::R => (Dir::D, None),
            Dir::L => (Dir::U, None),
        },
        b'/' => match dir {
            Dir::U => (Dir::R, None),
            Dir::D => (Dir::L, None),
            Dir::R => (Dir::U, None),
            Dir::L => (Dir::D, None),
        },
        _ => panic!(),
    }
}
