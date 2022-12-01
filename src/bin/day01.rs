use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;

fn main() {
    let reader = BufReader::new(File::open("input/day01.txt").unwrap());

    let mut tot = 0;
    let mut max = [0,0,0];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            push(&mut max, tot);
            tot = 0;
        } else {
            tot += i32::from_str(&line).unwrap();
        }
    }
    push(&mut max, tot);

    println!("The Elf is carrying {} Calories", max[0]);
    println!("The 3 Elves are carrying {} Calories", max.iter().sum::<i32>());
}

fn push(max: &mut [i32; 3], n: i32) {
    if n > max[2] {
        if n > max[1] {
            max[2] = max[1];
            if n > max[0] {
                max[1] = max[0];
                max[0] = n;
            } else {
                max[1] = n;
            }
        } else {
            max[2] = n;
        }
    }
}
