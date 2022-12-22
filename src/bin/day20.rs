use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Clone, Copy)]
struct Item { order: usize, value: isize }

fn main() {
    let reader = BufReader::new(File::open("input/day20.txt").unwrap());
    let list = reader.lines().enumerate().map(|(i,l)| Item { order: i, value: l.unwrap().parse().unwrap() }).collect::<Vec<_>>();
    {
        let mut list = list.clone();
        mix(&mut list);
        println!("The grove coordinates are {}", grove_coordinates(&list));
    }
    {
        let key = 811589153;
        let mut list = list;
        for x in &mut list { x.value *= key; }
        for _ in 0..10 { mix(&mut list); }
        println!("The grove coordinates are {}", grove_coordinates(&list));
    }
}

fn slide<T>(input: &mut [T], pos: usize, dist: isize) {
    let newpos = (pos as isize + dist).rem_euclid(input.len() as isize - 1) as usize;
    if newpos < pos {
        input[newpos..=pos].rotate_right(1);
    } else {
        input[pos..=newpos].rotate_left(1);
    }
}

fn mix(input: &mut [Item]) {
    for order in 0..input.len() {
        let (pos, item) = input.iter().enumerate().find(|(_,x)| x.order == order).unwrap();
        slide(input, pos, item.value);
    }
}

fn grove_coordinates(input: &[Item]) -> isize {
    let zero = input.iter().enumerate().find(|(_,x)| x.value == 0).unwrap().0;
    let n1 = input[(zero + 1000) % input.len()].value;
    let n2 = input[(zero + 2000) % input.len()].value;
    let n3 = input[(zero + 3000) % input.len()].value;
    n1 + n2 + n3
}
