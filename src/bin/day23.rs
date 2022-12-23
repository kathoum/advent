use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::{HashMap, HashSet};

fn main() {
    let reader = BufReader::new(File::open("input/day23.txt").unwrap());
    let _reader = std::io::Cursor::new(
"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............");
    let mut map = HashSet::new();
    for (row,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col,c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((row as i32, col as i32));
            }
        }
    }

    //print(&map); println!();
    for round in 0..10 {
        (map, _) = play_round(map, round);
        //print(&map); println!();
        //println!("Round {round}: area {}", covered_area(&map));
    }

    println!("There are {} empty tiles in the rectangle", covered_area(&map) - map.len() as i32);

    for round in 10.. {
        let (newmap, moved) = play_round(map, round);
        map = newmap;
        //println!("Round {round}: area {}", covered_area(&map));
        if !moved {
            println!("The first round where no elf move is {}", round + 1);
            break;
        }
    }
}

fn play_round(map: HashSet<(i32,i32)>, counter: u32) -> (HashSet<(i32,i32)>, bool) {
    let mut proposal = Vec::new();
    let mut newmap = HashMap::new();
    for &elf in &map {
        let p = desired_place(&map, elf, counter);
        proposal.push((elf, p));
        newmap.entry(p).and_modify(|n| *n += 1).or_insert(1);
    }
    //println!("proposals: {proposal:?}");
    assert_eq!(proposal.len(), map.len());

    let mut moves = 0;
    let result: HashSet<(i32,i32)> = proposal.into_iter().map(|(old, new)| {
        if old == new || newmap[&new] > 1 { old } else { moves += 1; new }
    }).collect();
    assert_eq!(result.len(), map.len());
    // if counter < 20 || counter % 1000 == 0 {
    //     println!("Round {counter}: {moves} have moved");
    // }
    (result, moves > 0)
}

fn desired_place(map: &HashSet<(i32,i32)>, (row,col): (i32,i32), counter: u32) -> (i32,i32) {
    let n = map.contains(&(row-1,col));
    let s = map.contains(&(row+1,col));
    let e = map.contains(&(row,col+1));
    let w = map.contains(&(row,col-1));
    let ne = map.contains(&(row-1,col+1));
    let nw = map.contains(&(row-1,col-1));
    let se = map.contains(&(row+1,col+1));
    let sw = map.contains(&(row+1,col-1));
    if n || s || e || w || ne || nw || se || sw {
        for i in 0..4 {
            match (counter + i) % 4 {
                0 => { if !(n || ne || nw) { return (row-1,col); } }
                1 => { if !(s || se || sw) { return (row+1,col); } }
                2 => { if !(w || nw || sw) { return (row,col-1); } }
                3 => { if !(e || ne || se) { return (row,col+1); } }
                _ => unreachable!()
            }
        }
    }
    (row,col)
}

fn covered_area(map: &HashSet<(i32,i32)>) -> i32 {
    let x1 = map.iter().map(|&(x,_)| x).min().unwrap();
    let x2 = map.iter().map(|&(x,_)| x).max().unwrap();
    let y1 = map.iter().map(|&(_,y)| y).min().unwrap();
    let y2 = map.iter().map(|&(_,y)| y).max().unwrap();
    //print!("row {x1}..{x2} col {y1}..{y2} ");
    (y2 - y1 + 1) * (x2 - x1 + 1)
}
/*
fn print(map: &HashSet<(i32,i32)>) {
    let x1 = map.iter().map(|&(x,_)| x).min().unwrap();
    let x2 = map.iter().map(|&(x,_)| x).max().unwrap();
    let y1 = map.iter().map(|&(_,y)| y).min().unwrap();
    let y2 = map.iter().map(|&(_,y)| y).max().unwrap();
    for row in x1..=x2 {
        for col in y1..=y2 {
            print!("{}", if map.contains(&(row,col)) { '#' } else { '.' });
        }
        println!();
    }
}
*/
