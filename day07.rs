use std::fs::File;
use std::io::{BufReader,BufRead};
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

fn run_program_io(prog: &mut[i32], pc: &mut i32, input: &[i32]) -> Result<Option<i32>> {
    let mut input_pos: usize = 0;
    loop {
        let opcode = *prog.get(usize::try_from(*pc)?).ok_or_else(|| myerr("End of program"))?;
        let arg1mode = opcode / 100 % 10;
        let arg2mode = opcode / 1000 % 10;
        let opcode = opcode % 100;
        match opcode {
            1|2 => {
                let x = prog_get(prog, *pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                let y = prog_get(prog, *pc+2)?;
                let y = prog_get_arg(prog, y, arg2mode)?;
                let z = prog_get(prog, *pc+3)?;
                prog_set(prog, z, if opcode == 1 { x + y } else { x * y })?;
                *pc += 4
            }
            3 => {
                let input = input[input_pos];
                input_pos += 1;
                prog_set(prog, prog_get(prog, *pc+1)?, input)?;
                *pc += 2
            }
            4 => {
                let x = prog_get(prog, *pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                *pc += 2;
                return Ok(Some(x));
            }
            5|6 => {
                let x = prog_get(prog, *pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                if if opcode == 5 { x != 0 } else { x == 0 } {
                    let y = prog_get(prog, *pc+2)?;
                    *pc = prog_get_arg(prog, y, arg2mode)?;
                } else {
                    *pc += 3;
                }
            }
            7|8 => {
                let x = prog_get(prog, *pc+1)?;
                let x = prog_get_arg(prog, x, arg1mode)?;
                let y = prog_get(prog, *pc+2)?;
                let y = prog_get_arg(prog, y, arg2mode)?;
                let test = if opcode == 7 { x < y } else { x == y };
                let z = prog_get(prog, *pc+3)?;
                prog_set(prog, z, if test { 1 } else { 0 })?;
                *pc += 4
            }
            99 => {
                return Ok(None)
            }
            _ => return std::result::Result::Err(myerr(
                &format!("Something went wrong (pc {} opcode {})", pc, opcode)).into())
        }
    }
}

fn run_program(prog: &mut [i32], input: &[i32]) -> Result<i32> {
    let mut pc = 0;
    run_program_io(prog, &mut pc, input)?.ok_or_else(|| myerr("No output").into())
}

fn run_ampli(prog: &[i32], phase: &[i32; 5]) -> Result<i32> {
    let mut output = 0;
    for p in phase {
        let mut prog = prog.to_vec();
        output = run_program(&mut prog, &[*p, output])?;
    }
    Ok(output)
}

fn run_ampli_loop(prog: &[i32], phase: &[i32; 5]) -> Result<i32> {
    let mut output = 0;
    let mut progs = [prog.to_vec(), prog.to_vec(), prog.to_vec(), prog.to_vec(), prog.to_vec()];
    let mut pcs = [0; 5];  
    for i in 0..5 {
        output = match run_program_io(&mut progs[i], &mut pcs[i], &[phase[i], output])? {
            Some(n) => n,
            None => return Ok(output),
        }
    }
    loop {
        for i in 0..5 {
            output = match run_program_io(&mut progs[i], &mut pcs[i], &[output])? {
                Some(n) => n,
                None => return Ok(output),
            }
        }
    }
}

fn main1() -> Result<()> {
    let filename = "input7.txt";
    let reader = BufReader::new(File::open(filename)?);

    let _reader = std::io::Cursor::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let _reader = std::io::Cursor::new("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    let _reader = std::io::Cursor::new("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

    let prog : Vec<_> = reader.split(b',').map(parse_int).collect::<Result<_>>()?;
    let mut max = -1;
    let mut best = Vec::new();
    for p1 in 0..5 {
        for p2 in 0..5 { if p1 != p2 {
            for p3 in 0..5 { if p1 != p3 && p2 != p3 {
                for p4 in 0..5 { if p1 != p4 && p2 != p4 && p3 != p4 {
                    for p5 in 0..5 { if p1 != p5 && p2 != p5 && p3 != p5 && p4 != p5 {
                        let perm = [p1, p2, p3, p4, p5];
                        match run_ampli(&prog, &perm) {
                            Ok(n) => {
                                if n > max {
                                    max = n;
                                    best = perm.to_vec();
                                }
                            }
                            Err(_) => ()
                        }
                    }}
                }}
            }}
        }}
    }
    println!("{:?} output = {}", best, max);
    Ok(())
}

fn main2() -> Result<()> {
    let filename = "input7.txt";
    let reader = BufReader::new(File::open(filename)?);

    let _reader = std::io::Cursor::new("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    let _reader = std::io::Cursor::new("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");

    let prog : Vec<_> = reader.split(b',').map(parse_int).collect::<Result<_>>()?;
    let mut max = -1;
    let mut best = Vec::new();
    for p1 in 5..10 {
        for p2 in 5..10 { if p1 != p2 {
            for p3 in 5..10 { if p1 != p3 && p2 != p3 {
                for p4 in 5..10 { if p1 != p4 && p2 != p4 && p3 != p4 {
                    for p5 in 5..10 { if p1 != p5 && p2 != p5 && p3 != p5 && p4 != p5 {
                        let perm = [p1, p2, p3, p4, p5];
                        match run_ampli_loop(&prog, &perm) {
                            Ok(n) => {
                                if n > max {
                                    max = n;
                                    best = perm.to_vec();
                                }
                            }
                            Err(_) => ()
                        }
                    }}
                }}
            }}
        }}
    }
    println!("{:?} output = {}", best, max);
    Ok(())
}

fn main() -> Result<()> {
    main1()?;
    main2()
}
