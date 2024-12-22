use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let seeds = BufReader::new(File::open("input/day22.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap());

    let mut total = 0;
    let mut gains = vec![(0u32, 0u32); 19 * 19 * 19 * 19];
    let mut maxgain = 0;
    for (id, buyer) in (1..).zip(seeds) {
        let mut price1 = buyer % 10;
        let (secret, mut price2) = step(buyer);
        let (secret, mut price3) = step(secret);
        let (mut secret, mut price4) = step(secret);
        for _ in 3..2000 {
            let (newsecret, price) = step(secret);
            let hash = (9 + price2 - price1)
                + 19 * (9 + price3 - price2)
                + 19 * 19 * (9 + price4 - price3)
                + 19 * 19 * 19 * (9 + price - price4);
            let (gain, gen) = &mut gains[hash as usize];
            if *gen != id {
                let newgain = *gain + price;
                *gen = id;
                *gain = newgain;
                maxgain = maxgain.max(newgain);
            }
            price1 = price2;
            price2 = price3;
            price3 = price4;
            price4 = price;
            secret = newsecret;
        }
        total += secret as u64;
    }

    println!("Day 22 part one: {total}");
    println!("Day 22 part two: {maxgain}");
}

fn step(x: u32) -> (u32, u32) {
    const MOD: u32 = 0xffffff;
    let y = ((x << 6) ^ x) & MOD;
    let z = ((y >> 5) ^ y) & MOD;
    let w = ((z << 11) ^ z) & MOD;
    (w, w % 10)
}
