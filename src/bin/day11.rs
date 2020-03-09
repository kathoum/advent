use std::io::{BufRead,Cursor};
use std::error::Error;
use std::convert::{TryFrom,TryInto};

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

fn run_program(prog: &mut Vec<Integer>, state: &mut HullState) -> Result<()> {
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
                let input = state.get().into();
                prog_set_arg(prog, rb, prog_get(prog, pc+1)?, arg1mode, input)?;
                pc += 2
            }
            4 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                state.put(x.try_into()?);
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

enum PanelColor { Black, White }
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos ( i32, i32 );
enum Orient { Up, Right, Down, Left }
struct RobotPos ( Pos, Orient );
enum Action { Paint, Move }
struct HullState {
    position: RobotPos,
    panels: std::collections::HashMap<Pos, PanelColor>,
    next_action: Action,
}

impl HullState {
    fn new() -> HullState {
        HullState { 
            position: RobotPos(Pos(0, 0), Orient::Up),
            panels: std::collections::HashMap::new(),
            next_action: Action::Paint }
    }
    fn get(&self) -> i32 {
        match self.panels.get(&self.position.0).unwrap_or(&PanelColor::Black) {
            PanelColor::Black => 0,
            PanelColor::White => 1,
        }
    }
    fn put(&mut self, val: i32) {
        match self.next_action {
            Action::Paint => {
                let color = match val {
                    0 => PanelColor::Black,
                    1 => PanelColor::White,
                    _ => panic!("Invalid color {}", val)
                };
                self.panels.insert(self.position.0, color);
                self.next_action = Action::Move;
            }
            Action::Move => {
                self.position.1 = match (val, &self.position.1) {
                    (0, Orient::Up) | (1, Orient::Down) => Orient::Left,
                    (0, Orient::Right) | (1, Orient::Left) => Orient::Up,
                    (0, Orient::Down) | (1, Orient::Up) => Orient::Right,
                    (0, Orient::Left) | (1, Orient::Right) => Orient::Down,
                    _ => panic!("Invalid direction {}", val)
                };
                let pos = &mut self.position.0;
                match self.position.1 {
                    Orient::Up => (*pos).1 -= 1,
                    Orient::Right => (*pos).0 += 1,
                    Orient::Down => (*pos).1 += 1,
                    Orient::Left => (*pos).0 -= 1,
                };
                self.next_action = Action::Paint;
            }
        }
    }
}

impl std::fmt::Display for HullState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x1 = self.panels.keys().map(|p| p.0).min().unwrap_or_default();
        let x2 = self.panels.keys().map(|p| p.0).max().unwrap_or_default();
        let y1 = self.panels.keys().map(|p| p.1).min().unwrap_or_default();
        let y2 = self.panels.keys().map(|p| p.1).max().unwrap_or_default();
        for y in y1..=y2 {
            for x in x1..=x2 {
                let c = match self.panels.get(&Pos(x, y)) {
                    None => " ",
                    Some(PanelColor::Black) => ".",
                    Some(PanelColor::White) => "\u{2588}",
                };
                write!(f, "{}", c)?;
            }
            write!(f, "{}", if y == y2 { "" } else { "\n" })?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let reader = Cursor::new(include_str!("input11.txt"));
    let mut prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;
    let mut state = HullState::new();
    state.panels.insert(Pos(0,0), PanelColor::White);
    run_program(&mut prog, &mut state)?;
    println!("{}", state);
    println!("{}", state.panels.len());
    Ok(())
}
