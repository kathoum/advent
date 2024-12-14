use std::fs::File;
use std::io::{BufRead, BufReader};

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    let robots: Vec<Robot> = BufReader::new(File::open("input/day14.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.strip_prefix("p=").unwrap();
            let (x, line) = line.split_once(',').unwrap();
            let (y, line) = line.split_once(" v=").unwrap();
            let (vx, vy) = line.split_once(',').unwrap();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect();

    let width = 101;
    let height = 103;
    let positions = positions_after(&robots, width, height, 100);

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for (x, y) in positions {
        if x < width / 2 && y < height / 2 {
            q1 += 1;
        }
        if x < width / 2 && y > height / 2 {
            q2 += 1;
        }
        if x > width / 2 && y < height / 2 {
            q3 += 1;
        }
        if x > width / 2 && y > height / 2 {
            q4 += 1;
        }
    }
    println!("Day 14 part one: {}", q1 * q2 * q3 * q4);

    let mut map = vec![vec![false; (width + 2) as _]; (height + 2) as _];
    for t in 0.. {
        for m in &mut map {
            m.fill(false);
        }
        let positions = positions_after(&robots, width, height, t);
        for &(x, y) in &positions {
            map[(y + 1) as usize][(x + 1) as usize] = true;
        }
        let num_isolated = positions
            .iter()
            .filter(|&&(x, y)| {
                [(x, y + 1), (x + 1, y), (x + 2, y + 1), (x + 1, y + 2)]
                    .iter()
                    .all(|&(x1, y1)| !map[y1 as usize][x1 as usize])
            })
            .count();

        if num_isolated < robots.len() / 2 {
            println!("Day 14 part two: {t}");

            #[cfg(debug_assertions)]
            for m in map {
                for n in m {
                    print!("{}", if n { '#' } else { '.' });
                }
                println!();
            }

            break;
        }
    }
}

fn positions_after(robots: &[Robot], width: i32, height: i32, steps: i32) -> Vec<(i32, i32)> {
    robots
        .iter()
        .map(|&Robot { x, y, vx, vy }| {
            let x1 = (x + steps * vx).rem_euclid(width);
            let y1 = (y + steps * vy).rem_euclid(height);
            (x1, y1)
        })
        .collect()
}
