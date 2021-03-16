fn main() {
    let input = include_str!("input06.txt");

    let mut grid = vec![vec![false; 1000]; 1000];
    for line in input.lines() {
        Action::parse(line).run_1(&mut grid);
    }
    let tot = grid.iter().map(|v| v.iter().filter(|t| **t).count()).sum::<usize>();
    println!("Part one: {} lights on", tot);

    let mut grid = vec![vec![0u32; 1000]; 1000];
    for line in input.lines() {
        Action::parse(line).run_2(&mut grid);
    }
    let tot = grid.iter().map(|v| v.iter().sum::<u32>()).sum::<u32>();
    println!("Part two: brightness is {}", tot);
}

struct Point(i32, i32);
struct Rect(Point, Point);
enum Act { TurnOn, TurnOff, Toggle }
struct Action(Act, Rect);

impl Action {
    fn parse(str: &str) -> Self {
        let act;
        let rest;
        if let Some(s) = str.strip_prefix("turn on ") {
            act = Act::TurnOn;
            rest = s;
        } else if let Some(s) = str.strip_prefix("turn off ") {
            act = Act::TurnOff;
            rest = s;
        } else if let Some(s) = str.strip_prefix("toggle ") {
            act = Act::Toggle;
            rest = s;
        } else {
            panic!();
        }
        let (a, b, c, d);
        text_io::scan!(rest.bytes() => "{},{} through {},{}", a, b, c, d);
        Action(act, Rect(Point(a, b), Point(c, d)))
    }

    fn run_1(&self, grid: &mut Vec<Vec<bool>>) {
        let Rect(Point(x0, y0), Point(x1, y1)) = self.1;
        for x in x0 as usize ..= x1 as usize {
            for y in y0 as usize ..= y1 as usize {
                match self.0 {
                    Act::TurnOn => grid[x][y] = true,
                    Act::TurnOff => grid[x][y] = false,
                    Act::Toggle => grid[x][y] = !grid[x][y],
                }
            }
        }
    }

    fn run_2(&self, grid: &mut Vec<Vec<u32>>) {
        let Rect(Point(x0, y0), Point(x1, y1)) = self.1;
        for x in x0 as usize ..= x1 as usize {
            for y in y0 as usize ..= y1 as usize {
                match self.0 {
                    Act::TurnOn => grid[x][y] += 1,
                    Act::TurnOff => grid[x][y] -= if grid[x][y] == 0 { 0 } else { 1 },
                    Act::Toggle => grid[x][y] += 2,
                }
            }
        }
    }
}
