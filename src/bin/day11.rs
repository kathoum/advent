use std::io::BufRead;

type Area = Vec<Vec<u8>>;

fn count_adjacent_occupied(area: &Area, row: usize, col: usize) -> u32 {
    let mut count = 0;
    let (rm, cm) = (area.len() - 2, area[0].len() - 2);
    for r in (row.max(1)-1)..=(row.min(rm)+1) {
        for c in (col.max(1)-1)..=(col.min(cm)+1) {
            if (r, c) != (row, col) && area[r][c] == b'#' {
                count += 1;
            }
        }
    }
    count
}

fn count_visible_occupied(area: &Area, row: usize, col: usize) -> u32 {
    let (rows, cols) = (area.len() as i32, area[0].len() as i32);
    let mut count = 0;
    for r in -1..=1 {
        for c in -1..=1 {
            if (r, c) != (0, 0) {
                let (mut row, mut col) = (row as i32 + r, col as i32 + c);
                while 0 <= row && row < rows && 0 <= col && col < cols {
                    match area[row as usize][col as usize] {
                        b'L' => { break; }
                        b'#' => { count += 1; break; },
                        _ => { row += r; col += c; }
                    }
                }
            }
        }
    }
    count
}

fn step<F: Fn(&Area, usize, usize) -> u32>(area: &Area, counter: F, limit: u32) -> (Area, bool) {
    let mut modified = false;
    let result = area.iter().enumerate().map(|(row, line)| {
        line.iter().enumerate().map(|(col, &ch)| {
            let mut newch = ch;
            if ch == b'L' && counter(area, row, col) == 0 {
                newch = b'#';
                modified = true;
            } else if ch == b'#' && counter(area, row, col) >= limit {
                newch = b'L';
                modified = true;
            }
            newch
        }).collect()
    }).collect();
    (result, modified)
}

fn run_to_steady_state<F: Fn(&Area, usize, usize) -> u32>(area: &Area, counter: F, limit: u32) -> usize {
    let mut current = area.clone();
    loop {
        let (next, modified) = step(&current, &counter, limit);
        if !modified {
            let total_occupied: usize = current.iter().map(|l|
                l.iter().filter(|c| **c == b'#').count()
            ).sum();
            return total_occupied;
        }
        current = next;
    }
}

fn main() {
    let reader = std::io::Cursor::new(include_str!("input11.txt"));

    let area: Area = reader.lines().map(|l| l.unwrap().as_bytes().to_vec()).collect();

    println!("Part One");
    let total_occupied = run_to_steady_state(&area, count_adjacent_occupied, 4);
    println!("{} occupied seats in steady state", total_occupied);

    println!("Part Two");
    let total_occupied = run_to_steady_state(&area, count_visible_occupied, 5);
    println!("{} occupied seats in steady state", total_occupied);
}
