use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

fn main() {
    let machines: Vec<Machine> = BufReader::new(File::open("input/day10.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let n = machines.iter().map(buttons_for_lights).sum::<u32>();
    println!("Day 10 part one: {n}");

    let n = machines[..]
        .iter()
        .map(|m| buttons_for_joltage(m).unwrap())
        .sum::<u32>();
    println!("Day 10 part two: {n}");
}

#[derive(Default, Debug, Clone)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

fn buttons_for_lights(machine: &Machine) -> u32 {
    let num_buttons = machine.buttons.len();
    (0..1u32 << num_buttons)
        .filter_map(|mask| {
            let a = (0..num_buttons)
                .map(|p| {
                    if (mask >> p) & 1 != 0 {
                        machine.buttons[p]
                    } else {
                        0
                    }
                })
                .fold(0, |a, f| a ^ f);
            (a == machine.lights).then_some(mask.count_ones())
        })
        .min()
        .unwrap()
}

fn buttons_for_joltage(machine: &Machine) -> Option<u32> {
    let nl = machine.joltage.len();
    let nb = machine.buttons.len();
    let mut equations: Vec<(i32, Vec<i32>)> = machine
        .joltage
        .iter()
        .enumerate()
        .map(|(i, &j)| {
            (
                j as i32,
                machine
                    .buttons
                    .iter()
                    .map(|&b| 1 & (b >> i) as i32)
                    .collect(),
            )
        })
        .collect();

    let bounds: Vec<i32> = (0..nb)
        .map(|nb| {
            equations
                .iter()
                .filter(|eq| eq.1[nb] != 0)
                .map(|eq| eq.0)
                .min()
                .unwrap_or_default()
        })
        .collect();

    // for (i, eq) in equations.iter().enumerate() {
    //     println!("{i} {eq:?}");
    // }
    // println!("{bounds:?}");

    for i in 0..nl {
        if let Some((i1, b)) = (0..nb).find_map(|b| {
            (i..nl)
                .find(|&i1| equations[i1].1[b] != 0)
                .map(|i1| (i1, b))
        }) {
            equations.swap(i, i1);
            let n = equations[i].1[b];
            assert!(n != 0);
            let (eq, rest) = equations[i..].split_first_mut().unwrap();
            for eq1 in rest {
                let m = eq1.1[b];
                if m != 0 {
                    eq1.0 = eq1.0 * n - eq.0 * m;
                    for (b1, b0) in zip(&mut eq1.1, &eq.1) {
                        *b1 = *b1 * n - *b0 * m;
                    }
                }
            }
        } else if equations[i].0 != 0 {
            return None;
        }
    }

    // for (i, eq) in equations.iter().enumerate() {
    //     println!("{i} {eq:?}");
    // }

    smallest_solution(&equations, &bounds, nb, &[])
}

fn smallest_solution(
    equations: &[(i32, Vec<i32>)],
    bounds: &[i32],
    b: usize,
    b1: &[i32],
) -> Option<u32> {
    assert_eq!(b + b1.len(), bounds.len());
    if b == 0 {
        return Some(b1.iter().sum::<i32>() as u32);
    }
    let b = b - 1;
    if let Some(eq) = equations
        .iter()
        .find(|eq| eq.1[b] != 0 && eq.1[..b].iter().all(|&c| c == 0))
    {
        let k = eq.1[b];
        let v = eq.0 - zip(&eq.1[b + 1..], b1).map(|(a, b)| a * b).sum::<i32>();
        if v % k == 0 {
            let x = v / k;
            if 0 <= x && x <= bounds[b] {
                let b2 = [&[x], b1].concat();
                return smallest_solution(equations, bounds, b, &b2);
            }
        }
        None
    } else {
        let mut b2 = [&[0], b1].concat();
        // println!("{} for var {b}", bounds[b]);
        (0..=bounds[b])
            .filter_map(|x| {
                b2[0] = x;
                smallest_solution(equations, bounds, b, &b2)
            })
            .min()
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut machine = Machine::default();
        for w in s.split_ascii_whitespace() {
            if w.starts_with('[') {
                machine.lights = w
                    .trim_matches(['[', ']'])
                    .chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!(),
                    })
                    .enumerate()
                    .map(|(i, f)| f << i)
                    .sum::<u32>();
            }
            if w.starts_with('(') {
                machine.buttons.push(
                    w.trim_matches(['(', ')'])
                        .split(',')
                        .map(|n| 1u32 << (n.parse::<u32>().unwrap()))
                        .sum::<u32>(),
                );
            }
            if w.starts_with('{') {
                machine.joltage = w
                    .trim_matches(['{', '}'])
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();
            }
        }
        Ok(machine)
    }
}
