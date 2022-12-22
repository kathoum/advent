use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day22.txt").unwrap());
    let _reader = std::io::Cursor::new(
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
");
    let (map, moves) = read_map(reader);

    let mut pos = Position(
        0,
        map[0].iter().enumerate().find(|(_,t)| **t == Tile::Open).unwrap().0 as i32,
        Facing::Right
    );

    //show(&map, pos);
    for &m in moves.iter() {
        //println!("{:?}", m);
        pos = walk(&map, pos, m);
        //show(&map, pos);
    }
    println!("The final position is {}", (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2.value());

    let mut pos = Position(
        0,
        map[0].iter().enumerate().find(|(_,t)| **t == Tile::Open).unwrap().0 as i32,
        Facing::Right
    );
    for &m in moves.iter() {
        pos = walk_on_cube(&map, pos, m);
    }
    println!("The final position is {}", (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2.value());
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile { Empty, Open, Wall }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Movement { Forward(u32), Right, Left }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Facing { Right, Down, Left, Up }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Position(i32,i32,Facing);

fn walk(map: &[Vec<Tile>], pos: Position, m: Movement) -> Position {
    match m {
        Movement::Right => Position(pos.0, pos.1, pos.2.next()),
        Movement::Left => Position(pos.0, pos.1, pos.2.prev()),
        Movement::Forward(steps) => {
            let mut pos = pos;
            for _ in 0..steps {
                let newpos = match pos.2 {
                    Facing::Right => {
                        let col = pos.1 + 1;
                        match tile_at(map, pos.0, col) {
                            Tile::Open => (pos.0, col),
                            Tile::Wall => (pos.0, pos.1),
                            Tile::Empty => {
                                let col = first_in_row(map, pos.0);
                                match tile_at(map, pos.0, col) {
                                    Tile::Open => (pos.0, col),
                                    Tile::Wall => (pos.0, pos.1),
                                    Tile::Empty => panic!()
                                }
                            }
                        }
                    }
                    Facing::Down => {
                        let row = pos.0 + 1;
                        match tile_at(map, row, pos.1) {
                            Tile::Open => (row, pos.1),
                            Tile::Wall => (pos.0, pos.1),
                            Tile::Empty => {
                                let row = first_in_col(map, pos.1);
                                match tile_at(map, row, pos.1) {
                                    Tile::Open => (row, pos.1),
                                    Tile::Wall => (pos.0, pos.1),
                                    Tile::Empty => panic!()
                                }
                            }
                        }
                    }
                    Facing::Left => {
                        let col = pos.1 - 1;
                        match tile_at(map, pos.0, col) {
                            Tile::Open => (pos.0, col),
                            Tile::Wall => (pos.0, pos.1),
                            Tile::Empty => {
                                let col = last_in_row(map, pos.0);
                                match tile_at(map, pos.0, col) {
                                    Tile::Open => (pos.0, col),
                                    Tile::Wall => (pos.0, pos.1),
                                    Tile::Empty => panic!()
                                }
                            }
                        }
                    }
                    Facing::Up => {
                        let row = pos.0 - 1;
                        match tile_at(map, row, pos.1) {
                            Tile::Open => (row, pos.1),
                            Tile::Wall => (pos.0, pos.1),
                            Tile::Empty => {
                                let row = last_in_col(map, pos.1);
                                match tile_at(map, row, pos.1) {
                                    Tile::Open => (row, pos.1),
                                    Tile::Wall => (pos.0, pos.1),
                                    Tile::Empty => panic!()
                                }
                            }
                        }
                    }
                };
                pos.0 = newpos.0;
                pos.1 = newpos.1;
            }
            pos
        }
    }
}

fn tile_at(map: &[Vec<Tile>], row: i32, col: i32) -> Tile {
    match map.get(row as usize) {
        None => Tile::Empty,
        Some(v) => match v.get(col as usize) {
            None => Tile::Empty,
            Some(t) => *t,
        }
    }
}

fn first_in_row(map: &[Vec<Tile>], row: i32) -> i32 {
    map[row as usize].iter().enumerate().find(|(_,t)| **t != Tile::Empty).unwrap().0 as i32
}

fn last_in_row(map: &[Vec<Tile>], row: i32) -> i32 {
    map[row as usize].iter().enumerate().rev().find(|(_,t)| **t != Tile::Empty).unwrap().0 as i32
}

fn first_in_col(map: &[Vec<Tile>], col: i32) -> i32 {
    (0..map.len() as i32).find(|row| tile_at(map, *row, col) != Tile::Empty).unwrap()
}

fn last_in_col(map: &[Vec<Tile>], col: i32) -> i32 {
    (0..map.len() as i32).rev().find(|row| tile_at(map, *row, col) != Tile::Empty).unwrap()
}

fn walk_on_cube(map: &[Vec<Tile>], pos: Position, m: Movement) -> Position {
    match m {
        Movement::Right => Position(pos.0, pos.1, pos.2.next()),
        Movement::Left => Position(pos.0, pos.1, pos.2.prev()),
        Movement::Forward(steps) => {
            let mut pos = pos;
            for _ in 0..steps {
                let mut newpos = match pos.2 {
                    Facing::Right => Position(pos.0, pos.1 + 1, pos.2),
                    Facing::Down => Position(pos.0 + 1, pos.1, pos.2),
                    Facing::Left => Position(pos.0, pos.1 - 1, pos.2),
                    Facing::Up => Position(pos.0 - 1, pos.1, pos.2),
                };
                if tile_at(map, newpos.0, newpos.1) == Tile::Empty {
                    newpos = overflow(newpos);
                    assert_ne!(tile_at(map, newpos.0, newpos.1), Tile::Empty);
                }
                if tile_at(map, newpos.0, newpos.1) == Tile::Open {
                    pos = newpos;
                }
            }
            pos
        }
    }
}

fn overflow(Position(row, col, facing): Position) -> Position {
    //      ab cd
    //      -- --   r=0
    //    e|11|22|g
    //    f|11|22|h
    //      --+--   r=50
    //    i|33|kl
    //   ij|33|l
    //   --+--      r=100
    // f|44|55|h
    // e|44|55|g
    //   -- --      r=150
    // a|66|mn
    // b|66|n
    //   --         r=200
    //   cd
    match facing {
        Facing::Up => match (row, col / 50) {
            /*ab*/ (-1, 1) => Position(col + 100, 0, Facing::Right),
            /*cd*/(-1, 2) => Position(199, col - 100, Facing::Up),
            /*ij*/(99, 0) => Position(col + 50, 50, Facing::Right),
            _ => panic!("Unwrappable position {:?}", Position(row, col, facing))
        }
        Facing::Down => match (row, col / 50) {
            /*kl*/(50, 2) => Position(col - 50, 99, Facing::Left),
            /*mn*/(150, 1) => Position(col + 100, 49, Facing::Left),
            /*cd*/(200, 0) => Position(0, col + 100, Facing::Down),
            _ => panic!("Unwrappable position {:?}", Position(row, col, facing))
        }
        Facing::Right => match (row / 50, col) {
            /*gh*/(0, 150) => Position(149 - row, 99, Facing::Left),
            /*kl*/(1, 100) => Position(49, row + 50, Facing::Up),
            /*hg*/(2, 100) => Position(149 - row, 149, Facing::Left),
            /*mn*/(3, 50) => Position(149, row - 100, Facing::Up),
            _ => panic!("Unwrappable position {:?}", Position(row, col, facing))
        }
        Facing::Left => match (row / 50, col) {
            /*ef*/(0, 49) => Position(149 - row, 0, Facing::Right),
            /*ij*/(1, 49) => Position(100, row - 50, Facing::Down),
            /*fe*/(2, -1) => Position(149 - row, 50, Facing::Right),
            /*ab*/(3, -1) => Position(0, row - 100, Facing::Down),
            _ => panic!("Unwrappable position {:?}", Position(row, col, facing))
        }
    }
}

fn read_map(reader: impl BufRead) -> (Vec<Vec<Tile>>, Vec<Movement>) {
    let mut lines = reader.lines();
    let mut map = Vec::new();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        map.push(line.chars().map(|c| match c {
            ' ' => Tile::Empty,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => panic!("{}", c)
        }).collect::<Vec<Tile>>());
    }

    let password = lines.next().unwrap().unwrap();
    let mut moves = Vec::new();
    for word in password.split_inclusive(['R','L']) {
        if let Some(num) = word.strip_suffix('R') {
            moves.push(Movement::Forward(num.parse().unwrap()));
            moves.push(Movement::Right);
        } else if let Some(num) = word.strip_suffix('L') {
            moves.push(Movement::Forward(num.parse().unwrap()));
            moves.push(Movement::Left);
        } else {
            moves.push(Movement::Forward(word.parse().unwrap()));
        }
    }

    (map, moves)
}

impl Facing {
    pub fn next(self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
    pub fn value(&self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

/*
fn show(map: &[Vec<Tile>], pos: Position) {
    assert_eq!(tile_at(map, pos.0, pos.1), Tile::Open);
    for row in 0..map.len() {
        for col in 0..map[row].len()  {
            if row as i32 == pos.0 && col as i32 == pos.1 {
                print!("{}", match pos.2 {
                    Facing::Right => '>',
                    Facing::Down => 'v',
                    Facing::Left => '<',
                    Facing::Up => '^',
                });
            } else {
                print!("{}", match tile_at(map, row as i32, col as i32) {
                    Tile::Empty => ' ',
                    Tile::Open => '.',
                    Tile::Wall => '#',
                });
            }
        }
        println!();
    }
}
*/
