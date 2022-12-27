use std::fs::File;
use std::io::{BufRead,BufReader};

struct Snafu(Vec<u8>);

fn main() {
    let reader = BufReader::new(File::open("input/day25.txt").unwrap());

    let total: Snafu = reader.lines().map(Result::unwrap).map(String::into_bytes).map(Snafu::from).sum();
    println!("The total fuel is {total}");
}

impl Snafu {
    pub fn from(mut v: Vec<u8>) -> Self {
        v.reverse();
        assert!(!v.is_empty());
        assert!(v.iter().all(|b| b"012-=".contains(b)));
        assert!(v.len() == 1 || v.last().unwrap() != &b'0');
        Snafu(v)
    }
}

impl std::ops::AddAssign<Snafu> for Snafu {
    fn add_assign(&mut self, rhs: Snafu) {
        let mut carry = 0;
        let mut i = 0;
        while i < rhs.0.len() || carry != 0 {
            if self.0.len() <= i {
                self.0.push(b'0');
            }
            let x = self.0[i];
            let y = rhs.0.get(i).cloned().unwrap_or(b'0');
            let mut n = from_digit_5(x) + from_digit_5(y) + carry;
            carry = 0;
            while n > 2 { n -= 5; carry += 1; }
            while n < -2 { n += 5; carry -= 1; }
            self.0[i] = to_digit_5(n);
            i += 1;
        }
        while self.0.len() > 1 && self.0.last().unwrap() == &b'0' {
            self.0.pop();
        }
    }
}

impl std::iter::Sum<Snafu> for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        let mut result = Snafu::from(vec![b'0']);
        for n in iter {
            result += n;
        }
        result
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.iter().rev() {
            write!(f, "{}", char::from(*c))?;
        }
        Ok(())
    }
}

fn from_digit_5(i: u8) -> i32 {
    match i {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => panic!()
    }
}

fn to_digit_5(i: i32) -> u8 {
    match i {
        2 => b'2',
        1 => b'1',
        0 => b'0',
        -1 => b'-',
        -2 => b'=',
        _ => panic!()
    }
}
