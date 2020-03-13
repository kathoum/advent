use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Cell(i32, i32);
enum Tile { Wall, Open, OuterPortal(String), InnerPortal(String) }
struct Maze {
    tiles: HashMap<Cell, Tile>
}

fn neighbours(cell: Cell) -> [Cell; 4] {
    let Cell(r,c) = cell;
    [Cell(r-1,c), Cell(r+1,c), Cell(r,c-1), Cell(r,c+1)]
}

impl Maze {
    fn read(reader: impl std::io::Read) -> Maze {
        // Read into a 2D vector of bytes
        let mut matrix: Vec<Vec<u8>> = vec![Vec::new()];
        for ch in reader.bytes() {
            let ch = ch.unwrap();
            if ch == b'\n' {
                matrix.push(Vec::new())
            } else {
                matrix.last_mut().unwrap().push(ch)
            }
        }
        let nrows = matrix.len() as i32;
        let ncols = matrix.iter().map(|v| v.len()).max().unwrap() as i32;
        let is_outer = |r: i32, c: i32| -> bool {
            r <= 3 || c <= 3 || r >= nrows - 3 || c >= ncols - 3
        };
        // Helper to access the 2D vector with bound checks
        let get_at = |r: i32, c: i32| -> char {
            if r >= 0 && c >= 0 {
                if let Some(v) = matrix.get(r as usize) {
                    *v.get(c as usize).unwrap_or(&b' ') as char
                } else { ' ' }
            } else { ' ' }
        };
        let mut tiles = HashMap::new();
        for (row, vec) in matrix.iter().enumerate() {
            for (col, ch) in vec.iter().enumerate() {
                let ch = *ch as char;
                let r = row as i32;
                let c = col as i32;
                let tile = match ch {
                    ' ' => None,
                    '#' => Some(Tile::Wall),
                    '.' => Some(Tile::Open),
                    'A'..='Z' => {
                        let mut s = String::new();
                        if get_at(r-1, c) == '.' { s.push(ch); s.push(get_at(r+1, c)); }
                        if get_at(r, c-1) == '.' { s.push(ch); s.push(get_at(r, c+1)); }
                        if get_at(r+1, c) == '.' { s.push(get_at(r-1, c)); s.push(ch); }
                        if get_at(r, c+1) == '.' { s.push(get_at(r, c-1)); s.push(ch); }
                        if s.len() > 0 {
                            Some(if is_outer(r, c) { Tile::OuterPortal(s) } else { Tile::InnerPortal(s) })
                        } else {
                            None
                        }
                    },
                    _ => panic!(format!("Unexpected char {}", ch))
                };
                if let Some(t) = tile {
                    tiles.insert(Cell(r, c), t);
                }
            }
        }
        Maze { tiles }
    }

    fn find_portal(&self, name: &str) -> Vec<Cell> {
        self.tiles.iter().filter_map(|(cell, tile)|
            match tile {
                Tile::OuterPortal(s)|Tile::InnerPortal(s) if s == name => Some(*cell),
                _ => None
            }
        ).collect()
    }

    fn cell_next_to_portal(&self, portal_cell: &Cell) -> Cell {
        for cell in neighbours(*portal_cell).iter() {
            if let Some(Tile::Open) = self.tiles.get(&cell) {
                return *cell;
            }
        }
        panic!(format!("({},{}) is not a portal", portal_cell.0, portal_cell.1))
    }

    fn find_portal_exit(&self, name: &str, entrance: &Cell) -> Option<Cell> {
        let other_portal = self.find_portal(name).into_iter().find(|p| p != entrance);
        other_portal.map(|p| self.cell_next_to_portal(&p))
    }

