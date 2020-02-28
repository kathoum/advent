use std::io::BufRead;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Ast(i32, i32);
type Field = std::collections::HashSet<Ast>;

fn visible(f: &Field, eye: &Ast, p: &Ast) -> bool {
    if p == eye {
        return false;
    }
    let diff = (p.0 - eye.0, p.1 - eye.1);
    for div in 2 ..= diff.0.abs().max(diff.1.abs()) {
        if diff.0 % div == 0 && diff.1 % div == 0 {
            for mul in 1..div {
                let d = Ast(eye.0 + diff.0 / div * mul, eye.1 + diff.1 / div * mul);
                if f.contains(&d) {
                    return false;
                }
            }
        }
    }
    true
}

fn count_visible(f: &Field, eye: &Ast) -> usize {
    f.iter().filter(|a| visible(&f, &eye, &a)).count()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::File::open("input10.txt")?;
    let reader = std::io::BufReader::new(input);

    let _reader = std::io::Cursor::new(
".#..#
.....
#####
....#
...##"
);

    let mut asteroids = Field::new();
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Ast(x as i32, y as i32));
            }
        }
    }
    println!("Total {}", asteroids.len());
    
    let station = *asteroids.iter().max_by_key(|a| { count_visible(&asteroids, a) }).unwrap();
    println!("{:?} sees {}", station, count_visible(&asteroids, &station));

    let mut start = 1;
    while asteroids.len() > 1 {
        let mut delenda: Vec<_> = asteroids.iter().filter(|ast| visible(&asteroids, &station, &ast)).copied().collect();
        delenda.sort_by(|ast1, ast2| {
            let (x1, y1) = (ast1.0 - station.0, ast1.1 - station.1);
            let key1 = (x1 as f64).atan2(y1 as f64);
            let (x2, y2) = (ast2.0 - station.0, ast2.1 - station.1);
            let key2 = (x2 as f64).atan2(y2 as f64);
            key2.partial_cmp(&key1).unwrap()
        });
        for (ast, index) in delenda.iter().zip(start..) {
            if index == 200 {
                println!("{}th disintegrated asteroid is {:?}", index, ast);
            }
        }
        start += delenda.len();
        for a in delenda {
            asteroids.remove(&a);
        }
    }

    Ok(())
}
