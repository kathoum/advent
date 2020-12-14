fn main() {
    let input = include_str!("input14.txt");

    println!("Part One");
    let mut mask0 = 0u64;
    let mut mask1 = 0u64;
    let mut mem = std::collections::HashMap::new();
    for line in input.lines() {
        if let Some(m) = line.strip_prefix("mask = ") {
            mask0 = 0;
            mask1 = 0;
            for (index, c) in m.chars().rev().enumerate() {
                match c {
                    '0' => mask0 |= 1 << index,
                    '1' => mask1 |= 1 << index,
                    'X' => (),
                    _ => panic!(format!("Unexpected mask character: {}", c))
                }
            }
        } else {
            let address: u64;
            let value: u64;
            text_io::scan!(line.bytes() => "mem[{}] = {}", address, value);
            mem.insert(address, (value & !mask0) | mask1);
        }
    }
    println!("The sum of all values is {}", mem.values().sum::<u64>());

    println!("Part Two");
    let mut mask1 = 0u64;
    let mut mask_x = Vec::new();
    let mut mem = std::collections::HashMap::new();
    for line in input.lines() {
        if let Some(m) = line.strip_prefix("mask = ") {
            mask1 = 0;
            mask_x = Vec::new();
            for (index, c) in m.chars().rev().enumerate() {
                match c {
                    '0' => (),
                    '1' => mask1 |= 1 << index,
                    'X' => mask_x.push(index),
                    _ => panic!(format!("Unexpected mask character: {}", c))
                }
            }
        } else {
            let address: u64;
            let value: u64;
            text_io::scan!(line.bytes() => "mem[{}] = {}", address, value);
            store_at_addresses(&mut mem, address | mask1, &mask_x, value);
        }
    }
    println!("The sum of all values is {}", mem.values().sum::<u64>());
}

fn store_at_addresses(mem: &mut std::collections::HashMap<u64,u64>, address: u64, floating: &[usize], value: u64) {
    if let Some(index) = floating.get(0) {
        let mask = 1u64 << index;
        store_at_addresses(mem, address | mask, &floating[1..], value);
        store_at_addresses(mem, address & !mask, &floating[1..], value);
    } else {
        mem.insert(address, value);
    }
}
