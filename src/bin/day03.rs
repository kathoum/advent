use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

fn main() {
    let mut lines: Vec<Line> = BufReader::new(File::open("input/day03.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into())
        .collect();
    lines.insert(0, Line::default());
    lines.push(Line::default());

    let mut total = 0;
    for block in lines.windows(3) {
        for (range, val) in &block[1].numbers {
            let mut symbols = block.iter().flat_map(|l| &l.symbols);
            let is_part_number = symbols.any(|&(pos, _)| is_adjacent(range, pos));
            if is_part_number {
                total += val;
            }
        }
    }
    println!("Day 3 Part One: {total}");

    let mut ratio = 0;
    for block in lines.windows(3) {
        for (pos, s) in &block[1].symbols {
            if *s == b'*' {
                let (count, product) = block
                    .iter()
                    .flat_map(|l| &l.numbers)
                    .filter_map(|(range, val)| is_adjacent(range, *pos).then_some(*val))
                    .fold((0, 1), |(count, product), number| {
                        (count + 1, product * number)
                    });
                if count == 2 {
                    ratio += product;
                }
            }
        }
    }
    println!("Day 3 Part Two: {ratio}");
}

#[derive(Default)]
struct Line {
    numbers: Vec<(Range<usize>, u32)>,
    symbols: Vec<(usize, u8)>,
}

impl From<String> for Line {
    fn from(s: String) -> Self {
        let mut numbers = vec![];
        let mut symbols = vec![];
        let b = s.as_bytes();
        let mut pos = 0;
        while pos < b.len() {
            let c = b[pos];
            if c.is_ascii_digit() {
                let len = b[pos..]
                    .iter()
                    .copied()
                    .take_while(u8::is_ascii_digit)
                    .count();
                let n = s[pos..pos + len].parse().unwrap();
                numbers.push((pos..pos + len, n));
                pos += len - 1;
            } else if c != b'.' {
                symbols.push((pos, c));
            }
            pos += 1;
        }
        Line { numbers, symbols }
    }
}

fn is_adjacent(number: &Range<usize>, symbol: usize) -> bool {
    symbol + 1 >= number.start && symbol <= number.end
}
