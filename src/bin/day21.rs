use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let garden: Vec<Vec<u8>> = BufReader::new(File::open("input/day21.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();

    // let garden = vec![
    //     b"...........".to_owned(),
    //     b".....###.#.".to_owned(),
    //     b".###.##..#.".to_owned(),
    //     b"..#.#...#..".to_owned(),
    //     b"....#.#....".to_owned(),
    //     b".##..S####.".to_owned(),
    //     b".##..#...#.".to_owned(),
    //     b".......##..".to_owned(),
    //     b".##.#.####.".to_owned(),
    //     b".##..##.##.".to_owned(),
    //     b"...........".to_owned(),
    // ];

    let sr = garden.iter().position(|row| row.contains(&b'S')).unwrap();
    let sc = garden[sr].iter().position(|&x| x == b'S').unwrap();

    println!(
        "Day 21 Part One: {}",
        count_reachable(&garden, (sr, sc), 64)
    );

    let rows = garden.len();
    let cols = garden[0].len();
    assert_eq!(rows, cols);
    assert_eq!(sr * 2 + 1, rows);
    assert_eq!(sc * 2 + 1, cols);
    let n = rows;
    let x = sr;
    let distance = distances(&garden, (x, x));
    assert_eq!(distance[0][0] as usize, 2 * x);
    for i in 0..=x {
        assert_eq!(distance[0][x - i] as usize, x + i);
        assert_eq!(distance[0][x + i] as usize, x + i);
        assert_eq!(distance[n - 1][x - i] as usize, x + i);
        assert_eq!(distance[n - 1][x + i] as usize, x + i);
        assert_eq!(distance[x - i][0] as usize, x + i);
        assert_eq!(distance[x + i][0] as usize, x + i);
        assert_eq!(distance[x - i][n - 1] as usize, x + i);
        assert_eq!(distance[x + i][n - 1] as usize, x + i);
    }
    assert_eq!(2 * x as u32, max_distance(&garden, (x, x)));
    assert_eq!(4 * x as u32, max_distance(&garden, (0, 0)));
    assert_eq!(4 * x as u32, max_distance(&garden, (n - 1, 0)));
    assert_eq!(4 * x as u32, max_distance(&garden, (0, n - 1)));
    assert_eq!(4 * x as u32, max_distance(&garden, (n - 1, n - 1)));
    assert_eq!(3 * x as u32, max_distance(&garden, (x, 0)));
    assert_eq!(3 * x as u32, max_distance(&garden, (x, n - 1)));
    assert_eq!(3 * x as u32, max_distance(&garden, (0, x)));
    assert_eq!(3 * x as u32, max_distance(&garden, (n - 1, x)));

    let (even, odd) = count_even_odd(&garden, (0, 0));
    let steps = 26501365;
    // completo se (a+b)(2x+1)+2x <= steps
    //              a+b <= (steps-2x)/(2x+1)
    let c = (steps - 2 * x) / n;
    //println!("{} {}", c, c*(2*x+1)+2*x);
    // toccato se a>0,b>0,(a+b)(2x+1)-2x <= steps
    //               a+1 <= (steps+2x)/(2x+1)
    //         se b=0,a*(2x+1)-x <= steps
    //               a <= (steps+x)/(2x+1)
    let d = (steps + 2 * x) / n;
    let d0 = (steps + x) / n;
    //println!("{} {}", d, d0);
    assert_eq!(c + 2, d);
    assert_eq!(c + 1, d0);

    // 0,0
    let mut answer = odd;
    // fino a <= c
    for i in 1..=c {
        let y = if i % 2 == 0 { odd } else { even };
        answer += (i * 4) * y;
    }
    // c+1 non allineati
    assert!(steps >= (c + 1) * n - 2 * x);
    let c1_steps = (steps - ((c + 1) * n - 2 * x)) as u32;
    answer += count_reachable(&garden, (0, 0), c1_steps) * c;
    answer += count_reachable(&garden, (0, n - 1), c1_steps) * c;
    answer += count_reachable(&garden, (n - 1, 0), c1_steps) * c;
    answer += count_reachable(&garden, (n - 1, n - 1), c1_steps) * c;
    // c+1 allineati
    assert!(c1_steps >= x as u32);
    let c1a_steps = c1_steps - x as u32;
    answer += count_reachable(&garden, (x, 0), c1a_steps);
    answer += count_reachable(&garden, (x, n - 1), c1a_steps);
    answer += count_reachable(&garden, (0, x), c1a_steps);
    answer += count_reachable(&garden, (n - 1, x), c1a_steps);
    assert!(c1a_steps < n as u32);
    // c+2 non allineati
    assert!(steps >= (c + 2) * n - 2 * x);
    let c2_steps = (steps - ((c + 2) * n - 2 * x)) as u32;
    answer += count_reachable(&garden, (0, 0), c2_steps) * (c + 1);
    answer += count_reachable(&garden, (0, n - 1), c2_steps) * (c + 1);
    answer += count_reachable(&garden, (n - 1, 0), c2_steps) * (c + 1);
    answer += count_reachable(&garden, (n - 1, n - 1), c2_steps) * (c + 1);
    assert!(c2_steps < n as u32);

    println!("Day 21 Part Two: {answer}");
}

fn distances(garden: &[Vec<u8>], start: (usize, usize)) -> Vec<Vec<u32>> {
    let rows = garden.len();
    let cols = garden[0].len();

    let mut distance = vec![vec![u32::MAX; cols]; rows];
    let mut frontier = VecDeque::new();
    frontier.push_back((start.0, start.1, 0));
    while let Some((r, c, d)) = frontier.pop_front() {
        if distance[r][c] > d && garden[r][c] != b'#' {
            distance[r][c] = d;
            if r > 0 {
                frontier.push_back((r - 1, c, d + 1));
            }
            if c > 0 {
                frontier.push_back((r, c - 1, d + 1));
            }
            if r + 1 < rows {
                frontier.push_back((r + 1, c, d + 1));
            }
            if c + 1 < cols {
                frontier.push_back((r, c + 1, d + 1));
            }
        }
    }
    distance
}

fn count_reachable(garden: &[Vec<u8>], start: (usize, usize), steps: u32) -> usize {
    let distance = distances(garden, start);
    distance
        .into_iter()
        .flatten()
        .filter(|&d| d <= steps && d % 2 == steps % 2)
        .count()
}

fn max_distance(garden: &[Vec<u8>], start: (usize, usize)) -> u32 {
    let distance = distances(garden, start);
    distance
        .into_iter()
        .flatten()
        .filter(|&x| x != u32::MAX)
        .max()
        .unwrap()
}

fn count_even_odd(garden: &[Vec<u8>], start: (usize, usize)) -> (usize, usize) {
    let distance = distances(garden, start);
    distance
        .into_iter()
        .flatten()
        .filter(|&x| x != u32::MAX)
        .fold((0, 0), |(even, odd), d| {
            if d % 2 == 0 {
                (even + 1, odd)
            } else {
                (even, odd + 1)
            }
        })
}
