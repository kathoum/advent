#[derive(Default)]
struct State {
    row: i32,
    col: i32,
    scaffolds: std::collections::HashSet<(i32, i32)>,
}

impl State {
    fn intersections(&self) -> Vec<(i32, i32)> {
        self.scaffolds.iter()
            .filter_map(|&(row, col)|
                if self.scaffolds.contains(&(row-1, col)) &&
                    self.scaffolds.contains(&(row+1, col)) &&
                    self.scaffolds.contains(&(row, col-1)) &&
                    self.scaffolds.contains(&(row, col+1)) {
                    Some((row, col))
                } else {
                    None
                }
            )
            .collect()
    }
}

impl advent::intcode::State for State {
    fn input(&mut self) -> advent::intcode::Integer {
        panic!()
    }
    fn output(&mut self, value: advent::intcode::Integer) {
        //print!("{}", value as u8 as char);
        match value as u8 as char {
            '\n' => {
                self.row += 1;
                self.col = 0;
            }
            '#'|'<'|'>'|'^'|'v' => {
                self.scaffolds.insert((self.row, self.col));
                self.col += 1;
            }
            _ => {
                self.col += 1;
            }
        }
    }
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input17.txt"));
    let mut program = advent::intcode::read_program(reader).unwrap();
    let mut state = State::default();
    advent::intcode::run_program(&mut program, &mut state).unwrap();
    //println!("{:?}", state.intersections());
    let align_params: i32 = state.intersections().into_iter().map(|(r,c)| r*c).sum();
    println!("{}", align_params);
}
