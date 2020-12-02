use std::io::{BufRead,Cursor};

struct Entry {
    n1: usize,
    n2: usize,
    ch: char,
    password: String,
}

fn main() {
    let reader = Cursor::new(include_str!("input02.txt"));

    let entries: Vec<Entry> = reader.lines().map(|line| {
        let line = line.unwrap();

        let mut tokens = line.splitn(2, '-');
        let n1 : usize = tokens.next().unwrap().parse().unwrap();

        let mut tokens = tokens.next().unwrap().splitn(2, ' ');
        let n2 : usize = tokens.next().unwrap().parse().unwrap();

        let mut tokens = tokens.next().unwrap().splitn(2, ": ");
        let ch = tokens.next().unwrap();
        assert_eq!(ch.chars().count(), 1);
        let ch = ch.chars().next().unwrap();

        let password = tokens.next().unwrap();

        Entry { n1, n2, ch, password: password.into() }
    }).collect();

    println!("Part One");
    let num_valid = entries.iter().filter(|entry| {
        let occurrences = entry.password.matches(entry.ch).count();
        occurrences >= entry.n1 && occurrences <= entry.n2
    }).count();
    println!("{} valid passwords", num_valid);

    println!("Part Two");
    let num_valid = entries.iter().filter(|entry| {
        let char1 = entry.password.chars().nth(entry.n1 - 1);
        let char2 = entry.password.chars().nth(entry.n2 - 1);
        return (char1 == Some(entry.ch)) != (char2 == Some(entry.ch));
    }).count();
    println!("{} valid passwords", num_valid);
}
