fn lcm(a: i128, b: i128) -> i128 {
    a / gcd(a, b) * b
}

fn gcd(a: i128, b: i128) -> i128 {
    match b {
        0 => a,
        b => gcd(b, a % b)
    }
}

fn find_remainder(n1: i128, r1: i128, n2: i128, r2: i128) -> i128 {
    if n1 < n2 {
        return find_remainder(n2, r2, n1, r1);
    }
    for k1 in 0.. {
        let x = k1 * n1 + r1;
        if x % n2 == r2 {
            return x;
        }
    }
    unreachable!();
}

fn main() {
    let mut input = include_str!("input13.txt").lines();
    let timestamp: i32 = input.next().unwrap().parse().unwrap();
    let bus_ids: Vec<&str> = input.next().unwrap().split(',').collect();

    println!("Part One");
    let (wait_time, id) = bus_ids.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .map(|id| (id - timestamp % id, id))
        .min().unwrap();
    println!("Earliest bus is {} and departs after {} minutes", id, wait_time);
    println!("Answer: {}", id * wait_time);

    println!("Part Two");
    let (_, time) = bus_ids.iter().enumerate()
        .filter_map(|(index, s)|
            if let Ok(id) = s.parse::<i128>() {
                Some((id, index as i128))
            } else {
                None
            }
        )
        .fold((1, 0), |(n1, r1), (n2, index2)| {
            let r2 = (-index2).rem_euclid(n2);
            let n = lcm(n1, n2);
            let r = find_remainder(n1, r1, n2, r2);
            (n, r)
        });
    println!("Earliest timestamp: {}", time);
}
