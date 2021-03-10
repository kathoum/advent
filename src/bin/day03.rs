fn main() {
    let input = include_str!("input03.txt");

    let visited = count_different(path(input.chars()));
    println!("Part one: {} visited houses", visited);

    let path_santa = path(input.chars().step_by(2));
    let path_robot = path(input.chars().skip(1).step_by(2));
    let visited = count_different(path_santa.chain(path_robot));
    println!("Part two: {} visited houses", visited);
}

fn path(directions: impl Iterator<Item=char>) -> impl Iterator<Item=(i32,i32)> {
    let start = std::iter::once((0,0));
    let rest = directions.scan((0,0), |pos, c| {
        match c {
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            '<' => pos.1 += 1,
            '>' => pos.1 -= 1,
            _ => panic!(),
        };
        Some(*pos)
    });
    start.chain(rest)
}

fn count_different<T: Eq + std::hash::Hash>(iter: impl Iterator<Item=T>) -> usize {
    iter.collect::<std::collections::HashSet<_>>().len()
}
