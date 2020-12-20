struct Tile {
    id: u32,
    data: Vec<Vec<bool>>,
    sides: [u32; 4],
}

fn bools_to_int<'a>(iter: impl Iterator<Item = &'a bool>) -> u32 {
    let (n, m, _) = iter.fold((0, 0, 0), |(n, m, i), &b|
        ((n << 1) + (b as u32), m + ((b as u32) << i), i + 1));
    n.min(m)
}

impl Tile {
    fn new(id: u32, data: Vec<Vec<bool>>) -> Self {
        let top = bools_to_int(data[0].iter());
        let bottom = bools_to_int(data.last().unwrap().iter());
        let left = bools_to_int(data.iter().map(|v| &v[0]));
        let right = bools_to_int(data.iter().map(|v| v.last().unwrap()));
        Tile{id, data, sides: [top, bottom, left, right]}
    }
}

fn main() {
    let input = include_str!("input20.txt");

    let mut tiles = Vec::new();
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

    println!("Part One");
    enum SideMatch {
        Outer(u32),
        Inner(u32, u32),
    }
    let mut counts = std::collections::HashMap::new();
    for &Tile{id, data: _, sides} in tiles.iter() {
        for &s in sides.iter() {
            let m = match counts.get(&s) {
                None => SideMatch::Outer(id),
                Some(SideMatch::Outer(x)) => SideMatch::Inner(*x, id),
                Some(SideMatch::Inner(x, y)) => panic!("Unexpected: side {} is present in 3 tiles: {}, {}, {}", s, x, y, id)
            };
            counts.insert(s, m);
        }
    }

    print!("Corner tiles: ");
    let corners: Vec<_> = tiles.iter().filter_map(|&Tile{id, data: _, sides}| {
        let num_outer = sides.iter().filter(|s| matches!(counts[&s], SideMatch::Outer(_))).count();
        if num_outer == 2 {
            print!("{} ", id);
            Some(id)
        } else {
            None
        }
    }).collect();
    println!("\nProduct = {}", corners.iter().map(|&n| n as i64).product::<i64>());

    println!("Part Two");
}
