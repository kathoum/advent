use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

enum What { Rock, Sand }

fn main() {
    let reader = BufReader::new(File::open("input/day14.txt").unwrap());

    let mut cave = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut v: Option<(i32,i32)> = None;
        for coord in line.split(" -> ") {
            let (x,y) = coord.split_once(',').unwrap();
            let to = (x.parse().unwrap(), y.parse().unwrap());
            if let Some(from) = v {
                for c in line_between(from, to) {
                    cave.insert(c, What::Rock);
                }
            }
            v = Some(to);
        }
    }

    let bottom = cave.keys().map(|(_,y)| *y).max().unwrap();

    let counter = (0..).take_while(|_| pour(&mut cave, Some(bottom), None, (500,0))).count();
    println!("The bottomless cave holds {counter} sand");

    let counter = counter + (0..).take_while(|_| pour(&mut cave, None, Some(bottom + 2), (500,0))).count();
    println!("The floored cave holds {counter} sand");
}

fn line_between(from: (i32,i32), to: (i32,i32)) -> impl Iterator<Item = (i32,i32)> {
    let dir = ((to.0 - from.0).signum(), (to.1 - from.1).signum());
    let len = (to.0 - from.0).abs() + (to.1 - from.1).abs();
    (0..=len).map(move |i| (from.0 + i * dir.0, from.1 + i * dir.1))
}

fn pour(cave: &mut HashMap<(i32,i32), What>, bottom: Option<i32>, floor: Option<i32>, source: (i32,i32)) -> bool {
    if cave.contains_key(&source) {
        return false;
    }
    let (mut x, mut y) = source;
    while bottom != Some(y) {
        if floor != Some(y + 1) {
            if !cave.contains_key(&(x, y + 1)) { y += 1; continue; }
            if !cave.contains_key(&(x - 1, y + 1)) { x -= 1; y += 1; continue; }
            if !cave.contains_key(&(x + 1, y + 1)) { x += 1; y += 1; continue; }
        }
        cave.insert((x, y), What::Sand);
        return true;
    }
    false
}
