use std::collections::HashMap;

struct Tile {
    id: u32,
    data: Vec<Vec<bool>>,
    sides: [u32; 4],
}

fn bools_to_int_forward<'a>(iter: impl Iterator<Item = &'a bool>) -> u32 {
    iter.fold(0, |n, &b| { (n << 1) + (b as u32) })
}

fn bools_to_int_backward<'a>(iter: impl Iterator<Item = &'a bool>) -> u32 {
    iter.fold((0, 0), |(n, i), &b| { (n + ((b as u32) << i), i + 1) }).0
}

fn bools_to_int_unoriented<'a>(iter: impl Iterator<Item = &'a bool>) -> u32 {
    let (n, m, _) = iter.fold((0, 0, 0), |(n, m, i), &b|
        ((n << 1) + (b as u32), m + ((b as u32) << i), i + 1));
    n.min(m)
}

impl Tile {
    fn new(id: u32, data: Vec<Vec<bool>>) -> Self {
        let top = bools_to_int_unoriented(data[0].iter());
        let bottom = bools_to_int_unoriented(data.last().unwrap().iter());
        let left = bools_to_int_unoriented(data.iter().map(|v| &v[0]));
        let right = bools_to_int_unoriented(data.iter().map(|v| v.last().unwrap()));
        Tile{id, data, sides: [top, bottom, left, right]}
    }

    fn rows(&self) -> usize { self.data.len() }
    fn cols(&self) -> usize { self.data[0].len() }

    fn top_row(&self) -> impl Iterator<Item = &bool> { self.data[0].iter() }
    fn bottom_row(&self) -> impl Iterator<Item = &bool> { self.data.last().unwrap().iter() }
    fn left_col(&self) -> impl Iterator<Item = &bool> { self.data.iter().map(|v| &v[0]) }
    fn right_col(&self) -> impl Iterator<Item = &bool> { self.data.iter().map(|v| v.last().unwrap()) }

    fn rotate(&mut self) { rotate(&mut self.data) }
    fn mirror_horizontal(&mut self) { mirror_horizontal(&mut self.data) }
    fn mirror_vertical(&mut self) { mirror_vertical(&mut self.data) }
}

fn rotate<T: Default + Clone + Copy>(data: &mut Vec<Vec<T>>) {
    let rows = data.len();
    let cols = data[0].len();
    let mut new_data = vec![vec![Default::default(); rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            new_data[c][rows - r - 1] = data[r][c];
        }
    }
    *data = new_data;
}

fn mirror_horizontal<T>(data: &mut Vec<Vec<T>>) {
    for row in data.iter_mut() {
        row.reverse();
    }
}

fn mirror_vertical<T>(data: &mut Vec<Vec<T>>) {
    data.reverse();
}

