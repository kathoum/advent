use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let map = read_map(File::open("input/day06.txt").unwrap());
    let mut grid = to_grid(&map);

    let (mut visited, looping) = walk(&mut grid, map.start);
    assert!(!looping);
    visited.sort();
    visited.dedup();
    println!("Day 6 part one: {}", visited.len());

    let mut count = 0;
    for (row, col) in visited {
        if (row, col) != (map.start.0, map.start.1) {
            set_grid(&mut grid, row, col, Cell::Obstacle);
            let (_, looping) = walk(&mut grid, map.start);
            if looping {
                count += 1;
            }
            set_grid(&mut grid, row, col, Cell::Free);
        }
    }
    println!("Day 6 part two: {count}");
}

struct Map {
    rows: u8,
    cols: u8,
    obstacles: Vec<(u8, u8)>,
    start: (u8, u8, Dir),
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Dir(u8);

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Outside,
    Obstacle,
    Free,
    Visited,
}

fn walk(grid: &mut [Cell], start: (u8, u8, Dir)) -> (Vec<(u8, u8)>, bool) {
    let mut visited = vec![(start.0, start.1)];
    let (mut row, mut col, mut dir) = start;
    let is_loop = loop {
        let next = match dir {
            Dir::UP => (row - 1, col),
            Dir::RIGHT => (row, col + 1),
            Dir::DOWN => (row + 1, col),
            Dir::LEFT => (row, col - 1),
            _ => panic!(),
        };
        let idx = index(next.0, next.1, dir);
        match &mut grid[idx] {
            Cell::Outside => break false,
            Cell::Visited => break true,
            Cell::Obstacle => dir = dir.rotate(),
            cell @ Cell::Free => {
                *cell = Cell::Visited;
                row = next.0;
                col = next.1;
                visited.push(next);
            }
        }
    };
    for &(row, col) in &visited {
        set_grid(grid, row, col, Cell::Free);
    }
    (visited, is_loop)
}

fn read_map(input: impl Read) -> Map {
    let mut rows = 0;
    let mut cols = 0;
    let mut obstacles = vec![];
    let mut start = None;
    for (row, line) in (1..).zip(BufReader::new(input).lines()) {
        let line = line.unwrap().into_bytes();

        assert_eq!(rows + 1, row);
        rows = row;
        if cols == 0 {
            cols = line.len().try_into().unwrap();
        }
        assert_eq!(usize::from(cols), line.len());

        for (col, ch) in (1..).zip(&line) {
            match ch {
                b'.' => (),
                b'#' => obstacles.push((row, col)),
                b'^' => assert!(start.replace((row, col, Dir::UP)).is_none()),
                b'>' => assert!(start.replace((row, col, Dir::RIGHT)).is_none()),
                b'v' => assert!(start.replace((row, col, Dir::DOWN)).is_none()),
                b'<' => assert!(start.replace((row, col, Dir::LEFT)).is_none()),
                _ => panic!(),
            }
        }
    }
    Map {
        rows,
        cols,
        obstacles,
        start: start.unwrap(),
    }
}

fn to_grid(map: &Map) -> Vec<Cell> {
    let mut grid = vec![Cell::Outside; 256 * 256 * 4];
    for row in 1..=map.rows {
        for col in 1..=map.cols {
            set_grid(&mut grid, row, col, Cell::Free);
        }
    }
    for &(row, col) in &map.obstacles {
        set_grid(&mut grid, row, col, Cell::Obstacle);
    }
    grid
}

fn set_grid(grid: &mut [Cell], row: u8, col: u8, cell: Cell) {
    let index = index(row, col, Dir::UP);
    grid[index..index + 4].fill(cell);
}

fn index(row: u8, col: u8, dir: Dir) -> usize {
    ((row as usize) << 10) + ((col as usize) << 2) + (dir.0 as usize)
}

impl Dir {
    const UP: Dir = Dir(0);
    const RIGHT: Dir = Dir(1);
    const DOWN: Dir = Dir(2);
    const LEFT: Dir = Dir(3);

    fn rotate(self) -> Self {
        Self((self.0 + 1) & 3)
    }
}
