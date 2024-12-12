use std::collections::HashMap;

fn main() {
    let input: Vec<u64> = std::fs::read_to_string("input/day11.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();
    let count = input
        .iter()
        .map(|&n| num_stones_after(n, 25, &mut cache))
        .sum::<u64>();
    println!("Day 11 part one: {count}");
    let count = input
        .iter()
        .map(|&n| num_stones_after(n, 75, &mut cache))
        .sum::<u64>();
    println!("Day 11 part one: {count}");
}

fn num_stones_after(start: u64, count: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if count == 0 {
        return 1;
    }
    if let Some(&n) = cache.get(&(start, count)) {
        return n;
    }
    let result = if start == 0 {
        num_stones_after(1, count - 1, cache)
    } else {
        let d = start.checked_ilog10().unwrap_or(0) + 1;
        if d % 2 == 0 {
            let k = 10u64.pow(d / 2);
            let a = start / k;
            let b = start % k;
            num_stones_after(a, count - 1, cache) + num_stones_after(b, count - 1, cache)
        } else {
            num_stones_after(start * 2024, count - 1, cache)
        }
    };
    cache.insert((start, count), result);
    result
}
