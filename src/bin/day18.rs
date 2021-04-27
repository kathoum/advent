fn main() {
    let input = include_str!("input18.txt");

    let mut field = Field::parse(input);
    for _ in 0..100 {
        field = field.step();
    }
    println!("Step one: {} lights are on", field.count_on());

    let mut field = FieldWithCornersOn::parse(input);
    for _ in 0..100 {
        field = field.step();
    }
    println!("Step two: {} lights are on", field.count_on());
}

#[derive(PartialEq)]
enum Cell { Off, On }
struct Field(Vec<Vec<Cell>>);

impl Field {
    fn step(&self) -> Field {
        Field(self.0.iter().enumerate().map(|(i,line)| {
            line.iter().enumerate().map(|(j,cell)| {
                let count = self.neighbours(i, j);
                match (cell, count) {
                    (Cell::On, 2) | (_, 3) => Cell::On,
                    _ => Cell::Off,
                }
            }).collect()
        }).collect())
    }

    fn parse(input: &str) -> Field {
        Field(input.lines().map(|s| s.chars().map(|c|
            match c {
                '.' => Cell::Off,
                '#' => Cell::On,
                _ => panic!("Unexpeted char {}", c)
            }
        ).collect()).collect())
    }

    fn neighbours(&self, i: usize, j: usize) -> usize {
        let mut count = 0;
        for x in i.saturating_sub(1) .. self.0.len().min(i+2) {
            let line = &self.0[x];
            for y in j.saturating_sub(1) .. line.len().min(j+2) {
                if (i, j) != (x, y) && line[y] == Cell::On {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_on(&self) -> usize {
        self.0.iter().map(|line| line.iter().filter(|c| **c == Cell::On).count()).sum()
    }
}

struct FieldWithCornersOn(Field);

impl std::ops::Deref for FieldWithCornersOn {
    type Target = Field;
    fn deref(&self) -> &Field { &self.0 }
}

impl FieldWithCornersOn {
    fn parse(input: &str) -> FieldWithCornersOn {
        let mut f = FieldWithCornersOn(Field::parse(input));
        f.turn_corners_on();
        f
    }

    fn step(&self) -> FieldWithCornersOn {
        let mut f = FieldWithCornersOn(self.0.step());
        f.turn_corners_on();
        f
    }

    fn turn_corners_on(&mut self) {
        *self.0.0.first_mut().unwrap().first_mut().unwrap() = Cell::On;
        *self.0.0.first_mut().unwrap().last_mut().unwrap() = Cell::On;
        *self.0.0.last_mut().unwrap().first_mut().unwrap() = Cell::On;
        *self.0.0.last_mut().unwrap().last_mut().unwrap() = Cell::On;
    }
}
