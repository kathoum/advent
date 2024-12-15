use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut reader = BufReader::new(File::open("input/day15.txt").unwrap())
        .lines()
        .map(Result::unwrap);

    let mut map: Vec<Vec<u8>> = reader
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.into_bytes())
        .collect();
    let mut pos = map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| **t == b'@')
                .map(move |(c, _)| (r, c))
        })
        .next()
        .unwrap();
    map[pos.0][pos.1] = b'.';

    let mut map2: Vec<Vec<u8>> = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|t| match *t {
                    b'.' => [b'.', b'.'],
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let mut pos2 = (pos.0, pos.1 * 2);

    for line in reader {
        for dir in line.bytes() {
            step(&mut map, &mut pos, dir);
            step2(&mut map2, &mut pos2, dir);
        }
    }

    println!("Day 15 part one: {}", gps_score(&map));
    println!("Day 15 part two: {}", gps_score(&map2));
}

fn gps_score(map: &[Vec<u8>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| **t == b'O' || **t == b'[')
                .map(move |(c, _)| r * 100 + c)
        })
        .sum()
}

fn step(map: &mut [Vec<u8>], pos: &mut (usize, usize), dir: u8) {
    let d = match dir {
        b'<' => (0, -1),
        b'>' => (0, 1),
        b'^' => (-1, 0),
        b'v' => (1, 0),
        _ => return,
    };
    for k in 1.. {
        let p = (
            pos.0.checked_add_signed(k * d.0).unwrap(),
            pos.1.checked_add_signed(k * d.1).unwrap(),
        );
        match map[p.0][p.1] {
            b'.' => {
                map[p.0][p.1] = b'O';
                pos.0 = pos.0.wrapping_add_signed(d.0);
                pos.1 = pos.1.wrapping_add_signed(d.1);
                map[pos.0][pos.1] = b'.';
                return;
            }
            b'#' => return,
            b'O' => (),
            _ => panic!(),
        }
    }
}

fn step2(map: &mut [Vec<u8>], pos: &mut (usize, usize), dir: u8) {
    let d = match dir {
        b'<' => (0, -1),
        b'>' => (0, 1),
        b'^' => (-1, 0),
        b'v' => (1, 0),
        _ => return,
    };
    let mut moved = HashSet::new();
    let mut to_move = VecDeque::new();
    to_move.push_back((
        pos.0.wrapping_add_signed(d.0),
        pos.1.wrapping_add_signed(d.1),
    ));
    while let Some((r, c)) = to_move.pop_front() {
        match map[r][c] {
            b'.' => (),
            b'#' => return,
            b'[' => {
                let next = (r.wrapping_add_signed(d.0), c.wrapping_add_signed(d.1));
                if moved.insert((r, c, b'[')) {
                    to_move.push_back(next);
                    if d.0 != 0 && moved.insert((r, c + 1, b']')) {
                        to_move.push_back((next.0, next.1 + 1))
                    }
                }
            }
            b']' => {
                let next = (r.wrapping_add_signed(d.0), c.wrapping_add_signed(d.1));
                if moved.insert((r, c, b']')) {
                    to_move.push_back(next);
                    if d.0 != 0 && moved.insert((r, c - 1, b'[')) {
                        to_move.push_back((next.0, next.1 - 1))
                    }
                }
            }
            _ => panic!(),
        }
    }
    for &(r, c, t) in &moved {
        assert_eq!(map[r][c], t);
        map[r][c] = b'.';
    }
    for (r, c, t) in moved {
        let r1 = r.wrapping_add_signed(d.0);
        let c1 = c.wrapping_add_signed(d.1);
        assert_eq!(map[r1][c1], b'.');
        map[r1][c1] = t;
    }
    pos.0 = pos.0.wrapping_add_signed(d.0);
    pos.1 = pos.1.wrapping_add_signed(d.1);
}
