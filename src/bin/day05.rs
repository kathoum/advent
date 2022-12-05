use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let reader = BufReader::new(File::open("input/day05.txt").unwrap());
    let mut lines = reader.lines();
    let stacks = parse_stacks(&mut lines);
    let moves = parse_moves(lines);

    {
        let mut stacks = stacks.clone();
        for m in &moves {
            apply_9000(m, &mut stacks);
        }
        println!("CrateMover 9000: The crates on top are {}", tops(&stacks));
    }
    {
        let mut stacks = stacks;
        for m in &moves {
            apply_9001(m, &mut stacks);
        }
        println!("CrateMover 9001: The crates on top are {}", tops(&stacks));
    }
}

fn apply_9000(&Move{count, from, to}: &Move, stacks: &mut [String]) {
    for _ in 0..count {
        let c = stacks[from-1].pop().unwrap();
        stacks[to-1].push(c);
    }
}

fn apply_9001(&Move{count, from, to}: &Move, stacks: &mut [String]) {
    let p = stacks[from-1].len() - count;
    let s = stacks[from-1].split_off(p);
    stacks[to-1] += &s;
}

fn tops(stacks: &[String]) -> String {
    stacks.iter().map(|s| s.chars().next_back().unwrap()).collect()
}

fn parse_stacks<B: BufRead>(lines: &mut std::io::Lines<B>) -> Vec<String> {
    let mut stacks: Vec<String> = Vec::new();
    for line in lines.by_ref() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        for index in (0..line.len()).step_by(4) {
            let stack_id = index / 4;
            if stacks.len() <= stack_id {
                stacks.resize(stack_id + 1, String::new());
            }
            let item = &line[index..][..3];
            match *item.as_bytes() {
                [b' ', _, b' '] => (),
                [b'[', i, b']'] => stacks[stack_id].push(char::from(i)),
                _ => panic!("Unexpected crate {item} in stack")
            }
        }
    }

    for stack in &mut stacks {
        let s = stack.chars().rev().collect();
        *stack = s;
    }

    stacks
}

fn parse_moves<B: BufRead>(lines: std::io::Lines<B>) -> Vec<Move> {
    lines.map(Result::unwrap).map(|line| {
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        Move {
            count: usize::from_str(words[1]).unwrap(),
            from: usize::from_str(words[3]).unwrap(),
            to: usize::from_str(words[5]).unwrap(),
        }
    }).collect()
}
