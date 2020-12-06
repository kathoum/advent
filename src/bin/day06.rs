use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let reader = std::io::Cursor::new(include_str!("input06.txt"));
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let groups = lines.split(|str| str == "");

    println!("Part One");
    let counts = groups.map(|group| {
        group.iter().map(|line| line.chars().collect::<HashSet<char>>())
            .fold(HashSet::new(), |acc, x| &acc | &x)
            .len()
    });
    println!("Sum of counts (any): {}", counts.sum::<usize>());

    println!("Part Two");
    let groups = lines.split(|str| str == "");
    let counts = groups.map(|group| {
        group.iter().map(|line| line.chars().collect::<HashSet<char>>())
            .fold(None, |acc, x| match acc {
                None => Some(x),
                Some(y) => Some(&x & &y)
            })
            .unwrap().len()
    });
    println!("Sum of counts (every): {}", counts.sum::<usize>());
}
