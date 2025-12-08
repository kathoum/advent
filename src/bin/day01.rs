use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut pos = 50;
    let mut n1 = 0;
    let mut n2 = 0;
    for line in BufReader::new(File::open("input/day01.txt").unwrap()).lines() {
        let line = line.unwrap();
        let (dir, count) = line.split_at(1);
        let delta = count.parse::<i32>().unwrap()
            * match dir {
                "L" => -1,
                "R" => 1,
                _ => panic!(),
            };

        let newpos = pos + delta;
        n2 += match newpos {
            ..=0 => (-newpos) / 100 + if pos == 0 { 0 } else { 1 },
            1..=99 => 0,
            100.. => newpos / 100,
        };
        pos = newpos.rem_euclid(100);
        if pos == 0 {
            n1 += 1
        }
    }
    println!("Day 1 part one: {n1}");
    println!("Day 1 part two: {n2}");
}
