use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Name([u8; 4]);

enum Expression {
    Number(i64),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
}

fn main() {
    let reader = BufReader::new(File::open("input/day21.txt").unwrap());
    let monkeys: HashMap<Name, Expression> = reader.lines().map(|l| {
        let line = l.unwrap();
        let (name, expr) = line.split_once(": ").unwrap();
        (name.parse().unwrap(), expr.parse().unwrap())
    }).collect();

    let mut numbers: HashMap<Name, i64> = HashMap::new();
    let root = find_number(&monkeys, &mut numbers, Name(*b"root"));
    println!("The monkey yells {root}");

    let mut equations: HashMap<Name, LinFun> = HashMap::new();
    equations.insert(Name(*b"humn"), LinFun { a: 1.into(), b: 0.into() });
    let (name1, name2) = match monkeys[&Name(*b"root")] {
        Expression::Add(n1, n2) => (n1, n2),
        _ => panic!()
    };
    let part1 = find_number::<LinFun>(&monkeys, &mut equations, name1);
    let part2 = find_number::<LinFun>(&monkeys, &mut equations, name2);
    // solve part1 == part2
    let solution = (part2.b - part1.b) / (part1.a - part2.a);
    assert_eq!(solution.den, 1);

    println!("The human yells {}", solution.num);
}

fn find_number<T>(monkeys: &HashMap<Name, Expression>, numbers: &mut HashMap<Name, T>, name: Name) -> T
where T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>, i64: std::convert::Into<T>
{
    if let Some(&n) = numbers.get(&name) { n }
    else {
        let n: T = match monkeys[&name] {
            Expression::Number(n) => n.into(),
            Expression::Add(a, b) => {
                let x = find_number(monkeys, numbers, a);
                let y = find_number(monkeys, numbers, b);
                x + y
            }
            Expression::Sub(a, b) => {
                let x = find_number(monkeys, numbers, a);
                let y = find_number(monkeys, numbers, b);
                x - y
            }
            Expression::Mul(a, b) => {
                let x = find_number(monkeys, numbers, a);
                let y = find_number(monkeys, numbers, b);
                x * y
            }
            Expression::Div(a, b) => {
                let x = find_number(monkeys, numbers, a);
                let y = find_number(monkeys, numbers, b);
                x / y
            }
        };
        numbers.insert(name, n);
        n
    }
}

impl std::str::FromStr for Name {
    type Err = std::array::TryFromSliceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Name(s.as_bytes().try_into()?))
    }
}

impl std::str::FromStr for Expression {
    type Err = std::array::TryFromSliceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(" + ") {
            Ok(Expression::Add(a.parse()?, b.parse()?))
        } else if let Some((a, b)) = s.split_once(" - ") {
            Ok(Expression::Sub(a.parse()?, b.parse()?))
        } else if let Some((a, b)) = s.split_once(" * ") {
            Ok(Expression::Mul(a.parse()?, b.parse()?))
        } else if let Some((a, b)) = s.split_once(" / ") {
            Ok(Expression::Div(a.parse()?, b.parse()?))
        } else {
            Ok(Expression::Number(s.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Q { num: i64, den: i64 }

impl std::convert::From<i64> for Q {
    fn from(n: i64) -> Self {
        Q { num: n, den: 1 }
    }
}
impl Add<Q> for Q {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Q { num: self.den * rhs.num + self.num * rhs.den, den: self.den * rhs.den }.reduce()
    }
}
impl Sub<Q> for Q {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Q { num: self.num * rhs.den - self.den * rhs.num, den: self.den * rhs.den }.reduce()
    }
}
impl Mul<Q> for Q {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Q { num: self.num * rhs.num, den: self.den * rhs.den }.reduce()
    }
}
impl Div<Q> for Q {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs.num, 0);
        Q { num: self.num * rhs.den, den: self.den * rhs.num }.reduce()
    }
}
impl Q {
    pub fn reduce(self) -> Self {
        let n = gcd(self.num, self.den);
        let n = n.abs() * self.den.signum();
        Q { num: self.num / n, den: self.den / n }
    }
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

#[derive(Debug, Clone, Copy)]
struct LinFun { a: Q, b: Q }

impl std::convert::From<i64> for LinFun {
    fn from(n: i64) -> Self {
        LinFun { a: 0.into(), b: n.into() }
    }
}
impl Add<LinFun> for LinFun {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        LinFun { a: self.a + rhs.a, b: self.b + rhs.b }
    }
}
impl Sub<LinFun> for LinFun {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        LinFun { a: self.a - rhs.a, b: self.b - rhs.b }
    }
}
impl Mul<LinFun> for LinFun {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.a * rhs.a, 0.into());
        LinFun { a: self.a * rhs.b + self.b * rhs.a, b: self.b * rhs.b }
    }
}
impl Div<LinFun> for LinFun {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(rhs.a, 0.into());
        LinFun { a: self.a / rhs.b, b: self.b / rhs.b }
    }
}
