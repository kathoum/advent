use std::io::{BufRead,Cursor};
use std::collections::HashSet;

fn main() {
    let reader = Cursor::new(include_str!("input01.txt"));
    let numbers: HashSet<i32> = reader.lines().map(
        |line| line.unwrap().parse().unwrap()
    ).collect();

    println!("Part One");
    for n in numbers.iter() {
        let m = 2020 - n;
        if numbers.contains(&m) {
            println!("Solution: {}*{} = {}", n, m, n * m);
        }
    }

    println!("Part Two");
    for n in numbers.iter() {
        for m in numbers.iter() {
            let l = 2020 - m - n;
            if numbers.contains(&l) {
                println!("Solution: {}*{}*{} = {}", n, m, l, n * m * l);
            }
        }
    }
}
