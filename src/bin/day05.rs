use std::io::BufRead;

fn main() {
    let reader = std::io::Cursor::new(include_str!("input05.txt"));
    let mut seat_ids: Vec<u32> = reader.lines().map(|line| {
        line.unwrap().chars().rev().enumerate().fold(0, |acc, (i, ch)| {
            let value = match ch {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!()
            };
            acc + (value << i)
        })
    }).collect();

    println!("Part One");
    println!("Highest seat ID: {}", seat_ids.iter().max().unwrap());

    println!("Part Two");
    seat_ids.sort();
    let my_seat = match seat_ids.iter().zip(&seat_ids[1..])
        .find(|(&n1, &n2)| n2 - n1 == 2) {
            Some((&n1, _)) => n1 + 1,
            None => panic!()
        };
    println!("My seat: {}", my_seat);
}
