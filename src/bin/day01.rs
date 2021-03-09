fn main() {
    let input = include_str!("input01.txt");
    /*
    let floors = input.chars().scan(0, |f, c| {
        *f += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        Some(*f)
    }).collect::<Vec<i32>>();
    println!("The final floor is {}", floors.last().unwrap());
    println!("The basement was entered on step {}", floors.iter().zip(1..).find(|x| *x.0 < 0).unwrap().1);
    */
    let mut floor = 0;
    let mut first_basement = None;
    for (c, step) in input.chars().zip(1..) {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if first_basement.is_none() && floor < 0 {
            first_basement = Some(step);
        }
    }
    println!("The final floor is {}", floor);
    println!("The basement was entered on step {}", first_basement.unwrap_or(-1));
}
