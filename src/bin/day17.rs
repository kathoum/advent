type Pos = (i32, i32, i32);
struct Board {
    size: Pos,
    data: Vec<bool>,
}

impl Board {
    fn new(size: Pos) -> Self {
        Board {
            size: size,
            data: vec![false; (size.0 * size.1 * size.2) as usize]
        }
    }

    fn contains(&self, index: Pos) -> bool {
        (0..self.size.0).contains(&index.0)
        && (0..self.size.1).contains(&index.1)
        && (0..self.size.2).contains(&index.2)
    }

    fn is_true(&self, index: Pos) -> bool {
        self.contains(index) && self[index]
    }

    fn count_true(&self) -> usize {
        self.data.iter().filter(|x| **x).count()
    }

    fn count_neighbours(&self, index: Pos) -> u32 {
        let mut count = 0;
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    let i = (index.0 + x, index.1 + y, index.2 + z);
                    if (x, y, z) != (0, 0, 0) && self.is_true(i) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    
    fn conway_step(&self) -> Self {
        let mut output = Self::new((self.size.0 + 2, self.size.1 + 2, self.size.2 + 2));
        for z in 0..output.size.2 {
            for y in 0..output.size.1 {
                for x in 0..output.size.0 {
                    let center = (x - 1, y - 1, z - 1);
                    let c = self.count_neighbours(center);
                    output[(x, y, z)] = c == 3 || (c == 2 && self.is_true(center));
                }
            }
        }
        output
    }
}

impl std::ops::Index<Pos> for Board {
    type Output = bool;
    fn index(&self, index: Pos) -> &bool {
        assert_eq!(true, self.contains(index));
        &self.data[(index.0 + self.size.0 * (index.1 + self.size.1 * index.2)) as usize]
    }
}

impl std::ops::IndexMut<Pos> for Board {
    fn index_mut(&mut self, index: Pos) -> &mut bool {
        assert_eq!(true, self.contains(index));
        self.data.index_mut((index.0 + self.size.0 * (index.1 + self.size.1 * index.2)) as usize)
    }
}

fn main() {
    let input = include_str!("input17.txt");

    println!("Step One");
    let ny = input.lines().count();
    let nx = input.lines().next().unwrap().len();
    println!("Initial size: {}x{}x1", nx, ny);
    let mut board = Board::new((nx as i32, ny as i32, 1));
    let mut i = 0;
    for c in input.chars() {
        match c {
            '.' => { board.data[i] = false; i += 1 },
            '#' => { board.data[i] = true; i += 1 },
            _ => ()
        }
    }
    assert_eq!(i, board.data.len());

    println!("Step 0: {} live cubes", board.count_true());
    for step in 1..=6 {
        board = board.conway_step();
        println!("Step {}: {} live cubes", step, board.count_true());
    }

    println!("Step Two");
    let mut board = Board4::new((nx as i32, ny as i32, 1, 1));
    let mut i = 0;
    for c in input.chars() {
        match c {
            '.' => { board.data[i] = false; i += 1 },
            '#' => { board.data[i] = true; i += 1 },
            _ => ()
        }
    }
    assert_eq!(i, board.data.len());

    println!("Step 0: {} live cubes", board.count_true());
    for step in 1..=6 {
        board = board.conway_step();
        println!("Step {}: {} live cubes", step, board.count_true());
    }
}

type Pos4 = (i32, i32, i32, i32);
struct Board4 {
    size: Pos4,
    data: Vec<bool>,
}

impl Board4 {
    fn new(size: Pos4) -> Self {
        Board4 {
            size: size,
            data: vec![false; (size.0 * size.1 * size.2 * size.3) as usize]
        }
    }

    fn contains(&self, index: Pos4) -> bool {
        (0..self.size.0).contains(&index.0)
        && (0..self.size.1).contains(&index.1)
        && (0..self.size.2).contains(&index.2)
        && (0..self.size.3).contains(&index.3)
    }

    fn is_true(&self, index: Pos4) -> bool {
        self.contains(index) && self[index]
    }

    fn count_true(&self) -> usize {
        self.data.iter().filter(|x| **x).count()
    }

    fn count_neighbours(&self, index: Pos4) -> u32 {
        let mut count = 0;
        for w in -1..=1 {
            for z in -1..=1 {
                for y in -1..=1 {
                    for x in -1..=1 {
                        let i = (index.0 + x, index.1 + y, index.2 + z, index.3 + w);
                        if (x, y, z, w) != (0, 0, 0, 0) && self.is_true(i) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
    
    fn conway_step(&self) -> Self {
        let mut output = Self::new((self.size.0 + 2, self.size.1 + 2, self.size.2 + 2, self.size.3 + 2));
        for w in 0..output.size.3 {
            for z in 0..output.size.2 {
                for y in 0..output.size.1 {
                    for x in 0..output.size.0 {
                        let center = (x - 1, y - 1, z - 1, w - 1);
                        let c = self.count_neighbours(center);
                        output[(x, y, z, w)] = c == 3 || (c == 2 && self.is_true(center));
                    }
                }
            }
        }
        output
    }
}

impl std::ops::Index<Pos4> for Board4 {
    type Output = bool;
    fn index(&self, index: Pos4) -> &bool {
        assert_eq!(true, self.contains(index));
        &self.data[(index.0 + self.size.0 * (index.1 + self.size.1 * (index.2 + self.size.2 * index.3))) as usize]
    }
}

impl std::ops::IndexMut<Pos4> for Board4 {
    fn index_mut(&mut self, index: Pos4) -> &mut bool {
        assert_eq!(true, self.contains(index));
        self.data.index_mut((index.0 + self.size.0 * (index.1 + self.size.1 * (index.2 + self.size.2 * index.3))) as usize)
    }
}
