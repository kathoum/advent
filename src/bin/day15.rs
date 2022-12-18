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
            ranges.push_hole(&(bx..=bx));
        }
    }
    println!("In row {} there are {} excluded positions", row, ranges.size());

    let size = 4_000_000;
    let mut beacons = Vec::new();
    for &[sx, sy, bx, by] in &sensors {
        let r = distance((sx,sy), (bx,by));
        for d in [sx + sy - r - 1, sx + sy - r - 2, sx + sy + r + 1, sx + sy + r + 2] {
            let ranges = excluded_diagonal_ranges(&sensors, d);
            let mut valid_range = RangeUnion(vec![intersection(&(0..=size), &((d-size)..=d))]);
            valid_range.subtract(&ranges);
            for x in valid_range.0.into_iter().flatten() {
                beacons.push((x, d - x));
            }
        }
    }
    beacons.sort();
    beacons.dedup();
    for (x,y) in beacons {
        let freq = x as i64 * size as i64 + y as i64;
        println!("The tuning frequency is {freq}");
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

fn excluded_diagonal_ranges(sensors: &[[i32; 4]], diagonal: i32) -> RangeUnion {
    let mut ranges = RangeUnion(Vec::new());
    for &[sx, sy, bx, by] in sensors {
        let d = distance((sx,sy), (bx,by));
        let r = diagonal_range((sx,sy), d, diagonal);
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

// x range of covered cells in the diagonal given by x+y=diagonal
fn diagonal_range((x,y): (i32,i32), radius: i32, diagonal: i32) -> Range {
    let h = diagonal - (x + y - radius);
    if h >= 0 && h <= 2 * radius {
        let low = x - radius + (h + 1) / 2;
        let high = x + h / 2;
        low..=high
    } else {
        (x+1)..=x // empty
    }
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

    pub fn push_hole(&mut self, x: &Range) {
        let mut result = Vec::new();
        for r in self.0.drain(..) {
            let (a,b) = r.into_inner();
            let r1 = a..=b.min(x.start()-1);
            if !r1.is_empty() {
                result.push(r1);
            }
            let r2 = a.max(x.end()+1)..=b;
            if !r2.is_empty() {
                result.push(r2);
            }
        }
        self.0 = result;
    }

    pub fn subtract(&mut self, r: &RangeUnion) {
        for r in &r.0 {
            self.push_hole(r);
        }
    }
}
