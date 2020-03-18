type Int = advent::intcode::Integer;
#[derive(Default)]
struct Adventure {
    buf: String
}

impl advent::intcode::State for Adventure {
    fn input(&mut self) -> Int {
        if self.buf.is_empty() {
            use std::io::{BufRead, Write};
            std::io::stdout().flush().unwrap();
            let stdin = std::io::stdin();
            self.buf = stdin.lock().lines().next().unwrap().unwrap();
            self.buf.push('\n');
        }
        let mut chars = self.buf.chars();
        let c = chars.next().unwrap();
        self.buf = chars.collect();
        c as Int
    }

    fn output(&mut self, val: Int) {
        if val >= 0 && val < 256 {
            print!("{}", val as u8 as char);
        } else {
            println!("Output: {}", val);
        }
    }
}

fn main() {
    let input = include_str!("input25.txt");
    let mut program = advent::intcode::read_program(std::io::Cursor::new(input)).unwrap();
    let mut state = Adventure::default();
    advent::intcode::run_program(&mut program, &mut state).unwrap();
}

/*
Y hypercube
L shell           too heavy
W whirled peas
6 spool of cat6   too heavy
M mouse
A antenna
H hologram        too heavy
S semiconductor

WMA light
YWMA light
YWMAS heavy
WMAS heavy

YMAS correct
20483
*/
