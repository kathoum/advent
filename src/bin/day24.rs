use std::cell::Cell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut wires = HashMap::new();
    for line in BufReader::new(File::open("input/day24.txt").unwrap()).lines() {
        let line = line.unwrap();
        if let Some((name, val)) = line.split_once(": ") {
            wires.insert(
                name.to_string(),
                Wire::Input(match val {
                    "0" => false,
                    "1" => true,
                    _ => panic!(),
                }),
            );
        } else if let Some((inputs, name)) = line.split_once(" -> ") {
            let mut w = inputs.split(' ');
            let a = w.next().unwrap();
            let o = w.next().unwrap();
            let b = w.next().unwrap();
            wires.insert(
                name.to_string(),
                Wire::Binary(
                    a.to_string(),
                    b.to_string(),
                    match o {
                        "AND" => Op::And,
                        "OR" => Op::Or,
                        "XOR" => Op::Xor,
                        _ => panic!(),
                    },
                    Cell::new(None),
                ),
            );
        }
    }

    println!("Day 24 part one: {}", solve(&wires));
}

enum Wire {
    Input(bool),
    Binary(String, String, Op, Cell<Option<bool>>),
}

enum Op {
    And,
    Or,
    Xor,
}

fn solve(wires: &HashMap<String, Wire>) -> u64 {
    let mut result = 0;
    for name in wires.keys() {
        if let Some(n) = name.strip_prefix('z') {
            let b = get(wires, name);
            let i: u8 = n.parse().unwrap();
            result |= (b as u64) << i;
        }
    }
    result
}

fn get(wires: &HashMap<String, Wire>, name: &str) -> bool {
    match wires.get(name).unwrap() {
        Wire::Input(res) => *res,
        Wire::Binary(a, b, op, res) => res.get().unwrap_or_else(|| {
            let x = get(wires, a);
            let y = get(wires, b);
            let b = match op {
                Op::And => x & y,
                Op::Or => x | y,
                Op::Xor => x ^ y,
            };
            res.set(Some(b));
            b
        }),
    }
}

/*
pattern:
    r_n1 = riporto del passo precedente

    x_n1 XOR y_n1 -> aaaa
    aaaa XOR r_n1 -> z_n1
    x_n1 AND y_n1 -> bbbb
    r_n1 AND aaaa -> cccc
    bbbb OR cccc -> r_n2
*/
