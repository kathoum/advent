type Vec3 = [i32; 3];
#[derive(Copy, Clone, Eq, PartialEq)]
struct Moon { pos: Vec3, vel: Vec3 }
type MoonSystem = Vec<Moon>;

fn gravity(m1: &Moon, m2: &Moon) -> Vec3 {
    let mut g: Vec3 = Default::default(); 
    for i in 0..3 {
        g[i] = (m2.pos[i] - m1.pos[i]).signum();
    }
    g
}

impl Moon {
    fn apply_gravity(&mut self, grav: &Vec3) {
        for i in 0..3 {
            self.vel[i] += grav[i];
        }
    }
    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }
    fn potential(&self) -> i32 {
        self.pos.iter().map(|x| x.abs()).sum()
    }
    fn kinetic(&self) -> i32 {
        self.vel.iter().map(|x| x.abs()).sum()
    }
    fn energy(&self) -> i32 {
        self.potential() * self.kinetic()
    }
}

fn step(sys: &mut MoonSystem) {
    for m1 in 0..sys.len() {
        for m2 in 0..sys.len() {
            let g = gravity(&sys[m1], &sys[m2]);
            sys[m1].apply_gravity(&g);
        }
    }
    for m in sys.iter_mut() {
        m.apply_velocity();
    }
}

fn find_projection_loop(sys: &MoonSystem, i: usize) -> usize {
    let pos = [sys[0].pos[i], sys[1].pos[i], sys[2].pos[i], sys[3].pos[i]];
    let vel = [sys[0].vel[i], sys[1].vel[i], sys[2].vel[i], sys[3].vel[i]];
    let [mut p1, mut p2, mut p3, mut p4] = pos;
    let [mut v1, mut v2, mut v3, mut v4] = vel;
    for counter in 1.. {
        v1 +=                    (p2-p1).signum() + (p3-p1).signum() + (p4-p1).signum();
        v2 += (p1-p2).signum() +                    (p3-p2).signum() + (p4-p2).signum();
        v3 += (p1-p3).signum() + (p2-p3).signum() +                    (p4-p3).signum();
        v4 += (p1-p4).signum() + (p2-p4).signum() + (p3-p4).signum();
        p1 += v1;
        p2 += v2;
        p3 += v3;
        p4 += v4;
        if pos == [p1, p2, p3, p4] && vel == [v1, v2, v3, v4] {
            return counter;
        }
    }
    panic!();
}

use std::ops::{Mul,Div,Rem};
fn gcd<T: Rem<Output = T> + Eq + Copy + Default>(a: T, b: T) -> T {
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm<T: Mul<Output = T> + Div<Output = T> + Rem<Output = T> + Eq + Copy + Default>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

fn find_loop(sys: &MoonSystem) -> usize {
    let mut period = [0; 3];
    for i in 0..3 {
        period[i] = find_projection_loop(&sys, i);
    }
    println!("periods = {:?}", period);
    period.iter().fold(1, |tot, &p| lcm(tot, p))
}

fn print(sys: &MoonSystem) {
    for m in sys.iter() {
        println!("pos {:3} {:3} {:3} vel {:3} {:3} {:3} pot {:4} kin {:4} total {}",
            m.pos[0], m.pos[1], m.pos[2],
            m.vel[0], m.vel[1], m.vel[2],
            m.potential(), m.kinetic(), m.energy());
    }
    println!("total energy {}", sys.iter().map(|m| m.energy()).sum::<i32>());
}

fn main() {
    {
        let mut example1 = vec![
            Moon { pos: [-1, 0, 2], vel: [0; 3] },
            Moon { pos: [2, -10, -7], vel: [0; 3] },
            Moon { pos: [4, -8, 8], vel: [0; 3] },
            Moon { pos: [3, 5, -1], vel: [0; 3] },
        ];
        let ex = example1.clone();
        for _ in 0..10 {
            step(&mut example1);
        }
        print(&example1);
        let c = find_loop(&ex);
        println!("Loop length {}", c);
    }
    {
        let mut example2 = vec![
            Moon { pos: [-8, -10, 0], vel: [0; 3] },
            Moon { pos: [5, 5, 10], vel: [0; 3] },
            Moon { pos: [2, -7, 3], vel: [0; 3] },
            Moon { pos: [9, -8, -3], vel: [0; 3] },
        ];
        let ex = example2.clone();
        for _ in 0..100 {
            step(&mut example2);
        }
        print(&example2);
        let c = find_loop(&ex);
        println!("Loop length {}", c);
    }
    {
        let mut jupiter = vec![
            Moon { pos: [-7, -8, 9], vel: [0; 3] },
            Moon { pos: [-12, -3, -4], vel: [0; 3] },
            Moon { pos: [6, -17, -9], vel: [0; 3] },
            Moon { pos: [4, -10, -6], vel: [0; 3] },
        ];
        let jc = jupiter.clone();
        for _ in 0..1000 {
            step(&mut jupiter);
        }
        print(&jupiter);
        let c = find_loop(&jc);
        println!("Loop length {}", c);
    }
}
