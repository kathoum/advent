use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("input/day08.txt").unwrap()).lines();
    let seq = lines.next().unwrap().unwrap();
    let dirs = lines
        .filter_map(|line| {
            line.ok().and_then(|line| {
                line.split_once(" = ").map(|(a, b)| {
                    let bl = &b[1..4];
                    let br = &b[6..9];
                    (node(a), (node(bl), node(br)))
                })
            })
        })
        .collect::<HashMap<Node, (Node, Node)>>();

    println!(
        "Part One: {}",
        steps(&seq, &dirs, node("AAA"), |n| n == node("ZZZ"))
    );

    let answer = dirs
        .keys()
        .filter(|n| n.ends_with(&[b'A']))
        .map(|start| steps(&seq, &dirs, *start, |n| n.ends_with(&[b'Z'])))
        .reduce(lcm)
        .unwrap();
    println!("Part Two: {answer}");
}

type Node = [u8; 3];

fn node(s: &str) -> Node {
    s.as_bytes().try_into().unwrap()
}

fn steps(
    lr: &str,
    dirs: &HashMap<Node, (Node, Node)>,
    from: Node,
    to: impl Fn(Node) -> bool,
) -> usize {
    let mut current = from;
    for (i, d) in lr.chars().cycle().enumerate() {
        if to(current) {
            return i;
        } else {
            current = match d {
                'L' => dirs[&current].0,
                'R' => dirs[&current].1,
                _ => panic!(),
            }
        }
    }
    unreachable!()
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}
