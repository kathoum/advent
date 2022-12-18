use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    let reader = BufReader::new(File::open("input/day18.txt").unwrap());
    let blocks = reader.lines().map(|line| {
        let line = line.unwrap();
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();
        [x,y,z]
    }).collect::<HashSet<[i32; 3]>>();

    let area = blocks.iter()
        .map(|b| adjacent(*b).iter()
            .filter(|b| !blocks.contains(*b))
            .count()
        ).sum::<usize>();

    println!("Surface area is {area}");

    let x = outer_range(blocks.iter().map(|&[x,_,_]| x));
    let y = outer_range(blocks.iter().map(|&[_,y,_]| y));
    let z = outer_range(blocks.iter().map(|&[_,_,z]| z));

    let mut exterior: HashSet<[i32; 3]> = HashSet::new();
    let mut queue = vec![[*x.start(),*y.start(),*z.start()]];
    while !queue.is_empty() {
        let mut next = Vec::new();
        for block in queue {
            if exterior.insert(block) {
                for b in adjacent(block) {
                    if x.contains(&b[0]) &&
                        y.contains(&b[1]) &&
                        z.contains(&b[2]) &&
                        !blocks.contains(&b) &&
                        !exterior.contains(&b)
                    {
                        next.push(b);
                    }
                }
            }
        }
        queue = next;
    }

    let external_area = blocks.iter()
        .map(|b| adjacent(*b).iter()
            .filter(|b| exterior.contains(*b))
            .count()
        ).sum::<usize>();

    println!("External surface area is {external_area}");
}

fn adjacent([x,y,z]: [i32; 3]) -> [[i32; 3]; 6] {
    [[x-1,y,z],
     [x+1,y,z],
     [x,y-1,z],
     [x,y+1,z],
     [x,y,z-1],
     [x,y,z+1]]
}

fn outer_range(it: impl IntoIterator<Item = i32>) -> RangeInclusive<i32> {
    let (min,max) = it.into_iter()
        .fold((i32::MAX,i32::MIN), |(min,max),x| (min.min(x), max.max(x)));
    (min-1)..=(max+1)
}
