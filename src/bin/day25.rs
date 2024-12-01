use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
fn main() {
    let mut f = File::create("input/day25o.dot").unwrap();
    writeln!(f, "graph G {{");
    for line in BufReader::new(File::open("input/day25.txt").unwrap()).lines() {
        let line = line.unwrap();
        let mut w = line.split_ascii_whitespace();
        let x = w.next().unwrap().strip_suffix(':').unwrap();
        for w in w {
            writeln!(f, "{} -- {}", x, w);
        }
    }
    writeln!(f, "}}");
}
