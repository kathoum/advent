use std::io::{BufRead,BufReader};
use std::collections::HashMap;

fn indirect_orbits(body: &str, orbits: &HashMap<String, String>) -> i32 {
    if body == "COM" { 0 } else {
        let parent = orbits.get(body).unwrap();
        1 + indirect_orbits(parent, orbits)
    }
}

fn path_to_root(body: &str, orbits: &HashMap<String, String>) -> Vec<String> {
    let mut path = match orbits.get(body) {
        None => Vec::new(),
        Some(parent) => path_to_root(parent, orbits),
    };
    path.push(body.to_string());
    path
}

fn common_prefix_len(v1: &[String], v2: &[String]) -> usize {
    v1.iter().zip(v2.iter()).take_while(|(x,y)| x == y).count()
}

fn distance(body1: &str, body2: &str, orbits: &HashMap<String, String>) -> usize {
    let path1 = path_to_root(body1, orbits);
    let path2 = path_to_root(body2, orbits);
    path1.len() + path2.len() - 2 * common_prefix_len(&path1, &path2)
}

fn main() -> std::io::Result<()> {
    let _reader = std::io::Cursor::new(
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN");
    
    let filename = "input06.txt";
    let reader = BufReader::new(std::fs::File::open(filename)?);
    let mut orbits = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split(")");
        let parent = line.next().unwrap();
        let child = line.next().unwrap();
        orbits.insert(child.to_string(), parent.to_string());
    }
    let total: i32 = orbits.keys().map(|k| indirect_orbits(k, &orbits)).sum();
    println!("Total indirect orbits = {}", total);
    println!("Jumps from YOU to SAN = {}", distance("YOU", "SAN", &orbits) - 2);
    Ok(())
}
