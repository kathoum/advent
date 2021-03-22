fn main() {
    let input = b"3113322113";
    let s = repeat_describe(input, 40);
    println!("After 40 steps, length is {}", s.len());
    let s = repeat_describe(input, 50);
    println!("After 50 steps, length is {}", s.len());
}

fn repeat_describe(input: &[u8], count: usize) -> Vec<u8> {
    let mut v = Vec::from(input);
    for _ in 0..count {
        v = describe(&v);
    }
    v
}

fn describe(mut input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    while let Some(c) = input.first() {
        let n = input.iter().position(|x| x != c).unwrap_or(input.len());
        output.push(match n {
            1 => b'1',
            2 => b'2',
            3 => b'3',
            _ => panic!()
        });
        output.push(*c);
        input = &input[n..];
    }
    output
}