    fn find_path(&self, from: &str, to: &str) -> Option<usize> {
        let mut tile_queue = VecDeque::new();
        let mut visited = HashSet::new();

        let start = self.find_portal(from);
        assert_eq!(1, start.len());
        let start = self.cell_next_to_portal(&start[0]);
        tile_queue.push_back((start, 0usize));

        while let Some((c, distance)) = tile_queue.pop_front() {
            visited.insert(c);
            for cell in neighbours(c).iter() {
                if !visited.contains(&cell) {
                    match self.tiles.get(&cell) {
                        Some(Tile::Wall) => (),
                        Some(Tile::Open) => tile_queue.push_back((*cell, distance + 1)),
                        Some(Tile::OuterPortal(name))|Some(Tile::InnerPortal(name)) => {
                            if name == to {
                                return Some(distance);
                            } else if let Some(exit) = self.find_portal_exit(name, cell) {
                                tile_queue.push_back((exit, distance + 1))
                            }
                        },
                        None => panic!("Out of maze!")
                    }
                }
            }
        }
        None
    }

    fn find_path_recursive(&self, from: &str, to: &str) -> Option<usize> {
        let mut tile_queue = VecDeque::new();
        let mut visited = HashSet::new();

        let start = self.find_portal(from);
        assert_eq!(1, start.len());
        let start = self.cell_next_to_portal(&start[0]);
        tile_queue.push_back((start, 0usize, 0usize));

        while let Some((c, level, distance)) = tile_queue.pop_front() {
            visited.insert((c, level));
            for cell in neighbours(c).iter() {
                if !visited.contains(&(*cell, level)) {
                    match self.tiles.get(&cell) {
                        Some(Tile::Wall) => (),
                        Some(Tile::Open) => tile_queue.push_back((*cell, level, distance + 1)),
                        Some(Tile::OuterPortal(name)) => {
                            match level {
                                0 => if name == to {
                                    return Some(distance);
                                },
                                _ => {
                                    if name != from && name != to {
                                        let exit = self.find_portal_exit(name, cell).unwrap();
                                        tile_queue.push_back((exit, level - 1, distance + 1));
                                    }
                                }
                            };
                        },
                        Some(Tile::InnerPortal(name)) => {
                            assert_ne!(name, from);
                            assert_ne!(name, to);
                            let exit = self.find_portal_exit(name, cell).unwrap();
                            tile_queue.push_back((exit, level + 1, distance + 1));
                        },
                        None => panic!("Out of maze!")
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let test = "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z";
    let maze = Maze::read(std::io::Cursor::new(test));
    let dist = maze.find_path("AA", "ZZ");
    println!("Test 1a: result {:?}. {}", dist, if dist == Some(23) { "Ok" } else { "Error" });
    let dist = maze.find_path_recursive("AA", "ZZ");
    println!("Test 1b: result {:?}. {}", dist, if dist == Some(26) { "Ok" } else { "Error" });

    let test = "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               
        ";
    let maze = Maze::read(std::io::Cursor::new(test));
    let dist = maze.find_path("AA", "ZZ");
    println!("Test 2a: result {:?}. {}", dist, if dist == Some(58) { "Ok" } else { "Error" });
    let dist = maze.find_path_recursive("ZZ", "AA");
    println!("Test 2b: result {:?}. {}", dist, if dist == None { "Ok" } else { "Error" });

    let test = "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
        ";
    let maze = Maze::read(std::io::Cursor::new(test));
    let dist = maze.find_path_recursive("AA", "ZZ");
    println!("Test 3b: result {:?}. {}", dist, if dist == Some(396) { "Ok" } else { "Error" });

    let input = include_str!("input20.txt");
    let maze = Maze::read(std::io::Cursor::new(input));
    let dist = maze.find_path("AA", "ZZ");
    match dist {
        Some(dist) => println!("Distance is {}", dist),
        None => println!("Unreachable")
    };
    let dist = maze.find_path_recursive("AA", "ZZ");
    match dist {
        Some(dist) => println!("Recursive: distance is {}", dist),
        None => println!("Recursive: unreachable")
    };
}
