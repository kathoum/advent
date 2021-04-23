fn main() {
    let input = include_str!("input17.txt");
    let containers: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
    println!("Part one: {} possible combinations", count_combinations(containers.as_slice(), 150));
    let Counter{length, count} = count_shortest_combinations(containers.as_slice(), 150);
    println!("Part two: {} possible combinations of length {}", count, length);
}

fn count_combinations(available: &[u32], total: u32) -> usize {
    if let Some((&first, rest)) = available.split_first() {
        let count_with_first = if first <= total {
            count_combinations(rest, total - first)
        } else { 0 };
        let count_without_first = count_combinations(rest, total);
        count_with_first + count_without_first
    } else {
        if total == 0 { 1 } else { 0 }
    }
}

struct Counter {
    length: usize,
    count: usize,
}
impl Counter {
    fn none() -> Self { Counter { length: 0, count: 0 } }

    fn increment_length(self) -> Counter {
        Counter { length: self.length + 1, count: self.count }
    }

    fn combine_shortest(self, other: Counter) -> Counter {
        if self.count == 0 { other }
        else if other.count == 0 { self }
        else if self.length < other.length { self }
        else if other.length < self.length { other }
        else { Counter { length: self.length, count: self.count + other.count } }
    }
}

fn count_shortest_combinations(available: &[u32], total: u32) -> Counter {
    if let Some((&first, rest)) = available.split_first() {
        let count_with_first = if first <= total {
            count_shortest_combinations(rest, total - first).increment_length()
        } else { Counter::none() };
        let count_without_first = count_shortest_combinations(rest, total);

        count_with_first.combine_shortest(count_without_first)
    } else {
        if total == 0 { Counter{length: 0, count: 1} } else { Counter::none() }
    }
}
