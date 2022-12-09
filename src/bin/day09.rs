use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

fn main() {
    let reader = BufReader::new(File::open("input/day09.txt").unwrap());

    let mut rope = ((0,0), (0,0));
    let mut visited_positions = std::collections::HashSet::new();

    let mut snake = [(0,0); 10];
    let mut visited_snake_tail = std::collections::HashSet::new();

    for line in reader.lines() {
        let Movement(direction, count) = line.unwrap().parse().unwrap();
        for _ in 0..count {
            step_rope(&mut rope, direction);
            visited_positions.insert(rope.1);

            step_snake(&mut snake, direction);
            visited_snake_tail.insert(*snake.last().unwrap());
        }
    }

    println!("The tail visited {} positions", visited_positions.len());
    println!("The tail of the long rope visited {} positions", visited_snake_tail.len());
}

#[derive(Clone, Copy)]
enum Direction { D, L, R, U }
struct Movement(Direction, u32);

fn step(pos: &mut (i32,i32), dir: Direction) {
    match dir {
        Direction::D => pos.1 += 1,
        Direction::L => pos.0 -= 1,
        Direction::R => pos.0 += 1,
        Direction::U => pos.1 -= 1,
    }
}

fn follow(tail: &mut (i32,i32), head: (i32,i32)) {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        tail.0 += diff.0.signum();
        tail.1 += diff.1.signum();
    }
}

fn step_rope((ref mut head, ref mut tail): &mut ((i32,i32), (i32,i32)), dir: Direction) {
    step(head, dir);
    follow(tail, *head);
}

fn step_snake(snake: &mut [(i32,i32)], dir: Direction) {
    step(&mut snake[0], dir);
    for t in 1..snake.len() {
        let head = snake[t-1];
        follow(&mut snake[t], head);
    }
}

impl FromStr for Movement {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let dir = match it.next().unwrap() {
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            "U" => Direction::U,
            _ => panic!("Unexpected direction {s}")
        };
        let count = it.next().unwrap().parse()?;
        Ok(Movement(dir, count))
    }
}
