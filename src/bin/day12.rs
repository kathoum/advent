use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day12.txt").unwrap());
    let mut grid: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();

    let mut start = (0,0);
    let mut end = (0,0);
    for (i,r) in grid.iter_mut().enumerate() {
        for (j,p) in r.iter_mut().enumerate() {
            if *p == b'S' { *p = b'a'; start = (i,j); }
            if *p == b'E' { *p = b'z'; end = (i,j); }
        }
    }
    let (grid, start, end) = (grid, start, end);

    let d = distance_to_end(&grid, [start].into(), end);
    println!("Can reach the location in {d} steps");

    let start_points = grid
        .iter().enumerate()
        .flat_map(|(i,r)| r
            .iter().enumerate()
            .filter_map(move |(j,p)|
                if *p == b'a' { Some((i,j)) } else { None }
            )
        ).collect();
    let d = distance_to_end(&grid, start_points, end);

    println!("Can reach the location from the nearest start in {d} steps");
}

fn distance_to_end(grid: &[Vec<u8>], mut start: Vec<(usize,usize)>, end: (usize,usize)) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut steps = vec![vec![-1; cols]; rows];
    for (i,j) in &start {
        steps[*i][*j] = 0;
    }
    for round in 1.. {
        let mut next = Vec::new();
        for pos in start {
            for p in adjacent(pos, rows, cols) {
                if steps[p.0][p.1] < 0 && grid[p.0][p.1] <= grid[pos.0][pos.1] + 1 {
                    steps[p.0][p.1] = round;
                    if p == end {
                        return round;
                    }
                    next.push(p);
                }
            }
        }
        start = next;
    }
    unreachable!()
}

fn adjacent((i,j): (usize,usize), rows: usize, cols: usize) -> Vec<(usize,usize)> {
    let mut v = Vec::new();
    if i > 0 { v.push((i-1, j)); }
    if j > 0 { v.push((i, j-1)); }
    if i+1 < rows { v.push((i+1, j)); }
    if j+1 < cols { v.push((i, j+1)); }
    v
}
