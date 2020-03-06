
fn coefficient_pattern(index: usize) -> impl Iterator<Item = i32> {
    assert!(index > 0);
    std::iter::repeat(0).take(index)
        .chain(std::iter::repeat(1).take(index))
        .chain(std::iter::repeat(0).take(index))
        .chain(std::iter::repeat(-1).take(index))
        .cycle()
        .skip(1)
}

fn scalar_product(input: &[i32], pattern: impl Iterator<Item = i32>) -> i32 {
    input.iter().zip(pattern).map(|(x, y)| x * y).sum()
}

fn flawed_frequency_transmission(input: &[i32]) -> Vec<i32> {
    (1..=input.len())
       .map(|index| scalar_product(input, coefficient_pattern(index)))
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

fn flawed_frequency_transmission_2(input: &[i8]) -> Vec<i8> {
    let n = input.len();
    let mut output = Vec::with_capacity(n);
    for i in 0 .. n/3 {
        let mut sum: i32 = 0;
        let mut idx = i;
        while idx < n {
            for _ in 0..(i+1).min(n-idx) {
                sum += input[idx] as i32;
                idx += 1;
            }
            idx += i+1;
            if idx < n {
                for _ in 0..(i+1).min(n-idx) {
                    sum -= input[idx] as i32;
                    idx += 1;
                }
                idx += i+1;
            }
        }
        output.push((sum.abs() % 10) as i8);
    }
    let mut sum: i32 = input[n/3 ..= n/3*2].iter().map(|i| *i as i32).sum();
    output.push((sum.abs() % 10) as i8);
    for i in n/3+1 .. n {
        sum -= input[i-1] as i32;
        if 2*i-1 < n {
            sum += input[2*i-1] as i32;
        }
        if 2*i < n {
            sum += input[2*i] as i32;
        }
        output.push((sum.abs() % 10) as i8);
    }
    assert_eq!(n, output.len());
    output
}

fn iterated_fft_2(input: &[i8], count: usize) -> Vec<i8> {
    assert!(count > 0);
    let v = flawed_frequency_transmission_2(&input);
    match count {
        1 => v,
        _ => iterated_fft_2(&v, count - 1),
    }
}

fn digits_2(input: &str) -> Vec<i8> {
    input.chars().filter_map(|c| c.to_digit(10)).map(|u| u as i8).collect()
}

fn format_digits_2(n: &[i8]) -> String {
    n.iter().map(|n| n.to_string()).collect()
}

fn test1(input: &str, output: &str, phases: usize) {
    let vin = digits(input);
    let vout = iterated_fft(&vin, phases);
    let check = format_digits(&vout[..8]);
    if check != output {
        println!("In: {}", input);
        println!("Out: expected {} actual {}", output, check);
    } else {
        println!("{} Ok", input);
    }
}

fn test2(input: &str, output: &str, phases: usize) {
    let vin = digits_2(input);
    let vout = iterated_fft_2(&vin, phases);
    let check = format_digits_2(&vout[..8]);
    if check != output {
        println!("In: {}", input);
        println!("Out: expected {} actual {}", output, check);
    } else {
        println!("{} Ok", input);
    }
}

fn test(input: &str, output: &str, phases: usize) {
    test1(input, output, phases);
    test2(input, output, phases);
}

fn test_long(input: &str, output: &str) {
    let vin = digits_2(input);
    let vout = iterated_fft_2(&vin.repeat(1000), 100);
    let offset: usize = input[0..4].parse().unwrap();
    let check = format_digits_2(&vout[offset..offset+8]);
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
    {
        let a = digits(&s);
        println!("len {}", a.len());
        let b = iterated_fft(&a, 100);
        println!("{}", format_digits(&b[..8]));
    }
    {
        let a8 = digits_2(&s);
        let b8 = iterated_fft_2(&a8, 100);
        println!("{}", format_digits_2(&b8[..8]));
    }
    if false {
        let vin = digits_2(&s);
        let vout = iterated_fft_2(&vin.repeat(10000), 100);
        let offset: usize = s[0..8].parse().unwrap();
        let check = format_digits_2(&vout[offset..offset+8]);
        println!("{}", check);
    }
}
