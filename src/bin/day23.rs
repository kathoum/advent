fn main() {
    let input = &[9, 6, 2, 7, 1, 3, 8, 5, 4];
    assert_eq!(input.len() as u32, *input.iter().max().unwrap());

    println!("Part One");
    let mut data = input.to_vec();
    for _ in 0..100 {
        data = step(&data[..]);
    }
    let one = data.iter().position(|&x| x == 1).unwrap();
    data.rotate_left(one);
    println!("The labels after 1 are {}",
        data[1..].iter()
        .map(|&i| std::char::from_digit(i as u32, 10).unwrap())
        .collect::<String>()
    );

    println!("Part Two");
    let mut data = input.to_vec();
    data.extend(input.len() as u32 + 1..=1_000_000);
    for _ in 0..100 {
        data = step(&data[..]);
    }
    let one = data.iter().position(|&x| x == 1).unwrap();
    let n1 = data[(one + 1) % data.len()];
    let n2 = data[(one + 2) % data.len()];
    println!("The stars are under {} and {} = {} (but I did only 100 iterations!)", n1, n2, n1 * n2);
}

fn step(data: &[u32]) -> Vec<u32> {
    let len = data.len();
    assert!(len >= 5);
    let current = data[0];
    let mut destination = current;
    while data[0..=3].contains(&destination) {
        destination = if destination == 1 { len as u32 } else { destination - 1 }
    }
    let dest = data.iter().position(|&x| x == destination).unwrap();
    assert!(dest > 3);
    let mut result = Vec::from(&data[4..=dest]);
    result.extend(&data[1..=3]);
    result.extend(&data[dest+1..]);
    result.extend(&data[0..1]);
    assert_eq!(result.len(), len);
    result
}
