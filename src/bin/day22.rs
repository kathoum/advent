use std::io::BufRead;

#[derive(Copy, Clone)]
enum Technique {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(i64),
    Any(i64, i64)
}

impl std::str::FromStr for Technique {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Technique, Self::Err> {
        if s == "deal into new stack" {
            Ok(Technique::DealIntoNewStack)
        }
        else if s.starts_with("cut ") {
            let n = s.trim_start_matches("cut ");
            Ok(Technique::Cut(n.parse()?))
        }
        else if s.starts_with("deal with increment ") {
            let n = s.trim_start_matches("deal with increment ");
            Ok(Technique::DealWithIncrement(n.parse()?))
        } else {
            Err("".parse::<i32>().unwrap_err())
        }
    }
}

fn next_position(size: i64, tech: &Technique, position: i64) -> i64 {
    assert!(0 <= position && position < size);
    match tech {
        Technique::DealIntoNewStack => size - 1 - position,
        Technique::Cut(step) => (position - step).rem_euclid(size),
        Technique::DealWithIncrement(incr) => (position * incr).rem_euclid(size),
        Technique::Any(mul, add) => (position * mul + add).rem_euclid(size)
    }
}

fn final_position(size: i64, moves: &[Technique], position: i64) -> i64 {
    moves.iter().fold(position, |pos, tech| next_position(size, tech, pos))
}

fn final_deck(size: i64, moves: &[Technique]) -> Vec<i64> {
    let mut result = vec![-1; size as usize];
    for i in 0..size {
        let j = final_position(size, moves, i);
        result[j as usize] = i;
    }
    result
}

impl Technique {
    fn coeff(&self) -> (i64, i64) {
        match self {
            Technique::DealIntoNewStack => (-1, -1),
            Technique::Cut(n) => (1, -n),
            Technique::DealWithIncrement(n) => (*n, 0),
            Technique::Any(a, b) => (*a, *b)
        }
    }

    fn compose(&self, next: &Technique, size: i64) -> Technique {
        let (xa, xb) = self.coeff();
        let (ya, yb) = next.coeff();
        let a = fma_mod(xa, ya, 0, size);
        let b = fma_mod(xb, ya, yb, size);
        Technique::Any(a, b)
    }

    fn power(&self, times: usize, size: i64) -> Technique {
        if times == 1 {
            *self
        } else {
            let rec = self.compose(&self, size).power(times / 2, size);
            if times % 2 == 0 {
                rec
            } else {
                self.compose(&rec, size)
            }
        }
    }

    fn inverse(&self, size: i64) -> Technique {
        let (a, b) = self.coeff();
        let (gcd, m, n) = extended_gcd(a, size);
        assert_eq!(gcd, 1);
        assert_eq!((m as i128) * (a as i128) + (n as i128) * (size as i128), 1);
        Technique::Any(m, fma_mod(-m, b, 0, size))
    }
}

/// Returns (a*b+c)%m, without overflows
fn fma_mod(a: i64, b: i64, c: i64, modulus: i64) -> i64 {
    ((a as i128) * (b as i128) + (c as i128)).rem_euclid(modulus as i128) as i64
}

/// Returns a triple such that: extgcd.0 = extgcd.1 * a + extgcd.2 * b
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, m, n) = extended_gcd(b, a % b);
        (gcd, n, m - n * (a / b))
    }
}

fn compress(size: i64, moves: &[Technique]) -> Technique {
    moves.iter().fold(Technique::Cut(0), |curr, next| curr.compose(next, size))
}

fn main() {
    let moves = [
        Technique::DealWithIncrement(7),
        Technique::DealIntoNewStack,
        Technique::DealIntoNewStack
    ];
    let test = final_deck(10, &moves);
    assert_eq!(test.as_slice(), &[0,3,6,9,2,5,8,1,4,7]);
    let test = final_deck(10, &[compress(10, &moves)]);
    assert_eq!(test.as_slice(), &[0,3,6,9,2,5,8,1,4,7]);

    let moves = [
        Technique::Cut(6),
        Technique::DealWithIncrement(7),
        Technique::DealIntoNewStack
    ];
    let test = final_deck(10, &moves);
    assert_eq!(test.as_slice(), &[3,0,7,4,1,8,5,2,9,6]);
    let test = final_deck(10, &[compress(10, &moves)]);
    assert_eq!(test.as_slice(), &[3,0,7,4,1,8,5,2,9,6]);

    let moves = [
        Technique::DealWithIncrement(7),
        Technique::DealWithIncrement(9),
        Technique::Cut(-2)
    ];
    let test = final_deck(10, &moves);
    assert_eq!(test.as_slice(), &[6,3,0,7,4,1,8,5,2,9]);
    let test = final_deck(10, &[compress(10, &moves)]);
    assert_eq!(test.as_slice(), &[6,3,0,7,4,1,8,5,2,9]);

    let moves = [
        Technique::DealIntoNewStack,
        Technique::Cut(-2),
        Technique::DealWithIncrement(7),
        Technique::Cut(8),
        Technique::Cut(-4),
        Technique::DealWithIncrement(7),
        Technique::Cut(3),
        Technique::DealWithIncrement(9),
        Technique::DealWithIncrement(3),
        Technique::Cut(-1)
    ];
    let test = final_deck(10, &moves);
    assert_eq!(test.as_slice(), &[9,2,5,8,1,4,7,0,3,6]);
    let test = final_deck(10, &[compress(10, &moves)]);
    assert_eq!(test.as_slice(), &[9,2,5,8,1,4,7,0,3,6]);

    let reader = std::io::Cursor::new(include_str!("input22.txt"));
    let moves: Vec<_> = reader.lines().map(|line| line.unwrap().parse::<Technique>().unwrap()).collect();

    let deck_size = 10007;
    let start_position = 2019;
    let p1 = final_position(deck_size, &moves, start_position);
    let composition = compress(deck_size, &moves);
    let p2 = next_position(deck_size, &composition, start_position);
    println!("Final position of card {} is {}", start_position, p1);
    println!("Final position of card {} is {}", start_position, p2);

    let deck_size = 119315717514047;
    let iterations = 101741582076661;
    let end_position = 2020;
    let composition = compress(deck_size, &moves);
    let repetition = composition.power(iterations, deck_size);
    let inverse = repetition.inverse(deck_size);
    let p3 = next_position(deck_size, &inverse, end_position);
    println!("Card {} ends in position {}", p3, end_position);
}
