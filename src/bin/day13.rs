use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<Vec<u8>> = BufReader::new(File::open("input/day13.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect();
    let mut blocks: Vec<Block> = lines
        .split(Vec::is_empty)
        .map(|b| Block(b.to_owned()))
        .collect();

    let answer: usize = blocks.iter().map(Block::reflect).sum();
    println!("Day 13 Part One: {answer}");

    let answer: usize = blocks.iter_mut().map(Block::smudge).sum();
    println!("Day 13 Part Two: {answer}");
}

struct Block(Vec<Vec<u8>>);

impl Block {
    fn col(&self, i: usize) -> impl Iterator<Item = u8> + '_ {
        self.0.iter().map(move |r| r[i])
    }

    fn hor_reflect_except(&self, e: Option<usize>) -> Option<usize> {
        let r = self.0.len();
        for i in 0..r - 1 {
            if e != Some(i + 1) && self.0[i] == self.0[i + 1] {
                let n1 = i + 1;
                let n2 = r - i - 1;
                if (1..n1.min(n2)).all(|j| self.0[i - j] == self.0[i + j + 1]) {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn ver_reflect_except(&self, e: Option<usize>) -> Option<usize> {
        let c = self.0[0].len();
        for i in 0..c - 1 {
            if e != Some(i + 1) && self.col(i).eq(self.col(i + 1)) {
                let n1 = i + 1;
                let n2 = c - i - 1;
                if (1..n1.min(n2)).all(|j| self.col(i - j).eq(self.col(i + j + 1))) {
                    return Some(i + 1);
                }
            }
        }
        None
    }

    fn reflect(&self) -> usize {
        if let Some(r) = self.hor_reflect_except(None) {
            r * 100
        } else {
            self.ver_reflect_except(None).unwrap()
        }
    }

    fn reflect_except(&self, h: Option<usize>, v: Option<usize>) -> Option<usize> {
        if let Some(r) = self.hor_reflect_except(h) {
            Some(r * 100)
        } else {
            self.ver_reflect_except(v)
        }
    }

    fn smudge(&mut self) -> usize {
        let a = self.reflect();
        let h = (a >= 100).then_some(a / 100);
        let v = (a < 100).then_some(a);
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                let x = self.0[i][j];
                self.0[i][j] = if x == b'.' { b'#' } else { b'.' };
                let ans = self.reflect_except(h, v);
                self.0[i][j] = x;
                if let Some(ans) = ans {
                    return ans;
                }
            }
        }
        panic!()
    }
}
