use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (a, b, c, program) = parse(BufReader::new(File::open("input/day17.txt").unwrap()));

    print!("Day 17 part one: ");
    run(std::io::stdout(), &program, a, b, c);
    println!("\x08 ");

    let mut solutions = vec![];
    for start in 0..(1 << 10) {
        if is_quine_prefix(&program, start, b, c, 1) {
            complete_quine(&program, start, b, c, 10, 1, &mut solutions);
        }
    }
    println!("Day 17 part two: {}", solutions.iter().min().unwrap());
}

fn run(mut dst: impl std::io::Write, program: &[u32], mut a: u64, mut b: u64, mut c: u64) {
    let mut ip = 0;
    while let Some(&[op, arg]) = program.get(ip..ip + 2) {
        ip += 2;
        if let Some(out) = step(op, arg, &mut ip, &mut a, &mut b, &mut c) {
            write!(dst, "{out},").unwrap();
        }
    }
}

fn is_quine_prefix(program: &[u32], mut a: u64, mut b: u64, mut c: u64, len: usize) -> bool {
    let mut ip = 0;
    let mut outpos = 0;
    while let Some(&[op, arg]) = program.get(ip..ip + 2) {
        ip += 2;
        if let Some(out) = step(op, arg, &mut ip, &mut a, &mut b, &mut c) {
            if program.get(outpos) != Some(&out) {
                return false;
            }
            outpos += 1;
            if outpos == len {
                return true;
            }
        }
    }
    false
}

fn complete_quine(
    program: &[u32],
    start: u64,
    b: u64,
    c: u64,
    digits: u32,
    len: usize,
    solutions: &mut Vec<u64>,
) {
    for triplet in 0..8 {
        let a = start + (triplet << digits);
        if is_quine_prefix(program, a, b, c, len + 1) {
            if len + 1 == program.len() {
                solutions.push(a);
            } else {
                complete_quine(program, a, b, c, digits + 3, len + 1, solutions);
            }
        }
    }
}

fn step(op: u32, arg: u32, ip: &mut usize, a: &mut u64, b: &mut u64, c: &mut u64) -> Option<u32> {
    match op {
        0 => *a = *a >> combo(arg, *a, *b, *c),
        1 => *b ^= arg as u64,
        2 => *b = combo(arg, *a, *b, *c) & 7,
        3 => {
            if *a != 0 {
                *ip = arg as usize;
            }
        }
        4 => *b ^= *c,
        5 => return Some(combo(arg, *a, *b, *c) as u32 & 7),
        6 => *b = *a >> combo(arg, *a, *b, *c),
        7 => *c = *a >> combo(arg, *a, *b, *c),
        _ => panic!(),
    };
    None
}

fn combo(arg: u32, a: u64, b: u64, c: u64) -> u64 {
    match arg {
        0..=3 => arg as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

fn parse(input: impl BufRead) -> (u64, u64, u64, Vec<u32>) {
    let mut lines = input.lines().map(Result::unwrap);
    let a: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let b: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let c: u64 = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();
    assert!(lines.next().unwrap().is_empty());
    let program: Vec<u32> = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    (a, b, c, program)
}
