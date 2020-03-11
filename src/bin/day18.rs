#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Cell { row: i32, col: i32 }
#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile { Wall, Open, Key(char), Door(char) }
#[derive(Clone, Default)]
struct Maze {
    tiles: std::collections::HashMap<Cell, Tile>,
    players: Vec<Cell>,
}

impl Maze {
    fn read(reader: impl std::io::Read) -> Maze {
        let mut maze = Maze::default();
        let mut cur = Cell { row: 0, col: 0 };
        for b in reader.bytes() {
            let b = b.unwrap();
            let tile = match b {
                b'#' => Some(Tile::Wall),
                b'.' => Some(Tile::Open),
                b'@' => { maze.players.push(cur); Some(Tile::Open) },
                b'a'..=b'z' => Some(Tile::Key(b as char)),
                b'A'..=b'Z' => Some(Tile::Door((b as char).to_ascii_lowercase())),
                b'\n' => None,
                _ => panic!("Unexpected input {}", b)
            };
            if let Some(tile) = tile {
                maze.tiles.insert(cur, tile);
                cur.col += 1;
            } else {
                cur.col = 0;
                cur.row += 1;
            }
        }
        maze
    }

    fn open_door(&mut self, key: char) {
        for tile in self.tiles.values_mut() {
            if *tile == Tile::Key(key) || *tile == Tile::Door(key) {
                *tile = Tile::Open;
            }
        }
    }

    fn reachable_keys(&self) -> Vec<(Cell, usize, char, usize)> {
        let mut tile_queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        for (player_index, player_cell) in self.players.iter().enumerate() {
            tile_queue.push_back((*player_cell, player_index, 0usize));
        }

        while let Some((cell, idx, distance)) = tile_queue.pop_front() {
            if !visited.contains(&cell) {
                visited.insert(cell);
                match self.tiles.get(&cell) {
                    Some(Tile::Wall) => (),
                    Some(Tile::Door(_)) => (),
                    Some(Tile::Key(key)) => result.push((cell, idx, *key, distance)),
                    Some(Tile::Open) => {
                        let Cell { row: r, col: c } = cell;
                        for rc in &[(r-1,c), (r+1,c), (r,c-1), (r,c+1)] {
                            tile_queue.push_back((Cell { row: rc.0, col: rc.1 }, idx, distance + 1));
                        }
                    }
                    None => panic!("Out of the maze!"),
                };
            }
        }
        result
    }

    fn children(&self) -> Vec<(Maze, usize, char)> {
        unsafe {
            SOLVE_CALL_COUNT += 1;
            /*if SOLVE_CALL_COUNT % 10000 == 0 {
                println!("{} calls, backlog size {}", SOLVE_CALL_COUNT, n);
            }*/
        }
        self.reachable_keys().into_iter().map(|(cell, player_idx, key, distance)| {
            let mut maze = self.clone();
            maze.players[player_idx] = cell;
            maze.open_door(key);
            (maze, distance, key)
        }).collect()
    }

    fn solve(&self) -> (usize, String) {
        unsafe { SOLVE_CALL_COUNT = 0; }
        struct Node {
            maze: Maze,
            keys: String,
            penalty: usize,
            prefix: String,
        }
        impl std::cmp::Ord for Node {
            fn cmp(&self, other: &Node) -> std::cmp::Ordering {
                other.penalty.cmp(&self.penalty)
            }
        }
        impl std::cmp::PartialOrd for Node {
            fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
                Some(self.cmp(&other))
            }
        }
        impl std::cmp::PartialEq for Node {
            fn eq(&self, other: &Node) -> bool {
                self.cmp(&other) == std::cmp::Ordering::Equal
            }
        }
        impl std::cmp::Eq for Node {}

