fn main() {
    let input = std::fs::read_to_string("input/day06.txt").unwrap();
    let (l1, l2) = input.split_once('\n').unwrap();
    let l1 = l1.strip_prefix("Time:").unwrap();
    let l2 = l2.strip_prefix("Distance:").unwrap();
    let times: Vec<u64> = l1.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let distances: Vec<u64> = l2.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let answer: u64 = std::iter::zip(&times, &distances)
        .map(|(t, d)| winrange(*t, *d))
        .product();
    println!("Day 6 Part One: {answer}");

    let time = l1.replace(char::is_whitespace, "").parse().unwrap();
    let distance = l2.replace(char::is_whitespace, "").parse().unwrap();
    println!("Day 6 Part Two: {}", winrange(time, distance));
}

fn winrange(time: u64, distance: u64) -> u64 {
    let a = search(0, time / 2, |n| n * (time - n) > distance);
    let b = search(time / 2, time, |n| n * (time - n) <= distance);
    b - a
}

fn search(mut low: u64, mut high: u64, f: impl Fn(u64) -> bool) -> u64 {
    assert!(high > low);
    assert!(!f(low));
    assert!(f(high));
    while high - low > 1 {
        let mid = (low + high) / 2;
        if f(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    high
}
