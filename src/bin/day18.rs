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

    fn reachable_from(&self, start: Cell) -> Vec<(Cell, Tile, usize)> {
        let mut tile_queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        tile_queue.push_back((start, 0usize));
        while let Some((cell, distance)) = tile_queue.pop_front() {
            if !visited.contains(&cell) {
                visited.insert(cell);
                let Cell { row: r, col: c } = cell;
                for rc in &[(r-1,c), (r+1,c), (r,c-1), (r,c+1)] {
                    let c = Cell { row: rc.0, col: rc.1 };
                    if !visited.contains(&c) {
                        match self.tiles.get(&c) {
                            Some(Tile::Wall) => (),
                            Some(Tile::Door(x)) => result.push((c, Tile::Door(*x), distance + 1)),
                            Some(Tile::Key(x)) => result.push((c, Tile::Key(*x), distance + 1)),
                            Some(Tile::Open) if self.players.contains(&c) => result.push((c, Tile::Open, distance + 1)),
                            Some(Tile::Open) => tile_queue.push_back((c, distance + 1)),
                            None => panic!("Out of the maze!")
                        };
                    }
                }
            }
        }
        result
    }

    fn reachable_keys(&self) -> Vec<(Cell, usize, char, usize)> {
        let mut result = Vec::new();
        for (player_index, player_cell) in self.players.iter().enumerate() {
            let keys = self.reachable_from(*player_cell);
            for (key_cell, tile, distance) in keys {
                if let Tile::Key(key) = tile {
                    result.push((key_cell, player_index, key, distance))
                }
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
            //println!("Visiting {}", node.prefix);
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Vertex { Droid(i32), Key(char), Door(char) }
#[derive(Clone)]
struct Edge {
    v1: Vertex,
    v2: Vertex,
    length: usize,
}
#[derive(Clone)]
struct Graph {
    edges: Vec<Edge>,
    keys: String,
    taken_keys: String,
    distance: usize,
}

impl std::cmp::Ord for Graph {
    fn cmp(&self, other: &Graph) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
            .then(self.taken_keys.cmp(&other.taken_keys))
            .then(self.keys.cmp(&other.keys))
            .reverse()
    }
}
impl std::cmp::PartialOrd for Graph { 
    fn partial_cmp(&self, other: &Graph) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl std::cmp::PartialEq for Graph {
    fn eq(&self, other: &Graph) -> bool {
        self.distance == other.distance &&
        self.taken_keys == other.taken_keys &&
        self.keys == other.keys
    }
}
impl std::cmp::Eq for Graph {}

impl Graph {
    fn from_maze(maze: &Maze) -> Graph {
        let vertices: Vec<(Vertex, Cell)> =
            maze.tiles.iter().filter_map(|(cell, tile)| match tile {
                Tile::Key(c) => Some((Vertex::Key(*c), *cell)),
                Tile::Door(c) => Some((Vertex::Door(*c), *cell)),
                Tile::Wall|Tile::Open => None
            })
            .chain(maze.players.iter().enumerate().map(|(i, cell)| (Vertex::Droid(i as i32 + 1), *cell)))
            .collect();
        let mut keys: Vec<char> = vertices.iter()
            .filter_map(|(v,_)| if let Vertex::Key(c) = v { Some(*c) } else { None }).collect();
        keys.sort();

        let mut edges: Vec<Edge> = Vec::new();
        for (v1, cell1) in vertices {
            for (cell2, tile, dist) in maze.reachable_from(cell1) {
                let v2 = match tile {
                    Tile::Key(c) => Vertex::Key(c),
                    Tile::Door(c) => Vertex::Door(c),
                    Tile::Open => Vertex::Droid(maze.players.iter().position(|p| *p == cell2).unwrap() as i32 + 1),
                    _ => panic!("Cell shouldn't be reachable because it contains nothing")
                };
                match edges.iter().find(|edge| edge.v1 == v2 && edge.v2 == v1) {
                    Some(edge) => assert_eq!(edge.length, dist),
                    None => edges.push(Edge { v1, v2, length: dist })
                }
            }
        }
        Graph {
            edges,
            keys: keys.iter().collect(),
            taken_keys: String::new(),
            distance: 0,
        }
    }

    fn edges_from(&self, vertex: Vertex) -> std::collections::HashMap<Vertex, usize> {
        self.edges.iter().filter_map(|edge|
            if edge.v1 == vertex {
                Some((edge.v2, edge.length))
            } else if edge.v2 == vertex {
                Some((edge.v1, edge.length))
            } else {
                None
            }
        ).collect()
    }

    fn replace_vertex(&mut self, current: Vertex, new: Vertex) {
        for edge in self.edges.iter_mut() {
            if edge.v1 == current { edge.v1 = new }
            if edge.v2 == current { edge.v2 = new }
        }
        self.edges.retain(|edge| edge.v1 != edge.v2);
    }

    fn remove_vertex(&mut self, vertex: Vertex) {
        let connected = self.edges_from(vertex);
        self.edges.retain(|edge| edge.v1 != vertex && edge.v2 != vertex);
        for (&v1, &d1) in connected.iter() {
            for (&v2, &d2) in connected.iter() {
                if v1 != v2 {
                    match self.edges.iter_mut().find(|edge| (edge.v1 == v1 && edge.v2 == v2) || (edge.v1 == v2 && edge.v2 == v1)) {
                        Some(edge) => edge.length = edge.length.min(d1 + d2),
                        None => self.edges.push(Edge { v1, v2, length: d1 + d2 }),
                    }
                }
            }
        }
    }

    fn allowed_moves(&self) -> Vec<(i32, char, usize)> {
        self.edges.iter().filter_map(|edge| {
            match edge {
                Edge { v1: Vertex::Droid(d), v2: Vertex::Key(k), length: l }|
                Edge { v1: Vertex::Key(k), v2: Vertex::Droid(d), length: l } =>
                    Some((*d, *k, *l)),
                _ => None
            }
        }).collect()
    }

    fn children(&self) -> Vec<Graph> {
        let moves = self.allowed_moves();
        let mut result = Vec::with_capacity(moves.len());
        for (droid, key, length) in moves {
            let mut graph = self.clone();
            graph.keys.remove(graph.keys.find(key).unwrap());
            graph.taken_keys.push(key);
            graph.distance += length;
            graph.remove_vertex(Vertex::Droid(droid));
            graph.remove_vertex(Vertex::Door(key));
            graph.replace_vertex(Vertex::Key(key), Vertex::Droid(droid));
            result.push(graph)
        }
        result
    }

    fn footprint(&self) -> String {
        let mut edges: Vec<String> = self.edges.iter().map(|edge| {
            let v1 = format!("{}", edge.v1);
            let v2 = format!("{}", edge.v2);
            format!("{}-{}-{}", (&v1).min(&v2), (&v1).max(&v2), edge.length)
        }).collect();
        edges.sort();
        format!("{} {}", self.keys, edges.join(" "))
    }

    fn solve(self) -> (usize, String) {
        let mut counter = 0;

        let mut queue = std::collections::BinaryHeap::new();
        let mut visited = std::collections::HashMap::new();
        queue.push(self);
        while let Some(graph) = queue.pop() {
            counter += 1;
            if counter % 100000 == 0 {
                println!("Current graph: keys taken {} remaining {} distance {}. Visited {}, in list: {}",
                    graph.taken_keys, graph.keys, graph.distance, visited.len(), queue.len());
            }
            //println!("Visiting {}", graph.taken_keys);

            if graph.keys.is_empty() {
                return (graph.distance, graph.taken_keys)
            } else {
                let footprint = graph.footprint();
                let pruned = match visited.get_mut(&footprint) {
                    None => { visited.insert(footprint, graph.distance); false },
                    Some(dist) if *dist > graph.distance => { *dist = graph.distance; false },
                    Some(_) => true
                };
                if !pruned {
                    for child in graph.children() {
                        queue.push(child);
                    }
                }
            }
        }
        (std::usize::MAX, String::new())
    }
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Vertex::Droid(i) => i.fmt(f),
            Vertex::Key(c) => c.to_ascii_lowercase().fmt(f),
            Vertex::Door(c) => c.to_ascii_uppercase().fmt(f),
        }
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Graph with keys: {}", self.keys)?;
        for edge in self.edges.iter() {
            writeln!(f, "{}--{} len={}", edge.v1, edge.v2, edge.length)?;
        }
        write!(f, "Used keys: {} Starting distance: {}", self.taken_keys, self.distance)
    }
}

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
        {
            let graph = Graph::from_maze(&maze);
            //println!("{}", graph);
            unsafe { SOLVE_CALL_COUNT = 0 }
            let (result, sequence) = graph.solve();
            if result != *output {
                println!("Error for maze:");
                println!("{}", input);
                println!("Expected: {}, result: {}", output, result);
            } else {
                println!("Graph Ok: len {}, sequence is {}", result, sequence);
            }
        }
        if *output != 136
        {
            let (result, sequence) = maze.solve();
            if result != *output {
                println!("Error for maze:");
                println!("{}", input);
                println!("Expected: {}, result: {}", output, result);
            } else {
                println!(" Maze Ok: len {}, sequence is {}, {} calls", result, sequence, unsafe { SOLVE_CALL_COUNT });
            }
        }
    }

    let input = include_str!("input18.txt");
    let maze = Maze::read(std::io::Cursor::new(input));
    let (len, keys) = Graph::from_maze(&maze).solve();
    println!("Shortest path length: {}", len);
    println!("Key sequence: {}", keys);
    //println!("{} pathfinder calls", unsafe { SOLVE_CALL_COUNT });

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
    let (len, keys) = Graph::from_maze(&maze).solve();
    println!("Shortest path length: {}", len);
    println!("Key sequence: {}", keys);
    //println!("{} pathfinder calls", unsafe { SOLVE_CALL_COUNT });
}
