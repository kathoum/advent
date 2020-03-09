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

trait State {
    fn input(&mut self) -> Integer;
    fn output(&mut self, Integer) -> ();
}

fn run_program(prog: &mut Vec<Integer>, state: &mut impl State) -> Result<()> {
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
                let input = state.input();
                prog_set_arg(prog, rb, prog_get(prog, pc+1)?, arg1mode, input)?;
                pc += 2
            }
            4 => {
                let x = prog_get(prog, pc+1)?;
                let x = prog_get_arg(prog, rb, x, arg1mode)?;
                state.output(x);
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
enum Tile { Empty, Wall, Oxygen }
#[derive(Copy, Clone, Debug)]
enum Direction { Up, Down, Left, Right }
struct DroidState {
    tiles: std::collections::HashMap<Pos, Tile>,
    cursor: Pos,
    last_action: Direction,
    last_successful_move: Direction,
}

impl DroidState {
    fn new() -> DroidState {
        DroidState { 
            tiles: std::collections::HashMap::new(),
            cursor: Pos(0, 0),
            last_action: Direction::Up,
            last_successful_move: Direction::Up,
        }
    }
}

impl State for DroidState {
    fn input(&mut self) -> Integer {
        //println!("{}", self);
        //std::thread::sleep(std::time::Duration::from_millis(10));

        let mut next_probe = match self.last_successful_move {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        };
        loop {
            let (pos_to_probe, retval) = match next_probe {
                Direction::Up => (Pos(self.cursor.0, self.cursor.1 - 1), 1),
                Direction::Down => (Pos(self.cursor.0, self.cursor.1 + 1), 2),
                Direction::Left => (Pos(self.cursor.0 - 1, self.cursor.1), 3),
                Direction::Right => (Pos(self.cursor.0 + 1, self.cursor.1), 4),
            };
            if self.tiles.get(&pos_to_probe) != Some(&Tile::Wall) {
                //println!("Going {:?}", next_probe);
                //let stdin = std::io::stdin();
                //let line = stdin.lock().lines().next();
                self.last_action = next_probe;
                return retval; 
            } else {
                //println!("Not going {:?} because it's a wall", next_probe);
                next_probe = match next_probe {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                }
            }
        }
    }
    
    fn output(&mut self, value: Integer) {
        println!("Got {}", value);
        let target_pos = match self.last_action {
            Direction::Up => Pos(self.cursor.0, self.cursor.1 - 1),
            Direction::Down => Pos(self.cursor.0, self.cursor.1 + 1),
            Direction::Left => Pos(self.cursor.0 - 1, self.cursor.1),
            Direction::Right => Pos(self.cursor.0 + 1, self.cursor.1),
        };
        self.tiles.insert(target_pos, match value {
            0 => Tile::Wall,
            1 => {
                self.cursor = target_pos;
                self.last_successful_move = self.last_action;
                Tile::Empty
            },
            2 => {
                self.cursor = target_pos;
                self.last_successful_move = self.last_action;
                println!("{}", self);
                println!("Oxygen found at {:?}", target_pos);
                let d = find_path(&self.tiles, target_pos, Pos(0, 0));
                println!("Distance is {}", d.0);
                println!("Time to fill is {}", d.1);
                std::thread::sleep(std::time::Duration::from_secs(2));
                Tile::Oxygen
            },
            _ => panic!()
        });
    }
}

impl std::fmt::Display for DroidState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let x1 = self.tiles.keys().map(|p| p.0).min().unwrap_or_default();
        let x2 = self.tiles.keys().map(|p| p.0).max().unwrap_or_default();
        let y1 = self.tiles.keys().map(|p| p.1).min().unwrap_or_default();
        let y2 = self.tiles.keys().map(|p| p.1).max().unwrap_or_default();
        for y in y1..=y2 {
            for x in x1..=x2 {
                let pos = Pos(x, y);
                let c = if pos == self.cursor {
                    if self.tiles.get(&pos) == Some(&Tile::Oxygen) { "O" } else { "x" }
                } else {
                    match self.tiles.get(&pos) {
                        None => " ",
                        Some(Tile::Empty) => if pos == Pos(0, 0) { "o" } else { "." },
                        Some(Tile::Wall) => "\u{2588}",
                        Some(Tile::Oxygen) => "!",
                    }
                };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn find_path(tiles: &std::collections::HashMap<Pos, Tile>, from: Pos, to: Pos) -> (usize, usize) {
    let mut maze = std::collections::HashMap::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_front((from, 0usize));
    while let Some((pos, dist)) = queue.pop_back() {
        if !maze.contains_key(&pos) && tiles.get(&pos) != Some(&Tile::Wall) && tiles.get(&pos) != None {
            maze.insert(pos, dist);
            let Pos(x, y) = pos;
            queue.push_front((Pos(x + 1, y), dist + 1));
            queue.push_front((Pos(x - 1, y), dist + 1));
            queue.push_front((Pos(x, y + 1), dist + 1));
            queue.push_front((Pos(x, y - 1), dist + 1));
        }
    }
    (*maze.get(&to).unwrap_or(&0), *maze.values().max().unwrap_or(&0))
}

fn main() -> Result<()> {
    let filename = "input15.txt";
    let reader = BufReader::new(File::open(filename)?);
    let mut prog : Vec<_> = reader
        .split(b',')
        .map(parse_int)
        .collect::<Result<_>>()?;

    let mut state = DroidState::new();
    run_program(&mut prog, &mut state)?;
    Ok(())
}