fn main() {
    let input = include_str!("input20.txt");

    let mut tiles: Vec<Tile> = Vec::new();
    let mut next_tile = 0;
    let mut tile_data = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            tiles.push(Tile::new(next_tile, tile_data));
            tile_data = Vec::new();
        } else if line.starts_with("Tile ") {
            next_tile = line.trim_start_matches("Tile ").trim_end_matches(":").parse().unwrap();
        } else {
            tile_data.push(line.chars().map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!()
            }).collect());
        }
    }
    let id_to_index: HashMap<u32, usize> = tiles.iter().enumerate()
        .map(|(index, tile)| (tile.id, index)).collect();

    println!("Part One");
    enum SideMatch {
        Outer(u32),
        Inner(u32, u32),
    }
    let mut border_kinds: HashMap<u32, SideMatch> = HashMap::new();
    for &Tile{id, data: _, sides} in tiles.iter() {
        for &s in sides.iter() {
            let m = match border_kinds.get(&s) {
                None => SideMatch::Outer(id),
                Some(SideMatch::Outer(x)) => SideMatch::Inner(*x, id),
                Some(SideMatch::Inner(x, y)) => panic!("Unexpected: side {} is present in 3 tiles: {}, {}, {}", s, x, y, id)
            };
            border_kinds.insert(s, m);
        }
    }

    print!("Corner tiles: ");
    let corners: Vec<u32> = tiles.iter().filter_map(|&Tile{id, data: _, sides}| {
        let num_outer = sides.iter().filter(|s| matches!(border_kinds[&s], SideMatch::Outer(_))).count();
        if num_outer == 2 {
            print!("{} ", id);
            Some(id)
        } else {
            None
        }
    }).collect();
    println!("\nProduct = {}", corners.iter().map(|&n| n as i64).product::<i64>());

    println!("Part Two");
    let side = (tiles.len() as f32).sqrt() as usize;
    assert_eq!(side * side, tiles.len());
    let mut picture: Vec<Vec<u32>> = vec![vec![0u32; side]; side];
    picture[0][0] = corners[0];

    // Rotate the first corner until its external sides are to the top and left
    let (rows, cols) = {
        let first_corner = &mut tiles[id_to_index[&corners[0]]];
        loop {
            let top_row = bools_to_int_unoriented(first_corner.top_row());
            let left_col = bools_to_int_unoriented(first_corner.left_col());
            match (&border_kinds[&top_row], &border_kinds[&left_col]) {
                (&SideMatch::Outer(_), &SideMatch::Outer(_)) => break,
                _ => first_corner.rotate(),
            };
        }
        (first_corner.rows(), first_corner.cols())
    };

    let matching_tile = |id: u32, side: u32| {
        match border_kinds[&side] {
            SideMatch::Inner(a, b) => if a == id { b } else if b == id { a } else { panic!() },
            SideMatch::Outer(_) => panic!()
        }
    };

    for row in 0..side {
        if row != 0 {
            let id = picture[row - 1][0];
            let top_tile = id_to_index[&id];
            let bottom_tile = id_to_index[&matching_tile(id, bools_to_int_unoriented(tiles[top_tile].bottom_row()))];
            let row_code = bools_to_int_forward(tiles[top_tile].bottom_row());
            loop {
                if bools_to_int_forward(tiles[bottom_tile].top_row()) == row_code {
                    break;
                } else if bools_to_int_backward(tiles[bottom_tile].top_row()) == row_code {
                    tiles[bottom_tile].mirror_horizontal();
                    break;
                } else {
                    tiles[bottom_tile].rotate();
                }
            }
            picture[row][0] = tiles[bottom_tile].id;
        }
        for col in 1..side {
            let id = picture[row][col - 1];
            let left_tile = id_to_index[&id];
            let right_tile = id_to_index[&matching_tile(id, bools_to_int_unoriented(tiles[left_tile].right_col()))];
            let col_code = bools_to_int_forward(tiles[left_tile].right_col());
            loop {
                if bools_to_int_forward(tiles[right_tile].left_col()) == col_code {
                    break;
                } else if bools_to_int_backward(tiles[right_tile].left_col()) == col_code {
                    tiles[right_tile].mirror_vertical();
                    break;
                } else {
                    tiles[right_tile].rotate();
                }
            }
            picture[row][col] = tiles[right_tile].id;
        }
    }

    let mut image: Vec<Vec<u8>> = vec![vec![b'?'; side * (cols - 2)]; side * (rows - 2)];
    for tr in 0..side {
        for tc in 0..side {
            let tile = &tiles[id_to_index[&picture[tr][tc]]];
            for r in 1..(rows - 1) {
                for c in 1..(cols - 1) {
                    image[tr * (rows - 2) + r - 1][tc * (cols - 2) + c - 1] = if tile.data[r][c] { b'#' } else { b'.' };
                }
            }
        }
    }

    let mut count = 0;
    count += find_monster(&mut image); mirror_horizontal(&mut image);
    count += find_monster(&mut image); mirror_vertical(&mut image);
    count += find_monster(&mut image); mirror_horizontal(&mut image);
    count += find_monster(&mut image); rotate(&mut image);
    count += find_monster(&mut image); mirror_horizontal(&mut image);
    count += find_monster(&mut image); mirror_vertical(&mut image);
    count += find_monster(&mut image); mirror_horizontal(&mut image);
    count += find_monster(&mut image);

    // for r in image.iter() {
    //     for c in r.iter() {
    //         print!("{}", *c as char);
    //     }
    //     print!("\n");
    // }
    println!("{} monsters found", count);
    let rough: usize = image.iter().map(|row|
        row.iter().filter(|c| **c == b'#').count()
    ).sum();
    println!("Water roughness is {}", rough);
}

fn match_monster(image: &mut Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    // 01234567890123456789
    //                   # 
    // #    ##    ##    ###
    //  #  #  #  #  #  #   
    if row + 3 > image.len() || col + 20 > image[0].len() {
        false
    } else {
        let body = [(1,0), (2,1), (2,4), (1,5), (1,6), (2,7), (2,10), (1,11), (1,12), (2,13), (2,16), (1,17), (1,18), (1,19), (0,18)];
        if body.iter().all(|(r, c)| b"#O".contains(&image[row + r][col + c])) {
            for (r, c) in body.iter() {
                image[row + r][col + c] = b'O';
            }
            true
        } else {
            false
        }
    }
}

fn find_monster(image: &mut Vec<Vec<u8>>) -> usize {
    let mut total = 0;
    for row in 0..image.len() {
        for col in 0..image[0].len() {
            if match_monster(image, row, col) {
                total += 1;
            }
        }
    }
    total
}
