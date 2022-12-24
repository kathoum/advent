use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;

enum Direction { Up, Down, Right, Left }

fn main() {
    let reader = BufReader::new(File::open("input/day24.txt").unwrap());

    let mut rows = 0;
    let mut cols = 0;
    let mut blizzards = Vec::new();
    for (row,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        assert_eq!(line.chars().next(), Some('#'));
        assert_eq!(line.chars().last(), Some('#'));
        if row == 0 {
            cols = line.len() - 2;
            assert_eq!(&line[0..2], "#.");
            assert!(line[2..].chars().all(|c| c == '#'));
        } else if line.starts_with("##") {
            rows = row - 1;
            assert!(line[..line.len()-2].chars().all(|c| c == '#'));
            assert_eq!(&line[line.len()-2..], ".#");
        } else {
            for (col,c) in line.chars().enumerate() {
                match c {
                    '#' => assert!(col == 0 || col == cols + 1),
                    '.' => (),
                    '>' => blizzards.push((row, col, Direction::Right)),
                    '<' => blizzards.push((row, col, Direction::Left)),
                    '^' => blizzards.push((row, col, Direction::Up)),
                    'v' => blizzards.push((row, col, Direction::Down)),
                    _ => panic!(),
                }
            }
        }
        assert_eq!(cols, line.len() - 2);
    }

    let start = (0, 1);
    let finish = (rows + 1, cols);
    let t1 = time_to_reach((rows,cols), &mut blizzards, start, finish);
    println!("You need {t1} minutes to reach the exit");
    let t2 = time_to_reach((rows,cols), &mut blizzards, finish, start);
    let t3 = time_to_reach((rows,cols), &mut blizzards, start, finish);
    println!("You need {} minutes to reach the exit", t1 + t2 + t3);
}

fn time_to_reach((rows,cols): (usize,usize), blizzards: &mut [(usize,usize,Direction)], start: (usize,usize), finish: (usize,usize)) -> i32 {
    let mut reachable = HashSet::from([start]);
    let mut round = 0;
    while !reachable.contains(&finish) {
        //println!("Minute {}, tiles = {:?}", round, reachable);
        round += 1;
        let mut occupied = HashSet::new();
        for blizz in blizzards.iter_mut() {
            let p = match blizz.2 {
                Direction::Right => (blizz.0, if blizz.1 < cols { blizz.1 + 1 } else { 1 }),
                Direction::Left => (blizz.0, if blizz.1 > 1 { blizz.1 - 1 } else { cols }),
                Direction::Up => (if blizz.0 > 1 { blizz.0 - 1 } else { rows }, blizz.1),
                Direction::Down => (if blizz.0 < rows { blizz.0 + 1 } else { 1 }, blizz.1),
            };
            blizz.0 = p.0;
            blizz.1 = p.1;
            occupied.insert(p);
        }

        let mut new_reachable = HashSet::new();
        for (r,c) in reachable {
            new_reachable.insert((r,c));
            if r > 1 || (r,c) == (1,1) { new_reachable.insert((r - 1, c)); }
            if r < rows || (r,c) == (rows,cols) { new_reachable.insert((r + 1, c)); }
            if r > 0 && r <= rows && c > 1 { new_reachable.insert((r, c - 1)); }
            if r > 0 && r <= rows && c < cols { new_reachable.insert((r, c + 1)); }
        }
        reachable = &new_reachable - &occupied;
    }
    round
}
