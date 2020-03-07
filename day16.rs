fn partial_sum(input: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(input.len() + 1);
    let mut acc = 0;
    out.push(acc);
    for x in input {
        acc += x;
        out.push(acc);
    }
    out
}

fn alternating_sum(skip: usize, input_sum: &[i32]) -> i32 {
    assert!(0 < skip && skip < input_sum.len());
    let limit = input_sum.len() - 1;
    let mut indexes = ((skip-1)..).step_by(skip);
    let mut total = 0;
    let mut sign = 1;
    loop {
        let i = indexes.next().unwrap();
        if i > limit {
            return total;
        }
        let j = indexes.next().unwrap().min(limit);
        total += sign * (input_sum[j] - input_sum[i]);
        sign *= -1;
    }
}

fn flawed_frequency_transmission(input: &[i32]) -> Vec<i32> {
    let input_sums = partial_sum(input);
    (1..=input.len())
        .map(|index| alternating_sum(index, &input_sums))
        .map(|n| n.abs() % 10)
        .collect()
}

fn iterated_fft(input: &[i32], count: usize) -> Vec<i32> {
    assert!(count > 0);
    let v = flawed_frequency_transmission(&input);
    match count {
        1 => v,
        _ => iterated_fft(&v, count - 1),
    }
}

fn digits(input: &str) -> Vec<i32> {
    input.chars().filter_map(|c| c.to_digit(10)).map(|u| u as i32).collect()
}

fn format_digits(n: &[i32]) -> String {
    n.iter().map(|n| n.to_string()).collect()
}

fn run(input: &str, phases: usize) -> String {
    let vin = digits(input);
    let vout = iterated_fft(&vin, phases);
    format_digits(&vout[..8])
}

fn test(input: &str, output: &str, phases: usize) {
    let check = run(input, phases);
    if check != output {
        println!("In: {}", input);
        println!("Out: expected {} actual {}", output, check);
    } else {
        println!("{} Ok", input);
    }
}

fn run_long(input: &str) -> (String, usize) {
    let vin = digits(input);
    let vout = iterated_fft(&vin.repeat(10000), 100);
    let offset: usize = input[0..7].parse().unwrap();
    let output = format_digits(&vout[offset..offset+8]);
    (output, offset)
}

fn test_long(input: &str, output: &str) {
    let (check, offset) = run_long(input);
    if check != output {
        println!("In: {}", input);
        println!("Offset: {}", offset);
        println!("Out: expected {} actual {}", output, check);
    } else {
        println!("{} Ok", input);
    }
}

fn main() {
    test("12345678", "48226158", 1);
    test("12345678", "01029498", 4);
    test("80871224585914546619083218645595", "24176176", 100);
    test("19617804207202209144916044189917", "73745418", 100);
    test("69317163492948606335995924319873", "52432133", 100);
    test_long("03036732577212944063491565474664", "84462026");
    test_long("02935109699940807407585447034323", "78725270");
    test_long("03081770884921959731165446850517", "53553731");

    use std::io::Read;
    let mut f = std::fs::File::open("input16.txt").unwrap();
    let mut s: String = Default::default();
    f.read_to_string(&mut s).unwrap();
    println!("{}", run(&s, 100));
    let o = run_long(&s);
    println!("offset {} result {}", o.1, o.0);
}
