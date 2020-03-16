struct State {
    script: String,
    input_pos: usize,
}
impl State {
    fn new(script: &str) -> State {
        State {
            script: script.to_string(),
            input_pos: 0,
        }
    }
}

impl advent::intcode::State for State {
    fn input(&mut self) -> advent::intcode::Integer {
        let byte = self.script.as_bytes()[self.input_pos];
        print!("{}", byte as char);
        self.input_pos += 1;
        byte as advent::intcode::Integer
    }
    fn output(&mut self, value: advent::intcode::Integer) {
        if value > 255 {
            println!("Output: {}", value);
        } else {
            let c = value as u8 as char;
            print!("{}", c);
        }
    }
}

fn main() {
    let input = include_str!("input21.txt");
    let mut program = advent::intcode::read_program(std::io::Cursor::new(input)).unwrap();
    let mut state = State::new(
"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
");
    advent::intcode::run_program(&mut program, &mut state).unwrap();

    let mut program = advent::intcode::read_program(std::io::Cursor::new(input)).unwrap();
    let mut state = State::new(
"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT I T
NOT T T
OR F T
AND E T
OR H T
AND T J
RUN
");
    advent::intcode::run_program(&mut program, &mut state).unwrap();
}
