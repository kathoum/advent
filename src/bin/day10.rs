use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day10.txt").unwrap());

    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut strength = 0;

    let mut crt = vec![0u8; 6 * 40];

    for line in reader.lines().map(Result::unwrap) {
        for word in line.split_ascii_whitespace() {
            if cycle % 40 == 20 {
                strength += cycle * x;
            }

            let pos = cycle - 1;
            crt[pos as usize] = if (pos % 40 - x).abs() <= 1 { b'#' } else { b'.' };

            if let Ok(n) = word.parse::<i32>() {
                x += n;
            } else { match word {
                "noop" => (),
                "addx" => (),
                _ => panic!("Unexpected line {line}")
            }}
            cycle += 1;
        }
    }

    println!("The sum of the signal strengths is {strength}");
    for scanline in crt.chunks(40) {
        println!("{}", String::from_utf8_lossy(scanline));
    }
}
