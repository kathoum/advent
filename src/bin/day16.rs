use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let maze: Vec<Vec<u8>> = BufReader::new(File::open("input/day16.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    let start = find(&maze, b'S');
    let finish = find(&maze, b'E');

    let (distance, touched) = walk(
        &maze,
        Pos {
            r: start.0,
            c: start.1,
            dir: Dir::Right,
        },
        finish,
    );
    println!("Day 16 part one: {distance}");
    println!("Day 16 part two: {touched}");
}

fn find(maze: &[Vec<u8>], val: u8) -> (usize, usize) {
    maze.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, x)| **x == val)
                .map(move |(c, _)| (r, c))
        })
        .next()
        .unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    r: usize,
    c: usize,
    dir: Dir,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    cost: i32,
    pos: Pos,
    prev: Option<Pos>,
}

fn walk(maze: &[Vec<u8>], start: Pos, finish: (usize, usize)) -> (i32, usize) {
    let mut reached: HashMap<Pos, (i32, Vec<Pos>)> = HashMap::new();
    let mut frontier = BinaryHeap::new();
    frontier.push(Cell {
        cost: 0,
        pos: start,
        prev: None,
    });
    while let Some(Cell { cost, pos, prev }) = frontier.pop() {
        if let Some(entry) = reached.get_mut(&pos) {
            assert!(entry.0 >= cost);
            if entry.0 == cost {
                entry.1.extend(prev);
            }
        } else {
            reached.insert(pos, (cost, prev.into_iter().collect()));
            let (r1, c1, dir1, dir2) = match pos.dir {
                Dir::Up => (pos.r - 1, pos.c, Dir::Right, Dir::Left),
                Dir::Right => (pos.r, pos.c + 1, Dir::Up, Dir::Down),
                Dir::Down => (pos.r + 1, pos.c, Dir::Right, Dir::Left),
                Dir::Left => (pos.r, pos.c - 1, Dir::Up, Dir::Down),
            };
            if maze[r1][c1] != b'#' {
                frontier.push(Cell {
                    cost: cost - 1,
                    pos: Pos {
                        r: r1,
                        c: c1,
                        ..pos
                    },
                    prev: Some(pos),
                });
            }
            frontier.push(Cell {
                cost: cost - 1000,
                pos: Pos { dir: dir1, ..pos },
                prev: Some(pos),
            });
            frontier.push(Cell {
                cost: cost - 1000,
                pos: Pos { dir: dir2, ..pos },
                prev: Some(pos),
            });
        }
    }
    let heads = [Dir::Up, Dir::Right, Dir::Down, Dir::Left].map(|dir| Pos {
        r: finish.0,
        c: finish.1,
        dir,
    });
    let cost = heads.iter().map(|pos| reached[pos].0).max().unwrap();
    let mut touched = HashSet::new();
    for pos in heads {
        if reached[&pos].0 == cost {
            unwind(&mut touched, &reached, pos);
        }
    }
    (-cost, touched.len())
}

fn unwind(
    touched: &mut HashSet<(usize, usize)>,
    reached: &HashMap<Pos, (i32, Vec<Pos>)>,
    from: Pos,
) {
    touched.insert((from.r, from.c));
    for prev in &reached[&from].1 {
        unwind(touched, reached, *prev);
    }
}
