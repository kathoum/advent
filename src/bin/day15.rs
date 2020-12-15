fn nth_element(start: &[i32], pos: i32) -> i32 {
    let mut last_time = vec![0i32; pos as usize];
    for (index, n) in (1..start.len()).zip(start) {
        last_time[*n as usize] = index as i32;
    }
    let mut previous = *start.last().unwrap();
    for index in (start.len() as i32 + 1)..=pos {
        let next = match last_time[previous as usize] {
            0 => 0,
            t => index - t - 1
        };
        last_time[previous as usize] = index - 1;
        previous = next;
    }
    previous
}

fn main() {
    println!("Part One");
    // assert_eq!(nth_element(&[0,3,6], 10), 0);
    // assert_eq!(nth_element(&[1,3,2], 2020), 1);
    // assert_eq!(nth_element(&[2,1,3], 2020), 10);
    // assert_eq!(nth_element(&[1,2,3], 2020), 27);
    // assert_eq!(nth_element(&[2,3,1], 2020), 78);
    // assert_eq!(nth_element(&[3,2,1], 2020), 438);
    // assert_eq!(nth_element(&[3,1,2], 2020), 1836);
    println!("{}", nth_element(&[0,6,1,7,2,19,20], 2020));

    println!("Part Two");
    // assert_eq!(nth_element(&[0,3,6], 30000000), 175594);
    // assert_eq!(nth_element(&[1,3,2], 30000000), 2578);
    // assert_eq!(nth_element(&[2,1,3], 30000000), 3544142);
    // assert_eq!(nth_element(&[1,2,3], 30000000), 261214);
    // assert_eq!(nth_element(&[2,3,1], 30000000), 6895259);
    // assert_eq!(nth_element(&[3,2,1], 30000000), 18);
    // assert_eq!(nth_element(&[3,1,2], 30000000), 362);
    println!("{}", nth_element(&[0,6,1,7,2,19,20], 30000000));
}
