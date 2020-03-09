use std::io::{BufRead,Cursor};
use std::error::Error;

#[derive(Debug)] struct MyError(String);
impl Error for MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
fn myerr(s: &str) -> MyError {
    MyError(s.to_string())
}
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn parse_int(b: std::result::Result<Vec<u8>, std::io::Error>) -> Result<usize> {
    Ok(std::str::from_utf8(&b?)?.trim().parse::<usize>()?)
}
fn prog_get(prog: &[usize], i: usize) -> std::result::Result<usize, MyError> {
    let n = prog.get(i).ok_or_else(|| myerr("Out of range"))?;
    Ok(*n)
}
fn run_program(prog: &mut [usize]) -> Result<()> {
    let mut pc = 0;
    loop {
        let opcode = *prog.get(pc).ok_or_else(|| myerr("End of program"))?;
        match opcode {
            1|2 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get(prog, x)?;
                let y = prog_get(prog, pc+2)?;
                let y = prog_get(prog, y)?;
                let z = prog_get(prog, pc+3)?;
                match prog.get_mut(z) {
                    None => return std::result::Result::Err(myerr("Out of range").into()),
                    Some(p) => *p = if opcode == 1 { x + y }  else { x * y }
                }
                pc += 4
            }
            99 => {
                //println!("Position 0 has {}", prog_get(prog, 0)?);
                return Ok(())
            }
            _ => return std::result::Result::Err(myerr("Something went wrong (pc {} opcode {})").into())
        }
    }
}

fn result_of(noun: usize, verb: usize, prog: &[usize]) -> usize {
    let mut clone = prog.to_vec();
    clone[1] = noun;
    clone[2] = verb;
    match run_program(&mut clone) {
        Ok(_) => clone[0],
        Err(_) => 0
    }
}

fn main() -> Result<()> {
    let reader = Cursor::new(include_str!("input02.txt"));
    
    let prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;

    for a in 0..100 {
        for b in 0..100 {
            let r = result_of(a, b, &prog);
            if r == 19690720 {
                println!("{},{} -> {}", a, b, r)
            }
        }
    }
    Ok(())
}
