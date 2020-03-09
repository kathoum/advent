use std::fs::File;
use std::io::{BufReader,BufRead,Write};
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

fn run_program(prog: &mut Vec<Integer>, state: &mut ArcadeState) -> Result<()> {
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos ( i32, i32 );
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Tile { Empty, Wall, Block, Paddle, Ball }
enum Action { X, Y, T }
struct ArcadeState {
    tiles: std::collections::HashMap<Pos, Tile>,
    cursor: Pos,
    next_action: Action,
    score: i32,
}

impl ArcadeState {
    fn new() -> ArcadeState {
        ArcadeState { 
            tiles: std::collections::HashMap::new(),
            cursor: Pos(0, 0),
            next_action: Action::X,
            score: 0,
        }
    }
    fn get(&self) -> i32 {
        println!("{}", self);
        std::thread::sleep_ms(10);
        let (Pos(ball, _), _) = self.tiles.iter().find(|tile| *tile.1 == Tile::Ball).unwrap();
        let (Pos(paddle, _), _) = self.tiles.iter().find(|tile| *tile.1 == Tile::Paddle).unwrap();
        (ball - paddle).signum()
        /*println!("{:?} {:?}", ball, paddle);
        loop {
            print!("Input [a,s,d]: ");
            std::io::stdout().flush().unwrap();
            let stdin = std::io::stdin();
            let line = stdin.lock().lines().next()
                .expect("there was no next line")
                .expect("the line could not be read");
            match line.as_str() {
                "a" => return -1,
                "s" => return 0,
                "d" => return 1,
                _ => ()
            };
        }*/
    }
    fn put(&mut self, value: i32) {
        match self.next_action {
            Action::X => {
                self.cursor.0 = value;
                self.next_action = Action::Y;
            }
            Action::Y => {
                self.cursor.1 = value;
                self.next_action = Action::T;
            }
            Action::T => {
                if self.cursor == Pos(-1, 0) {
                    self.score = value;
                } else {
                    self.tiles.insert(self.cursor, match value {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        _ => panic!("invalid tile {}", value)
                    });
                }
                self.next_action = Action::X;
            }
        }
    }
}

impl std::fmt::Display for ArcadeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x1 = self.tiles.keys().map(|p| p.0).min().unwrap_or_default();
        let x2 = self.tiles.keys().map(|p| p.0).max().unwrap_or_default();
        let y1 = self.tiles.keys().map(|p| p.1).min().unwrap_or_default();
        let y2 = self.tiles.keys().map(|p| p.1).max().unwrap_or_default();
        for y in y1..=y2 {
            for x in x1..=x2 {
                let c = match self.tiles.get(&Pos(x, y)) {
                    None|Some(Tile::Empty) => " ",
                    Some(Tile::Wall) => "\u{2588}",
                    Some(Tile::Block) => "#",
                    Some(Tile::Paddle) => "_",
                    Some(Tile::Ball) => "o",
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "Score: {}", self.score)
    }
}

fn main() -> Result<()> {
    let filename = "input13.txt";
    let reader = BufReader::new(File::open(filename)?);
    let mut prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;
    prog[0] = 2;
    let mut state = ArcadeState::new();
    run_program(&mut prog, &mut state)?;
    println!("{}", state);
    println!("Blocks: {}", state.tiles.values().filter(|v| **v == Tile::Block).count());
    Ok(())
}
