use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let maze: Vec<Vec<i32>> = BufReader::new(File::open("input/day20.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .bytes()
                .map(|b| match b {
                    b'#' => -1,
                    b'E' => 0,
                    b'.' | b'S' => -2,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let maze = walk(maze);

    let minimum_cheat = 100;
    let mut cheats = 0;
    for (r, row) in maze.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell >= 0 {
                for (dr, dc) in [
                    (0, 2),
                    (1, 1),
                    (2, 0),
                    (1, -1),
                    (0, -2),
                    (-1, -1),
                    (-2, 0),
                    (-1, 1),
                ] {
                    let r1 = r.wrapping_add_signed(dr);
                    let c1 = c.wrapping_add_signed(dc);
                    if let Some(&cell1) = maze.get(r1).and_then(|row| row.get(c1)) {
                        if cell1 >= cell + 2 + minimum_cheat {
                            cheats += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Day 20 part one: {cheats}");

    let limit = 20;
    let mut cheats = 0;
    for (r, row) in maze.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell >= 0 {
                let rmin = r.saturating_sub(limit);
                let rmax = maze.len().min(r + limit + 1);
                for (r1, row1) in maze[rmin..rmax].iter().enumerate() {
                    let r1 = r1 + rmin;
                    let dr = r.abs_diff(r1);
                    let climit = limit - dr;
                    let cmin = c.saturating_sub(climit);
                    let cmax = row1.len().min(c + climit + 1);
                    for (c1, &cell1) in row1[cmin..cmax].iter().enumerate() {
                        let c1 = c1 + cmin;
                        let dist = dr + c.abs_diff(c1);
                        if cell1 >= cell + dist as i32 + minimum_cheat {
                            cheats += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Day 20 part two: {cheats}");
}

fn walk(mut maze: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    for (r, row) in maze.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 0 {
                queue.push_back((r, c, 0));
            }
        }
    }
    while let Some((r, c, d)) = queue.pop_front() {
        for (r1, c1) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
            let d1 = &mut maze[r1][c1];
            if *d1 == -2 {
                *d1 = d + 1;
                queue.push_back((r1, c1, d + 1));
            }
        }
    }
    maze
}
