#[derive(Debug)] struct MyError(String);
impl std::error::Error for MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
fn myerr(s: &str) -> MyError {
    MyError(s.to_string())
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Copy, Clone)]
enum Direction { U, D, L, R }
impl std::str::FromStr for Direction {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "" => Err(s.parse::<i32>().unwrap_err()),
            _ => Err("x".parse::<i32>().unwrap_err())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Step(Direction, u32);
impl std::str::FromStr for Step {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dir : Direction = s[..1].parse()?;
        let dist : u32 = s[1..].parse()?;
        Ok(Step(dir, dist))
    }
}

fn parse_csv(buf: impl std::io::Read) -> std::io::Result<Vec<Vec<String>>> {
    use std::io::BufRead;
    let reader = std::io::BufReader::new(buf);
    reader.lines().map(|maybe_line|
        maybe_line.map(|line|
            line.split(",").map(String::from).collect()
        )
    ).collect()
}
/*
fn read_input(buf: impl std::io::Read) -> Result<Vec<Vec<Step>>> {
    let strs = parse_csv(buf)?;
    strs.iter().map(|v|
        v.iter().map(|s|
            s.parse::<Step>()
        ).collect()
    ).collect()
    //if strs.len() != 2 {
    //    Err(myerr("Invalid input length").into())
    //} else {
    //    let strs = [strs[0].iter().map(|s| s.parse()), strs[1].iter().map(|s| s.parse())];
    //}
}*/

fn get_trail(pos: &mut (i32, i32, i32), step: Step) -> Vec<(i32, i32, i32)> {
    let mut trail: Vec<(i32, i32, i32)> = Vec::new();
    let Step(dir, len) = step;
    for _ in 0..len {
        *pos = match dir {
            Direction::U => (pos.0, pos.1 + 1, pos.2 + 1),
            Direction::D => (pos.0, pos.1 - 1, pos.2 + 1),
            Direction::L => (pos.0 - 1, pos.1, pos.2 + 1),
            Direction::R => (pos.0 + 1, pos.1, pos.2 + 1),
        };
        trail.push(*pos);
    }
    trail
}

fn main() -> Result<()> {
    let text1 = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let text2 = concat!(
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\n",
        "U62,R66,U55,R34,D71,R55,D58,R83");
    let text3 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    use std::io::BufRead;
    let reader = std::io::BufReader::new(text3.as_bytes());
    let reader = std::io::BufReader::new(std::fs::File::open("input03.txt")?);
    let mut knot = Vec::new();
    for line in reader.lines() {
        let mut pos: (i32, i32, i32) = (0, 0, 0);
        let mut touched: Vec<(i32, i32, i32)> = Vec::new();
        for word in line.unwrap().split(",") {
            let step: Step = word.parse().unwrap();
            touched.append(&mut get_trail(&mut pos, step));
        }
        knot.push(touched);
    }
    use std::collections::{HashMap, HashSet};
    let mut cellsA: HashMap<(i32, i32), i32> = HashMap::new();
    for c in &knot[0] {
        cellsA.entry((c.0, c.1)).or_insert(c.2);
    }
    let mut cellsB = HashMap::new();
    for c in &knot[1] {
        cellsB.entry((c.0, c.1)).or_insert(c.2);
    }
    //println!("{:?}", cellsA);

    let keysA: HashSet<(i32, i32)> = cellsA.keys().cloned().collect();
    let keysB: HashSet<(i32, i32)> = cellsB.keys().cloned().collect();

    let tangles = keysA.intersection(&keysB);
    println!("{:?}", tangles);
    let d = |x: (i32, i32)| cellsA.get(&x).unwrap() + cellsB.get(&x).unwrap();
    let closest = tangles.min_by_key(|x| d(**x));
    println!("{:?} {}", closest, d(*closest.unwrap()));
    Ok(())
}
