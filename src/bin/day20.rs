fn main() {
    let minimum_presents = 33_100_000;

    let i = (1..).find(|&n| presents_for(n) >= minimum_presents).unwrap();
    println!("Part one: lowest number is {}", i);

    let i = (1..).find(|&n| presents_for_2(n) >= minimum_presents).unwrap();
    println!("Part two: lowest number is {}", i);
}

fn presents_for(n: u32) -> u32 {
    10 * sigma(n, 2)
}

fn sigma(n: u32, start: u32) -> u32 {
    if n < start {
        return 1
    }
    for d in start..=n {
        let pk = divisor_power(n, d);
        if pk > 1 {
            let pk1 = pk + (pk - 1) / (d - 1);
            let rest = sigma(n / pk, d + 1);
            return pk1 * rest;
        }
        if d * d > n {
            return n + 1;
        }
    }
    unreachable!()
}

fn divisor_power(n: u32, d: u32) -> u32 {
    let mut m = n;
    let mut pk = 1;
    while m % d == 0 {
        m /= d;
        pk *= d;
    }
    pk
}

fn presents_for_2(n: u32) -> u32 {
    let mut s = 0;
    for d in 1..=50 {
        if n % d == 0 {
            s += (n / d) * 11;
        }
    }
    s
}
