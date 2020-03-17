#[derive(Copy, Clone)]
struct Eris(u32);

impl Eris {
    fn get(self, row: u32, col: u32) -> bool {
        ((self.0 >> (col + row * 5)) & 1) != 0
    }

    fn set(&mut self, row: u32, col: u32) {
        self.0 |= 1 << (col + row * 5);
    }

    fn clear(&mut self, row: u32, col: u32) {
        self.0 &= !(1 << (col + row * 5));
    }

    fn biodiversity(self) -> u32 {
        self.0
    }

    fn count_adjacent(self, row: u32, col: u32) -> u32 {
        let a = row > 0 && self.get(row-1, col);
        let b = row < 4 && self.get(row+1, col);
        let c = col > 0 && self.get(row, col-1);
        let d = col < 4 && self.get(row, col+1);
        (a as u32) + (b as u32) + (c as u32) + (d as u32)
    }

    fn next(self) -> Eris {
        let mut eris = Eris(0);
        for row in 0..5 {
            for col in 0..5 {
                let n = self.count_adjacent(row, col);
                if n == 1 || (n == 2 && !self.get(row, col)) {
                    eris.set(row, col)
                } else {
                    eris.clear(row, col)
                }
            }
        }
        eris
    }
}

impl std::fmt::Display for Eris {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                write!(f, "{}", if self.get(row, col) { '#' } else { '.' })?;
            }
            if row != 4 { writeln!(f, "")?; }
        }
        Ok(())
    }
}

impl std::str::FromStr for Eris {
    type Err = String;
    fn from_str(s: &str) -> Result<Eris, Self::Err> {
        let mut n = 0;
        let mut pos = 0;
        for c in s.chars() {
            match c {
                '.' => { pos += 1; }
                '#' => { n |= 1 << pos; pos += 1; }
                '\n' => { }
                _ => Err("Invalid input string. Allowed characters are '#' and '.'")?
            }
        }
        Ok(Eris(n))
    }
}

#[derive(Default)]
struct Eris2 {
    positive: Vec<u32>,
    negative: Vec<u32>
}

impl Eris2 {
    fn levels(&self) -> std::ops::Range<i32> {
        std::ops::Range {
            start: -(self.negative.len() as i32),
            end: self.positive.len() as i32
        }
    }

    fn levels_adjacent(&self) -> std::ops::Range<i32> {
        let levels = self.levels();
        std::ops::Range {
            start: levels.start - 1,
            end: levels.end + 1
        }
    }

    fn get(&self, lev: i32, row: u32, col: u32) -> bool {
        assert!(row != 2 || col != 2);
        let n = if lev >= 0 {
            self.positive.get(lev as usize).unwrap_or(&0)
        } else {
            self.negative.get((-lev-1) as usize).unwrap_or(&0)
        };
        ((n >> (col + row * 5)) & 1) != 0
    }

    fn set(&mut self, lev: i32, row: u32, col: u32) {
        assert!(row != 2 || col != 2);
        let n = if lev >= 0 {
            let idx = lev as usize;
            if idx >= self.positive.len() { self.positive.resize(idx + 1, 0); }
            self.positive.get_mut(idx).unwrap()
        } else {
            let idx = (-lev-1) as usize;
            if idx >= self.negative.len() { self.negative.resize(idx + 1, 0); }
            self.negative.get_mut(idx).unwrap()
        };
        *n |= 1 << (col + row * 5);
    }

    fn clear(&mut self, lev: i32, row: u32, col: u32) {
        assert!(row != 2 || col != 2);
        let n = if lev >= 0 {
            let idx = lev as usize;
            if idx >= self.positive.len() { return; }
            self.positive.get_mut(idx).unwrap()
        } else {
            let idx = (-lev-1) as usize;
            if idx >= self.negative.len() { return; }
            self.negative.get_mut(idx).unwrap()
        };
        *n &= !(1 << (col + row * 5));
    }

    fn count_adjacent(&self, lev: i32, row: u32, col: u32) -> u32 {
        assert!(row < 5 && col < 5 && (row != 2 || col != 2));
        let mut count = 0;
    
        if row == 0 {
            count += self.get(lev-1, 1, 2) as u32;
        } else if row == 3 && col == 2 {
            for c in 0..5 {
                count += self.get(lev+1, 4, c) as u32;
            }
        } else {
            count += self.get(lev, row-1, col) as u32;
        }

        if row == 4 {
            count += self.get(lev-1, 3, 2) as u32;
        } else if row == 1 && col == 2 {
            for c in 0..5 {
                count += self.get(lev+1, 0, c) as u32;
            }
        } else {
            count += self.get(lev, row+1, col) as u32;
        }

        if col == 0 {
            count += self.get(lev-1, 2, 1) as u32;
        } else if row == 2 && col == 3 {
            for r in 0..5 {
                count += self.get(lev+1, r, 4) as u32;
            }
        } else {
            count += self.get(lev, row, col-1) as u32;
        }

        if col == 4 {
            count += self.get(lev-1, 2, 3) as u32;
        } else if row == 2 && col == 1 {
            for r in 0..5 {
                count += self.get(lev+1, r, 0) as u32;
            }
        } else {
            count += self.get(lev, row, col+1) as u32;
        }

        count
    }

    fn next(&self) -> Eris2 {
        let mut eris = Eris2::default();
        for lev in self.levels_adjacent() {
            for row in 0..5 {
                for col in 0..5 {
                    if row != 2 || col != 2 {
                        let n = self.count_adjacent(lev, row, col);
                        if n == 1 || (n == 2 && !self.get(lev, row, col)) {
                            eris.set(lev, row, col)
                        } else {
                            eris.clear(lev, row, col)
                        }
                    }
                }
            }
        }
        while eris.positive.last() == Some(&0) { eris.positive.pop(); }
        while eris.negative.last() == Some(&0) { eris.negative.pop(); }
        eris
    }

    fn total(&self) -> u32 {
        self.negative.iter().chain(self.positive.iter())
            .map(|n| n.count_ones())
            .sum()
    }
}

impl std::fmt::Display for Eris2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for lev in self.levels() {
            writeln!(f, "Depth {}", lev)?;
            for row in 0..5 {
                for col in 0..5 {
                    write!(f, "{}", if row == 2 && col == 2 { '?' } else if self.get(lev, row, col) { '#' } else { '.' })?;
                }
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("input24.txt");
    let eris: Eris = input.parse().unwrap();
    println!("Start:\n{}", eris);

    let mut seen = std::collections::HashMap::new();
    let mut eris = eris;
    for iteration in 0.. {
        let n = eris.biodiversity();
        if seen.contains_key(&n) {
            println!("Seen {} at iteration {} and {}", n, seen[&n], iteration);
            println!("{}", eris);
            break;
        } else {
            seen.insert(n, iteration);
        }
        eris = eris.next();
    }

    let mut eris2 = Eris2 {
        positive: vec![input.parse::<Eris>().unwrap().biodiversity()],
        negative: Vec::new()
    };
    for _ in 0..200 {
        eris2 = eris2.next();
    }
    println!("After 200 minutes, the total is {}", eris2.total());
}
