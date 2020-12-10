use std::io::BufRead;

fn main() {
    let reader = std::io::Cursor::new(include_str!("input10.txt"));

    let mut numbers: Vec<i64> = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    numbers.push(0);
    numbers.push(*numbers.iter().max().unwrap() + 3);
    numbers.sort();
    let numbers = numbers;

    println!("Part One");
    let mut count_one = 0;
    let mut count_three = 0;
    for pair in numbers.windows(2) {
        match pair[1] - pair[0] {
            1 => count_one += 1,
            3 => count_three += 1,
            _ => ()
        }
    }
    println!("1-jolt differences * 3-jolt differences = {} * {} = {}", count_one, count_three, count_one * count_three);

    println!("Part Two");
    let mut paths = Vec::<i64>::new();
    paths.push(1);
    for idx in 1..numbers.len() {
        let mut total = 0;
        for d in 1..4 {
            if d <= idx && numbers[idx - d] >= numbers[idx] - 3 {
                total += paths[idx - d];
            }
        }
        paths.push(total);
    }
    println!("{} total possible arrangements", paths.last().unwrap());
}
