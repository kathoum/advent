use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(File::open("input/day07.txt").unwrap()).lines();
    let start = lines.next().unwrap().unwrap();
    let bean = start.find('S').unwrap();

    let mut splits = 0;
    let mut beans = HashMap::from([(bean, 1)]);
    for line in lines.map(Result::unwrap) {
        let mut new_beans = HashMap::<usize, u64>::new();
        for (bean, count) in beans {
            if &line[bean..bean + 1] == "^" {
                *new_beans.entry(bean - 1).or_insert(0) += count;
                *new_beans.entry(bean + 1).or_insert(0) += count;
                splits += 1;
            } else {
                *new_beans.entry(bean).or_insert(0) += count;
            }
        }
        beans = new_beans;
    }
    println!("Day 7 part one: {splits}");
    let timelines = beans.values().sum::<u64>();
    println!("Day 7 part two: {timelines}");
}
