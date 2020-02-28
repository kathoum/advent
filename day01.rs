use std::fs::File;
use std::io::{BufReader,BufRead};

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
    let filename = "input1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut f = 0;
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let m : i32 = line.parse().unwrap();
        f += fuel(m);
    }
    println!("{}", f);
}