use std::io::BufRead;

fn main() {
    let input = include_str!("input12.txt");

    println!("Part One");
    struct State {
        x: i32, y: i32,
        dx: i32, dy: i32,
    }
    let initial_state = State { x: 0, y: 0, dx: 1, dy: 0 };
    let final_state = std::io::Cursor::new(input).lines().fold(initial_state, |state, line| {
        let line = line.unwrap();
        let (act, val) = line.split_at(1);
        let val: i32 = val.parse().unwrap();
        let mut state = state;
        match (act, val) {
            ("N", _) => state.y += val,
            ("S", _) => state.y -= val,
            ("E", _) => state.x += val,
            ("W", _) => state.x -= val,
            ("L", 90) | ("R", 270) => {
                let (dx, dy) = (-state.dy, state.dx);
                state.dx = dx;
                state.dy = dy;
            }
            ("L", 180) | ("R", 180) => {
                state.dx = -state.dx;
                state.dy = -state.dy;
            }
            ("L", 270) | ("R", 90) => {
                let (dx, dy) = (state.dy, -state.dx);
                state.dx = dx;
                state.dy = dy;
            }
            ("L", _) | ("R", _) => panic!(format!("Unexpected rotation: {}", line)),
            ("F", _) => {
                state.x += val * state.dx;
                state.y += val * state.dy;
            }
            _ => panic!(format!("Invalid action: {}", line)),
        };
        state
    });
    println!("Final position: ({},{}) distance = {}", final_state.x, final_state.y, final_state.x.abs() + final_state.y.abs());

    println!("Part Two");
    let initial_state = State { x: 0, y: 0, dx: 10, dy: 1 };
    let final_state = std::io::Cursor::new(input).lines().fold(initial_state, |state, line| {
        let line = line.unwrap();
        let (act, val) = line.split_at(1);
        let val: i32 = val.parse().unwrap();
        let mut state = state;
        match (act, val) {
            ("N", _) => state.dy += val,
            ("S", _) => state.dy -= val,
            ("E", _) => state.dx += val,
            ("W", _) => state.dx -= val,
            ("L", 90) | ("R", 270) => {
                let (dx, dy) = (-state.dy, state.dx);
                state.dx = dx;
                state.dy = dy;
            }
            ("L", 180) | ("R", 180) => {
                state.dx = -state.dx;
                state.dy = -state.dy;
            }
            ("L", 270) | ("R", 90) => {
                let (dx, dy) = (state.dy, -state.dx);
                state.dx = dx;
                state.dy = dy;
            }
            ("L", _) | ("R", _) => panic!(format!("Unexpected rotation: {}", line)),
            ("F", _) => {
                state.x += val * state.dx;
                state.y += val * state.dy;
            }
            _ => panic!(format!("Invalid action: {}", line)),
        };
        state
    });
    println!("Final position: ({},{}) distance = {}", final_state.x, final_state.y, final_state.x.abs() + final_state.y.abs());
}
