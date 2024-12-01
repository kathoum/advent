use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let mut stones: Vec<Stone> = BufReader::new(File::open("input/day24.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let range = 200000000000000 ..= 400000000000000;
    //    let range = 7 ..= 27;
    let intersecting_pairs = (0..stones.len()).map(|i| stones[..i].iter().filter(|s| intersect_2d_inside(s, &stones[i], range.clone())).count()).sum::<usize>();
    println!("Day 24 Part One: {}", intersecting_pairs);

    solve(&stones);
    // println!("{:?}", solve_for_xy(&stones[0], &stones[1], &stones[2]));
    stones[0] = Stone { pos: (102352610405511, 54441177479725, 0), vel: (288, 406, 0) };
    stones[1] = Stone { pos: (225555584244738, 158175058414862, 0), vel: (97, 177, 0) };
    stones[2] = Stone { pos: (227688767181124, 197748595030382, 0), vel: (104, -576, 0) };
    stones[3] = Stone { pos: (131875812841558, 232138196193858, 0), vel: (206, 131, 0) };
    solve(&stones);

    // println!("{:?}", solve_for_xy(&a, &b, &c));

}

fn solve(stones: &[Stone]) {
    for ru in -1000..1000 {
        for rv in -1000..1000 {
            let xab = solve_for_xy(&stones[0], &stones[1], ru, rv);
            let xac = solve_for_xy(&stones[0], &stones[2], ru, rv);
            let xbd = solve_for_xy(&stones[1], &stones[3], ru, rv);
            if xab[0]*xac[1] == xac[0]*xab[1] &&
                xab[0]*xbd[1] == xbd[0]*xab[1] &&
                xab[2]*xac[3] == xac[2]*xab[3] &&
                xab[2]*xbd[3] == xbd[2]*xab[3]
                {
                    println!("{:?}", xab);
                }
        }
    }
}

fn solve_for_xy(a: &Stone, b: &Stone, ru: i128, rv: i128) -> [i128; 4] {
    let ax = a.pos.0 as i128;
    let ay = a.pos.1 as i128;
    let au = a.vel.0 as i128;
    let av = a.vel.1 as i128;
    let bx = b.pos.0 as i128;
    let by = b.pos.1 as i128;
    let bu = b.vel.0 as i128;
    let bv = b.vel.1 as i128;
    /*
    (rx,ry,rz,ru,rv,rw, ta,tb,...)
    (rx-ax)=ta*(au-ru)
    (ry-ay)=ta*(av-rv)

    (rx-ax)/(au-ru) = (ry-ay)/(av-rv) = (rz-az)/(aw-rw)

    (rx-ax)*(av-rv) = (ry-ay)*(au-ru)
    (rx-bx)*(bv-rv) = (ry-by)*(bu-ru)

    rx = (ry-ay)*(au-ru)/(av-rv) + ax
    (ry-ay)*(au-ru)*(bv-rv) + (ax-bx)*(av-rv)*(bv-rv) = (ry-by)*(bu-ru)*(av-rv)
    ry*(au-ru)*(bv-rv) - ay*(au-ru)*(bv-rv) + (ax-bx)*(av-rv)*(bv-rv) = ry*(bu-ru)*(av-rv) - by*(bu-ru)*(av-rv)
    ry*((au-ru)*(bv-rv)-(bu-ru)*(av-rv)) = ay*(au-ru)*(bv-rv) - by*(bu-ru)*(av-rv) - (ax-bx)*(av-rv)*(bv-rv)

     */
            let r1y_num = ay*(au-ru)*(bv-rv) - by*(bu-ru)*(av-rv) - (ax-bx)*(av-rv)*(bv-rv);
            let r1y_den = (au-ru)*(bv-rv)-(bu-ru)*(av-rv);

            let r1x_num = ax*(av-rv)*(bu-ru) - bx*(bv-rv)*(au-ru) - (ay-by)*(au-ru)*(bu-ru);
            let r1x_den = (av-rv)*(bu-ru)-(bv-rv)*(au-ru);

            [r1x_num, r1x_den, r1y_num, r1y_den]
}

struct Stone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

fn intersect_2d_inside(a: &Stone, b: &Stone, range: RangeInclusive<i64>) -> bool {
    let ax = a.pos.0 as i128;
    let ay = a.pos.1 as i128;
    let au = a.vel.0 as i128;
    let av = a.vel.1 as i128;
    let bx = b.pos.0 as i128;
    let by = b.pos.1 as i128;
    let bu = b.vel.0 as i128;
    let bv = b.vel.1 as i128;
    /*
        (x,y) on a iff (x-a.x,y-a.y) = k*(a.u,a.v)
                   iff a.v*(x-a.x) = a.u*(y-a.y)
                   (in the future if k > 0)

        a.v*(x-a.x) = a.u*(y-a.y)
        b.v*(x-b.x) = b.u*(y-b.y)

        x = a.u*(y-a.y)/a.v+a.x
        b.v*(a.u*(y-a.y)/a.v+a.x-b.x) = b.u*(y-b.y)
        a.u*(y-a.y)/a.v+a.x-b.x = (y-b.y)*b.u/b.v
        y*(a.u/a.v - b.u/b.v) = a.y*a.u/a.v - b.y*b.u/b.v + b.x-a.x
        y * (a.u*b.v - b.u*a.v) = a.y*a.u*b.v - b.y*b.u*a.v + b.x*a.v*b.v - a.x*a.v*b.v
        y = (a.y*a.u*b.v - b.y*b.u*a.v + b.x*a.v*b.v - a.x*a.v*b.v) / (a.u*b.v - b.u*a.v)
        x = (a.y*b.u*a.u - b.y*b.u*a.u + b.x*a.u*b.v - a.x*b.u*a.v) / (a.u*b.v - b.u*a.v)

        min <= x <= max
        min*|den| <= x.num*|den|/den <= max*|den|

        (x-a.x,y-a.y) has same sign as (a.u,a.v)
        (x-b.x,y-b.y) has same sign as (b.u,b.v)
           x.num*|den|/x.den - a.x*|den|
    */
    let den = au * bv - av * bu;
    let xnum = (ay - by)*au*bu + bx*au*bv - ax*av*bu;
    let ynum = (bx - ax)*av*bv + ay*au*bv - by*av*bu;
    if den == 0 {
        //println!("{}-{} are parallel: {}", a.pos.0, b.pos.0, xnum == 0 && ynum == 0);
        xnum == 0 && ynum == 0
    } else {
        let (min, max) = range.into_inner();
        let r = (min as i128) * den.abs() ..= (max as i128) * den.abs();
        if r.contains(&(xnum * den.signum())) && r.contains(&(ynum * den.signum())) {
            //println!("{}-{}: {}", a.pos.0, b.pos.0, t);
            let afuture = if au != 0 {
                (xnum * den.signum() - ax * den.abs()).signum() == au.signum()
            } else {
                (ynum * den.signum() - ay * den.abs()).signum() == av.signum()
            };
            let bfuture = if bu != 0 {
                (xnum * den.signum() - bx * den.abs()).signum() == bu.signum()
            } else {
                (ynum * den.signum() - by * den.abs()).signum() == bv.signum()
            };
            afuture && bfuture
        } else {
            //println!("{}-{} intersect outside", a.pos.0, b.pos.0);
            false
        }
    }
}

impl FromStr for Stone {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let mut pos = pos.split(", ").map(|s| s.parse().unwrap());
        let mut vel = vel.split(", ").map(|s| s.parse().unwrap());
        Ok(Stone {
            pos: (pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap()),
            vel: (vel.next().unwrap(), vel.next().unwrap(), vel.next().unwrap()),
        })
    }
}
