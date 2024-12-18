use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut space = vec![vec![-1; 71]; 71];
    let pixels: Vec<(usize, usize)> = BufReader::new(File::open("input/day18.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (x, y)
        })
        .collect();

    for &(x, y) in &pixels[..1024] {
        space[x][y] = -2;
    }
    println!("Day 18 part one: {}", steps_to_exit(&mut space).unwrap());

    let indices: Vec<usize> = (0..pixels.len()).collect();
    let idx = indices.partition_point(|&idx| is_exit_reachable(&mut space, &pixels[..=idx]));
    let (x, y) = pixels[idx];
    println!("Day 18 part two: {x},{y}");
}

fn steps_to_exit(space: &mut [Vec<i32>]) -> Option<i32> {
    let mut pos = VecDeque::from([(0, 0, 0)]);
    while let Some((x, y, step)) = pos.pop_front() {
        if space[x][y] == -1 {
            space[x][y] = step;
            if x > 0 {
                pos.push_back((x - 1, y, step + 1));
            }
            if y > 0 {
                pos.push_back((x, y - 1, step + 1));
            }
            if x < 70 {
                pos.push_back((x + 1, y, step + 1));
            }
            if y < 70 {
                pos.push_back((x, y + 1, step + 1));
            }
        }
    }
    let n = space[70][70];
    (n >= 0).then_some(n)
}

fn is_exit_reachable(space: &mut [Vec<i32>], pixels: &[(usize, usize)]) -> bool {
    for line in space.iter_mut() {
        line.fill(-1);
    }
    for &(x, y) in pixels {
        space[x][y] = -2;
    }
    steps_to_exit(space).is_some()
}
