use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cell(u8);

impl Dir {
    pub fn from_char(c: u8) -> Self {
        match c {
            b'^' => Dir::Up,
            b'>' => Dir::Right,
            b'v' => Dir::Down,
            b'<' => Dir::Left,
            _ => panic!(),
        }
    }
    pub fn step(self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::Up => Some((pos.0.checked_sub(1)?, pos.1)),
            Dir::Right => Some((pos.0, pos.1.checked_add(1)?)),
            Dir::Down => Some((pos.0.checked_add(1)?, pos.1)),
            Dir::Left => Some((pos.0, pos.1.checked_sub(1)?)),
        }
    }
    pub fn rotate(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

impl Cell {
    pub fn free() -> Self {
        Cell(0)
    }
    pub fn obstacle() -> Self {
        Cell(1)
    }
    pub fn visited(dir: Dir) -> Self {
        Cell(match dir {
            Dir::Up => 2,
            Dir::Right => 4,
            Dir::Down => 8,
            Dir::Left => 16,
        })
    }
    pub fn from_char(c: u8) -> Self {
        match c {
            b'.' => Cell::free(),
            b'#' => Cell::obstacle(),
            c => Cell::visited(Dir::from_char(c)),
        }
    }
}

fn main() {
    let mut grid = Vec::new();
    let mut pos = None;
    for line in BufReader::new(File::open("input/day06.txt").unwrap()).lines() {
        let line = line.unwrap().into_bytes();
        if let Some(col) = line.iter().position(|ch| !b".#".contains(ch)) {
            let dir = Dir::from_char(line[col]);
            assert!(pos.is_none());
            pos = Some((grid.len(), col, dir));
        }
        grid.push(line.into_iter().map(Cell::from_char).collect::<Vec<Cell>>());
    }
    let pos = pos.unwrap();

    let (visited, _, visited_path) = walk(grid.clone(), pos);
    println!("Day 6 part one: {visited}");

    let mut count = 0;
    for (r, row) in visited_path.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != Cell::obstacle() && cell != Cell::free() && grid[r][c] == Cell::free() {
                let mut grid = grid.clone();
                grid[r][c] = Cell::obstacle();
                if walk(grid, pos).1 {
                    count += 1;
                }
            }
        }
    }
    println!("Day 6 part two: {count}");
}

fn walk(mut grid: Vec<Vec<Cell>>, mut pos: (usize, usize, Dir)) -> (usize, bool, Vec<Vec<Cell>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = 1;
    loop {
        let Some((r, c)) = pos.2.step((pos.0, pos.1)) else {
            return (visited, false, grid);
        };
        if r >= rows || c >= cols {
            return (visited, false, grid);
        }
        if grid[r][c] == Cell::obstacle() {
            pos.2 = pos.2.rotate();
        } else {
            pos.0 = r;
            pos.1 = c;
        }
        let flag = Cell::visited(pos.2).0;
        if grid[pos.0][pos.1].0 & flag != 0 {
            return (visited, true, grid);
        }
        if grid[r][c] == Cell::free() {
            visited += 1;
        }
        grid[pos.0][pos.1].0 |= flag;
    }
}