        let mut keys: Vec<char> = self.tiles.values()
            .filter_map(|t| if let Tile::Key(c) = t { Some(*c) } else { None })
            .collect();
        keys.sort();
        let root = Node {
            maze: self.clone(),
            keys: keys.into_iter().collect(),
            penalty: 0,
            prefix: String::new(),
        };
        let mut queue = std::collections::BinaryHeap::new();
        let mut visited: Vec<Node> = Vec::new();
        let mut solution: Option<Node> = None;
        queue.push(root);
        while let Some(node) = queue.pop() {
            //println!("Backlog {}: expanding {} + {} dist {}", queue.len(), node.prefix, node.keys, node.penalty);
            if node.keys.is_empty() {
                solution = match solution {
                    Some(sol) if sol.penalty < node.penalty => Some(sol),
                    _ => Some(node),
                }
            } else {
                let same_node = visited.iter_mut().find(|n| n.maze.players == node.maze.players && n.keys == node.keys);
                let current: &Node;
                if let Some(same_node) = same_node {
                    if same_node.penalty < node.penalty {
                        //println!("Pruned");
                        continue;
                    } else {
                        *same_node = node;
                        current = same_node;
                    }
                } else {
                    visited.push(node);
                    current = visited.last().unwrap();
                }
                for (child, distance, key) in current.maze.children() {
                    let node = Node {
                        maze: child,
                        keys: current.keys.replace(key, ""),
                        penalty: current.penalty + distance,
                        prefix: format!("{}{}", current.prefix, key),
                    };
                    //println!("Append {} + {} dist {}", node.prefix, node.keys, node.penalty);
                    queue.push(node);
                }
            }
        }
        match solution {
            None => (std::usize::MAX, String::new()),
            Some(solution) => (solution.penalty, solution.prefix),
        }
    }
}

static mut SOLVE_CALL_COUNT: usize = 0;

static TESTS: &[(usize, &str)] = &[
(8, "\
#########
#b.A.@.a#
#########"),

(86, "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"),

(132, "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"),

(136, "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"),

(81, "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"),

(8, "\
#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#.b#
#######"),

(24, "\
###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############"),

(32, "\
#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############"),

(72, "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"),

];

fn main() {
    println!("Running tests...");
    for (output, input) in TESTS {
        let maze = Maze::read(std::io::Cursor::new(input));
        let (result, sequence) = maze.solve();
        if result != *output {
            println!("Error for maze:");
            println!("{}", input);
            println!("Expected: {}, result: {}", output, result);
        } else {
            println!("Ok: len {}, sequence is {}, {} calls", result, sequence, unsafe { SOLVE_CALL_COUNT });
        }
    }

    let input = include_str!("input18.txt");
    let maze = Maze::read(std::io::Cursor::new(input));
    let (len, keys) = maze.solve();
    println!("Shortest path length: {}", len);
    println!("Key sequence: {}", keys);
    println!("{} pathfinder calls", unsafe { SOLVE_CALL_COUNT });

    let mut maze = Maze::read(std::io::Cursor::new(input));
    assert_eq!(maze.players.len(), 1);
    let Cell { row: r, col: c } = *maze.players.first().unwrap();
    for row in r-1..=r+1 {
        for col in c-1..=c+1 {
            assert!(maze.tiles.get(&Cell { row, col }) == Some(&Tile::Open));
        }
    }
    maze.tiles.insert(Cell { row: r, col: c }, Tile::Wall);
    maze.tiles.insert(Cell { row: r-1, col: c }, Tile::Wall);
    maze.tiles.insert(Cell { row: r+1, col: c }, Tile::Wall);
    maze.tiles.insert(Cell { row: r, col: c-1 }, Tile::Wall);
    maze.tiles.insert(Cell { row: r, col: c+1 }, Tile::Wall);
    maze.players = vec![
        Cell { row: r-1, col: c-1 },
        Cell { row: r+1, col: c-1 },
        Cell { row: r-1, col: c+1 },
        Cell { row: r+1, col: c+1 },
    ];
    let (len, keys) = maze.solve();
    println!("Shortest path length: {}", len);
    println!("Key sequence: {}", keys);
    println!("{} pathfinder calls", unsafe { SOLVE_CALL_COUNT });
}
