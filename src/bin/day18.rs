use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut state1 = State::default();
    let mut state2 = State::default();
    for line in BufReader::new(File::open("input/day18.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut i = line.split(' ');

        let dir = i.next().unwrap();
        let len = i.next().unwrap().parse().unwrap();
        state1.step(dir, len);

        let color = &i.next().unwrap()[2..8];
        let dir = &color[5..];
        let len = i64::from_str_radix(&color[..5], 16).unwrap();
        state2.step(dir, len);
    }

    println!("Day 18 Part One: {}", state1.size());
    println!("Day 18 Part Two: {}", state2.size());
}

#[derive(Default)]
struct State {
    x: i64,
    y: i64,
    area: i64,
    perimeter: i64,
}

impl State {
    fn step(&mut self, dir: &str, len: i64) {
        self.perimeter += len;
        match dir {
            "U" | "3" => {
                self.x -= len;
                self.area += len * self.y;
            }
            "D" | "1" => {
                self.x += len;
                self.area -= len * self.y;
            }
            "L" | "2" => {
                self.y -= len;
            }
            "R" | "0" => {
                self.y += len;
            }
            _ => panic!(),
        }
    }

    fn size(&self) -> i64 {
        self.area.abs() + self.perimeter / 2 + 1
    }
}
