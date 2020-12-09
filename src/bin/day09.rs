use std::io::BufRead;

fn main() {
    let reader = std::io::Cursor::new(include_str!("input09.txt"));
    let numbers: Vec<i64> = reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    println!("Part One");
    let errors = numbers.windows(25).zip(numbers[25..].iter()).filter(|(slice, next)| {
        for n1 in *slice {
            for n2 in *slice {
                if n1 != n2 && n1 + n2 == **next {
                    return false;
                }
            }
        }
        true
    }).map(|(_,n)| n).collect::<Vec<_>>();
    println!("The numbers without the property are: {:?}", errors);
    let key = errors[0];

    println!("Part Two");
    let sums: Vec<i64> = [0].iter().chain(numbers.iter()).scan(0, |sum, &n| { *sum += n; Some(*sum) }).collect();
    for i in 0..sums.len() {
        let j = sums.binary_search(&(key + sums[i]));
        if let Ok(j) = j {
            if j > i + 1 {
                println!("Range {}-{}", i, j);
                let range = &numbers[i..j];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                println!("The encryption weakness is {}+{} = {}", min, max, min + max);
            }
        }
    }
}
