use std::io::{BufRead,Cursor};

enum Tile { Open, Tree }
type Map = Vec<Vec<Tile>>;

fn parse_map(reader: impl BufRead) -> Map {
    reader.lines().map(|line| {
        line.unwrap().chars().map(|c| {
            match c {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                _ => panic!("Unexpected character {}", c)
            }
        }).collect()
    }).collect()
}

fn count_trees(map: &Map, slope: (usize, usize)) -> usize {
    let mut current = (0, 0);
    let mut tree_count = 0;
    while current.0 < map.len() {
        let line = &map[current.0];
        match line[current.1 % line.len()] {
            Tile::Tree => tree_count += 1,
            _ => ()
        }
        current.0 += slope.0;
        current.1 += slope.1;
    }
    tree_count
}

fn main() {
    let map = parse_map(Cursor::new(include_str!("input03.txt")));

    println!("Part One");
    let count = count_trees(&map, (1, 3));
    println!("Trees encountered: {}", count);

    println!("Part Two");
    let counts = [
        count_trees(&map, (1, 1)),
        count_trees(&map, (1, 3)),
        count_trees(&map, (1, 5)),
        count_trees(&map, (1, 7)),
        count_trees(&map, (2, 1)),
    ];
    let product = counts.iter().fold(1, |x, y| x * y);
    println!("Product of trees: {}", product);
}
