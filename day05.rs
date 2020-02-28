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

fn parse_int(b: std::result::Result<Vec<u8>, std::io::Error>) -> Result<i32> {
    Ok(std::str::from_utf8(&b?)?.trim().parse::<i32>()?)
}

fn prog_get(prog: &[i32], i: i32) -> Result<i32> {
    let i = usize::try_from(i)?;
    let n = prog.get(i).ok_or_else(|| myerr("Out of range"))?;
    Ok(*n)
}

fn prog_get_arg(prog: &[i32], i: i32, mode: i32) -> Result<i32> {
    if mode == 1 { Ok(i) } else { prog_get(prog, i) }
}

fn prog_set(prog: &mut [i32], i: i32, val: i32) -> Result<()> {
    let i = usize::try_from(i)?;
    let p = prog.get_mut(i).ok_or_else(|| myerr("Out of range"))?;
    *p = val;
    Ok(())
}

fn run_program(prog: &mut [i32]) -> Result<()> {
    let mut pc = 0;
    loop {
        let opcode = *prog.get(usize::try_from(pc)?).ok_or_else(|| myerr("End of program"))?;
        let arg1mode = opcode / 100 % 10;
        let arg2mode = opcode / 1000 % 10;
        //let arg3mode = opcode / 10000 % 10;
        let opcode = opcode % 100;
        match opcode {
            1|2 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                let y = prog_get(prog, pc+2)?;
                let y = prog_get_arg(prog, y, arg2mode)?;
                let z = prog_get(prog, pc+3)?;
                prog_set(prog, z, if opcode == 1 { x + y } else { x * y })?;
                pc += 4
            }
            3 => {
                print!("Input: ");
                std::io::stdout().flush().unwrap();
                let stdin = std::io::stdin();
                let line = stdin.lock().lines().next()
                    .expect("there was no next line")
                    .expect("the line could not be read");
                let input: i32 = line.parse()?;
                prog_set(prog, prog_get(prog, pc+1)?, input)?;
                pc += 2
            }
            4 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                println!("{}", x);
                pc += 2
            }
            5|6 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                if if opcode == 5 { x != 0 } else { x == 0 } {
                    let y = prog_get(prog, pc+2)?;
                    pc = prog_get_arg(prog, y, arg2mode)?;
                } else {
                    pc += 3;
                }
            }
            7|8 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                let y = prog_get(prog, pc+2)?;
                let y = prog_get_arg(prog, y, arg2mode)?;
                let test = if opcode == 7 { x < y } else { x == y };
                let z = prog_get(prog, pc+3)?;
                prog_set(prog, z, if test { 1 } else { 0 })?;
                pc += 4
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
    let filename = "input05.txt";
    let reader = BufReader::new(File::open(filename)?);
    
    let mut prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;
    
    run_program(&mut prog)
}
