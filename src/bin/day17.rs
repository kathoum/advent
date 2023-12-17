use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let grid: Vec<Vec<u32>> = BufReader::new(File::open("input/day17.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    println!("Day 17 Part One: {}", find(&grid, 1, 3));
    println!("Day 17 Part Two: {}", find(&grid, 4, 10));
}

fn find(grid: &[Vec<u32>], min: usize, max: usize) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited: Vec<Vec<[u32; 2]>> = vec![vec![[u32::MAX; 2]; cols]; rows];
    let mut frontier: BinaryHeap<(Reverse<u32>, usize, usize, bool)> = BinaryHeap::new();
    frontier.push((Reverse(0), 0, 0, true));
    frontier.push((Reverse(0), 0, 0, false));
    while let Some((Reverse(cost), r, c, vertical)) = frontier.pop() {
        let finish = visited[rows - 1][cols - 1].iter().copied().min().unwrap();
        if finish <= cost {
            return finish;
        }
        if visited[r][c][vertical as usize] > cost {
            visited[r][c][vertical as usize] = cost;
            match vertical {
                true => {
                    if c >= min {
                        let mut cost = cost + (1..=min).map(|x| grid[r][c - x]).sum::<u32>();
                        let mut c = c - min;
                        frontier.push((Reverse(cost), r, c, false));
                        for _ in min..max {
                            if c > 0 {
                                c -= 1;
                                cost += grid[r][c];
                                frontier.push((Reverse(cost), r, c, false));
                            }
                        }
                    }
                    if c + min < cols {
                        let mut cost = cost + (1..=min).map(|x| grid[r][c + x]).sum::<u32>();
                        let mut c = c + min;
                        frontier.push((Reverse(cost), r, c, false));
                        for _ in min..max {
                            if c + 1 < cols {
                                c += 1;
                                cost += grid[r][c];
                                frontier.push((Reverse(cost), r, c, false));
                            }
                        }
                    }
                }
                false => {
                    if r >= min {
                        let mut cost = cost + (1..=min).map(|x| grid[r - x][c]).sum::<u32>();
                        let mut r = r - min;
                        frontier.push((Reverse(cost), r, c, true));
                        for _ in min..max {
                            if r > 0 {
                                r -= 1;
                                cost += grid[r][c];
                                frontier.push((Reverse(cost), r, c, true));
                            }
                        }
                    }
                    if r + min < rows {
                        let mut cost = cost + (1..=min).map(|x| grid[r + x][c]).sum::<u32>();
                        let mut r = r + min;
                        frontier.push((Reverse(cost), r, c, true));
                        for _ in min..max {
                            if r + 1 < rows {
                                r += 1;
                                cost += grid[r][c];
                                frontier.push((Reverse(cost), r, c, true));
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("Exit is not reachable")
}
