use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let maze: Vec<Vec<u8>> = BufReader::new(File::open("input/day10.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let rows = maze.len();
    let cols = maze[0].len();
    let maze = Maze(maze, rows, cols);

    let start = 'find: loop {
        for r in 0..rows {
            for c in 0..cols {
                if maze.0[r][c] == b'S' {
                    break 'find (r, c);
                }
            }
        }
        unreachable!()
    };

    let (walls, length, dir) = maze.find_loop(start);
    println!("Day 10 Part One: {}", length / 2);

    let num_inside = zip(maze.0, walls)
        .map(|(ml, wl)| {
            zip(ml, wl)
                .scan(false, |inside, (p, w)| {
                    Some(if w {
                        if p == b'|' || p == b'L' || p == b'J' || (p == b'S' && dir == b'^') {
                            *inside = !*inside;
                        }
                        0
                    } else {
                        *inside as i32
                    })
                })
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("Day 10 Part Two: {num_inside}");
}

struct Maze(Vec<Vec<u8>>, usize, usize);
#[derive(Clone, Copy)]
struct Pos(usize, usize, u8);

impl Maze {
    fn find_loop(&self, start: (usize, usize)) -> (Vec<Vec<bool>>, usize, u8) {
        for d in [b'^', b'v', b'<', b'>'] {
            let mut walls = vec![vec![false; self.2]; self.1];
            let mut pos = Pos(start.0, start.1, d);
            let mut length = 0;
            while let Some(p) = self.step(pos) {
                walls[p.0][p.1] = true;
                length += 1;
                if p.0 == start.0 && p.1 == start.1 {
                    return (walls, length, d);
                }
                pos = p;
            }
        }
        unreachable!()
    }

    fn step(&self, pos: Pos) -> Option<Pos> {
        let Pos(r, c, d) = pos;
        let p = self.0[r][c];
        if can_go(p, d) {
            if let Some(Pos(r1, c1, d1)) = go_dir(pos) {
                if r1 < self.1 && c1 < self.2 {
                    if let Some(d1) = exit(self.0[r1][c1], d1) {
                        return Some(Pos(r1, c1, d1));
                    }
                }
            }
        }
        None
    }
}

fn can_go(pipe: u8, dir: u8) -> bool {
    matches!(
        (pipe, dir),
        (b'S', _)
            | (b'|', b'^')
            | (b'|', b'v')
            | (b'-', b'<')
            | (b'-', b'>')
            | (b'7', b'<')
            | (b'7', b'v')
            | (b'L', b'^')
            | (b'L', b'>')
            | (b'J', b'^')
            | (b'J', b'<')
            | (b'F', b'v')
            | (b'F', b'>')
    )
}

fn exit(pipe: u8, enter: u8) -> Option<u8> {
    match (pipe, enter) {
        (b'|', b'^') => Some(b'^'),
        (b'|', b'v') => Some(b'v'),
        (b'-', b'<') => Some(b'<'),
        (b'-', b'>') => Some(b'>'),
        (b'7', b'>') => Some(b'v'),
        (b'7', b'^') => Some(b'<'),
        (b'L', b'v') => Some(b'>'),
        (b'L', b'<') => Some(b'^'),
        (b'J', b'v') => Some(b'<'),
        (b'J', b'>') => Some(b'^'),
        (b'F', b'^') => Some(b'>'),
        (b'F', b'<') => Some(b'v'),
        (b'S', _) => Some(enter),
        _ => None,
    }
}

fn go_dir(Pos(r, c, d): Pos) -> Option<Pos> {
    match d {
        b'^' => (r > 0).then_some(Pos(r - 1, c, d)),
        b'v' => Some(Pos(r + 1, c, d)),
        b'<' => (c > 0).then_some(Pos(r, c - 1, d)),
        b'>' => Some(Pos(r, c + 1, d)),
        _ => None,
    }
}
