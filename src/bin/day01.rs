use std::io::{BufRead,Cursor};

fn fuel(mass : i32) -> i32 {
    let mut tot = 0;
    let mut p = mass;
    while p > 0 {
        p = (p / 3 - 2).max(0);
        tot += p
    }
    tot
}

fn main() {
    let reader = Cursor::new(include_str!("input01.txt"));

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut f = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let m : i32 = line.parse().unwrap();
        f += fuel(m);
    }
    println!("{}", f);
}