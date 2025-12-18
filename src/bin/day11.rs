use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let edges: HashMap<String, Vec<String>> =
        BufReader::new(File::open("input/day11.txt").unwrap())
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (from, to) = line.split_once(": ").unwrap();
                (
                    from.to_owned(),
                    to.split_ascii_whitespace().map(str::to_owned).collect(),
                )
            })
            .collect();

    let n = count_paths(&edges, "you", "out");
    println!("Day 11 part one: {n}");

    let n1 = count_paths(&edges, "svr", "dac");
    let n2 = count_paths(&edges, "dac", "fft");
    let n3 = count_paths(&edges, "fft", "out");

    let m1 = count_paths(&edges, "svr", "fft");
    let m2 = count_paths(&edges, "fft", "dac");
    let m3 = count_paths(&edges, "dac", "out");
    println!("Day 11 part two: {}", n1 * n2 * n3 + m1 * m2 * m3);
}

fn count_paths(edges: &HashMap<String, Vec<String>>, from: &str, to: &str) -> usize {
    let mut count = HashMap::new();
    count.insert(to, 1usize);
    let mut stack = vec![from];
    while let Some(&a) = stack.last() {
        if !count.contains_key(a) {
            let mut s = Some(0);
            if let Some(ab) = edges.get(a) {
                for b in ab {
                    if let Some(c) = count.get(b.as_str()) {
                        s = s.map(|s| s + c);
                    } else {
                        s = None;
                        stack.push(b);
                    }
                }
            }
            if let Some(s) = s {
                stack.pop();
                count.insert(a, s);
            }
        }
    }
    *count.get(from).unwrap()
}
