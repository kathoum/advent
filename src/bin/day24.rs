use std::collections::HashSet;

fn main() {
    let input = include_str!("input24.txt");
    let paths = input.lines().map(parse_path).collect::<Vec<_>>();

    println!("Part One");
    let mut flipped = HashSet::new();
    for path in paths.iter() {
        let last = path.iter().fold((0, 0), |pos, dir| {
            step(dir, pos)
        });
        if flipped.contains(&last) {
            flipped.remove(&last);
        } else {
            flipped.insert(last);
        }
    }
    println!("{} black tiles", flipped.len());

    println!("Part Two");
    for _ in 0..100 {
        flipped = life(&flipped);
    }
    println!("{} black tiles after 100 days", flipped.len());
}

enum Dir { E, NE, SE, W, NW, SW }

fn parse_path(input: &str) -> Vec<Dir> {
    enum Prefix { None, N, S }
    let mut pref = Prefix::None;
    let mut result = Vec::new();
    for c in input.chars() {
        pref = match (pref, c) {
            (Prefix::None, 'n') => Prefix::N,
            (Prefix::None, 's') => Prefix::S,
            (Prefix::None, 'e') => { result.push(Dir::E); Prefix::None },
            (Prefix::None, 'w') => { result.push(Dir::W); Prefix::None },
            (Prefix::N, 'n') => panic!("Invalid combination 'nn'"),
            (Prefix::N, 's') => panic!("Invalid combination 'ns'"),
            (Prefix::N, 'e') => { result.push(Dir::NE); Prefix::None },
            (Prefix::N, 'w') => { result.push(Dir::NW); Prefix::None },
            (Prefix::S, 'n') => panic!("Invalid combination 'sn'"),
            (Prefix::S, 's') => panic!("Invalid combination 'ss'"),
            (Prefix::S, 'e') => { result.push(Dir::SE); Prefix::None },
            (Prefix::S, 'w') => { result.push(Dir::SW); Prefix::None },
            (_, _) => panic!("Unexpected char {}", c),
        }
    }
    result
}

fn step(dir: &Dir, from: (i32, i32)) -> (i32, i32) {
    let (row, col) = from;
    match dir {
        Dir::E => (row, col + 1),
        Dir::W => (row, col - 1),
        Dir::NE => (row + 1, col),
        Dir::NW => (row + 1, col - 1),
        Dir::SE => (row - 1, col + 1),
        Dir::SW => (row - 1, col),
    }
}

fn life(board: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let row_min = board.iter().map(|t| t.0).min().unwrap_or_default();
    let row_max = board.iter().map(|t| t.0).max().unwrap_or_default();
    let col_min = board.iter().map(|t| t.1).min().unwrap_or_default();
    let col_max = board.iter().map(|t| t.1).max().unwrap_or_default();
    
    let mut result = HashSet::new();
    for row in row_min-1 ..= row_max+1 {
        for col in col_min-1 ..= col_max+1 {
            let pos = (row, col);
            let adjacent = [Dir::E, Dir::NE, Dir::SE, Dir::W, Dir::NW, Dir::SW].iter()
                .filter(|dir| board.contains(&step(&dir, pos)))
                .count();
            if board.contains(&pos) {
                if adjacent > 0 && adjacent <= 2 {
                    result.insert(pos);
                }
            } else {
                if adjacent == 2 {
                    result.insert(pos);
                }
            }
        }
    }
    result
}
