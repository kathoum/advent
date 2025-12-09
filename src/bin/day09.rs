use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let red: Vec<[i64; 2]> = BufReader::new(File::open("input/day09.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (a, b) = line.split_once(',').unwrap();
            [a.parse().unwrap(), b.parse().unwrap()]
        })
        .collect();

    let a = red
        .iter()
        .enumerate()
        .map(|(i, p)| red[..=i].iter().map(|q| area(*p, *q)).max().unwrap())
        .max()
        .unwrap();
    println!("Day 9 part one: {a}");

    let mut hseg = vec![];
    let mut vseg = vec![];
    for pq in red
        .windows(2)
        .chain([[*red.last().unwrap(), *red.first().unwrap()].as_slice()])
    {
        let (p, q) = (pq[0], pq[1]);
        if p[1] == q[1] {
            hseg.push((p[1], min(p[0], q[0]), max(p[0], q[0])));
        }
        if p[0] == q[0] {
            vseg.push((p[0], min(p[1], q[1]), max(p[1], q[1])));
        }
    }
    hseg.sort();
    vseg.sort();
    for d in 0..2 {
        let mut c: Vec<i64> = red.iter().map(|p| p[d]).collect();
        c.sort();
        for w in c.windows(2) {
            assert!(
                w[0] == w[1]
                    || w[0] + 2 <= w[1]
                    || (d == 0 && w[0] == 2508 && w[1] == 2509)
                    || (d == 1 && w[0] == 97757 && w[1] == 97758),
                "({d}) {} - {}",
                w[0],
                w[1]
            );
        }
    }
    let mut colors = HashMap::new();
    let mut dir = dir_from_to(*red.last().unwrap(), *red.first().unwrap());
    for (i, pq) in red
        .windows(2)
        .chain([[*red.last().unwrap(), *red.first().unwrap()].as_slice()])
        .enumerate()
    {
        let (p, q) = (pq[0], pq[1]);
        colors.insert(p, 'r');
        let dirout = dir_from_to(p, q);
        let pencils = match (dir, dirout) {
            ('L', 'U') => "gg grgggg",
            ('R', 'D') => "ggggrg gg",
            ('U', 'R') => "ggggrggg ",
            ('D', 'L') => " gggrgggg",
            ('L', 'D') => "    rg gg",
            ('R', 'U') => "gg gr    ",
            ('U', 'L') => "   gr gg ",
            ('D', 'R') => " gg rg   ",
            _ => panic!("{dir} -> {dirout} at step {i}"),
        };
        assert_eq!(pencils.len(), 9);
        assert_eq!(&pencils[4..5], "r");
        let mut k = pencils.chars();
        colors.insert([p[0] - 1, p[1] + 1], k.next().unwrap());
        colors.insert([p[0], p[1] + 1], k.next().unwrap());
        colors.insert([p[0] + 1, p[1] + 1], k.next().unwrap());
        colors.insert([p[0] - 1, p[1]], k.next().unwrap());
        colors.insert([p[0], p[1]], k.next().unwrap());
        colors.insert([p[0] + 1, p[1]], k.next().unwrap());
        colors.insert([p[0] - 1, p[1] - 1], k.next().unwrap());
        colors.insert([p[0], p[1] - 1], k.next().unwrap());
        colors.insert([p[0] + 1, p[1] - 1], k.next().unwrap());

        dir = dirout;
    }

    let a = red
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            red[..=i]
                .iter()
                .filter(|&q| is_red_green(*p, *q, &hseg, &vseg, &colors))
                .map(|q| area(*p, *q))
        })
        .max()
        .unwrap();
    println!("Day 9 part two: {a}");
}

fn area(p: [i64; 2], q: [i64; 2]) -> i64 {
    ((p[0] - q[0]).abs() + 1) * ((p[1] - q[1]).abs() + 1)
}

fn is_red_green(
    p: [i64; 2],
    q: [i64; 2],
    hseg: &[(i64, i64, i64)],
    vseg: &[(i64, i64, i64)],
    colors: &HashMap<[i64; 2], char>,
) -> bool {
    let (x1, x2) = (min(p[0], q[0]), max(p[0], q[0]));
    let (y1, y2) = (min(p[1], q[1]), max(p[1], q[1]));
    !crosses((x1, y1, y2), hseg, 0, colors)
        && !crosses((x2, y1, y2), hseg, 0, colors)
        && !crosses((y1, x1, x2), vseg, 1, colors)
        && !crosses((y2, x1, x2), vseg, 1, colors)
}

fn crosses(
    (sx, sy1, sy2): (i64, i64, i64),
    seg: &[(i64, i64, i64)],
    d: usize,
    colors: &HashMap<[i64; 2], char>,
) -> bool {
    let i1 = seg.partition_point(|&(ty, _, _)| ty < sy1);
    let i2 = seg.partition_point(|&(ty, _, _)| ty <= sy2);
    for &(ty, tx1, tx2) in &seg[i1..i2] {
        if sy1 < ty && ty < sy2 && tx1 < sx && sx < tx2 {
            return true;
        } else if sy1 < ty && ty < sy2 && (tx1 == sx || sx == tx2) {
            assert!(color(colors, sx, ty, d) == 'r');
            if color(colors, sx, ty - 1, d) != 'g' || color(colors, sx, ty + 1, d) != 'g' {
                return true;
            }
        } else if sy1 == ty && (tx1 == sx || sx == tx2) {
            assert!(color(colors, sx, ty, d) == 'r');
            if color(colors, sx, ty + 1, d) != 'g' {
                return true;
            }
        } else if sy2 == ty && (tx1 == sx || sx == tx2) {
            assert!(color(colors, sx, ty, d) == 'r');
            if color(colors, sx, ty - 1, d) != 'g' {
                return true;
            }
        } else if sy1 == ty && tx1 < sx && sx < tx2 {
            let c1 = color(colors, tx1 + 1, ty + 1, d);
            let c2 = color(colors, tx2 - 1, ty + 1, d);
            assert_eq!(c1, c2);
            if c1 != 'g' {
                return true;
            }
        } else if sy2 == ty && tx1 < sx && sx < tx2 {
            let c1 = color(colors, tx1 + 1, ty - 1, d);
            let c2 = color(colors, tx2 - 1, ty - 1, d);
            assert_eq!(c1, c2);
            if c1 != 'g' {
                return true;
            }
        } else {
            assert!(!(sy1 <= ty && ty <= sy2 && tx1 <= sx && sx <= tx2));
        }
    }
    false
}

fn color(colors: &HashMap<[i64; 2], char>, x: i64, y: i64, d: usize) -> char {
    let coord = if d == 0 { [x, y] } else { [y, x] };
    *colors.get(&coord).expect("{coord} not found")
}

fn dir_from_to(p: [i64; 2], q: [i64; 2]) -> char {
    if p[0] < q[0] {
        'R'
    } else if p[0] > q[0] {
        'L'
    } else if p[1] < q[1] {
        'U'
    } else if p[1] > q[1] {
        'D'
    } else {
        panic!()
    }
}
