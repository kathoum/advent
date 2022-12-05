use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

fn main() {
    let reader = BufReader::new(File::open("input/day04.txt").unwrap());

    let pairs: Vec<_> = reader.lines()
        .map(Result::unwrap)
        .map(|l| {
            let mut i = l.split([',','-']).map(i32::from_str).map(Result::unwrap);
            [i.next().unwrap(), i.next().unwrap(), i.next().unwrap(), i.next().unwrap()]
        })
        .collect();

    let contained = pairs.iter().clone()
        .filter(|[a1,a2,b1,b2]| (a1 <= b1 && b2 <= a2) || (b1 <= a1 && a2 <= b2))
        .count();
    let overlapped = pairs.iter().clone()
        .filter(|[a1,a2,b1,b2]| a1.max(b1) <= a2.min(b2))
        .count();

    println!("The number of fully contained pairs is {contained}");
    println!("The number of overlapped pairs is {overlapped}");
}
