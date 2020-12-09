use std::io::BufRead;

#[derive(Copy, Clone)]
enum Opcode {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

fn parse_number(s: &str) -> Result<i32, String> {
    let n = s[1..].parse().map_err(|_| "error parsing number")?;
    match s.chars().next() {
        Some('+') => Ok(n),
        Some('-') => Ok(-n),
        _ => Err(format!("invalid signed number: {}", s))
    }
}

impl std::str::FromStr for Opcode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("acc ") {
            Ok(Opcode::ACC(parse_number(s)?))
        } else if let Some(s) = s.strip_prefix("jmp ") {
            Ok(Opcode::JMP(parse_number(s)?))
        } else if let Some(s) = s.strip_prefix("nop ") {
            Ok(Opcode::NOP(parse_number(s)?))
        } else {
            Err(format!("Invalid opcode: {}", s))
        }
    }
}

struct State {
    ip: usize,
    acc: i32,
}

fn step(code: &[Opcode], state: &mut State) {
    match code[state.ip] {
        Opcode::ACC(n) => {
            state.acc += n;
            state.ip += 1;
        }
        Opcode::JMP(n) => {
            if n >= 0 { state.ip += n as usize } else { state.ip -= (-n) as usize }
        }
        Opcode::NOP(_) => {
            state.ip += 1;
        }
    }
}

fn run_until_loop(code: &[Opcode]) -> State {
    let mut state = State { ip: 0, acc: 0 };
    let mut visited = std::collections::HashSet::new();
    while visited.insert(state.ip) {
        if state.ip == code.len() {
            return state;
        }
        step(code, &mut state);
    }
    state
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input08.txt"));
    let code = reader.lines().map(|l| l.unwrap().parse()).collect::<Result<Vec<Opcode>, _>>().unwrap();

    println!("Part One");
    let s = run_until_loop(&code);
    println!("Accumulator = {}", s.acc);

    println!("Part Two");
    for i in 0..code.len() {
        let mut patched_code = code.clone();
        match patched_code[i] {
            Opcode::ACC(_) => continue,
            Opcode::JMP(n) => patched_code[i] = Opcode::NOP(n),
            Opcode::NOP(n) => patched_code[i] = Opcode::JMP(n),
        }
        let state = run_until_loop(&patched_code);
        if state.ip == code.len() {
            println!("Program terminated normally with accumulator = {}", state.acc);
        }
    }
}
