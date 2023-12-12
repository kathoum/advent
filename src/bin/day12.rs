use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let springs: Vec<Spring> = BufReader::new(File::open("input/day12.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (s1, s2) = line.split_once(' ').unwrap();
            Spring(
                s1.as_bytes().to_owned(),
                s2.split(',').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect();

    let mut memory = HashMap::new();
    let answer = springs
        .iter()
        .map(|s| arrangements(&mut memory, &s.0, &s.1))
        .sum::<usize>();
    println!("Day 12 Part One: {answer}");

    let springs5: Vec<Spring> = springs
        .iter()
        .map(|Spring(v, l)| Spring([&v[..]; 5].join(&b'?'), l.repeat(5)))
        .collect();

    let answer = springs5
        .iter()
        .map(|s| arrangements(&mut memory, &s.0, &s.1))
        .sum::<usize>();
    println!("Day 12 Part Two: {answer}");
}

struct Spring(Vec<u8>, Vec<usize>);

fn can_fit_group(v: &[u8], start: usize, len: usize) -> bool {
    !v[..start].contains(&b'#')
        && !v[start..start + len].contains(&b'.')
        && !v[start + len..].contains(&b'#')
}

fn arrangements<'a>(
    memory: &mut HashMap<(&'a [u8], &'a [usize]), usize>,
    v: &'a [u8],
    l: &'a [usize],
) -> usize {
    if let Some(r) = memory.get(&(v, l)) {
        return *r;
    }
    let r = match l {
        [] => !v.contains(&b'#') as usize,
        &[n] => {
            let max = v.len().saturating_sub(n);
            (0..=max).filter(|&i| can_fit_group(v, i, n)).count()
        }
        &[n, ref rest @ ..] => {
            let req = n + rest.iter().sum::<usize>() + rest.len();
            let max = v.len().saturating_sub(req);
            (0..=max)
                .filter(|&i| can_fit_group(&v[..i + n + 1], i, n))
                .map(|i| arrangements(memory, &v[i + n + 1..], rest))
                .sum()
        }
    };
    memory.insert((v, l), r);
    r
}
