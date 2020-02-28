use std::fs::File;
use std::io::{BufReader,BufRead,Write};
use std::error::Error;
use std::convert::TryFrom;

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
type Integer = i128;

fn parse_int(b: std::result::Result<Vec<u8>, std::io::Error>) -> Result<Integer> {
    Ok(std::str::from_utf8(&b?)?.trim().parse::<Integer>()?)
}

fn prog_get(prog: &Vec<Integer>, idx: Integer) -> Result<Integer> {
    let idx = usize::try_from(idx)?;
    let n = prog.get(idx).unwrap_or(&0);
    Ok((*n).into())
}

fn prog_get_arg(prog: &Vec<Integer>, relative_base: Integer, idx: Integer, mode: i32) -> Result<Integer> {
    match mode {
        0 => prog_get(prog, idx),
        1 => Ok(idx),
        2 => prog_get(prog, relative_base + idx),
        _ => Err(myerr(&format!("Invalid argument mode {}", mode)).into()),
    }
}

fn prog_set_arg(prog: &mut Vec<Integer>, relative_base: Integer, idx: Integer, mode: i32, val: Integer) -> Result<()> {
    let idx = match mode {
        0 => idx,
        1 => return Err(myerr("Cannot use immediate mode for write operations").into()),
        2 => relative_base + idx,
        _ => return Err(myerr(&format!("Invalid argument mode {}", mode)).into()),
    };
    let idx = usize::try_from(idx)?;
    if prog.len() <= idx {
        prog.resize(idx+1, 0);
    }
    let p = prog.get_mut(idx).ok_or_else(|| myerr("Out of range"))?;
    *p = val;
    Ok(())
}

fn run_program(prog: &mut Vec<Integer>) -> Result<()> {
    let mut pc = 0;
    let mut rb = 0;
    loop {
        let opcode = *prog.get(usize::try_from(pc)?).ok_or_else(|| myerr("End of program"))?;
        let arg1mode = (opcode / 100 % 10) as i32;
        let arg2mode = (opcode / 1000 % 10) as i32;
        let arg3mode = (opcode / 10000 % 10) as i32;
        let opcode = opcode % 100;
        match opcode {
            1|2 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                let y = prog_get(prog, pc+2)?;
                let y = prog_get_arg(prog, rb, y, arg2mode)?;
                let value = if opcode == 1 { x + y } else { x * y };
                let z = prog_get(prog, pc+3)?;
                prog_set_arg(prog, rb, z, arg3mode, value)?;
                pc += 4
            }
            3 => {
                print!("Input: ");
                std::io::stdout().flush().unwrap();
                let stdin = std::io::stdin();
                let line = stdin.lock().lines().next()
                    .expect("there was no next line")
                    .expect("the line could not be read");
                let input: Integer = line.parse()?;
                prog_set_arg(prog, rb, prog_get(prog, pc+1)?, arg1mode, input)?;
                pc += 2
            }
            4 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                println!("{}", x);
                pc += 2
            }
            5|6 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                if if opcode == 5 { x != 0 } else { x == 0 } {
                    let y = prog_get(prog, pc+2)?;
                    pc = prog_get_arg(prog, rb, y, arg2mode)?;
                } else {
                    pc += 3;
                }
            }
            7|8 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                let y = prog_get(prog, pc+2)?;
                let y = prog_get_arg(prog, rb, y, arg2mode)?;
                let test = if opcode == 7 { x < y } else { x == y };
                let z = prog_get(prog, pc+3)?;
                prog_set_arg(prog, rb, z, arg3mode, if test { 1 } else { 0 })?;
                pc += 4
            }
            9 => {
                let x = prog_get_arg(prog, rb, prog_get(prog, pc+1)?, arg1mode)?;
                rb += x;
                pc += 2
            }
            99 => {
                return Ok(())
            }
            _ => return std::result::Result::Err(myerr(
                &format!("Something went wrong (pc {} opcode {})", pc, opcode)).into())
        }
    }
}

fn main() -> Result<()> {
    let filename = "input9.txt";
    let _reader = BufReader::new(File::open(filename)?);
    let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let reader = std::io::Cursor::new(quine);
    let mut prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;
    
    run_program(&mut prog)
}
