use std::collections::{HashMap,HashSet};

#[derive(Copy, Clone, Debug)]
enum Orientation { North, South, East, West }

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Cell(i32, i32);

#[derive(Copy, Clone, Debug)]
struct Pos {
    cell: Cell,
    orient: Orientation,
}

#[derive(Default)]
struct State {
    cell: Cell,
    scaffolds: HashSet<Cell>,
    start: Option<Pos>,
    instructions: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Move { Left, Right, Forward(i32) }

fn ahead_of(pos: &Pos) -> Cell {
    match pos.orient {
        Orientation::North => Cell(pos.cell.0 - 1, pos.cell.1),
        Orientation::South => Cell(pos.cell.0 + 1, pos.cell.1),
        Orientation::East => Cell(pos.cell.0, pos.cell.1 + 1),
        Orientation::West => Cell(pos.cell.0, pos.cell.1 - 1),
    }
}

fn left_of(pos: &Pos) -> Cell {
    ahead_of(&make_step(pos, Move::Left))
}

fn right_of(pos: &Pos) -> Cell {
    ahead_of(&make_step(pos, Move::Right))
}

fn make_step(pos: &Pos, step: Move) -> Pos {
    match step {
        Move::Left => Pos { cell: pos.cell, orient: match pos.orient {
            Orientation::North => Orientation::West,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
        }},
        Move::Right => Pos { cell: pos.cell, orient: match pos.orient {
            Orientation::North => Orientation::East,
            Orientation::South => Orientation::West,
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
        }},
        Move::Forward(_) => Pos { cell: ahead_of(&pos), orient: pos.orient }
    }
}

impl State {
    fn intersections(&self) -> Vec<Cell> {
        self.scaffolds.iter()
            .filter_map(|&Cell(row, col)|
                if self.scaffolds.contains(&Cell(row-1, col)) &&
                    self.scaffolds.contains(&Cell(row+1, col)) &&
                    self.scaffolds.contains(&Cell(row, col-1)) &&
                    self.scaffolds.contains(&Cell(row, col+1)) {
                    Some(Cell(row, col))
                } else {
                    None
                }
            )
            .collect()
    }

    fn full_path(&self) -> Vec<Move> {
        let mut position = match self.start {
            Some(pos) => pos,
            None => panic!("Missing starting position")
        };
        let mut path = Vec::new();
        loop {
            if self.scaffolds.contains(&ahead_of(&position)) {
                if let Some(Move::Forward(steps)) = path.last_mut() {
                    *steps += 1;
                } else {
                    path.push(Move::Forward(1));
                }
            } else if self.scaffolds.contains(&left_of(&position)) {
                path.push(Move::Left);
            } else if self.scaffolds.contains(&right_of(&position)) {
                path.push(Move::Right);
            } else {
                return path;
            }
            position = make_step(&position, *path.last().unwrap());
        }
    }
}

impl advent::intcode::State for State {
    fn input(&mut self) -> advent::intcode::Integer {
        if self.instructions.is_empty() {
            panic!("End of instructions");
        }
        let ch = self.instructions.remove(0);
        print!("{}", ch);
        ch as u8 as advent::intcode::Integer
    }

    fn output(&mut self, value: advent::intcode::Integer) {
        if value > 255 {
            println!("Output: {}", value);
            return;
        }
        let c = value as u8 as char;
        print!("{}", c);
        match c {
            '\n' => {
                if self.cell.1 == 0 {
                    // Pause after an empty line
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                self.cell.0 += 1;
                self.cell.1 = 0;
            }
            '#'|'<'|'>'|'^'|'v' => {
                if c != '#' {
                    /*if self.start.is_some() {
                        panic!("Multiple starting positions")
                    }*/
                    self.start = Some(Pos {
                        cell: self.cell,
                        orient: match c {
                            '<' => Orientation::West,
                            '>' => Orientation::East,
                            '^' => Orientation::North,
                            'v' => Orientation::South,
                            _ => unreachable!()
                        }
                    });
                }
                self.scaffolds.insert(self.cell);
                self.cell.1 += 1;
            }
            _ => {
                self.cell.1 += 1;
            }
        }
    }
}

fn slice_contains<T: Eq>(a: &[T], b: &[T]) -> bool {
    a.windows(b.len()).any(|sub| sub == b)
}

fn repeated_patterns<T: Eq + std::hash::Hash>(path: &[T]) -> HashMap<&[T], usize> {
    let mut result = HashMap::new();
    for a in (0..path.len()-2).step_by(2) {
        for b in (a+2..path.len()).step_by(2) {
            let pattern = &path[a..b];
            if !result.contains_key(pattern) {
                let repeat_count = path.windows(pattern.len()).filter(|p| p == &pattern).count();
                if repeat_count > 1 {
                    result.insert(pattern, repeat_count);
                }
            }
        }
    }
    let is_dominated = |(pattern, count): (&[T], usize)|
        result.iter().any(|(p, &c)| c >= count && p.len() > pattern.len() && slice_contains(p, pattern));
    result.iter().map(|(&p, &c)| (p, c)).filter(|&it| !is_dominated(it)).collect()
}

fn encode(path: &[Move]) -> String {
    path.iter().map(|&m| match m {
        Move::Left => "L".to_string(),
        Move::Right => "R".to_string(),
        Move::Forward(n) => n.to_string(),
    }).collect::<Vec<String>>().join(",")
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input17.txt"));
    let program = advent::intcode::read_program(reader).unwrap();
    let mut state = State::default();
    {
        let mut program = program.clone();
        advent::intcode::run_program(&mut program, &mut state).unwrap();
    }
    let align_params: i32 = state.intersections().into_iter().map(|Cell(r,c)| r*c).sum();
    println!("Total alignment parameters: {}\n", align_params);

    let path = state.full_path();
    println!("Movements: total {}", path.len());
    println!("{}", encode(&path));

    let rep = repeated_patterns(&path);
    for (s, c) in rep.iter() {
        println!("{} repetitions for (length {}) {}", c, s.len(), encode(&s));
    }

    let path = encode(&path);
    let dictionary = &["L,6,L,12,R,12,L,4,", "R,12,L,10,L,10,", "L,12,R,12,L,6,"];
    let compressed = (path + ",")
        .replace(dictionary[0], "A,")
        .replace(dictionary[1], "B,")
        .replace(dictionary[2], "C,");
    println!("Compressed path: {}", compressed);

    let mut state = State::default();
    state.instructions = compressed;
    state.instructions.pop();
    state.instructions.push('\n');
    for d in dictionary {
        state.instructions += d;
        state.instructions.pop();
        state.instructions.push('\n');
    }
    state.instructions += "n\n";
    {
        let mut program = program.clone();
        program[0] = 2;
        advent::intcode::run_program(&mut program, &mut state).unwrap();
    }
}
