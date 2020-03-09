use std::convert::TryFrom;

#[derive(Debug)]
struct Error(String);
impl Error {
    fn new(s: &str) -> Error {
        Error(s.to_string())
    }
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Incode error: {}", self.0)
    }
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Integer = i128;

enum ArgMode { Position, Immediate, Relative }
impl TryFrom<Integer> for ArgMode {
    type Error = Error;
    fn try_from(n: Integer) -> std::result::Result<ArgMode, Error> {
        match n {
            0 => Ok(ArgMode::Position),
            1 => Ok(ArgMode::Immediate),
            2 => Ok(ArgMode::Relative),
            _ => Err(Error::new(&format!("Invalid argument mode {}", n)))
        }
    }
}

fn parse_int(b: &Vec<u8>) -> Result<Integer> {
    Ok(std::str::from_utf8(b)?.trim().parse::<Integer>()?)
}

fn prog_get(prog: &Vec<Integer>, idx: Integer) -> Result<Integer> {
    let idx = usize::try_from(idx)?;
    let n = prog.get(idx).unwrap_or(&0);
    Ok(*n)
}

fn prog_get_arg(prog: &Vec<Integer>, relative_base: Integer, idx: Integer, mode: ArgMode) -> Result<Integer> {
    let idx = prog_get(prog, idx)?;
    match mode {
        ArgMode::Position => prog_get(prog, idx),
        ArgMode::Immediate => Ok(idx),
        ArgMode::Relative => prog_get(prog, relative_base + idx),
    }
}

fn prog_set_arg(prog: &mut Vec<Integer>, relative_base: Integer, idx: Integer, mode: ArgMode, value: Integer) -> Result<()> {
    let idx = prog_get(prog, idx)?;
    let idx = match mode {
        ArgMode::Position => idx,
        ArgMode::Immediate => Err(Error::new("Cannot use immediate mode for write operations"))?,
        ArgMode::Relative => relative_base + idx,
    };
    let idx = usize::try_from(idx)?;
    if prog.len() <= idx {
        prog.resize(idx+1, 0);
    }
    let p = prog.get_mut(idx).ok_or_else(|| Error::new("Unable to extend program size"))?;
    *p = value;
    Ok(())
}

pub trait State {
    fn input(&mut self) -> Integer;
    fn output(&mut self, Integer) -> ();
}

pub fn run_program(prog: &mut Vec<Integer>, state: &mut impl State) -> Result<()> {
    let mut pc = 0;
    let mut rb = 0;
    loop {
        let opcode = *prog.get(usize::try_from(pc)?).ok_or_else(|| Error::new("Unexpected end of program"))?;
        let arg1mode = ArgMode::try_from(opcode / 100 % 10)?;
        let arg2mode = ArgMode::try_from(opcode / 1000 % 10)?;
        let arg3mode = ArgMode::try_from(opcode / 10000 % 10)?;
        let opcode = opcode % 100;
        match opcode {
            1|2 => {
                let x = prog_get_arg(prog, rb, pc+1, arg1mode)?;
                let y = prog_get_arg(prog, rb, pc+2, arg2mode)?;
                let value = if opcode == 1 { x + y } else { x * y };
                prog_set_arg(prog, rb, pc+3, arg3mode, value)?;
                pc += 4
            }
            3 => {
                let input = state.input();
                prog_set_arg(prog, rb, pc+1, arg1mode, input)?;
                pc += 2
            }
            4 => {
                let x = prog_get_arg(prog, rb, pc+1, arg1mode)?;
                state.output(x);
                pc += 2
            }
            5|6 => {
                let x = prog_get_arg(prog, rb, pc+1, arg1mode)?;
                if if opcode == 5 { x != 0 } else { x == 0 } {
                    pc = prog_get_arg(prog, rb, pc+2, arg2mode)?;
                } else {
                    pc += 3;
                }
            }
            7|8 => {
                let x = prog_get_arg(prog, rb, pc+1, arg1mode)?;
                let y = prog_get_arg(prog, rb, pc+2, arg2mode)?;
                let test = if opcode == 7 { x < y } else { x == y };
                prog_set_arg(prog, rb, pc+3, arg3mode, if test { 1 } else { 0 })?;
                pc += 4
            }
            9 => {
                let x = prog_get_arg(prog, rb, pc+1, arg1mode)?;
                rb += x;
                pc += 2
            }
            99 => {
                return Ok(())
            }
            _ => Err(Error::new(&format!("Something went wrong (pc {} opcode {})", pc, opcode)))?
        }
    }
}

pub fn read_program(reader: impl std::io::BufRead) -> Result<Vec<Integer>> {
    reader.split(b',')
        .map(|bytes| parse_int(&bytes?))
        .collect()
}
