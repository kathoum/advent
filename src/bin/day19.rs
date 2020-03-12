enum Step { X, Y }
struct State {
    cell: (i32, i32),
    next: Step,
    output: bool,
}
type Integer = advent::intcode::Integer;
impl advent::intcode::State for State {
    fn input(&mut self) -> Integer {
        match self.next {
            Step::X => { self.next = Step::Y; self.cell.0 as Integer }
            Step::Y => self.cell.1 as Integer
        }
    }
    fn output(&mut self, value: Integer) -> () {
        self.output = value != 0;
    }
}

fn value(program: Vec<Integer>, row: i32, col: i32) -> bool {
    let mut program = program;
    let mut state = State { cell: (col, row), next: Step::X, output: false };
    advent::intcode::run_program(&mut program, &mut state).unwrap();
    state.output
}

fn row_range(program: &Vec<Integer>, row: i32, hint: Option<i32>) -> (i32, i32) {
    let hint = hint.unwrap_or(0);
    let colmin = (hint..2*row).find(|col| value(program.clone(), row, *col)).unwrap_or(-1);
    let colmax = if colmin < 0 { -1 } else { (colmin..).find(|col| !value(program.clone(), row, *col)).unwrap() - 1 };
    (colmin, colmax)
}

fn largest_square(program: &Vec<Integer>, rowmin: i32) -> (i32, (i32, i32)) {
    let (colmin, colmax) = row_range(program, rowmin, None);
    let mut hint = colmin;
    let mut bestsize = 1;
    for rowmax in rowmin+1.. {
        let (colmin, _) = row_range(program, rowmax, Some(hint));
        hint = colmin;
        let w = colmax - colmin + 1;
        let h = rowmax - rowmin + 1;
        let size = w.min(h);
        if size > bestsize {
            bestsize = size;
        } else if size < bestsize {
            break;
        }
    }
    (bestsize, (colmax - bestsize + 1, rowmin))
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input19.txt"));
    let program = advent::intcode::read_program(reader).unwrap();
    let mut total = 0;
    for y in 0..50 {
        print!("{:3}", y);
        for x in 0..50 {
            let mut program = program.clone();
            let mut state = State { cell: (x, y), next: Step::X, output: false };
            advent::intcode::run_program(&mut program, &mut state).unwrap();
            print!("{}", if state.output { '#' } else { '.' });
            total += state.output as i32;
        }
        println!("");
    }
    println!("Count {}", total);
    for y in 10..20 {
        let x = row_range(&program, y, None);
        let (s, xy) = largest_square(&program, y);
        println!("{}: {}-{} square: {} at {},{}", y, x.0, x.1, s, xy.0, xy.1);
    }
    let mut max_square = 0;
    for y in 10.. {
        let (s, xy) = largest_square(&program, y);
        if s > max_square {
            max_square = s;
            println!("New max size {} at {},{}", s, xy.0, xy.1);
            if s >= 100 {
                break;
            }
        }
    }
}
