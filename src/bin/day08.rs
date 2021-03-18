fn main() {
    let input = include_bytes!("input08.txt");

    let tot: usize = input.split(|&c| c == b'\n').map(overhead).sum();
    println!("Part one: the difference is {} bytes", tot);

    let tot: usize = input
        .split(|&c| c == b'\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.iter().filter(|&&c| c == b'\\' || c == b'"').count() + 2)
        .sum();
    println!("Part two: the difference is {} bytes", tot);
}

fn overhead(str: &[u8]) -> usize {
    if str.is_empty() {
        return 0;
    }
    assert_eq!(str.first(), Some(&b'"'));
    assert_eq!(str.last(), Some(&b'"'));
    let mut count = 0;
    let mut iter = str.iter();
    while let Some(c) = iter.next() {
        match c {
            b'\\' => match iter.next().unwrap() {
                b'x' => { iter.next(); iter.next(); }
                b'\\' | b'"' => (),
                _ => panic!("Invalid escape")
            },
            _ => (),
        };
        count += 1;
    }
    str.len() - count + 2
}
