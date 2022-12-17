use std::fs::File;
use std::io::{BufRead,BufReader};

type Range = std::ops::RangeInclusive<i32>;
struct RangeUnion(Vec<Range>);

fn main() {
    let reader = BufReader::new(File::open("input/day15.txt").unwrap());
    let sensors: Vec<[i32; 4]> = reader.lines().map(|line| {
        let line = line.unwrap();
        let mut iter = line.split(['=',',',':']).skip(1).step_by(2);
        let sx = iter.next().unwrap().parse().unwrap();
        let sy = iter.next().unwrap().parse().unwrap();
        let bx = iter.next().unwrap().parse().unwrap();
        let by = iter.next().unwrap().parse().unwrap();
        [sx, sy, bx, by]
    }).collect();

    let row = 2_000_000;
    let mut ranges = excluded_ranges(&sensors, row);
    for &[_, _, bx, by] in &sensors {
        if by == row {
            ranges.push_hole(bx);
        }
    }
    println!("In row {} there are {} excluded positions", row, ranges.size());

    let size = 4_000_000;
    for y in 0..=size {
        let mut ranges = excluded_ranges(&sensors, y);
        ranges.intersect(0..=size);
        if ranges.0.len() > 1 {
            let x = ranges.0[0].end().min(ranges.0[1].end()) + 1;
            let f = x as i64 * size as i64 + y as i64;
            println!("The tuning frequency is {f}");
            break;
        }
    }
}

fn excluded_ranges(sensors: &[[i32; 4]], row: i32) -> RangeUnion {
    let mut ranges = RangeUnion(Vec::new());
    for &[sx, sy, bx, by] in sensors {
        let d = distance((sx,sy), (bx,by));
        let r = range((sx,sy), d, row);
        if !r.is_empty() {
            ranges.push(r);
        }
    }
    ranges
}

fn distance(p: (i32,i32), q: (i32,i32)) -> i32 {
    (p.0-q.0).abs() + (p.1-q.1).abs()
}

fn range((x,y): (i32,i32), radius: i32, row: i32) -> Range {
    let w = radius - (y-row).abs();
    (x-w)..=(x+w)
}

fn union(a: &Range, b: &Range) -> Option<Range> {
    let a0 = *a.start();
    let a1 = *a.end();
    let b0 = *b.start();
    let b1 = *b.end();
    if b1 + 1 < a0 || a1 + 1 < b0 {
        None
    } else {
        Some(a0.min(b0)..=a1.max(b1))
    }
}

fn intersection(a: &Range, b: &Range) -> Range {
    *a.start().max(b.start()) ..= *a.end().min(b.end())
}

impl RangeUnion {
    pub fn size(&self) -> i32 {
        self.0.iter().map(|r| r.end() - r.start() + 1).sum()
    }

    pub fn push(&mut self, mut r: Range) {
        self.0 = self.0.drain(..).filter(|x| {
            if let Some(y) = union(&r, x) {
                r = y;
                false
            } else {
                true
            }
        }).collect();
        self.0.push(r);
    }

    pub fn push_hole(&mut self, x: i32) {
        if let Some((i, r)) = self.0.iter_mut().enumerate().find(|(_,r)| r.contains(&x)) {
            let a = *r.start()..=(x-1);
            let b = (x+1)..=*r.end();

            if a.is_empty() && b.is_empty() {
                self.0.remove(i);
            } else if a.is_empty() {
                *r = b;
            } else if b.is_empty() {
                *r = a;
            } else {
                *r = a;
                self.0.push(b);
            }
        }
    }

    pub fn intersect(&mut self, r: Range) {
        self.0 = self.0.drain(..).filter_map(|x| {
            let s = intersection(&x, &r);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }).collect();
    }
}
